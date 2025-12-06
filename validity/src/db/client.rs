use alloy_primitives::{Address, B256};
use anyhow::Result;
use serde_json::Value;
use sqlx::{postgres::PgQueryResult, Error, PgPool};
use std::time::Duration;
use tracing::info;

use crate::{
    db::queries::{
        add_chain_lock, cancel_all_requests_with_statuses,
        cancel_prove_requests_with_different_commitment_config, delete_all_requests_with_statuses,
        fetch_active_agg_proofs_count, fetch_all_requests_by_status,
        fetch_completed_agg_proof_after_block, fetch_completed_ranges,
        fetch_failed_agg_request_with_checkpointed_block_hash,
        fetch_failed_request_count_by_block_range, fetch_first_unrequested_range_proof,
        fetch_highest_end_block_for_range_request, fetch_ranges_after_block, fetch_request,
        fetch_request_count, fetch_requests_by_status, fetch_unrequested_agg_proof,
        get_consecutive_complete_range_proofs, insert_execution_statistics, insert_request,
        insert_requests, is_chain_locked, update_chain_lock, update_l1_head_block_number,
        update_proof_to_complete, update_prove_duration, update_request_status,
        update_request_to_prove, update_request_to_relayed, update_witnessgen_duration,
    },
    CommitmentConfig, DriverDBClient, OPSuccinctRequest, RequestStatus,
};

pub async fn init_db(database_url: &str) -> Result<PgPool> {
    let pool = PgPool::connect(database_url).await?;

    // Run migrations.
    sqlx::migrate!("./migrations").run(&pool).await?;

    info!("Database configured successfully.");

    Ok(pool)
}

impl DriverDBClient {
    pub fn new(pool: PgPool) -> Self {
        DriverDBClient { pool }
    }

    /// Adds a chain lock to the database.
    pub async fn add_chain_lock(
        &self,
        l1_chain_id: i64,
        l2_chain_id: i64,
    ) -> Result<PgQueryResult, Error> {
        add_chain_lock(l1_chain_id, l2_chain_id, &self.pool).await
    }

    /// Checks if a proposer already has a lock on the chain.
    pub async fn is_chain_locked(
        &self,
        l1_chain_id: i64,
        l2_chain_id: i64,
        interval: Duration,
    ) -> Result<bool, Error> {
        is_chain_locked(l1_chain_id, l2_chain_id, interval, &self.pool).await
    }

    /// Updates the chain lock for a given chain.
    pub async fn update_chain_lock(
        &self,
        l1_chain_id: i64,
        l2_chain_id: i64,
    ) -> Result<PgQueryResult, Error> {
        update_chain_lock(l1_chain_id, l2_chain_id, &self.pool).await
    }

    /// Inserts a request into the database.
    pub async fn insert_request(&self, req: &OPSuccinctRequest) -> Result<PgQueryResult, Error> {
        insert_request(req, &self.pool).await
    }

    /// Fetch all requests with a specific block range and status FAILED.
    ///
    /// Checks that the request has the same range vkey commitment and rollup config hash as the
    /// commitment.
    pub async fn fetch_failed_request_count_by_block_range(
        &self,
        start_block: i64,
        end_block: i64,
        l1_chain_id: i64,
        l2_chain_id: i64,
        commitment: &CommitmentConfig,
    ) -> Result<i64, Error> {
        fetch_failed_request_count_by_block_range(
            start_block,
            end_block,
            l1_chain_id,
            l2_chain_id,
            commitment,
            &self.pool,
        )
        .await
    }
    /// Fetch the highest end block of a request with one of the given statuses and commitment.
    pub async fn fetch_highest_end_block_for_range_request(
        &self,
        statuses: &[RequestStatus],
        commitment: &CommitmentConfig,
        l1_chain_id: i64,
        l2_chain_id: i64,
    ) -> Result<Option<i64>, Error> {
        fetch_highest_end_block_for_range_request(
            statuses,
            commitment,
            l1_chain_id,
            l2_chain_id,
            &self.pool,
        )
        .await
    }

    /// Fetch all range requests that have one of the given statuses and start block >=
    /// latest_contract_l2_block. Returns only the start and end block as tuples.
    pub async fn fetch_ranges_after_block(
        &self,
        statuses: &[RequestStatus],
        latest_contract_l2_block: i64,
        commitment: &CommitmentConfig,
        l1_chain_id: i64,
        l2_chain_id: i64,
    ) -> Result<Vec<(i64, i64)>, Error> {
        fetch_ranges_after_block(
            statuses,
            latest_contract_l2_block,
            commitment,
            l1_chain_id,
            l2_chain_id,
            &self.pool,
        )
        .await
    }

    /// Fetch the number of requests with a specific status.
    pub async fn fetch_request_count(
        &self,
        status: RequestStatus,
        commitment: &CommitmentConfig,
        l1_chain_id: i64,
        l2_chain_id: i64,
    ) -> Result<i64, Error> {
        fetch_request_count(status, commitment, l1_chain_id, l2_chain_id, &self.pool).await
    }

    /// Fetch requests from provided commitment with a specific status from the database.
    pub async fn fetch_requests_by_status(
        &self,
        status: RequestStatus,
        commitment: &CommitmentConfig,
        l1_chain_id: i64,
        l2_chain_id: i64,
    ) -> Result<Vec<OPSuccinctRequest>, Error> {
        fetch_requests_by_status(status, commitment, l1_chain_id, l2_chain_id, &self.pool).await
    }

    /// Fetch all requests with a specific status from the database.
    pub async fn fetch_all_requests_by_status(
        &self,
        status: RequestStatus,
        from: Option<i64>,
        to: Option<i64>,
    ) -> Result<Vec<OPSuccinctRequest>, Error> {
        fetch_all_requests_by_status(status, from, to, &self.pool).await
    }

    /// Fetch a request by its ID.
    pub async fn fetch_request(&self, id: i64) -> Result<Option<OPSuccinctRequest>, Error> {
        fetch_request(id, &self.pool).await
    }

    /// Get the consecutive range proofs for a given start block and end block that are complete
    /// with the same range vkey commitment.
    pub async fn get_consecutive_complete_range_proofs(
        &self,
        start_block: i64,
        end_block: i64,
        commitment: &CommitmentConfig,
        l1_chain_id: i64,
        l2_chain_id: i64,
    ) -> Result<Vec<OPSuccinctRequest>, Error> {
        get_consecutive_complete_range_proofs(
            start_block,
            end_block,
            commitment,
            l1_chain_id,
            l2_chain_id,
            &self.pool,
        )
        .await
    }

    /// Fetch the checkpointed block hash and number for an aggregation request with the same start
    /// block, end block, and commitment config.
    pub async fn fetch_failed_agg_request_with_checkpointed_block_hash(
        &self,
        start_block: i64,
        end_block: i64,
        commitment: &CommitmentConfig,
        l1_chain_id: i64,
        l2_chain_id: i64,
    ) -> Result<Option<(Vec<u8>, i64)>, Error> {
        fetch_failed_agg_request_with_checkpointed_block_hash(
            start_block,
            end_block,
            commitment,
            l1_chain_id,
            l2_chain_id,
            &self.pool,
        )
        .await
    }
    /// Fetch the count of active (non-failed, non-cancelled) Aggregation proofs with the same start
    /// block, range vkey commitment, and aggregation vkey.
    pub async fn fetch_active_agg_proofs_count(
        &self,
        start_block: i64,
        commitment: &CommitmentConfig,
        l1_chain_id: i64,
        l2_chain_id: i64,
    ) -> Result<i64, Error> {
        fetch_active_agg_proofs_count(start_block, commitment, l1_chain_id, l2_chain_id, &self.pool)
            .await
    }

    /// Fetch the sorted list of Aggregation proofs with status Unrequested that have a start_block
    /// >= latest_contract_l2_block.
    ///
    /// Checks that the request has the same range vkey commitment, aggregation vkey, and rollup
    /// config hash as the commitment.
    ///
    /// NOTE: There should only be one "pending" Unrequested agg proof at a time for a specific
    /// start block.
    pub async fn fetch_unrequested_agg_proof(
        &self,
        latest_contract_l2_block: i64,
        commitment: &CommitmentConfig,
        l1_chain_id: i64,
        l2_chain_id: i64,
    ) -> Result<Option<OPSuccinctRequest>, Error> {
        fetch_unrequested_agg_proof(
            latest_contract_l2_block,
            commitment,
            l1_chain_id,
            l2_chain_id,
            &self.pool,
        )
        .await
    }

    /// Fetch the first Range proof with status Unrequested that has a start_block >=
    /// latest_contract_l2_block.
    pub async fn fetch_first_unrequested_range_proof(
        &self,
        latest_contract_l2_block: i64,
        commitment: &CommitmentConfig,
        l1_chain_id: i64,
        l2_chain_id: i64,
    ) -> Result<Option<OPSuccinctRequest>, Error> {
        fetch_first_unrequested_range_proof(
            latest_contract_l2_block,
            commitment,
            l1_chain_id,
            l2_chain_id,
            &self.pool,
        )
        .await
    }

    /// Fetch start and end blocks of all completed range proofs with matching range_vkey_commitment
    /// and start_block >= latest_contract_l2_block
    pub async fn fetch_completed_ranges(
        &self,
        commitment: &CommitmentConfig,
        latest_contract_l2_block: i64,
        l1_chain_id: i64,
        l2_chain_id: i64,
    ) -> Result<Vec<(i64, i64)>, Error> {
        fetch_completed_ranges(
            commitment,
            latest_contract_l2_block,
            l1_chain_id,
            l2_chain_id,
            &self.pool,
        )
        .await
    }

    /// Update the l1_head_block_number for a request.
    pub async fn update_l1_head_block_number(
        &self,
        id: i64,
        l1_head_block_number: i64,
    ) -> Result<PgQueryResult, Error> {
        update_l1_head_block_number(id, l1_head_block_number, &self.pool).await
    }

    /// Update the prove_duration based on the current time and the proof_request_time.
    pub async fn update_prove_duration(&self, id: i64) -> Result<PgQueryResult, Error> {
        update_prove_duration(id, &self.pool).await
    }

    /// Add a completed proof to the database.
    pub async fn update_proof_to_complete(
        &self,
        id: i64,
        proof: &[u8],
    ) -> Result<PgQueryResult, Error> {
        update_proof_to_complete(id, proof, &self.pool).await
    }

    /// Update the witness generation duration of a request in the database.
    pub async fn update_witnessgen_duration(
        &self,
        id: i64,
        duration: i64,
    ) -> Result<PgQueryResult, Error> {
        update_witnessgen_duration(id, duration, &self.pool).await
    }

    /// Update the status of a request in the database.
    pub async fn update_request_status(
        &self,
        id: i64,
        new_status: RequestStatus,
    ) -> Result<PgQueryResult, Error> {
        update_request_status(id, new_status, &self.pool).await
    }

    /// Update the status of a request to Prove.
    ///
    /// Updates the proof_request_time to the current time.
    pub async fn update_request_to_prove(
        &self,
        id: i64,
        proof_id: B256,
    ) -> Result<PgQueryResult, Error> {
        update_request_to_prove(id, proof_id, &self.pool).await
    }

    /// Update status of a request to RELAYED.
    pub async fn update_request_to_relayed(
        &self,
        id: i64,
        relay_tx_hash: B256,
        contract_address: Address,
    ) -> Result<PgQueryResult, Error> {
        update_request_to_relayed(id, relay_tx_hash, contract_address, &self.pool).await
    }

    /// Fetch a single completed aggregation proof after the given start block.
    pub async fn fetch_completed_agg_proof_after_block(
        &self,
        latest_contract_l2_block: i64,
        commitment: &CommitmentConfig,
        l1_chain_id: i64,
        l2_chain_id: i64,
    ) -> Result<Option<OPSuccinctRequest>, Error> {
        fetch_completed_agg_proof_after_block(
            latest_contract_l2_block,
            commitment,
            l1_chain_id,
            l2_chain_id,
            &self.pool,
        )
        .await
    }

    /// Insert the execution statistics for a request.
    pub async fn insert_execution_statistics(
        &self,
        id: i64,
        execution_statistics: Value,
        execution_duration_secs: i64,
    ) -> Result<PgQueryResult, Error> {
        insert_execution_statistics(id, execution_statistics, execution_duration_secs, &self.pool)
            .await
    }

    /// Cancel all requests with the given status
    pub async fn cancel_all_requests_with_statuses(
        &self,
        statuses: &[RequestStatus],
        l1_chain_id: i64,
        l2_chain_id: i64,
    ) -> Result<PgQueryResult, Error> {
        cancel_all_requests_with_statuses(statuses, l1_chain_id, l2_chain_id, &self.pool).await
    }

    /// Drop all requests with the given statuses
    pub async fn delete_all_requests_with_statuses(
        &self,
        statuses: &[RequestStatus],
        l1_chain_id: i64,
        l2_chain_id: i64,
    ) -> Result<PgQueryResult, Error> {
        delete_all_requests_with_statuses(statuses, l1_chain_id, l2_chain_id, &self.pool).await
    }

    /// Cancel all prove requests with a different commitment config and same chain id's
    pub async fn cancel_prove_requests_with_different_commitment_config(
        &self,
        commitment: &CommitmentConfig,
        l1_chain_id: i64,
        l2_chain_id: i64,
    ) -> Result<PgQueryResult, Error> {
        cancel_prove_requests_with_different_commitment_config(
            commitment,
            l1_chain_id,
            l2_chain_id,
            &self.pool,
        )
        .await
    }

    pub async fn insert_requests(
        &self,
        requests: &[OPSuccinctRequest],
    ) -> Result<PgQueryResult, Error> {
        let mut tx = self.pool.begin().await?;

        let result = insert_requests(requests, &mut *tx).await?;

        tx.commit().await?;

        Ok(result)
    }
}
