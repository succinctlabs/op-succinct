use anyhow::Result;
use chrono::{Local, NaiveDateTime};
use serde_json::Value;
use sqlx::types::BigDecimal;
use sqlx::Error;
use sqlx::{postgres::PgQueryResult, FromRow, PgPool};

use crate::CommitmentConfig;

#[repr(i16)]
#[derive(sqlx::Type, Debug, Copy, Clone, PartialEq, Eq, Default)]
#[sqlx(type_name = "smallint")]
pub enum RequestStatus {
    #[default]
    Unrequested = 0,
    WitnessGeneration = 1,
    Execution = 2,
    Prove = 3,
    Complete = 4,
    Relayed = 5,
    Failed = 6,
    Cancelled = 7,
}

#[repr(i16)]
#[derive(sqlx::Type, Debug, Copy, Clone, PartialEq, Eq, Default)]
#[sqlx(type_name = "smallint")]
pub enum RequestType {
    #[default]
    Range = 0,
    Aggregation = 1,
}

#[repr(i16)]
#[derive(sqlx::Type, Debug, Copy, Clone, PartialEq, Eq, Default)]
#[sqlx(type_name = "smallint")]
pub enum RequestMode {
    #[default]
    Real = 0,
    Mock = 1,
}

#[derive(FromRow, Debug, Default, Clone)]
pub struct OPSuccinctRequest {
    pub id: i64,
    pub status: RequestStatus,
    pub req_type: RequestType,
    pub mode: RequestMode,
    pub start_block: i64,
    pub end_block: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub proof_request_id: Option<[u8; 32]>,
    pub checkpointed_l1_block_number: Option<i64>,
    pub checkpointed_l1_block_hash: Option<[u8; 32]>,
    pub execution_statistics: Value,
    pub witnessgen_duration_secs: Option<i64>,
    pub execution_duration_secs: Option<i64>,
    pub prove_duration_secs: Option<i64>,
    pub range_vkey_commitment: [u8; 32],
    pub aggregation_vkey_hash: Option<[u8; 32]>,
    pub rollup_config_hash: [u8; 32],
    pub relay_tx_hash: Option<[u8; 32]>,
    pub proof: Option<Vec<u8>>,
}

impl OPSuccinctRequest {
    pub fn new(
        status: RequestStatus,
        req_type: RequestType,
        mode: RequestMode,
        start_block: i64,
        end_block: i64,
        range_vkey_commitment: [u8; 32],
        rollup_config_hash: [u8; 32],
    ) -> Self {
        let now = Local::now().naive_local();
        Self {
            id: 0,
            status,
            req_type,
            mode,
            start_block,
            end_block,
            created_at: now,
            updated_at: now,
            proof_request_id: None,
            checkpointed_l1_block_number: None,
            checkpointed_l1_block_hash: None,
            execution_statistics: serde_json::Value::Null,
            witnessgen_duration_secs: None,
            execution_duration_secs: None,
            prove_duration_secs: None,
            range_vkey_commitment,
            aggregation_vkey_hash: None,
            rollup_config_hash,
            relay_tx_hash: None,
            proof: None,
        }
    }
}

#[derive(FromRow, Debug)]
pub struct EthMetrics {
    pub nb_transactions: i64,
    pub eth_gas_used: i64,
    // sqlx doesn't support u128, use BigDecimal instead for fees.
    // Fees are in wei, can be greater than 9 * 10^18.
    pub l1_fees: BigDecimal,
    pub tx_fees: BigDecimal,
}

pub struct DriverDBClient {
    pub pool: PgPool,
}

impl DriverDBClient {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = PgPool::connect(database_url).await?;

        // Run migrations.
        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok(DriverDBClient { pool })
    }

    pub async fn insert_request(&self, req: &OPSuccinctRequest) -> Result<PgQueryResult, Error> {
        sqlx::query(
            r#"
            INSERT INTO requests (
                status,
                req_type,
                mode,
                start_block,
                end_block,
                created_at,
                updated_at,
                proof_request_id,
                checkpointed_l1_block_number,
                checkpointed_l1_block_hash,
                execution_statistics,
                witnessgen_duration,
                execution_duration,
                prove_duration,
                range_vkey_commitment,
                aggregation_vkey,
                rollup_config_hash,
                relay_tx_hash,
                proof
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19
            )
            "#,
        )
        .bind(req.status as i16)
        .bind(req.req_type as i16)
        .bind(req.mode as i16)
        .bind(req.start_block as i64)
        .bind(req.end_block as i64)
        .bind(req.created_at)
        .bind(req.updated_at)
        .bind(req.proof_request_id.as_ref().map(|arr| &arr[..]))
        .bind(req.checkpointed_l1_block_number.map(|n| n as i64))
        .bind(req.checkpointed_l1_block_hash.as_ref().map(|arr| &arr[..]))
        .bind(&req.execution_statistics)
        // Storing durations in seconds.
        .bind(req.witnessgen_duration_secs)
        .bind(req.execution_duration_secs)
        .bind(req.prove_duration_secs)
        .bind(&req.range_vkey_commitment[..])
        .bind(req.aggregation_vkey_hash.as_ref().map(|arr| &arr[..]))
        .bind(&req.rollup_config_hash[..])
        .bind(req.relay_tx_hash.as_ref())
        .bind(req.proof.as_ref())
        .execute(&self.pool)
        .await
    }

    pub async fn insert_eth_metrics(&self, metrics: &EthMetrics) -> Result<PgQueryResult, Error> {
        sqlx::query(
            r#"
            INSERT INTO eth_metrics (
                nb_transactions,
                eth_gas_used,
                l1_fees,
                tx_fees
            ) VALUES ($1, $2, $3, $4)
            "#,
        )
        .bind(metrics.nb_transactions)
        .bind(metrics.eth_gas_used.clone())
        .bind(metrics.l1_fees.clone())
        .bind(metrics.tx_fees.clone())
        .execute(&self.pool)
        .await
    }

    /// Fetch all requests from the database.
    pub async fn fetch_requests(&self) -> Result<Vec<OPSuccinctRequest>, Error> {
        let requests = sqlx::query_as::<_, OPSuccinctRequest>("SELECT * FROM requests")
            .fetch_all(&self.pool)
            .await?;
        Ok(requests)
    }

    /// Fetch all Ethereum metrics from the database.
    pub async fn fetch_eth_metrics(&self) -> Result<Vec<EthMetrics>, Error> {
        let metrics = sqlx::query_as::<_, EthMetrics>("SELECT * FROM eth_metrics")
            .fetch_all(&self.pool)
            .await?;
        Ok(metrics)
    }

    /// Fetch all requests with a specific block range and status FAILED_RETRYABLE or FAILED_FATAL.
    ///
    /// Checks that the request has the same range vkey commitment and rollup config hash as the commitment.
    pub async fn fetch_failed_requests_by_block_range(
        &self,
        start_block: i64,
        end_block: i64,
        commitment: &CommitmentConfig,
    ) -> Result<Vec<OPSuccinctRequest>, Error> {
        let requests = sqlx::query_as::<_, OPSuccinctRequest>(
            "SELECT * FROM requests WHERE start_block = $1 AND end_block = $2 AND (status = $3 OR status = $4) AND range_vkey_commitment = $5 AND rollup_config_hash = $6",
        )
        .bind(start_block)
        .bind(end_block)
        .bind(RequestStatus::Failed as i16)
        .bind(RequestStatus::Cancelled as i16)
        .bind(&commitment.range_vkey_commitment[..])
        .bind(&commitment.rollup_config_hash[..])
        .fetch_all(&self.pool)
        .await?;
        Ok(requests)
    }

    /// Fetch the highest block number of a request with one of the given statuses and commitment.
    pub async fn fetch_highest_request_with_statuses_and_commitment(
        &self,
        statuses: &[RequestStatus],
        commitment: &CommitmentConfig,
    ) -> Result<Option<OPSuccinctRequest>, Error> {
        let request = sqlx::query_as::<_, OPSuccinctRequest>(
            "SELECT * FROM requests WHERE range_vkey_commitment = $1 AND rollup_config_hash = $2 AND status = ANY($3) ORDER BY start_block DESC LIMIT 1",
        )
        .bind(&commitment.range_vkey_commitment[..])
        .bind(&commitment.rollup_config_hash[..])
        .bind(&statuses[..])
        .fetch_optional(&self.pool)
        .await?;
        Ok(request)
    }

    /// Fetch the highest block number of a request with one of the given statuses.
    pub async fn fetch_highest_request_with_statuses(
        &self,
        statuses: &[RequestStatus],
    ) -> Result<Option<OPSuccinctRequest>, Error> {
        let request = sqlx::query_as::<_, OPSuccinctRequest>(
            "SELECT * FROM requests WHERE status = ANY($1) ORDER BY start_block DESC LIMIT 1",
        )
        .bind(&statuses[..])
        .fetch_optional(&self.pool)
        .await?;
        Ok(request)
    }

    /// Fetch all requests that have one of the given statuses.
    pub async fn fetch_requests_by_statuses(
        &self,
        statuses: &[RequestStatus],
        commitment: &CommitmentConfig,
    ) -> Result<Vec<OPSuccinctRequest>, Error> {
        let status_values: Vec<i16> = statuses.iter().map(|s| *s as i16).collect();
        let query = sqlx::query_as::<_, OPSuccinctRequest>(
            "SELECT * FROM requests WHERE range_vkey_commitment = $1 AND rollup_config_hash = $2 AND status = ANY($3)"
        )
        .bind(&commitment.range_vkey_commitment[..])
        .bind(&commitment.rollup_config_hash[..])
        .bind(&status_values[..])
        .fetch_all(&self.pool)
        .await?;
        Ok(query)
    }

    /// Fetch all requests with a specific status from the database.
    pub async fn fetch_requests_by_status(
        &self,
        status: RequestStatus,
        commitment: &CommitmentConfig,
    ) -> Result<Vec<OPSuccinctRequest>, Error> {
        let requests = sqlx::query_as::<_, OPSuccinctRequest>(
            "SELECT * FROM requests WHERE range_vkey_commitment = $1 AND rollup_config_hash = $2 AND status = $3"
        )
        .bind(&commitment.range_vkey_commitment[..])
        .bind(&commitment.rollup_config_hash[..])
        .bind(status as i16)
        .fetch_all(&self.pool)
        .await?;
        Ok(requests)
    }

    /// Get the consecutive range proofs for a given start block and end block that are complete with the same range vkey commitment.
    pub async fn get_consecutive_range_proofs(
        &self,
        start_block: i64,
        end_block: i64,
        commitment: &CommitmentConfig,
    ) -> Result<Vec<OPSuccinctRequest>, Error> {
        let requests = sqlx::query_as::<_, OPSuccinctRequest>(
            "SELECT * FROM requests WHERE range_vkey_commitment = $1 AND rollup_config_hash = $2 AND status = $3 AND req_type = $4 AND start_block >= $5 AND end_block <= $6 ORDER BY start_block ASC"
        )
        .bind(&commitment.range_vkey_commitment[..])
        .bind(&commitment.rollup_config_hash[..])
        .bind(RequestStatus::Complete as i16)
        .bind(RequestType::Range as i16)
        .bind(start_block)
        .bind(end_block)
        .fetch_all(&self.pool)
        .await?;
        Ok(requests)
    }

    /// Fetch all non-failed AGG proofs with the same start block, range vkey commitment, and aggregation vkey.
    ///
    /// TODO: Confirm this works
    pub async fn fetch_active_agg_proofs(
        &self,
        start_block: i64,
        commitment: &CommitmentConfig,
    ) -> Result<Vec<OPSuccinctRequest>, Error> {
        let requests = sqlx::query_as::<_, OPSuccinctRequest>(
            "SELECT * FROM requests WHERE range_vkey_commitment = $1 AND rollup_config_hash = $2 AND aggregation_vkey = $3 AND status != $4 AND status != $5 AND req_type = $6 AND start_block = $7 ORDER BY start_block ASC"
        )
        .bind(&commitment.range_vkey_commitment[..])
        .bind(&commitment.rollup_config_hash[..])
        .bind(&commitment.agg_vkey_hash[..])
        .bind(RequestStatus::Failed as i16)
        .bind(RequestStatus::Cancelled as i16)
        .bind(RequestType::Aggregation as i16)
        .bind(start_block)
        .fetch_all(&self.pool)
        .await?;
        Ok(requests)
    }

    /// Fetch the sorted list of AGG proofs with status UNREQ that have a start_block >= latest_contract_l2_block.
    ///
    /// Checks that the request has the same range vkey commitment, aggregation vkey, and rollup config hash as the commitment.
    ///
    /// NOTE: There should only be one "pending" UNREQ agg proof at a time for a specific start block.
    pub async fn fetch_unrequested_agg_proof(
        &self,
        latest_contract_l2_block: i64,
        commitment: &CommitmentConfig,
    ) -> Result<Option<OPSuccinctRequest>, Error> {
        let request = sqlx::query_as::<_, OPSuccinctRequest>(
            "SELECT * FROM requests WHERE range_vkey_commitment = $1 AND rollup_config_hash = $2 AND aggregation_vkey = $3 AND status = $4 AND req_type = $5 AND start_block >= $6 ORDER BY start_block ASC LIMIT 1"
        )
        .bind(&commitment.range_vkey_commitment[..])
        .bind(&commitment.rollup_config_hash[..])
        .bind(&commitment.agg_vkey_hash[..])
        .bind(RequestStatus::Unrequested as i16)
        .bind(RequestType::Aggregation as i16)
        .bind(latest_contract_l2_block)
        .fetch_optional(&self.pool)
        .await?;

        Ok(request)
    }

    /// Fetch the sorted list of RANGE proofs with status UNREQ that have a start_block >= latest_contract_l2_block.
    pub async fn fetch_unrequested_range_proofs(
        &self,
        latest_contract_l2_block: i64,
        commitment: &CommitmentConfig,
    ) -> Result<Option<OPSuccinctRequest>, Error> {
        let request = sqlx::query_as::<_, OPSuccinctRequest>(
            "SELECT * FROM requests WHERE range_vkey_commitment = $1 AND rollup_config_hash = $2 AND status = $4 AND req_type = $5 AND start_block >= $6 ORDER BY start_block ASC LIMIT 1"
        )
        .bind(&commitment.range_vkey_commitment[..])
        .bind(&commitment.rollup_config_hash[..])
        .bind(RequestStatus::Unrequested as i16)
        .bind(RequestType::Range as i16)
        .bind(latest_contract_l2_block)
        .fetch_optional(&self.pool)
        .await?;
        Ok(request)
    }

    /// Fetch all completed range proofs with matching range_vkey_commitment and start_block >= latest_contract_l2_block
    pub async fn fetch_completed_range_proofs(
        &self,
        commitment: &CommitmentConfig,
        latest_contract_l2_block: i64,
    ) -> Result<Vec<OPSuccinctRequest>, Error> {
        let requests = sqlx::query_as::<_, OPSuccinctRequest>(
            "SELECT * FROM requests WHERE range_vkey_commitment = $1 AND rollup_config_hash = $2 AND status = $3 AND req_type = $4 AND start_block >= $5 ORDER BY start_block ASC"
        )
        .bind(&commitment.range_vkey_commitment[..])
        .bind(&commitment.rollup_config_hash[..])
        .bind(RequestStatus::Complete as i16)
        .bind(RequestType::Range as i16)
        .bind(latest_contract_l2_block)
        .fetch_all(&self.pool)
        .await?;
        Ok(requests)
    }

    /// Add a completed proof to the database.
    pub async fn update_proof_to_complete(
        &self,
        id: i64,
        proof: &[u8],
    ) -> Result<PgQueryResult, Error> {
        sqlx::query(
            r#"
            UPDATE requests SET proof = $1, status = $2, updated_at = NOW() WHERE id = $3
            "#,
        )
        .bind(proof)
        .bind(RequestStatus::Complete as i16)
        .bind(id)
        .execute(&self.pool)
        .await
    }

    /// Update the status of a request in the database.
    pub async fn update_request_status(
        &self,
        id: i64,
        new_status: RequestStatus,
    ) -> Result<PgQueryResult, Error> {
        sqlx::query(
            r#"
            UPDATE requests 
            SET status = $1, updated_at = NOW()
            WHERE id = $2
            "#,
        )
        .bind(new_status as i16)
        .bind(id)
        .execute(&self.pool)
        .await
    }

    /// Update the status of a request to PROVE.
    pub async fn update_request_to_prove(
        &self,
        request_id: i64,
        proof_id: [u8; 32],
    ) -> Result<PgQueryResult, Error> {
        sqlx::query(
            r#"
            UPDATE requests 
            SET status = $1, proof_request_id = $2, updated_at = NOW()
            WHERE id = $3
            "#,
        )
        .bind(RequestStatus::Prove as i16)
        .bind(&proof_id[..])
        .bind(request_id)
        .execute(&self.pool)
        .await
    }

    /// Update status of a request to RELAYED.
    pub async fn update_request_to_relayed(
        &self,
        id: i64,
        relay_tx_hash: [u8; 32],
    ) -> Result<PgQueryResult, Error> {
        sqlx::query(
            r#"
            UPDATE requests SET status = $1, relay_tx_hash = $2, updated_at = NOW() WHERE id = $3
            "#,
        )
        .bind(RequestStatus::Relayed as i16)
        .bind(&relay_tx_hash[..])
        .bind(id)
        .execute(&self.pool)
        .await
    }

    /// Fetch a single completed aggregation proof.
    pub async fn fetch_completed_aggregation_proofs(
        &self,
        latest_contract_l2_block: i64,
        commitment: &CommitmentConfig,
    ) -> Result<Option<OPSuccinctRequest>, Error> {
        let request = sqlx::query_as::<_, OPSuccinctRequest>(
            "SELECT * FROM requests WHERE range_vkey_commitment = $1 AND rollup_config_hash = $2 AND aggregation_vkey = $3 AND status = $4 AND req_type = $5 AND start_block = $6 ORDER BY start_block ASC LIMIT 1"
        )
        .bind(&commitment.range_vkey_commitment[..])
        .bind(&commitment.rollup_config_hash[..])
        .bind(&commitment.agg_vkey_hash[..])
        .bind(RequestStatus::Complete as i16)
        .bind(RequestType::Aggregation as i16)
        .bind(latest_contract_l2_block)
        .fetch_optional(&self.pool)
        .await?;
        Ok(request)
    }

    /// Insert the execution statistics for a request.
    pub async fn insert_execution_statistics(
        &self,
        id: i64,
        execution_statistics: Value,
        execution_duration_secs: i64,
    ) -> Result<PgQueryResult, Error> {
        sqlx::query(
            r#"
            UPDATE requests SET execution_statistics = $1, execution_duration = $2 WHERE id = $3
            "#,
        )
        .bind(&execution_statistics)
        .bind(execution_duration_secs)
        .bind(id)
        .execute(&self.pool)
        .await
    }

    /// Create a query which sets the status of all requests in UNREQ, EXECUTE, WITNESSGEN, PROVE to cancelled.
    ///
    /// Cancel all requests with the given status
    pub async fn cancel_all_requests_with_statuses(
        &self,
        statuses: &[RequestStatus],
    ) -> Result<PgQueryResult, Error> {
        let status_values: Vec<i16> = statuses.iter().map(|s| *s as i16).collect();
        sqlx::query("UPDATE requests SET status = $1 WHERE status = ANY($2)")
            .bind(RequestStatus::Cancelled as i16)
            .bind(&status_values[..])
            .execute(&self.pool)
            .await
    }

    /// Batch insert requests.
    pub async fn insert_requests(
        &self,
        requests: &[OPSuccinctRequest],
    ) -> Result<PgQueryResult, Error> {
        let mut query_builder = sqlx::QueryBuilder::new(
            "INSERT INTO requests (
                status, req_type, mode, start_block, end_block, created_at, updated_at,
                proof_request_id, checkpointed_l1_block_number, checkpointed_l1_block_hash, execution_statistics,
                witnessgen_duration, execution_duration, prove_duration, range_vkey_commitment,
                aggregation_vkey, rollup_config_hash, relay_tx_hash, proof) ",
        );

        query_builder.push_values(requests, |mut b, req| {
            b.push_bind(req.status as i16)
                .push_bind(req.req_type as i16)
                .push_bind(req.mode as i16)
                .push_bind(req.start_block)
                .push_bind(req.end_block)
                .push_bind(req.created_at)
                .push_bind(req.updated_at)
                .push_bind(req.proof_request_id.as_ref().map(|arr| &arr[..]))
                .push_bind(req.checkpointed_l1_block_number)
                .push_bind(req.checkpointed_l1_block_hash.as_ref().map(|arr| &arr[..]))
                .push_bind(&req.execution_statistics)
                .push_bind(req.witnessgen_duration_secs)
                .push_bind(req.execution_duration_secs)
                .push_bind(req.prove_duration_secs)
                .push_bind(&req.range_vkey_commitment[..])
                .push_bind(req.aggregation_vkey_hash.as_ref().map(|arr| &arr[..]))
                .push_bind(&req.rollup_config_hash[..])
                .push_bind(req.relay_tx_hash.as_ref())
                .push_bind(req.proof.as_ref());
        });

        query_builder.build().execute(&self.pool).await
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let database_url = "postgres://user:password@localhost/dbname";
    let client = DriverDBClient::new(database_url).await?;

    let request = OPSuccinctRequest {
        id: 0, // ID is auto-generated by the database.
        status: RequestStatus::Unrequested,
        req_type: RequestType::Range,
        mode: RequestMode::Mock,
        start_block: 100,
        end_block: 200,
        created_at: Local::now().naive_local(),
        updated_at: Local::now().naive_local(),
        checkpointed_l1_block_number: None,
        checkpointed_l1_block_hash: None,
        execution_statistics: serde_json::json!({"example": "data"}),
        witnessgen_duration_secs: Some(500),
        execution_duration_secs: Some(300),
        prove_duration_secs: None,
        range_vkey_commitment: [0u8; 32],
        aggregation_vkey_hash: None,
        rollup_config_hash: [0u8; 32],
        relay_tx_hash: None,
        proof_request_id: None,
        proof: None,
    };

    client.insert_request(&request).await?;
    let requests = client.fetch_requests().await?;
    println!("Requests: {:?}", requests);

    client
        .update_request_status(1, RequestStatus::WitnessGeneration)
        .await?;

    let eth_metrics = EthMetrics {
        nb_transactions: 10,
        eth_gas_used: 20000,
        l1_fees: 300.into(),
        tx_fees: 150.into(),
    };

    client.insert_eth_metrics(&eth_metrics).await?;
    let metrics = client.fetch_eth_metrics().await?;
    println!("Eth Metrics: {:?}", metrics);

    Ok(())
}
