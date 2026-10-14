use std::{
    cmp::{max, min},
    collections::HashSet,
};

use crate::rpc_types::{OutputResponse, SafeHeadResponse};
use anyhow::{bail, Result};
use futures::StreamExt;
use serde::{Deserialize, Serialize};

use crate::fetcher::{OPSuccinctDataFetcher, RPCMode};

/// Get the start and end block numbers for a range, with validation.
pub async fn get_validated_block_range(
    data_fetcher: &OPSuccinctDataFetcher,
    start: Option<u64>,
    end: Option<u64>,
    default_range: u64,
) -> Result<(u64, u64)> {
    let end_number = data_fetcher.get_max_provable_l2_block_number().await?;

    // If end block not provided, use the resolved end.
    let l2_end_block = match end {
        Some(end) => {
            if end > end_number {
                bail!(
                    "The end block ({}) is greater than the max provable L2 block ({})",
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

/// Get a rolling range ending at the maximum provable L2 block for the L1 selection.
///
/// Returns an error if `range` exceeds the resolved end, rather than returning a shorter range.
pub async fn get_rolling_block_range(
    data_fetcher: &OPSuccinctDataFetcher,
    range: u64,
) -> Result<(u64, u64)> {
    let l2_end_block = data_fetcher.get_max_provable_l2_block_number().await?;

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
