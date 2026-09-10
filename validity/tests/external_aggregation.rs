//! Run through `just test-e2e-sysgo ./e2e/validity/agglayer/...` in `tests/`.
//! Sysgo owns the local nodes, database, and actual proposer process.
#![cfg(feature = "agglayer")]

use std::{future::Future, time::Duration};

use alloy_eips::BlockNumberOrTag;
use alloy_sol_types::SolValue;
use anyhow::{ensure, Context, Result};
use bincode::Options;
use op_succinct_client_utils::types::AggregationOutputs;
use op_succinct_host_utils::fetcher::OPSuccinctDataFetcher;
use op_succinct_validity::{
    grpc::proofs::{proofs_client::ProofsClient, AggProofRequest, GetMockProofRequest},
    DriverDBClient, OPSuccinctRequest, RequestMode, RequestStatus, RequestType,
};
use sp1_sdk::SP1ProofWithPublicValues;
use sqlx::PgPool;
use tokio::time::{sleep, timeout};
use tonic::{transport::Channel, Code};

async fn wait_for<T, F, Fut>(description: &str, mut check: F) -> Result<T>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<Option<T>>>,
{
    timeout(Duration::from_secs(180), async {
        loop {
            if let Some(value) = check().await? {
                return Ok(value);
            }
            sleep(Duration::from_millis(100)).await;
        }
    })
    .await
    .with_context(|| format!("Timed out waiting for {description}"))?
}

struct Fixture {
    client: ProofsClient<Channel>,
    db: DriverDBClient,
    fetcher: OPSuccinctDataFetcher,
    range: OPSuccinctRequest,
}

impl Fixture {
    async fn connect() -> Result<Self> {
        dotenv::from_filename(std::env::var("AGGLAYER_TEST_ENV_FILE")?)?;
        ensure!(std::env::var("OP_SUCCINCT_MOCK")? == "true", "Requires mock proving");
        let endpoint = format!("http://{}", std::env::var("GRPC_ADDRESS")?);
        let database = std::env::var("DATABASE_URL")?;
        // This test deliberately changes capacity rows. Never point it at a shared deployment.
        for value in [
            endpoint.clone(),
            database.clone(),
            std::env::var("L1_RPC")?,
            std::env::var("L2_RPC")?,
            std::env::var("L2_NODE_RPC")?,
        ] {
            let url = reqwest::Url::parse(&value)?;
            ensure!(
                matches!(url.host_str(), Some("127.0.0.1" | "localhost" | "[::1]")),
                "Requires loopback-only Sysgo services"
            );
        }
        let channel = Channel::from_shared(endpoint)?.timeout(Duration::from_secs(150));
        let client = wait_for("proposer RPC startup", || async {
            Ok(channel.connect().await.ok().map(ProofsClient::new))
        })
        .await?;
        let db = DriverDBClient { pool: PgPool::connect(&database).await? };
        let range = wait_for("a completed range proof", || async {
            Ok(sqlx::query_as::<_, OPSuccinctRequest>(
                "SELECT * FROM requests WHERE req_type = 0 AND status = 4 AND start_block = 1 ORDER BY id LIMIT 1"
            ).fetch_optional(&db.pool).await?)
        }).await?;
        ensure!(range.mode == RequestMode::Mock, "Requires mock range proofs");
        Ok(Self { client, db, fetcher: OPSuccinctDataFetcher::new(), range })
    }

    async fn request(&self) -> Result<AggProofRequest> {
        wait_for("a safe L1 checkpoint covering the range", || async {
            let header = self.fetcher.get_l1_header(BlockNumberOrTag::Latest.into()).await?;
            // Match the external API's 20-block safe-head lookback.
            let safe = self
                .fetcher
                .get_l2_safe_head_from_l1_block_number(header.number.saturating_sub(20))
                .await?;
            Ok((safe >= self.range.end_block as u64).then(|| AggProofRequest {
                last_proven_block: self.range.start_block as u64,
                requested_end_block: self.range.end_block as u64,
                l1_block_number: header.number,
                l1_block_hash: header.hash_slow().to_string(),
            }))
        })
        .await
    }

    async fn last_id(&self) -> Result<i64> {
        Ok(sqlx::query_scalar("SELECT COALESCE(MAX(id), 0) FROM requests")
            .fetch_one(&self.db.pool)
            .await?)
    }

    async fn new_aggregation(&self, after: i64) -> Result<OPSuccinctRequest> {
        wait_for("the external aggregation row", || async {
            Ok(sqlx::query_as::<_, OPSuccinctRequest>(
                "SELECT * FROM requests WHERE id > $1 AND req_type = 1 ORDER BY id LIMIT 1",
            )
            .bind(after)
            .fetch_optional(&self.db.pool)
            .await?)
        })
        .await
    }

    async fn assert_complete(&self, id: i64, request: &AggProofRequest) -> Result<()> {
        let row = wait_for("aggregation completion", || async {
            let row = self.db.fetch_request(id).await?;
            ensure!(
                !matches!(
                    row.status,
                    RequestStatus::Failed | RequestStatus::Cancelled | RequestStatus::Invalidated
                ),
                "Aggregation {id} ended as {:?}",
                row.status
            );
            Ok((row.status == RequestStatus::Complete).then_some(row))
        })
        .await?;
        assert_eq!(row.start_block as u64, request.last_proven_block);
        assert_eq!(row.end_block as u64, request.requested_end_block);
        assert_eq!(row.mode, RequestMode::Mock);
        assert_eq!(row.req_type, RequestType::Aggregation);
        let bytes = self
            .client
            .clone()
            .get_mock_proof(GetMockProofRequest { proof_id: id })
            .await?
            .into_inner()
            .proof;
        assert_eq!(row.proof.as_deref(), Some(bytes.as_slice()));
        let proof: SP1ProofWithPublicValues = bincode::DefaultOptions::new()
            .with_big_endian()
            .with_fixint_encoding()
            .deserialize(&bytes)?;
        let outputs = AggregationOutputs::abi_decode(proof.public_values.as_slice())?;
        assert_eq!(outputs.l2BlockNumber, request.requested_end_block);
        assert_eq!(outputs.l1Head.to_string(), request.l1_block_hash);
        assert_eq!(outputs.rollupConfigHash.as_slice(), row.rollup_config_hash);
        assert_eq!(outputs.multiBlockVKey.as_slice(), row.range_vkey_commitment);
        assert_eq!(Some(outputs.proverAddress.as_slice()), row.prover_address.as_deref());
        let start = self.fetcher.get_l2_output_at_block(request.last_proven_block).await?;
        let end = self.fetcher.get_l2_output_at_block(request.requested_end_block).await?;
        assert_eq!(outputs.l2PreRoot, start.output_root);
        assert_eq!(outputs.l2PostRoot, end.output_root);
        Ok(())
    }

    async fn submit(&self, request: &AggProofRequest) -> Result<i64> {
        let response = self.client.clone().request_agg_proof(request.clone()).await?.into_inner();
        assert_eq!(response.last_proven_block, request.last_proven_block);
        assert_eq!(response.end_block, request.requested_end_block);
        let bytes: [u8; 32] =
            response.proof_request_id.try_into().expect("32-byte mock request ID");
        assert_eq!(&bytes[..24], &[0; 24]);
        let id = i64::from_be_bytes(bytes[24..].try_into()?);
        self.assert_complete(id, request).await?;
        Ok(id)
    }
}

#[tokio::test]
#[ignore = "requires the Sysgo external aggregation fixture"]
async fn external_aggregation_completes() -> Result<()> {
    let fixture = Fixture::connect().await?;
    fixture.submit(&fixture.request().await?).await?;
    Ok(())
}

#[tokio::test]
#[ignore = "requires the Sysgo external aggregation fixture"]
async fn external_aggregation_rejects_full_capacity() -> Result<()> {
    let fixture = Fixture::connect().await?;
    let request = fixture.request().await?;
    let capacity: usize = std::env::var("MAX_CONCURRENT_PROOF_REQUESTS")?.parse()?;
    ensure!(capacity > 0, "Capacity must be bounded");
    let mut occupied = Vec::new();
    // Mock mode does not poll remote Prove rows. Occupy capacity without network requests.
    let mut active = fixture.range.clone();
    active.req_type = RequestType::Aggregation;
    active.status = RequestStatus::Prove;
    active.proof = None;
    for _ in 0..capacity {
        occupied.push(fixture.db.insert_request(&active).await?);
    }
    let before = fixture.last_id().await?;
    let response = fixture.client.clone().request_agg_proof(request.clone()).await;
    for id in occupied {
        fixture.db.update_request_status(id, RequestStatus::Failed).await?;
    }
    let error = response.unwrap_err();
    assert_eq!(error.code(), Code::ResourceExhausted);
    let rejected = fixture.new_aggregation(before).await?;
    assert_eq!(rejected.status, RequestStatus::Cancelled);
    assert!(rejected.proof.is_none());
    assert!(rejected.witnessgen_duration.is_none());
    let resumed = fixture.submit(&request).await?;
    assert_ne!(resumed, rejected.id);
    Ok(())
}

#[tokio::test]
#[ignore = "requires the Sysgo external aggregation fixture"]
async fn external_aggregation_survives_disconnect() -> Result<()> {
    let fixture = Fixture::connect().await?;
    let request = fixture.request().await?;
    let before = fixture.last_id().await?;
    let mut admission = fixture.db.pool.begin().await?;
    sqlx::query("SELECT pg_advisory_xact_lock(hashtextextended($1, 0))")
        .bind(format!(
            "proof-admission:{}:{}",
            fixture.range.l1_chain_id, fixture.range.l2_chain_id
        ))
        .execute(&mut *admission)
        .await?;
    let mut client = fixture.client.clone();
    let outgoing = request.clone();
    let call = tokio::spawn(async move { client.request_agg_proof(outgoing).await });
    let accepted = fixture.new_aggregation(before).await?;

    // Hand off from the admission lock to a row lock. A blocked UPDATE proves the
    // server task is executing, not merely that the handler inserted a queued row.
    let mut row_lock = fixture.db.pool.begin().await?;
    let blocker: i32 =
        sqlx::query_scalar("SELECT pg_backend_pid()").fetch_one(&mut *row_lock).await?;
    sqlx::query("SELECT id FROM requests WHERE id = $1 FOR UPDATE")
        .bind(accepted.id)
        .execute(&mut *row_lock)
        .await?;
    admission.commit().await?;
    wait_for("the proof task blocked on its row", || async {
        let blocked: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM pg_stat_activity WHERE $1 = ANY(pg_blocking_pids(pid)))",
        )
        .bind(blocker)
        .fetch_one(&fixture.db.pool)
        .await?;
        Ok(blocked.then_some(()))
    })
    .await?;
    assert!(!call.is_finished(), "RPC must still be pending when the caller disconnects");
    call.abort();
    assert!(call.await.unwrap_err().is_cancelled());
    row_lock.commit().await?;

    fixture.assert_complete(accepted.id, &request).await?;
    let next = fixture.submit(&request).await?;
    assert_ne!(next, accepted.id);
    Ok(())
}
