use clap::Parser;
use std::{num::NonZeroU64, path::PathBuf};

pub mod config_common;

/// The arguments for the host executable.
#[derive(Debug, Clone, Parser)]
pub struct HostExecutorArgs {
    /// The start block of the range to execute.
    #[arg(long)]
    pub start: Option<u64>,
    /// The end block of the range to execute.
    #[arg(long)]
    pub end: Option<u64>,
    /// The number of blocks to execute in a single batch. Must be greater
    /// than 0. If omitted and both `--start` and `--end` are provided, the
    /// full range is used as a single batch; otherwise defaults to 10.
    #[arg(long, value_parser = parse_positive_batch_size)]
    pub batch_size: Option<NonZeroU64>,
    /// Enable caching: load from cache if available, save to cache if not.
    #[arg(long)]
    pub cache: bool,
    /// Use a fixed recent range.
    #[arg(long)]
    pub rolling: bool,
    /// The number of blocks to use for the default range.
    #[arg(long, default_value = "5")]
    pub default_range: u64,
    /// The environment file to use.
    #[arg(long, default_value = ".env")]
    pub env_file: PathBuf,
    /// Whether to generate proofs.
    #[arg(long)]
    pub prove: bool,
    /// Whether to fallback to timestamp-based L1 head estimation even though SafeDB is not
    /// activated for op-node.
    #[clap(long)]
    pub safe_db_fallback: bool,
    /// Cluster proving timeout in seconds (only used when SP1_PROVER=cluster).
    #[arg(long, default_value = "21600")]
    pub cluster_timeout: u64,
}

/// Fallback batch size used when the user provides neither `--batch-size`
/// nor an explicit `--start`/`--end` range.
const DEFAULT_BATCH_SIZE: u64 = 10;

/// Clap value parser for `--batch-size`. Parses into `NonZeroU64` so the
/// non-zero invariant is carried by the type and cannot be bypassed by
/// programmatic construction of `HostExecutorArgs`.
fn parse_positive_batch_size(s: &str) -> Result<NonZeroU64, String> {
    let value: u64 = s.parse().map_err(|e: std::num::ParseIntError| e.to_string())?;
    NonZeroU64::new(value).ok_or_else(|| "--batch-size must be greater than 0".into())
}

impl HostExecutorArgs {
    /// Resolve the batch size used to split a block range.
    ///
    /// Precedence:
    /// 1. An explicit `--batch-size` is always honored (and is guaranteed non-zero by the
    ///    `NonZeroU64` type).
    /// 2. Otherwise, if both `--start` and `--end` are provided, the full range is processed as a
    ///    single batch (DX default for estimator or artifact runs targeting a specific range).
    /// 3. Otherwise, fall back to `DEFAULT_BATCH_SIZE`.
    pub fn effective_batch_size(&self) -> u64 {
        if let Some(batch_size) = self.batch_size {
            return batch_size.get();
        }
        match (self.start, self.end) {
            (Some(start), Some(end)) if end > start => end - start,
            _ => DEFAULT_BATCH_SIZE,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(start: Option<u64>, end: Option<u64>, batch_size: Option<u64>) -> HostExecutorArgs {
        let batch_size = batch_size.map(|v| {
            NonZeroU64::new(v).expect("test fixtures must not pass zero as an explicit batch size")
        });
        HostExecutorArgs {
            start,
            end,
            batch_size,
            cache: false,
            rolling: false,
            default_range: 5,
            env_file: PathBuf::from(".env"),
            prove: false,
            safe_db_fallback: false,
            cluster_timeout: 21600,
        }
    }

    #[test]
    fn explicit_batch_size_wins_over_start_end_range() {
        // Regression: before the fix, `effective_batch_size` silently
        // overrode an explicit `--batch-size` with `end - start` whenever
        // both bounds were provided. The estimator workaround
        // (`--start X --end Y --batch-size N`) relies on this precedence.
        let a = args(Some(1_000), Some(2_800), Some(120));
        assert_eq!(a.effective_batch_size(), 120);
    }

    #[test]
    fn explicit_batch_size_without_range() {
        let a = args(None, None, Some(42));
        assert_eq!(a.effective_batch_size(), 42);
    }

    #[test]
    fn explicit_batch_size_with_only_start() {
        let a = args(Some(1_000), None, Some(42));
        assert_eq!(a.effective_batch_size(), 42);
    }

    #[test]
    fn explicit_batch_size_with_only_end() {
        let a = args(None, Some(2_800), Some(42));
        assert_eq!(a.effective_batch_size(), 42);
    }

    #[test]
    fn omitted_batch_size_with_start_and_end_uses_full_range() {
        // DX default preserved from PR #820: user who just says
        // "estimate this specific range" gets a single batch.
        let a = args(Some(1_000), Some(2_800), None);
        assert_eq!(a.effective_batch_size(), 1_800);
    }

    #[test]
    fn omitted_batch_size_with_no_range_uses_default() {
        let a = args(None, None, None);
        assert_eq!(a.effective_batch_size(), DEFAULT_BATCH_SIZE);
    }

    #[test]
    fn omitted_batch_size_with_degenerate_range_uses_default() {
        // end <= start: the DX default is not applicable, so we fall
        // through to the hard-coded default rather than returning 0.
        let a = args(Some(2_800), Some(2_800), None);
        assert_eq!(a.effective_batch_size(), DEFAULT_BATCH_SIZE);

        let a = args(Some(2_800), Some(1_000), None);
        assert_eq!(a.effective_batch_size(), DEFAULT_BATCH_SIZE);
    }

    #[test]
    fn parser_rejects_batch_size_zero() {
        let result = HostExecutorArgs::try_parse_from([
            "test",
            "--start",
            "1000",
            "--end",
            "2800",
            "--batch-size",
            "0",
        ]);
        let err = result.expect_err("--batch-size 0 must fail at parse time").to_string();
        assert!(
            err.contains("--batch-size must be greater than 0"),
            "unexpected error message: {err}"
        );
    }

    #[test]
    fn parser_accepts_positive_batch_size() {
        let args = HostExecutorArgs::try_parse_from([
            "test",
            "--start",
            "1000",
            "--end",
            "2800",
            "--batch-size",
            "120",
        ])
        .expect("positive --batch-size must parse");
        assert_eq!(args.batch_size, NonZeroU64::new(120));
        // Precedence fix regression guard at the parse layer.
        assert_eq!(args.effective_batch_size(), 120);
    }

    #[test]
    fn parser_accepts_omitted_batch_size() {
        let args = HostExecutorArgs::try_parse_from(["test", "--start", "1000", "--end", "2800"])
            .expect("omitted --batch-size must parse");
        assert_eq!(args.batch_size, None);
        assert_eq!(args.effective_batch_size(), 1_800);
    }
}

#[derive(Debug, Clone, Parser)]
pub struct ConfigArgs {
    /// The environment file to use.
    #[arg(long)]
    pub env_file: Option<PathBuf>,
}
