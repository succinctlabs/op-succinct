use axum::{
    extract::Path,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{env, fs};

use host_utils::{fetcher::SP1KonaDataFetcher, get_sp1_stdin, ProgramType};
use kona_host::start_server_and_native_client;
use sp1_sdk::{
    network::client::NetworkClient,
    proto::network::{ProofMode, ProofStatus as SP1ProofStatus},
    NetworkProver, Prover,
};

pub const MULTI_BLOCK_ELF: &[u8] = include_bytes!("../../elf/validity-client-elf");

#[derive(Deserialize)]
struct ProofRequest {
    start: u64,
    end: u64,
}

#[derive(Serialize)]
struct ProofStatus {
    status: &'static str,
    bytestring: Vec<u8>,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/request_proof", post(request_proof))
        .route("/status/:proof_id", get(get_proof_status));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn request_proof(Json(payload): Json<ProofRequest>) -> (StatusCode, String) {
    dotenv::dotenv().ok();

    let data_fetcher = SP1KonaDataFetcher {
        l2_rpc: env::var("CLABBY_RPC_L2").expect("CLABBY_RPC_L2 is not set."),
        ..Default::default()
    };

    let host_cli = data_fetcher
        .get_host_cli_args(payload.start, payload.end, 0, ProgramType::Multi)
        .await
        .unwrap();

    let data_dir = host_cli
        .data_dir
        .clone()
        .expect("Data directory is not set.");

    // Overwrite existing data directory.
    fs::create_dir_all(&data_dir).unwrap();

    // Start the server and native client.
    start_server_and_native_client(host_cli.clone())
        .await
        .unwrap();

    let sp1_stdin = get_sp1_stdin(&host_cli).unwrap();

    let prover = NetworkProver::new();
    prover.setup(MULTI_BLOCK_ELF);

    let proof_id = prover
        .request_proof(MULTI_BLOCK_ELF, sp1_stdin, ProofMode::Compressed)
        .await
        .unwrap();

    (StatusCode::OK, proof_id)
}

async fn get_proof_status(Path(proof_id): Path<String>) -> Json<ProofStatus> {
    dotenv::dotenv().ok();
    let private_key = env::var("SP1_PRIVATE_KEY").unwrap();

    let client = NetworkClient::new(&private_key);
    let (status, maybe_proof) = client.get_proof_status(&proof_id).await.unwrap();

    let status = SP1ProofStatus::try_from(status.status).unwrap();
    let proof = maybe_proof.unwrap_or(vec![]);

    Json(ProofStatus {
        status: status.as_str_name(),
        bytestring: proof,
    })
}
