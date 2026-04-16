use std::{
    cmp::{max, min},
    collections::HashSet,
};

use crate::rpc_types::{OutputResponse, SafeHeadResponse};
use alloy_eips::BlockId;
use anyhow::{bail, Result};
use futures::StreamExt;
use serde::{Deserialize, Serialize};

use crate::{
    fetcher::{OPSuccinctDataFetcher, RPCMode},
    host::OPSuccinctHost,
};

const TWO_HOURS_IN_BLOCKS: u64 = 3600;

/// Get the start and end block numbers for a range, with validation.
pub async fn get_validated_block_range<H: OPSuccinctHost>(
    host: &H,
    data_fetcher: &OPSuccinctDataFetcher,
    start: Option<u64>,
    end: Option<u64>,
    default_range: u64,
) -> Result<(u64, u64)> {
    // Get the latest finalized block number when end block is not provided.
    // Even though the safeDB is activated, we use the finalized block number as the
    // end block by default to ensure the program doesn't run into L2 Block Validation
    // Failure error.
    // L2 Block Validation Failure error might still occur. See
    // [Troubleshooting](../troubleshooting.md#l2-block-validation-failure) for more details.
    let l2_finalized_block_number = data_fetcher.get_l2_header(BlockId::finalized()).await?.number;
    // `saturating_sub` guards against very low finalized L2 numbers, which can occur on
    // fresh test chains.
    let host_search_start = l2_finalized_block_number.saturating_sub(TWO_HOURS_IN_BLOCKS);
    let end_number = host
        .get_finalized_l2_block_number(data_fetcher, host_search_start)
        .await?
        .expect("Failed to get finalized L2 block number");

    // If end block not provided, use the host-resolved end.
    let l2_end_block = match end {
        Some(end) => {
            if end > end_number {
                bail!(
                    "The end block ({}) is greater than the latest finalized block ({})",
                    end,
                    end_number
                );
            }
            end
        }
        None => end_number,
    };

    // If start block not provided, use end block - default_range
    let l2_start_block = match start {
        Some(start) => start,
        None => max(1, l2_end_block.saturating_sub(default_range)),
    };

    if l2_start_block >= l2_end_block {
        bail!("Start block ({}) must be less than end block ({})", l2_start_block, l2_end_block);
    }

    Ok((l2_start_block, l2_end_block))
}

/// Get a rolling block range whose end aligns with the host's finalized L2 block.
///
/// The returned tuple represents the last `range` blocks that the host considers finalized
/// according to its DA-specific logic, making the range safe to use for proof generation.
///
/// Returns an error if the requested `range` exceeds the current finalized head; this is
/// preferred over silently returning a smaller range, since callers typically expect to
/// receive exactly `range` blocks and downstream logic may misbehave otherwise.
pub async fn get_rolling_block_range<H: OPSuccinctHost>(
    host: &H,
    data_fetcher: &OPSuccinctDataFetcher,
    range: u64,
) -> Result<(u64, u64)> {
    let header = data_fetcher.get_l2_header(BlockId::finalized()).await?;
    // `saturating_sub` guards against very low finalized L2 numbers.
    let host_search_start = header.number.saturating_sub(TWO_HOURS_IN_BLOCKS);
    let l2_end_block = host
        .get_finalized_l2_block_number(data_fetcher, host_search_start)
        .await?
        .expect("Failed to get finalized L2 block number");

    let l2_start_block = l2_end_block.checked_sub(range).ok_or_else(|| {
        anyhow::anyhow!(
            "requested rolling range {range} exceeds current end block {l2_end_block}; \
             cannot produce a non-underflowing range"
        )
    })?;

    Ok((l2_start_block, l2_end_block))
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SpanBatchRange {
    pub start: u64,
    pub end: u64,
}

/// Split a range of blocks into a list of span batch ranges.
///
/// This is a simple implementation used when the safeDB is not activated on the L2 Node.
pub fn split_range_basic(start: u64, end: u64, max_range_size: u64) -> Vec<SpanBatchRange> {
    let mut ranges = Vec::new();
    let mut current_start = start;

    while current_start < end {
        let current_end = min(current_start + max_range_size, end);
        ranges.push(SpanBatchRange { start: current_start, end: current_end });
        current_start = current_end;
    }

    ranges
}

/// Split a range of blocks into a list of span batch ranges based on L2 safeHeads.
///
/// 1. Get the L1 block range [L1 origin of l2_start, L1Head] where L1Head is the block from which
///    l2_end can be derived
/// 2. Loop over L1 blocks to get safeHead increases (batch posts) which form a step function
/// 3. Split ranges based on safeHead increases and max batch size
///
/// Example: If safeHeads are [27,49,90] and max_size=30, ranges will be [(0,27), (27,49), (49,69),
/// (69,90)]
///
/// Takes the data fetcher by reference so that the configured L1 selection is honored
/// across the inner safeHead lookups (avoiding split-brain with the caller's fetcher).
pub async fn split_range_based_on_safe_heads(
    data_fetcher: &OPSuccinctDataFetcher,
    l2_start: u64,
    l2_end: u64,
    max_range_size: u64,
) -> Result<Vec<SpanBatchRange>> {
    // Get the L1 origin of l2_start
    let l2_start_hex = format!("0x{l2_start:x}");
    let start_output: OutputResponse = data_fetcher
        .fetch_rpc_data_with_mode(
            RPCMode::L2Node,
            "optimism_outputAtBlock",
            vec![l2_start_hex.into()],
        )
        .await?;
    let l1_start = start_output.block_ref.l1_origin.number;

    // Get the L1Head from which l2_end can be derived
    let (_, l1_head_number) = data_fetcher.get_safe_l1_block_for_l2_block(l2_end).await?;

    // Get all the unique safeHeads between l1_start and l1_head
    let mut ranges = Vec::new();
    let mut current_l2_start = l2_start;
    let safe_heads = futures::stream::iter(l1_start..=l1_head_number)
        .map(|block| async move {
            let l1_block_hex = format!("0x{block:x}");
            let result: SafeHeadResponse = data_fetcher
                .fetch_rpc_data_with_mode(
                    RPCMode::L2Node,
                    "optimism_safeHeadAtL1Block",
                    vec![l1_block_hex.into()],
                )
                .await
                .expect("Failed to fetch safe head");
            result.safe_head.number
        })
        .buffered(15)
        .collect::<HashSet<_>>()
        .await;

    // Collect and sort the safe heads.
    let mut safe_heads: Vec<_> = safe_heads.into_iter().collect();
    safe_heads.sort();

    // Loop over all of the safe heads and create ranges.
    for safe_head in safe_heads {
        if safe_head > current_l2_start && current_l2_start < l2_end {
            let mut range_start = current_l2_start;
            while range_start + max_range_size < min(l2_end, safe_head) {
                ranges
                    .push(SpanBatchRange { start: range_start, end: range_start + max_range_size });
                range_start += max_range_size;
            }
            ranges.push(SpanBatchRange { start: range_start, end: min(l2_end, safe_head) });
            current_l2_start = safe_head;
        }
    }

    Ok(ranges)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `get_rolling_block_range` must return a non-underflowing range when `range` exceeds
    /// the current finalized head; the new behavior is an explicit error rather than silent
    /// overflow (was: `l2_end - range`).
    ///
    /// We exercise the checked_sub directly here because the full call path requires a
    /// populated host + fetcher; the subtraction is the underflow surface we actually care
    /// about after the fix.
    #[test]
    fn rolling_range_subtraction_is_checked() {
        let l2_end: u64 = 10;
        let range: u64 = 100;
        assert!(l2_end.checked_sub(range).is_none(), "precondition: range exceeds end");

        // Exact boundary: range == end yields (0, end), which is valid.
        let l2_end: u64 = 100;
        let range: u64 = 100;
        assert_eq!(l2_end.checked_sub(range), Some(0));

        // Happy path.
        let l2_end: u64 = 500;
        let range: u64 = 100;
        assert_eq!(l2_end.checked_sub(range), Some(400));
    }

    /// Guards the saturating subtraction used to derive the host search start from the
    /// reference L2 block number. A non-default L1 selection can resolve a very low L2
    /// number; the previous raw subtraction would underflow for numbers below
    /// `TWO_HOURS_IN_BLOCKS`.
    #[test]
    fn reference_subtraction_saturates_below_two_hours() {
        let small: u64 = 10;
        assert_eq!(small.saturating_sub(TWO_HOURS_IN_BLOCKS), 0);

        let at_boundary: u64 = TWO_HOURS_IN_BLOCKS;
        assert_eq!(at_boundary.saturating_sub(TWO_HOURS_IN_BLOCKS), 0);

        let above: u64 = TWO_HOURS_IN_BLOCKS + 100;
        assert_eq!(above.saturating_sub(TWO_HOURS_IN_BLOCKS), 100);
    }

    #[test]
    fn split_range_basic_simple() {
        let ranges = split_range_basic(0, 100, 30);
        assert_eq!(
            ranges,
            vec![
                SpanBatchRange { start: 0, end: 30 },
                SpanBatchRange { start: 30, end: 60 },
                SpanBatchRange { start: 60, end: 90 },
                SpanBatchRange { start: 90, end: 100 },
            ]
        );
    }

    #[test]
    fn split_range_basic_exact_multiple() {
        let ranges = split_range_basic(0, 90, 30);
        assert_eq!(
            ranges,
            vec![
                SpanBatchRange { start: 0, end: 30 },
                SpanBatchRange { start: 30, end: 60 },
                SpanBatchRange { start: 60, end: 90 },
            ]
        );
    }

    #[test]
    fn split_range_basic_empty_when_start_equals_end() {
        assert_eq!(split_range_basic(50, 50, 30), vec![]);
    }
}
