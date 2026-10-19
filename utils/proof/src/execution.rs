//! Validate successful range execution against the requested L2 claim.

use alloy_primitives::B256;
use anyhow::{ensure, Context, Result};
use bincode::Options;
use op_succinct_client_utils::boot::BootInfoStruct;
use sp1_sdk::{ExecutionReport, SP1PublicValues};

/// Reject guest failures, missing output, and results for a different L2 claim.
pub fn validate_range_execution(
    public_values: &SP1PublicValues,
    report: &ExecutionReport,
    end_block: u64,
    output_root: B256,
) -> Result<()> {
    // SDK execution can return Ok even when the guest panics.
    ensure!(report.exit_code == 0, "Range program exited with code {}", report.exit_code);
    let boot: BootInfoStruct = bincode::DefaultOptions::new()
        .with_fixint_encoding()
        .reject_trailing_bytes()
        .deserialize(public_values.as_slice())
        .context("Range program did not commit valid boot info")?;
    ensure!(
        boot.l2BlockNumber == end_block,
        "Range program committed block {}, expected {end_block}",
        boot.l2BlockNumber
    );
    ensure!(
        boot.l2PostRoot == output_root,
        "Range program committed output root {}, expected {output_root}",
        boot.l2PostRoot
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use alloy_primitives::B256;
    use op_succinct_client_utils::boot::BootInfoStruct;
    use sp1_sdk::{ExecutionReport, SP1PublicValues};

    use super::validate_range_execution;

    #[test]
    fn rejects_failed_missing_and_mismatched_results() {
        let root = B256::repeat_byte(1);
        let boot = BootInfoStruct {
            l1Head: B256::ZERO,
            l2PreRoot: B256::ZERO,
            l2PostRoot: root,
            l2BlockNumber: 6,
            rollupConfigHash: B256::ZERO,
        };
        let values = SP1PublicValues::from(bincode::serialize(&boot).unwrap().as_slice());
        let report = ExecutionReport::default();
        validate_range_execution(&values, &report, 6, root).unwrap();
        assert!(validate_range_execution(&values, &report, 7, root).is_err());
        assert!(validate_range_execution(&values, &report, 6, B256::ZERO).is_err());
        assert!(validate_range_execution(&SP1PublicValues::default(), &report, 6, root).is_err());
        let mut trailing = bincode::serialize(&boot).unwrap();
        trailing.push(0);
        let trailing = SP1PublicValues::from(trailing.as_slice());
        assert!(validate_range_execution(&trailing, &report, 6, root).is_err());
        let mut failed = report;
        failed.exit_code = 1;
        assert!(validate_range_execution(&values, &failed, 6, root).is_err());
    }
}
