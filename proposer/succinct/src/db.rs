use anyhow::Result;
use chrono::{Duration, Local, NaiveDateTime};
use serde_json::Value;
use sqlx::types::Uuid;
use sqlx::Error;
use sqlx::{postgres::PgQueryResult, FromRow, PgPool};

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
    FailedRetryable = 6,
    FailedFatal = 7,
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
    pub aggregation_vkey: Option<[u8; 32]>,
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
            aggregation_vkey: None,
            relay_tx_hash: None,
            proof: None,
        }
    }
}

#[derive(FromRow, Debug)]
pub struct EthMetrics {
    pub nb_transactions: i64,
    pub eth_gas_used: i64,
    pub l1_fees: i64,
    pub tx_fees: i64,
}

pub struct DriverDBClient {
    pub pool: PgPool,
}

impl DriverDBClient {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = PgPool::connect(database_url).await?;
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
                relay_tx_hash,
                proof
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18
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
        // Storing durations as milliseconds for simplicity.
        .bind(req.witnessgen_duration_secs)
        .bind(req.execution_duration_secs)
        .bind(req.prove_duration_secs)
        .bind(&req.range_vkey_commitment[..])
        .bind(req.aggregation_vkey.as_ref().map(|arr| &arr[..]))
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
        .bind(metrics.eth_gas_used)
        .bind(metrics.l1_fees)
        .bind(metrics.tx_fees)
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
    pub async fn fetch_failed_requests_by_block_range(
        &self,
        start_block: i64,
        end_block: i64,
        range_vkey_commitment: [u8; 32],
    ) -> Result<Vec<OPSuccinctRequest>, Error> {
        let requests = sqlx::query_as::<_, OPSuccinctRequest>(
            "SELECT * FROM requests WHERE start_block = $1 AND end_block = $2 AND (status = $3 OR status = $4) AND range_vkey_commitment = $5",
        )
        .bind(start_block)
        .bind(end_block)
        .bind(RequestStatus::FailedRetryable as i16)
        .bind(RequestStatus::FailedFatal as i16)
        .bind(&range_vkey_commitment[..])
        .fetch_all(&self.pool)
        .await?;
        Ok(requests)
    }

    /// Fetch all requests that have one of the given statuses.
    pub async fn fetch_requests_by_statuses(
        &self,
        statuses: &[RequestStatus],
    ) -> Result<Vec<OPSuccinctRequest>, Error> {
        let status_values: Vec<i16> = statuses.iter().map(|s| *s as i16).collect();
        let query =
            sqlx::query_as::<_, OPSuccinctRequest>("SELECT * FROM requests WHERE status = ANY($1)")
                .bind(&status_values[..])
                .fetch_all(&self.pool)
                .await?;
        Ok(query)
    }

    /// Fetch all requests with a specific status from the database.
    pub async fn fetch_requests_by_status(
        &self,
        status: RequestStatus,
    ) -> Result<Vec<OPSuccinctRequest>, Error> {
        let requests =
            sqlx::query_as::<_, OPSuccinctRequest>("SELECT * FROM requests WHERE status = $1")
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
        range_vkey_commitment: [u8; 32],
    ) -> Result<Vec<OPSuccinctRequest>, Error> {
        let requests = sqlx::query_as::<_, OPSuccinctRequest>(
            "SELECT * FROM requests WHERE status = $1 AND req_type = $2 AND range_vkey_commitment = $3 AND start_block >= $4 AND end_block <= $5 ORDER BY start_block ASC"
        )
        .bind(RequestStatus::Complete as i16)
        .bind(RequestType::Range as i16)
        .bind(&range_vkey_commitment[..])
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
        range_vkey_commitment: [u8; 32],
        agg_vkey: [u8; 32],
    ) -> Result<Vec<OPSuccinctRequest>, Error> {
        let requests = sqlx::query_as::<_, OPSuccinctRequest>(
            "SELECT * FROM requests WHERE status != $1 AND status != $2 AND req_type = $3 AND range_vkey_commitment = $4 AND aggregation_vkey = $5 AND start_block = $6 ORDER BY start_block ASC"
        )
        .bind(RequestStatus::FailedRetryable as i16)
        .bind(RequestStatus::FailedFatal as i16)
        .bind(RequestType::Aggregation as i16)
        .bind(&range_vkey_commitment[..])
        .bind(&agg_vkey[..])
        .bind(start_block)
        .fetch_all(&self.pool)
        .await?;
        Ok(requests)
    }

    /// Fetch the sorted list of AGG proofs with status UNREQ that have a start_block >= latest_contract_l2_block.
    pub async fn fetch_unrequested_agg_proofs(
        &self,
        latest_contract_l2_block: i64,
        range_vkey_commitment: [u8; 32],
        agg_vkey: [u8; 32],
    ) -> Result<Vec<OPSuccinctRequest>, Error> {
        let requests = sqlx::query_as::<_, OPSuccinctRequest>(
            "SELECT * FROM requests WHERE status = $1 AND req_type = $2 AND range_vkey_commitment = $3 AND aggregation_vkey = $4 AND start_block >= $5 ORDER BY start_block ASC"
        )
        .bind(RequestStatus::Unrequested as i16)
        .bind(latest_contract_l2_block)
        .bind(&range_vkey_commitment[..])
        .bind(&agg_vkey[..])
        .fetch_all(&self.pool)
        .await?;

        Ok(requests)
    }

    /// Fetch the sorted list of RANGE proofs with status UNREQ that have a start_block >= latest_contract_l2_block.
    pub async fn fetch_unrequested_range_proofs(
        &self,
        latest_contract_l2_block: i64,
        range_vkey_commitment: [u8; 32],
    ) -> Result<Vec<OPSuccinctRequest>, Error> {
        let requests = sqlx::query_as::<_, OPSuccinctRequest>(
            "SELECT * FROM requests WHERE status = $1 AND req_type = $2 AND range_vkey_commitment = $3 AND start_block >= $4 ORDER BY start_block ASC"
        )
        .bind(RequestStatus::Unrequested as i16)
        .bind(RequestType::Range as i16)
        .bind(&range_vkey_commitment[..])
        .bind(latest_contract_l2_block)
        .fetch_all(&self.pool)
        .await?;
        Ok(requests)
    }

    /// Fetch all completed range proofs with matching range_vkey_commitment and start_block >= latest_contract_l2_block
    pub async fn fetch_completed_range_proofs(
        &self,
        range_vkey_commitment: [u8; 32],
        latest_contract_l2_block: i64,
    ) -> Result<Vec<OPSuccinctRequest>, Error> {
        let requests = sqlx::query_as::<_, OPSuccinctRequest>(
            "SELECT * FROM requests WHERE status = $1 AND req_type = $2 AND range_vkey_commitment = $3 AND start_block >= $4 ORDER BY start_block ASC"
        )
        .bind(RequestStatus::Complete as i16)
        .bind(RequestType::Range as i16)
        .bind(&range_vkey_commitment[..])
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
                aggregation_vkey, relay_tx_hash, proof) ",
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
                .push_bind(req.aggregation_vkey.as_ref().map(|arr| &arr[..]))
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
        aggregation_vkey: None,
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
        l1_fees: 300,
        tx_fees: 150,
    };

    client.insert_eth_metrics(&eth_metrics).await?;
    let metrics = client.fetch_eth_metrics().await?;
    println!("Eth Metrics: {:?}", metrics);

    Ok(())
}
