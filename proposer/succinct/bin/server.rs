use alloy_primitives::hex;
use axum::{
    extract::{DefaultBodyLimit, Path},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use base64::{engine::general_purpose, Engine as _};
use log::info;
use op_succinct_client_utils::boot::BootInfoStruct;
use op_succinct_host_utils::{
    fetcher::{CacheMode, OPSuccinctDataFetcher},
    get_agg_proof_stdin, get_proof_stdin,
    witnessgen::WitnessGenExecutor,
    ProgramType,
};
use serde::{Deserialize, Deserializer, Serialize};
use sp1_sdk::{
    network::{
        client::NetworkClient,
        proto::network::{ProofMode, ProofStatus as SP1ProofStatus},
    },
    utils, NetworkProverV1, Prover, SP1Proof, SP1ProofWithPublicValues,
};
use std::{env, time::Duration};
use tower_http::limit::RequestBodyLimitLayer;

pub const MULTI_BLOCK_ELF: &[u8] = include_bytes!("../../../elf/range-elf");
pub const AGG_ELF: &[u8] = include_bytes!("../../../elf/aggregation-elf");

#[derive(Deserialize, Serialize, Debug)]
struct SpanProofRequest {
    start: u64,
    end: u64,
}

#[derive(Deserialize, Serialize, Debug)]
struct AggProofRequest {
    #[serde(deserialize_with = "deserialize_base64_vec")]
    subproofs: Vec<Vec<u8>>,
    head: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ProofResponse {
    proof_id: String,
}

#[derive(Serialize)]
struct ProofStatus {
    status: String,
    proof: Vec<u8>,
}

#[tokio::main]
async fn main() {
    utils::setup_logger();

    dotenv::dotenv().ok();

    env::set_var("SKIP_SIMULATION", "true");

    let app = Router::new()
        .route("/request_span_proof", post(request_span_proof))
        .route("/request_agg_proof", post(request_agg_proof))
        .route("/status/:proof_id", get(get_proof_status))
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(102400 * 1024 * 1024));

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    info!("Server listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

/// Request a proof for a span of blocks.
async fn request_span_proof(
    Json(payload): Json<SpanProofRequest>,
) -> Result<(StatusCode, Json<ProofResponse>), AppError> {
    info!("Received span proof request: {:?}", payload);
    let data_fetcher = OPSuccinctDataFetcher::default();

    let host_cli = data_fetcher
        .get_host_cli_args(
            payload.start,
            payload.end,
            ProgramType::Multi,
            CacheMode::DeleteCache,
        )
        .await?;

    // Start the server and native client with a timeout.
    // Note: Ideally, the server should call out to a separate process that executes the native
    // host, and return an ID that the client can poll on to check if the proof was submitted.
    let mut witnessgen_executor = WitnessGenExecutor::default();
    witnessgen_executor.spawn_witnessgen(&host_cli).await?;
    // Log any errors from running the witness generation process.
    let res = witnessgen_executor.flush().await;
    if let Err(e) = res {
        log::error!("Failed to generate witness: {}", e);
        return Err(AppError(anyhow::anyhow!(
            "Failed to generate witness: {}",
            e
        )));
    }

    let sp1_stdin = get_proof_stdin(&host_cli)?;

    let prover = NetworkProverV1::new();
    let res = prover
        .request_proof(MULTI_BLOCK_ELF, sp1_stdin, ProofMode::Compressed)
        .await;

    // Check if error, otherwise get proof ID.
    let proof_id = match res {
        Ok(proof_id) => proof_id,
        Err(e) => {
            log::error!("Failed to request proof: {}", e);
            return Err(AppError(anyhow::anyhow!("Failed to request proof: {}", e)));
        }
    };

    Ok((StatusCode::OK, Json(ProofResponse { proof_id })))
}

/// Request an aggregation proof for a set of subproofs.
async fn request_agg_proof(
    Json(payload): Json<AggProofRequest>,
) -> Result<(StatusCode, Json<ProofResponse>), AppError> {
    info!("Received agg proof request");
    let mut proofs_with_pv: Vec<SP1ProofWithPublicValues> = payload
        .subproofs
        .iter()
        .map(|sp| bincode::deserialize(sp).unwrap())
        .collect();

    let boot_infos: Vec<BootInfoStruct> = proofs_with_pv
        .iter_mut()
        .map(|proof| proof.public_values.read())
        .collect();

    let proofs: Vec<SP1Proof> = proofs_with_pv
        .iter_mut()
        .map(|proof| proof.proof.clone())
        .collect();

    let l1_head_bytes = hex::decode(
        payload
            .head
            .strip_prefix("0x")
            .expect("Invalid L1 head, no 0x prefix."),
    )?;
    let l1_head: [u8; 32] = l1_head_bytes.try_into().unwrap();

    let fetcher = OPSuccinctDataFetcher::default();
    let headers = fetcher
        .get_header_preimages(&boot_infos, l1_head.into())
        .await?;

    let prover = NetworkProverV1::new();
    let (_, vkey) = prover.setup(MULTI_BLOCK_ELF);

    let stdin = get_agg_proof_stdin(proofs, boot_infos, headers, &vkey, l1_head.into()).unwrap();

    // Set simulation to true on aggregation proofs as they're relatively small.
    env::set_var("SKIP_SIMULATION", "false");
    let proof_id = prover
        .request_proof(AGG_ELF, stdin, ProofMode::Groth16)
        .await?;
    env::set_var("SKIP_SIMULATION", "true");

    Ok((StatusCode::OK, Json(ProofResponse { proof_id })))
}

/// Get the status of a proof.
async fn get_proof_status(
    Path(proof_id): Path<String>,
) -> Result<(StatusCode, Json<ProofStatus>), AppError> {
    info!("Received proof status request: {:?}", proof_id);
    let private_key = env::var("SP1_PRIVATE_KEY")?;

    let client = NetworkClient::new(&private_key);

    // Time out this request if it takes too long.
    let timeout = Duration::from_secs(10);
    let (status, maybe_proof) = tokio::time::timeout(timeout, client.get_proof_status(&proof_id))
        .await
        .map_err(|_| AppError(anyhow::anyhow!("Proof status request timed out")))?
        .map_err(|e| AppError(anyhow::anyhow!("Failed to get proof status: {}", e)))?;

    let status: SP1ProofStatus = SP1ProofStatus::try_from(status.status)?;
    if status == SP1ProofStatus::ProofFulfilled {
        let proof: SP1ProofWithPublicValues = maybe_proof.unwrap();

        match proof.proof {
            SP1Proof::Compressed(_) => {
                // If it's a compressed proof, we need to serialize the entire struct with bincode.
                // Note: We're re-serializing the entire struct with bincode here, but this is fine
                // because we're on localhost and the size of the struct is small.
                let proof_bytes = bincode::serialize(&proof).unwrap();
                return Ok((
                    StatusCode::OK,
                    Json(ProofStatus {
                        status: status.as_str_name().to_string(),
                        proof: proof_bytes,
                    }),
                ));
            }
            SP1Proof::Groth16(_) => {
                // If it's a groth16 proof, we need to get the proof bytes that we put on-chain.
                let proof_bytes = proof.bytes();
                return Ok((
                    StatusCode::OK,
                    Json(ProofStatus {
                        status: status.as_str_name().to_string(),
                        proof: proof_bytes,
                    }),
                ));
            }
            SP1Proof::Plonk(_) => {
                // If it's a plonk proof, we need to get the proof bytes that we put on-chain.
                let proof_bytes = proof.bytes();
                return Ok((
                    StatusCode::OK,
                    Json(ProofStatus {
                        status: status.as_str_name().to_string(),
                        proof: proof_bytes,
                    }),
                ));
            }
            _ => (),
        }
    }
    Ok((
        StatusCode::OK,
        Json(ProofStatus {
            status: status.as_str_name().to_string(),
            proof: vec![],
        }),
    ))
}

pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", self.0)).into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

/// Deserialize a vector of base64 strings into a vector of vectors of bytes. Go serializes
/// the subproofs as base64 strings.
fn deserialize_base64_vec<'de, D>(deserializer: D) -> Result<Vec<Vec<u8>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Vec<String> = Deserialize::deserialize(deserializer)?;
    s.into_iter()
        .map(|base64_str| {
            general_purpose::STANDARD
                .decode(base64_str)
                .map_err(serde::de::Error::custom)
        })
        .collect()
}
