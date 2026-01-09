use alloy_primitives::{Address, Bytes, FixedBytes};
use alloy_provider::{Provider, ProviderBuilder, RootProvider};
use alloy_sol_macro::sol;
use alloy_transport_http::reqwest::Url;
use anyhow::{Context, Result};
use clap::Parser;
use fault_proof::config::RangeSplitCount;
use futures::stream::{self, StreamExt, TryStreamExt};
use op_succinct_client_utils::boot::BootInfoStruct;
use op_succinct_elfs::AGGREGATION_ELF;
use op_succinct_host_utils::{
    block_range::get_validated_block_range, fetcher::OPSuccinctDataFetcher, get_agg_proof_stdin,
    get_network_proof, get_range_proof_stdin, ProvingConfig,
};
use op_succinct_proof_utils::{get_range_elf_embedded, initialize_host};
use op_succinct_prove::DEFAULT_RANGE;
use sp1_sdk::{utils, HashableKey, Prover, ProverClient, SP1ProofMode};
use std::{env, num::NonZeroUsize, path::PathBuf, str::FromStr, sync::Arc};
use tracing::info;

/// Generates range proofs for a span of L2 blocks and aggregates them into a single proof.
///
/// # Pipeline
///
/// 1. **Setup**: Load config from env, initialize data fetcher and host
/// 2. **Block range**: Validate and split the L2 block range into sub-ranges
/// 3. **Range proofs**: Concurrently generate SP1 proofs for each sub-range via the prover network
/// 4. **Aggregation**: Combine all range proofs into a single aggregation proof
/// 5. **Verification** (optional): Verify the aggregation proof on-chain if `--verify` is passed
///
/// # Example usage
///
/// ```bash
/// cargo run --bin multi-and-agg -- --start 1000 --end 2000 --range-splits 4 --max-concurrent-splits 2 --verify
/// ```
///
/// See [`Args`] for CLI options and [`Config`] for environment variable configuration.
#[tokio::main]
async fn main() -> Result<()> {
    rustls::crypto::ring::default_provider().install_default().unwrap();

    let args = Args::parse();

    dotenv::from_path(&args.env_file)
        .context(format!("Environment file not found: {}", args.env_file.display()))?;
    utils::setup_logger();

    let config = Config::from_env().expect("failed to get config");
    let data_fetcher = OPSuccinctDataFetcher::new_with_rollup_config().await?;
    let host = initialize_host(Arc::new(data_fetcher.clone()));

    // If the end block is provided, check that it is less than the latest finalized block. If the
    // end block is not provided, use the latest finalized block.
    let (l2_start_block, l2_end_block) = get_validated_block_range(
        host.as_ref(),
        &data_fetcher,
        args.start,
        args.end,
        DEFAULT_RANGE,
    )
    .await?;

    // Split the range into sub-ranges.
    let ranges =
        args.range_splits.split(l2_start_block, l2_end_block).context("Failed to split range")?;
    let num_ranges = ranges.len();

    // Determine max concurrency.
    let max_concurrent = args.max_concurrent_splits.get();

    info!(
        "Processing blocks {} to {} split into {} sub-ranges with max {} concurrent operations",
        l2_start_block, l2_end_block, num_ranges, max_concurrent
    );

    let network_prover = Arc::new(ProverClient::builder().network().build());
    let (range_pk, range_vk) = network_prover.setup(get_range_elf_embedded());

    let tasks = ranges.into_iter().enumerate().map(|(idx, (start, end))| {
        // Clone these so that they can be moved into the async block.
        let network_prover = network_prover.clone();
        let range_pk = range_pk.clone();
        let host = host.clone();
        let config = config.clone();
        async move {
            // Get the stdin for the block.
            tracing::info!(
                "range {idx}: Generating SP1 stdin for blocks {start} to {end}, number: {}",
                end - start
            );
            let sp1_stdin =
                get_range_proof_stdin(host.as_ref(), start, end, None, config.safe_db_fallback)
                    .await?;
            let stdin_bytes = bincode::serialize(&sp1_stdin).unwrap();
            let stdin_len = stdin_bytes.len();
            tracing::info!(
                "range {idx}: SP1 stdin size: {stdin_len} bytes, for blocks {start} to {end}"
            );
            tracing::info!("range {idx}: Generating Range proof for blocks {start} to {end}");
            let mut range_proof = get_network_proof(
                sp1_stdin,
                &range_pk,
                &network_prover,
                &config.range_proving_config,
            )
            .await?;
            tracing::info!("range {idx}: Completed Range proof for blocks {start} to {end}");
            Ok::<_, anyhow::Error>((
                idx,
                range_proof.proof,
                range_proof.public_values.read::<BootInfoStruct>(),
            ))
        }
    });

    let task_stream = stream::iter(tasks);
    let outputs = task_stream.buffer_unordered(max_concurrent).try_collect::<Vec<_>>().await?;

    tracing::info!("Preparing stdin for Agg Proof");

    let mut proofs = vec![None; num_ranges];
    let mut boot_infos = vec![None; num_ranges];
    // Put the proofs and boot infos back into vectors in their original order.
    for (idx, proof, boot_info) in outputs {
        proofs[idx] = Some(proof);
        boot_infos[idx] = Some(boot_info);
    }

    // Unwrap the Options (will panic if any index was missing)
    let proofs: Vec<_> = proofs.into_iter().map(|p| p.unwrap()).collect();
    let boot_infos: Vec<_> = boot_infos.into_iter().map(|b| b.unwrap()).collect();
    let latest_l1_head = boot_infos.last().context("No boot infos generated")?.l1Head;

    let headers = match data_fetcher.get_header_preimages(&boot_infos, latest_l1_head).await {
        Ok(headers) => headers,
        Err(e) => {
            tracing::error!("Failed to get header preimages: {}", e);
            return Err(anyhow::anyhow!("Failed to get header preimages: {}", e));
        }
    };

    let sp1_stdin = match get_agg_proof_stdin(
        proofs,
        boot_infos,
        headers,
        &range_vk,
        latest_l1_head,
        config.proposer_address,
    ) {
        Ok(s) => s,
        Err(e) => {
            tracing::error!("Failed to get agg proof stdin: {}", e);
            return Err(anyhow::anyhow!("Failed to get agg proof stdin: {}", e));
        }
    };

    tracing::info!("Generating Agg Proof");
    let (agg_pk, agg_vk) = network_prover.setup(AGGREGATION_ELF);
    let agg_proof =
        get_network_proof(sp1_stdin, &agg_pk, &network_prover, &config.agg_proving_config).await?;
    tracing::info!("Aggregation proof generated successfully.");

    if args.verify {
        tracing::info!("Verifying aggregation proof on contract");

        let l1_provider: RootProvider<alloy_network::Ethereum> =
            ProviderBuilder::default().connect_http(config.l1_rpc.clone());
        let l1_chain_id = l1_provider.get_chain_id().await.context("failed to fetch chain ID")?;

        let verifier_address = match (l1_chain_id, config.agg_proving_config.mode) {
            (1, Some(SP1ProofMode::Groth16)) => "0x397A5f7f3dBd538f23DE225B51f532c34448dA9B",
            (1, Some(SP1ProofMode::Plonk)) => "0x3B6041173B80E77f038f3F2C0f9744f04837185e",
            (11155111, Some(SP1ProofMode::Groth16)) => "0x397A5f7f3dBd538f23DE225B51f532c34448dA9B",
            (11155111, Some(SP1ProofMode::Plonk)) => "0x3B6041173B80E77f038f3F2C0f9744f04837185e",
            _ => anyhow::bail!(
                "Unsupported verifier: l1_chain_id={l1_chain_id}, agg_proof_mode={:?}",
                config.agg_proving_config.mode
            ),
        };

        let verifier = ISP1Verifier::new(
            Address::from_str(verifier_address).context("failed to parse address")?,
            data_fetcher.l1_provider.clone(),
        );

        let public_values: Bytes = Bytes::copy_from_slice(agg_proof.public_values.as_slice());
        let proof_bytes: Bytes = agg_proof.bytes().to_vec().into();

        match verifier
            .verifyProof(FixedBytes(agg_vk.bytes32_raw()), public_values, proof_bytes)
            .gas(30_000_000)
            .call()
            .await
        {
            Ok(_) => tracing::info!("verifyProof call succeeded"),
            Err(e) => tracing::error!("verifyProof call failed: {e}"),
        }
    }

    Ok(())
}

/// Commandline arguments for the multi-and-agg proving script.
#[derive(Debug, Clone, Parser)]
pub struct Args {
    /// The start block of the range to execute.
    #[arg(long)]
    pub start: Option<u64>,
    /// The end block of the range to execute.
    #[arg(long)]
    pub end: Option<u64>,
    /// The environment file to use.
    #[arg(long, default_value = ".env")]
    pub env_file: PathBuf,
    /// Whether to verify proofs on chain.
    #[arg(long)]
    pub verify: bool,

    /// The number of segments to split the range into (1-16).
    #[arg(long, default_value = "1")]
    pub range_splits: RangeSplitCount,

    /// The maximum number of range splits to run concurrently. (default: 1)
    ///
    /// Increasing this feeds more work into the prover and host in parallel; tune carefully based
    /// on observed latency, and system resources before deviating from default.
    #[arg(long, default_value = "1")]
    pub max_concurrent_splits: NonZeroUsize,
}

#[derive(Debug, Clone)]
/// Configuration for the multi-and-agg proving pipeline.
///
/// # Required Environment Variables
///
/// | Env Var            | Description                                      |
/// |--------------------|--------------------------------------------------|
/// | `L1_RPC`           | L1 RPC endpoint URL                              |
/// | `PROPOSER_ADDRESS` | Address of the proposer (proof creator)          |
///
/// # Optional Environment Variables
///
/// | Env Var            | Default | Description                               |
/// |--------------------|---------|-------------------------------------------|
/// | `SAFE_DB_FALLBACK` | `false` | Use timestamp-based L1 head estimation    |
///
/// Additionally, proving parameters are loaded via [`ProvingConfig::from_env_with_prefix`]
/// using the `RANGE_*` prefix for range proofs and `AGG_*` prefix for aggregation proofs.
/// See [`ProvingConfig`] for the full list of supported env vars.
struct Config {
    /// L1 RPC endpoint URL.
    pub l1_rpc: Url,

    /// Proposer (Proof creator) address
    pub proposer_address: Address,

    /// Whether to fallback to timestamp-based L1 head estimation even though SafeDB is not
    /// activated for op-node.
    pub safe_db_fallback: bool,

    /// Proving configuration for aggregation proofs (loaded from `AGG_*` env vars).
    /// Defaults to Plonk mode if not specified.
    pub agg_proving_config: ProvingConfig,

    /// Proving configuration for range proofs (loaded from `RANGE_*` env vars).
    pub range_proving_config: ProvingConfig,
}

impl Config {
    /// Load configuration from environment variables.
    ///
    /// # Panics
    /// Panics if required env vars (`L1_RPC`, `PROPOSER_ADDRESS`) are missing or invalid.
    pub fn from_env() -> Result<Self> {
        let mut agg_proving_config =
            ProvingConfig::agg_from_env().expect("failed to get agg proving config");

        // Default agg proof mode to Plonk if not specified
        if agg_proving_config.mode.is_none() {
            agg_proving_config.mode = Some(SP1ProofMode::Plonk);
        }
        Ok(Self {
            l1_rpc: env::var("L1_RPC")?.parse().expect("L1_RPC not set"),
            proposer_address: env::var("PROPOSER_ADDRESS")?
                .parse()
                .expect("PROPOSER_ADDRESS not set"),

            safe_db_fallback: env::var("SAFE_DB_FALLBACK")
                .unwrap_or("false".to_string())
                .parse()?,
            agg_proving_config,
            range_proving_config: ProvingConfig::range_from_env()
                .expect("failed to get range proving config"),
        })
    }
}

sol! {
  #[allow(missing_docs)]
  #[sol(rpc)]
  interface ISP1Verifier {
    function verifyProof(
        bytes32 programVKey,
        bytes calldata publicValues,
        bytes calldata proofBytes
    ) view;
  }
}
