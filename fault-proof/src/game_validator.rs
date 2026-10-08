use alloy_primitives::{Address, B256, U256};
use anyhow::Result;
use async_trait::async_trait;

use crate::checked_l2_block_number;

/// Immutable game data needed by a chain-specific validator.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameValidationRequest {
    pub game_index: U256,
    pub game_address: Address,
    pub l1_head: B256,
    pub l2_block_number: U256,
    pub output_root: B256,
    pub deadline: u64,
    pub now_timestamp: u64,
}

/// Chain-specific claim validation used by the shared challenger lifecycle.
#[async_trait]
pub trait GameValidator: Send + Sync {
    /// Validates that the backend is correctly configured and ready to validate games.
    async fn validate_startup(&self) -> Result<()>;

    /// Validates one game without mutating challenger lifecycle state.
    async fn validate(&self, request: &GameValidationRequest) -> GameValidation;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GameValidation {
    Valid,
    Invalid(InvalidReason),
    Unavailable(UnavailableReason),
}

impl GameValidation {
    pub(crate) fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid(_))
    }

    pub(crate) fn is_unavailable(&self) -> bool {
        matches!(self, Self::Unavailable(_))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InvalidReason {
    L2BlockNumberOverflow,
    ClaimAheadOfLocalSafeHead,
    OutputRootMismatch,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UnavailableReason {
    ValidationPending,
    OpNodeBehind,
    ExecutionNodeBehind,
    ExecutionHistoryMissing,
    ExecutionStateUnavailable,
    SafeDBDisabled,
    SafeDBHistoryMissing,
    L1CanonicalHashMismatch,
    OpNodeExecutionMismatch,
    RpcFailure(String),
}

pub(crate) fn classify_computed_output_root(expected: B256, computed: B256) -> GameValidation {
    if computed == expected {
        GameValidation::Valid
    } else {
        GameValidation::Invalid(InvalidReason::OutputRootMismatch)
    }
}

pub(crate) fn initial_game_validation(l2_block_number: U256) -> GameValidation {
    if checked_l2_block_number(l2_block_number).is_err() {
        GameValidation::Invalid(InvalidReason::L2BlockNumberOverflow)
    } else {
        GameValidation::Unavailable(UnavailableReason::ValidationPending)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computed_output_root_has_valid_and_invalid_outcomes() {
        let expected = B256::repeat_byte(0x11);

        assert_eq!(classify_computed_output_root(expected, expected), GameValidation::Valid);
        assert_eq!(
            classify_computed_output_root(expected, B256::repeat_byte(0x22)),
            GameValidation::Invalid(InvalidReason::OutputRootMismatch)
        );
    }

    #[test]
    fn overflow_is_invalid_before_any_deadline_transition() {
        let oversized = U256::from(u64::MAX) + U256::from(1);

        assert_eq!(
            initial_game_validation(oversized),
            GameValidation::Invalid(InvalidReason::L2BlockNumberOverflow)
        );
    }

    #[test]
    fn unavailable_keeps_reason_and_is_not_invalid() {
        let reason = UnavailableReason::RpcFailure("execution node behind".to_string());
        let validation = GameValidation::Unavailable(reason.clone());

        assert!(validation.is_unavailable());
        assert!(!validation.is_invalid());
        assert_eq!(validation, GameValidation::Unavailable(reason));
    }
}
