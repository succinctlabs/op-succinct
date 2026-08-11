use alloy_eips::BlockNumberOrTag;
use alloy_primitives::{B256, U64};
use alloy_provider::Provider;
use anyhow::{bail, Context, Result};
use async_trait::async_trait;
use op_succinct_host_utils::{
    metrics::MetricsGauge,
    rpc_types::{RollupConfig, SafeHeadResponse, SyncStatus},
};

use crate::{
    checked_l2_block_number,
    game_validator::{
        classify_computed_output_root, GameValidation, GameValidationRequest, GameValidator,
        InvalidReason, UnavailableReason,
    },
    prometheus::ChallengerGauge,
    L1Provider, L2NodeProvider, L2Provider, L2ProviderTrait,
};

/// OP Stack claim validator backed by one op-node and its paired execution node.
pub struct OPStackGameValidator {
    l1_provider: L1Provider,
    l2_provider: L2Provider,
    l2_node_provider: L2NodeProvider,
}

impl OPStackGameValidator {
    pub fn new(
        l1_provider: L1Provider,
        l2_provider: L2Provider,
        l2_node_provider: L2NodeProvider,
    ) -> Self {
        Self { l1_provider, l2_provider, l2_node_provider }
    }

    async fn rollup_config(&self) -> Result<RollupConfig> {
        Ok(self.l2_node_provider.client().request("optimism_rollupConfig", ()).await?)
    }

    async fn op_node_sync_status(&self) -> Result<SyncStatus> {
        Ok(self.l2_node_provider.client().request("optimism_syncStatus", ()).await?)
    }

    async fn safe_head_at_l1(&self, l1_number: u64) -> Result<SafeHeadResponse> {
        Ok(self
            .l2_node_provider
            .client()
            .request("optimism_safeHeadAtL1Block", (U64::from(l1_number),))
            .await?)
    }

    async fn validate_node_pairing(&self) -> Result<()> {
        let rollup_config =
            self.rollup_config().await.context("Failed to fetch op-node rollup config")?;
        let l1_chain_id = self.l1_provider.get_chain_id().await?;
        let l2_chain_id = self.l2_provider.get_chain_id().await?;

        validate_rollup_chain_ids(
            rollup_config.l1_chain_id,
            rollup_config.l2_chain_id.id(),
            l1_chain_id,
            l2_chain_id,
        )?;

        let sync_status =
            self.op_node_sync_status().await.context("Failed to fetch op-node sync status")?;
        let processed_l1 = sync_status
            .current_l1
            .number
            .checked_sub(1)
            .context("op-node has not processed an L1 block yet")?;
        let safe_head = self.safe_head_at_l1(processed_l1).await.with_context(|| {
            format!("SafeDB is unavailable at processed L1 block {processed_l1}")
        })?;
        self.validate_safe_head_response(processed_l1, safe_head)
            .await
            .map_err(|reason| anyhow::anyhow!("Invalid SafeDB startup response: {reason:?}"))?;

        tracing::info!(
            l1_chain_id,
            l2_chain_id,
            safe_l2_number = safe_head.safe_head.number,
            safe_l2_hash = ?safe_head.safe_head.hash,
            "Validated op-node, L1 RPC, and paired L2 execution RPC"
        );
        Ok(())
    }

    async fn validate_safe_head_response(
        &self,
        requested_l1: u64,
        response: SafeHeadResponse,
    ) -> std::result::Result<(), UnavailableReason> {
        if !safe_db_record_is_at_or_before(response.l1_block.number, requested_l1) {
            return Err(UnavailableReason::L1CanonicalHashMismatch)
        }

        self.validate_canonical_l1_block(response.l1_block.number, response.l1_block.hash).await?;

        let l2_header = self
            .l2_provider
            .get_header_by_number(BlockNumberOrTag::Number(response.safe_head.number))
            .await
            .map_err(rpc_unavailable)?
            .ok_or(UnavailableReason::OpNodeExecutionMismatch)?;
        if l2_header.hash != response.safe_head.hash {
            return Err(UnavailableReason::OpNodeExecutionMismatch)
        }

        Ok(())
    }

    async fn validate_canonical_l1_block(
        &self,
        number: u64,
        expected_hash: B256,
    ) -> std::result::Result<(), UnavailableReason> {
        let header = self
            .l1_provider
            .get_header_by_number(BlockNumberOrTag::Number(number))
            .await
            .map_err(rpc_unavailable)?
            .ok_or(UnavailableReason::L1CanonicalHashMismatch)?;
        if header.hash != expected_hash {
            return Err(UnavailableReason::L1CanonicalHashMismatch)
        }
        Ok(())
    }

    async fn historical_local_safe_head(
        &self,
        request: &GameValidationRequest,
    ) -> std::result::Result<u64, UnavailableReason> {
        let game_l1_header = self
            .l1_provider
            .get_header_by_hash(request.l1_head)
            .await
            .map_err(rpc_unavailable)?
            .ok_or(UnavailableReason::L1CanonicalHashMismatch)?;
        let game_l1_number = game_l1_header.number;
        self.validate_canonical_l1_block(game_l1_number, request.l1_head).await?;

        let first_status = self.op_node_sync_status().await.map_err(rpc_unavailable)?;
        record_op_node_l1_lag(&first_status);
        if first_status.current_l1.number <= game_l1_number {
            return Err(UnavailableReason::OpNodeBehind)
        }

        let safe_head = self.safe_head_at_l1(game_l1_number).await.map_err(|error| {
            ChallengerGauge::SafeDbQueryErrors.increment(1.0);
            classify_safe_db_error(error)
        })?;
        self.validate_safe_head_response(game_l1_number, safe_head).await?;

        let second_status = self.op_node_sync_status().await.map_err(rpc_unavailable)?;
        record_op_node_l1_lag(&second_status);
        if !op_node_watermarks_are_usable(
            first_status.current_l1.number,
            second_status.current_l1.number,
            game_l1_number,
        ) {
            return Err(UnavailableReason::OpNodeBehind)
        }

        Ok(safe_head.safe_head.number)
    }

    async fn classify_execution_unavailability(
        &self,
        claim_l2_number: u64,
        root_error: &anyhow::Error,
    ) -> UnavailableReason {
        let latest = match self.l2_provider.get_header_by_number(BlockNumberOrTag::Latest).await {
            Ok(Some(header)) => header,
            Ok(None) => return UnavailableReason::ExecutionNodeBehind,
            Err(error) => return rpc_unavailable(error),
        };
        if latest.number < claim_l2_number {
            return UnavailableReason::ExecutionNodeBehind
        }

        match self.l2_provider.get_header_by_number(BlockNumberOrTag::Number(claim_l2_number)).await
        {
            Ok(None) => UnavailableReason::ExecutionHistoryMissing,
            Err(error) => rpc_unavailable(error),
            Ok(Some(_)) if is_state_unavailable_error(root_error) => {
                UnavailableReason::ExecutionStateUnavailable
            }
            Ok(Some(_)) => UnavailableReason::RpcFailure(root_error.to_string()),
        }
    }
}

#[async_trait]
impl GameValidator for OPStackGameValidator {
    async fn validate_startup(&self) -> Result<()> {
        self.validate_node_pairing().await
    }

    async fn validate(&self, request: &GameValidationRequest) -> GameValidation {
        let claim_l2_number = match checked_l2_block_number(request.l2_block_number) {
            Ok(number) => number,
            Err(_) => return GameValidation::Invalid(InvalidReason::L2BlockNumberOverflow),
        };

        let safe_l2_number = match self.historical_local_safe_head(request).await {
            Ok(number) => number,
            Err(reason) => {
                record_unavailable_reason(&reason);
                tracing::warn!(
                    game_index = %request.game_index,
                    game_address = ?request.game_address,
                    l1_head = ?request.l1_head,
                    l2_block_number = claim_l2_number,
                    ?reason,
                    "Game validation is temporarily unavailable; will retry"
                );
                return GameValidation::Unavailable(reason)
            }
        };

        if let Some(validation) = classify_claim_height(claim_l2_number, safe_l2_number) {
            return validation
        }

        match self.l2_provider.compute_output_root_at_block(request.l2_block_number).await {
            Ok(computed) => classify_computed_output_root(request.output_root, computed),
            Err(error) => {
                let reason = self.classify_execution_unavailability(claim_l2_number, &error).await;
                tracing::warn!(
                    game_index = %request.game_index,
                    game_address = ?request.game_address,
                    l2_block_number = claim_l2_number,
                    ?reason,
                    ?error,
                    "L2 output root is temporarily unavailable; will retry"
                );
                GameValidation::Unavailable(reason)
            }
        }
    }
}

fn rpc_unavailable(error: impl std::fmt::Display) -> UnavailableReason {
    UnavailableReason::RpcFailure(error.to_string())
}

fn classify_safe_db_error(error: anyhow::Error) -> UnavailableReason {
    let message = error.to_string();
    let lowercase = message.to_ascii_lowercase();
    if lowercase.contains("method not found") ||
        lowercase.contains("does not exist") ||
        lowercase.contains("safe head database not enabled") ||
        (lowercase.contains("safedb") && lowercase.contains("not enabled"))
    {
        UnavailableReason::SafeDBDisabled
    } else if lowercase.contains("not found") {
        UnavailableReason::SafeDBHistoryMissing
    } else {
        UnavailableReason::RpcFailure(message)
    }
}

fn is_state_unavailable_error(error: &anyhow::Error) -> bool {
    let message = error.to_string().to_ascii_lowercase();
    message.contains("missing trie node") ||
        message.contains("state is not available") ||
        message.contains("historical state")
}

fn record_op_node_l1_lag(status: &SyncStatus) {
    ChallengerGauge::OpNodeL1LagBlocks
        .set(status.head_l1.number.saturating_sub(status.current_l1.number) as f64);
}

fn record_unavailable_reason(reason: &UnavailableReason) {
    match reason {
        UnavailableReason::SafeDBHistoryMissing => {
            ChallengerGauge::SafeDbHistoryMissing.increment(1.0);
        }
        UnavailableReason::L1CanonicalHashMismatch => {
            ChallengerGauge::L1CanonicalMismatch.increment(1.0);
        }
        UnavailableReason::OpNodeExecutionMismatch => {
            ChallengerGauge::OpNodeExecutionMismatch.increment(1.0);
        }
        _ => {}
    }
}

fn validate_rollup_chain_ids(
    rollup_l1_chain_id: u64,
    rollup_l2_chain_id: u64,
    l1_rpc_chain_id: u64,
    l2_rpc_chain_id: u64,
) -> Result<()> {
    if rollup_l1_chain_id != l1_rpc_chain_id {
        bail!(
            "op-node rollup L1 chain ID {rollup_l1_chain_id} does not match L1_RPC chain ID {l1_rpc_chain_id}"
        );
    }
    if rollup_l2_chain_id != l2_rpc_chain_id {
        bail!(
            "op-node rollup L2 chain ID {rollup_l2_chain_id} does not match L2_RPC chain ID {l2_rpc_chain_id}"
        );
    }
    Ok(())
}

fn safe_db_record_is_at_or_before(record_l1_number: u64, requested_l1_number: u64) -> bool {
    record_l1_number <= requested_l1_number
}

fn op_node_watermarks_are_usable(first: u64, second: u64, game_l1_number: u64) -> bool {
    first > game_l1_number && second >= first
}

fn classify_claim_height(claim_l2_number: u64, safe_l2_number: u64) -> Option<GameValidation> {
    (claim_l2_number > safe_l2_number)
        .then_some(GameValidation::Invalid(InvalidReason::ClaimAheadOfLocalSafeHead))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contract::L2Output;
    use alloy::{rpc::client::RpcClient, transports::mock::Asserter};
    use alloy_eips::BlockNumHash;
    use alloy_primitives::{keccak256, Address, U256};
    use alloy_provider::RootProvider;
    use alloy_rpc_types_eth::Header;
    use alloy_sol_types::SolValue;

    fn test_request() -> GameValidationRequest {
        GameValidationRequest {
            game_index: U256::ZERO,
            game_address: Address::ZERO,
            l1_head: B256::repeat_byte(0x55),
            l2_block_number: U256::from(7),
            output_root: B256::ZERO,
            deadline: 100,
            now_timestamp: 0,
        }
    }

    fn test_sync_status(current_l1: u64) -> SyncStatus {
        let mut status = SyncStatus {
            current_l1: Default::default(),
            current_l1_finalized: Default::default(),
            head_l1: Default::default(),
            safe_l1: Default::default(),
            finalized_l1: Default::default(),
            unsafe_l2: Default::default(),
            safe_l2: Default::default(),
            finalized_l2: Default::default(),
            cross_unsafe_l2: Default::default(),
            local_safe_l2: Default::default(),
        };
        status.current_l1.number = current_l1;
        status.head_l1.number = current_l1;
        status
    }

    fn test_header(number: u64, hash: B256) -> Header {
        Header {
            hash,
            inner: alloy::consensus::Header { number, ..Default::default() },
            ..Default::default()
        }
    }

    fn test_output_header(number: u64) -> Header {
        let mut header = test_header(number, B256::repeat_byte(0x11));
        header.state_root = B256::repeat_byte(0x22);
        header.withdrawals_root = Some(B256::repeat_byte(0x33));
        header
    }

    fn expected_output_root(header: &Header) -> B256 {
        keccak256(
            L2Output {
                zero: 0,
                l2_state_root: header.state_root.0.into(),
                l2_storage_hash: header.withdrawals_root.unwrap().0.into(),
                l2_claim_hash: header.hash.0.into(),
            }
            .abi_encode(),
        )
    }

    fn test_validator(
        l1_asserter: &Asserter,
        l2_asserter: &Asserter,
        op_node_asserter: &Asserter,
    ) -> OPStackGameValidator {
        let l1_provider = RootProvider::new(RpcClient::mocked(l1_asserter.clone()));
        let l2_provider: L2Provider = RootProvider::new(RpcClient::mocked(l2_asserter.clone()));
        let l2_node_provider: L2NodeProvider =
            RootProvider::new(RpcClient::mocked(op_node_asserter.clone()));
        OPStackGameValidator::new(l1_provider, l2_provider, l2_node_provider)
    }

    fn push_safe_head_validation(
        l1_asserter: &Asserter,
        l2_asserter: &Asserter,
        op_node_asserter: &Asserter,
        first_current_l1: u64,
        second_current_l1: Option<u64>,
        safe_db_l1_number: u64,
        execution_safe_hash: B256,
    ) {
        let game_l1_hash = B256::repeat_byte(0x55);
        let safe_db_l1_hash =
            if safe_db_l1_number == 100 { game_l1_hash } else { B256::repeat_byte(0x44) };
        let safe_l2_hash = B256::repeat_byte(0x66);
        let l1_header = test_header(100, game_l1_hash);
        l1_asserter.push_success(&Some(l1_header.clone()));
        l1_asserter.push_success(&Some(l1_header));
        l1_asserter.push_success(&Some(test_header(safe_db_l1_number, safe_db_l1_hash)));

        op_node_asserter.push_success(&test_sync_status(first_current_l1));
        op_node_asserter.push_success(&SafeHeadResponse {
            l1_block: BlockNumHash { number: safe_db_l1_number, hash: safe_db_l1_hash },
            safe_head: BlockNumHash { number: 200, hash: safe_l2_hash },
        });
        if let Some(current_l1) = second_current_l1 {
            op_node_asserter.push_success(&test_sync_status(current_l1));
        }

        l2_asserter.push_success(&Some(test_header(200, execution_safe_hash)));
    }

    struct ValidationFixture {
        validator: OPStackGameValidator,
        l1_asserter: Asserter,
        l2_asserter: Asserter,
        op_node_asserter: Asserter,
    }

    impl ValidationFixture {
        fn assert_consumed(&self) {
            assert!(self.l1_asserter.read_q().is_empty());
            assert!(self.l2_asserter.read_q().is_empty());
            assert!(self.op_node_asserter.read_q().is_empty());
        }
    }

    fn validation_fixture(
        first_current_l1: u64,
        second_current_l1: Option<u64>,
        execution_safe_hash: B256,
    ) -> ValidationFixture {
        let l1_asserter = Asserter::new();
        let l2_asserter = Asserter::new();
        let op_node_asserter = Asserter::new();
        push_safe_head_validation(
            &l1_asserter,
            &l2_asserter,
            &op_node_asserter,
            first_current_l1,
            second_current_l1,
            100,
            execution_safe_hash,
        );
        let validator = test_validator(&l1_asserter, &l2_asserter, &op_node_asserter);
        ValidationFixture { validator, l1_asserter, l2_asserter, op_node_asserter }
    }

    #[test]
    fn startup_chain_pairing_checks_both_l1_and_l2() {
        assert!(validate_rollup_chain_ids(1, 10, 1, 10).is_ok());
        assert!(validate_rollup_chain_ids(1, 10, 2, 10)
            .unwrap_err()
            .to_string()
            .contains("L1 chain ID"));
        assert!(validate_rollup_chain_ids(1, 10, 1, 11)
            .unwrap_err()
            .to_string()
            .contains("L2 chain ID"));
    }

    #[test]
    fn safe_db_floor_and_watermark_boundaries_are_strict() {
        assert!(safe_db_record_is_at_or_before(99, 100));
        assert!(safe_db_record_is_at_or_before(100, 100));
        assert!(!safe_db_record_is_at_or_before(101, 100));

        assert!(!op_node_watermarks_are_usable(100, 101, 100));
        assert!(op_node_watermarks_are_usable(101, 101, 100));
        assert!(!op_node_watermarks_are_usable(102, 101, 100));
    }

    #[tokio::test]
    async fn safedb_validation_recovers_when_history_becomes_available() {
        let l1_asserter = Asserter::new();
        let l2_asserter = Asserter::new();
        let op_node_asserter = Asserter::new();
        let game_l1_header = test_header(100, B256::repeat_byte(0x55));

        l1_asserter.push_success(&Some(game_l1_header.clone()));
        l1_asserter.push_success(&Some(game_l1_header));
        op_node_asserter.push_success(&test_sync_status(102));
        op_node_asserter.push_failure_msg("SafeDB record not found");

        let output_header = test_output_header(7);
        push_safe_head_validation(
            &l1_asserter,
            &l2_asserter,
            &op_node_asserter,
            102,
            Some(102),
            99,
            B256::repeat_byte(0x66),
        );
        l2_asserter.push_success(&Some(output_header.clone()));
        push_safe_head_validation(
            &l1_asserter,
            &l2_asserter,
            &op_node_asserter,
            102,
            Some(102),
            99,
            B256::repeat_byte(0x66),
        );
        l2_asserter.push_success(&Some(output_header.clone()));

        let validator = test_validator(&l1_asserter, &l2_asserter, &op_node_asserter);
        let mut retry_request = test_request();
        retry_request.output_root = expected_output_root(&output_header);
        let mut sibling_request = retry_request.clone();
        sibling_request.game_index = U256::from(1);
        sibling_request.output_root = B256::ZERO;

        assert_eq!(
            validator.validate(&retry_request).await,
            GameValidation::Unavailable(UnavailableReason::SafeDBHistoryMissing)
        );
        assert_eq!(
            validator.validate(&sibling_request).await,
            GameValidation::Invalid(InvalidReason::OutputRootMismatch)
        );
        assert_eq!(validator.validate(&retry_request).await, GameValidation::Valid);
        assert!(l1_asserter.read_q().is_empty());
        assert!(l2_asserter.read_q().is_empty());
        assert!(op_node_asserter.read_q().is_empty());
    }

    #[tokio::test]
    async fn first_op_node_watermark_at_game_head_is_unavailable() {
        let l1_asserter = Asserter::new();
        let l2_asserter = Asserter::new();
        let op_node_asserter = Asserter::new();
        let game_l1_header = test_header(100, B256::repeat_byte(0x55));
        l1_asserter.push_success(&Some(game_l1_header.clone()));
        l1_asserter.push_success(&Some(game_l1_header));
        op_node_asserter.push_success(&test_sync_status(100));
        let validator = test_validator(&l1_asserter, &l2_asserter, &op_node_asserter);

        assert_eq!(
            validator.historical_local_safe_head(&test_request()).await,
            Err(UnavailableReason::OpNodeBehind)
        );
        assert!(l1_asserter.read_q().is_empty());
        assert!(l2_asserter.read_q().is_empty());
        assert!(op_node_asserter.read_q().is_empty());
    }

    #[tokio::test]
    async fn safe_db_l1_hash_mismatch_is_unavailable() {
        let l1_asserter = Asserter::new();
        let l2_asserter = Asserter::new();
        let op_node_asserter = Asserter::new();
        let game_l1_header = test_header(100, B256::repeat_byte(0x55));
        l1_asserter.push_success(&Some(game_l1_header.clone()));
        l1_asserter.push_success(&Some(game_l1_header));
        l1_asserter.push_success(&Some(test_header(99, B256::repeat_byte(0x45))));
        op_node_asserter.push_success(&test_sync_status(102));
        op_node_asserter.push_success(&SafeHeadResponse {
            l1_block: BlockNumHash { number: 99, hash: B256::repeat_byte(0x44) },
            safe_head: BlockNumHash { number: 200, hash: B256::repeat_byte(0x66) },
        });
        let validator = test_validator(&l1_asserter, &l2_asserter, &op_node_asserter);

        assert_eq!(
            validator.historical_local_safe_head(&test_request()).await,
            Err(UnavailableReason::L1CanonicalHashMismatch)
        );
        assert!(l1_asserter.read_q().is_empty());
        assert!(l2_asserter.read_q().is_empty());
        assert!(op_node_asserter.read_q().is_empty());
    }

    #[tokio::test]
    async fn second_op_node_watermark_regression_is_unavailable() {
        let fixture = validation_fixture(102, Some(101), B256::repeat_byte(0x66));

        assert_eq!(
            fixture.validator.historical_local_safe_head(&test_request()).await,
            Err(UnavailableReason::OpNodeBehind)
        );
        fixture.assert_consumed();
    }

    #[tokio::test]
    async fn paired_execution_safe_head_hash_mismatch_is_unavailable() {
        let fixture = validation_fixture(102, None, B256::repeat_byte(0x77));

        assert_eq!(
            fixture.validator.historical_local_safe_head(&test_request()).await,
            Err(UnavailableReason::OpNodeExecutionMismatch)
        );
        fixture.assert_consumed();
    }

    #[tokio::test]
    async fn future_claim_is_invalid_without_reading_claimed_root() {
        let fixture = validation_fixture(102, Some(102), B256::repeat_byte(0x66));
        let mut request = test_request();
        request.l2_block_number = U256::from(201);

        assert_eq!(
            fixture.validator.validate(&request).await,
            GameValidation::Invalid(InvalidReason::ClaimAheadOfLocalSafeHead)
        );
        fixture.assert_consumed();
    }

    #[test]
    fn claim_above_historical_safe_head_is_invalid() {
        assert_eq!(
            classify_claim_height(101, 100),
            Some(GameValidation::Invalid(InvalidReason::ClaimAheadOfLocalSafeHead))
        );
        assert_eq!(classify_claim_height(100, 100), None);
        assert_eq!(classify_claim_height(99, 100), None);
    }

    #[test]
    fn safedb_errors_keep_disabled_and_history_gap_distinct() {
        assert_eq!(
            classify_safe_db_error(anyhow::anyhow!("safe head database not enabled")),
            UnavailableReason::SafeDBDisabled
        );
        assert_eq!(
            classify_safe_db_error(anyhow::anyhow!("method not found")),
            UnavailableReason::SafeDBDisabled
        );
        assert_eq!(
            classify_safe_db_error(anyhow::anyhow!(
                "the method optimism_safeHeadAtL1Block does not exist"
            )),
            UnavailableReason::SafeDBDisabled
        );
        assert_eq!(
            classify_safe_db_error(anyhow::anyhow!("not found")),
            UnavailableReason::SafeDBHistoryMissing
        );
        assert!(matches!(
            classify_safe_db_error(anyhow::anyhow!("connection reset")),
            UnavailableReason::RpcFailure(_)
        ));
    }
}
