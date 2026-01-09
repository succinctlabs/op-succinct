use crate::{host::OPSuccinctHost, witness_generation::WitnessGenerator};
use alloy_consensus::Header;
use alloy_primitives::{Address, B256};
use anyhow::{Context, Result};
use op_succinct_client_utils::{boot::BootInfoStruct, types::AggregationInputs};
use sp1_sdk::{
    network::FulfillmentStrategy, HashableKey, NetworkProver, SP1Proof, SP1ProofMode,
    SP1ProofWithPublicValues, SP1ProvingKey, SP1Stdin,
};
use std::env;
use tokio::time::Duration;

/// Get the stdin for the aggregation proof.
pub fn get_agg_proof_stdin(
    proofs: Vec<SP1Proof>,
    boot_infos: Vec<BootInfoStruct>,
    headers: Vec<Header>,
    multi_block_vkey: &sp1_sdk::SP1VerifyingKey,
    latest_checkpoint_head: B256,
    prover_address: Address,
) -> Result<SP1Stdin> {
    let mut stdin = SP1Stdin::new();
    for proof in proofs {
        let SP1Proof::Compressed(compressed_proof) = proof else {
            return Err(anyhow::anyhow!("Invalid proof passed as compressed proof!"));
        };
        stdin.write_proof(*compressed_proof, multi_block_vkey.vk.clone());
    }

    // Write the aggregation inputs to the stdin.
    stdin.write(&AggregationInputs {
        boot_infos,
        latest_l1_checkpoint_head: latest_checkpoint_head,
        multi_block_vkey: multi_block_vkey.hash_u32(),
        prover_address,
    });
    // The headers have issues serializing with bincode, so use serde_json instead.
    let headers_bytes = serde_cbor::to_vec(&headers).unwrap();
    stdin.write_vec(headers_bytes);

    Ok(stdin)
}

/// Generates the SP1 stdin required for proving a range of L2 blocks.
///
/// This function performs the following steps:
/// 1. Fetches the host arguments for the given block range from the L1/L2 nodes
/// 2. Runs witness generation in a blocking task (to avoid starving the async runtime)
/// 3. Converts the witness data into SP1 stdin format
///
/// # Arguments
/// * `host` - The OP Succinct host implementation for fetching data and generating witnesses
/// * `start_block` - The starting L2 block number (exclusive)
/// * `end_block` - The ending L2 block number (inclusive)
/// * `l1_head_hash` - Optional L1 head hash to use; if None, will be determined automatically
/// * `safe_db_fallback` - Whether to fallback to timestamp-based L1 head estimation
///
/// # Returns
/// The SP1 stdin data ready to be passed to the range proof program.
///
/// # Errors
/// Returns an error if fetching host args, witness generation, or stdin conversion fails.
pub async fn get_range_proof_stdin<T: OPSuccinctHost + Clone + Send + Sync + 'static>(
    host: &T,
    start_block: u64,
    end_block: u64,
    l1_head_hash: Option<B256>,
    safe_db_fallback: bool,
) -> Result<SP1Stdin>
where
    <<T as OPSuccinctHost>::WitnessGenerator as WitnessGenerator>::WitnessData: std::marker::Send,
{
    let host_args = host
        .fetch(start_block, end_block, l1_head_hash, safe_db_fallback)
        .await
        .context("Failed to get host CLI args")?;

    let host_clone = host.clone();
    let witness_data = tokio::task::spawn_blocking(move || {
        let rt = tokio::runtime::Handle::current();
        rt.block_on(async move { host_clone.run(&host_args).await })
    })
    .await
    .map_err(|e| anyhow::anyhow!("Witness generation task failed: {}", e))??;

    let sp1_stdin = match host.witness_generator().get_sp1_stdin(witness_data) {
        Ok(stdin) => stdin,
        Err(e) => {
            tracing::error!("Failed to get proof stdin: {}", e);
            return Err(anyhow::anyhow!("Failed to get proof stdin: {}", e));
        }
    };

    Ok(sp1_stdin)
}

macro_rules! maybe_set {
    ($builder:expr, $opt:expr, $method:ident) => {
        match $opt {
            Some(val) => $builder.$method(val),
            None => $builder,
        }
    };
}

/// Submits a proof request to the SP1 prover network and awaits the result.
///
/// # Arguments
/// * `sp1_stdin` - The SP1 stdin data containing the program inputs
/// * `range_pk` - The proving key for the range/aggregation program
/// * `prover` - The SP1 network prover client
/// * `config` - Configuration controlling proof generation parameters
///
/// # Returns
/// The generated proof with its public values on success.
///
/// # Errors
/// Returns an error if the proof request fails or times out.
pub async fn get_network_proof(
    sp1_stdin: SP1Stdin,
    range_pk: &SP1ProvingKey,
    prover: &NetworkProver,
    config: &ProvingConfig,
) -> Result<SP1ProofWithPublicValues> {
    let builder = prover.prove(range_pk, &sp1_stdin);

    let builder = maybe_set!(builder, config.strategy, strategy);
    let builder = maybe_set!(builder, config.mode, mode);
    let builder = maybe_set!(builder, config.cycle_limit, cycle_limit);
    let builder = maybe_set!(builder, config.gas_limit, gas_limit);
    let builder = maybe_set!(builder, config.max_price_per_pgu, max_price_per_pgu);
    let builder = builder.skip_simulation(config.skip_simulation);
    let builder = maybe_set!(builder, config.proving_timeout, timeout);
    let builder = maybe_set!(builder, config.min_auction_period, min_auction_period);
    let builder = maybe_set!(builder, config.auction_timeout, auction_timeout);
    let builder = builder.whitelist(config.whitelist.clone());
    let builder = maybe_set!(builder, config.auctioneer, auctioneer);
    let builder = maybe_set!(builder, config.executor, executor);
    let builder = maybe_set!(builder, config.verifier, verifier);

    let proof = builder.run_async().await?;
    Ok(proof)
}
#[derive(Debug, Clone)]
pub struct ProvingConfig {
    pub strategy: Option<FulfillmentStrategy>,
    pub mode: Option<SP1ProofMode>,
    pub cycle_limit: Option<u64>,
    pub gas_limit: Option<u64>,
    pub max_price_per_pgu: Option<u64>,
    pub skip_simulation: bool,
    pub proving_timeout: Option<Duration>,
    pub min_auction_period: Option<u64>,
    pub auction_timeout: Option<Duration>,
    pub whitelist: Option<Vec<Address>>,
    pub auctioneer: Option<Address>,
    pub executor: Option<Address>,
    pub verifier: Option<Address>,
}

impl ProvingConfig {
    /// Load a ProvingConfig from environment variables with the given prefix.
    ///
    /// All fields are optional — missing env vars result in `None` (or `false` for bools).
    ///
    /// # Environment Variables
    ///
    /// Given a prefix (e.g., `RANGE` or `AGG`), the following env vars are read:
    ///
    /// | Env Var                      | Type     | Values / Format                          |
    /// |------------------------------|----------|------------------------------------------|
    /// | `{PREFIX}_PROOF_STRATEGY`    | enum     | `reserved`, `hosted`, `auction`          |
    /// | `{PREFIX}_PROOF_MODE`        | enum     | `core`, `compressed`, `plonk`, `groth16` |
    /// | `{PREFIX}_CYCLE_LIMIT`       | u64      | Max cycles for proving                   |
    /// | `{PREFIX}_GAS_LIMIT`         | u64      | Max gas for proving                      |
    /// | `{PREFIX}_MAX_PRICE_PER_PGU` | u64      | Max price per proof gas unit             |
    /// | `{PREFIX}_SKIP_SIMULATION`   | bool     | `true` or `false` (default: `false`)     |
    /// | `{PREFIX}_PROVING_TIMEOUT`   | duration | e.g., `4h`, `30m`, `1h30m`               |
    /// | `{PREFIX}_MIN_AUCTION_PERIOD`| u64      | Min auction period in seconds            |
    /// | `{PREFIX}_AUCTION_TIMEOUT`   | duration | e.g., `5m`, `300s`                       |
    /// | `{PREFIX}_WHITELIST`         | addresses| Comma-separated: `0x123...,0x456...`     |
    /// | `{PREFIX}_AUCTIONEER`        | address  | Auctioneer address for auction strategy  |
    /// | `{PREFIX}_EXECUTOR`          | address  | Executor address for auction strategy    |
    /// | `{PREFIX}_VERIFIER`          | address  | Verifier address for auction strategy    |
    ///
    /// # Example
    ///
    /// ```bash
    /// export RANGE_PROOF_STRATEGY=reserved
    /// export RANGE_PROOF_MODE=compressed
    /// export RANGE_PROVING_TIMEOUT=4h
    /// export AGG_PROOF_MODE=plonk
    /// ```
    pub fn from_env_with_prefix(prefix: &str) -> Result<Self> {
        let get = |suffix: &str| env::var(format!("{}_{}", prefix, suffix)).ok();

        let parse_duration = |suffix: &str| -> Option<Duration> {
            get(suffix).and_then(|s| humantime::parse_duration(&s).ok())
        };

        let parse_u64 = |suffix: &str| -> Option<u64> { get(suffix).and_then(|s| s.parse().ok()) };

        let parse_address =
            |suffix: &str| -> Option<Address> { get(suffix).and_then(|s| s.parse().ok()) };

        let parse_addresses = |suffix: &str| -> Option<Vec<Address>> {
            get(suffix).map(|s| {
                s.split(',')
                    .filter(|s| !s.trim().is_empty())
                    .filter_map(|addr| addr.trim().parse().ok())
                    .collect()
            })
        };

        Ok(Self {
            strategy: get("PROOF_STRATEGY").and_then(|s| match s.to_lowercase().as_str() {
                "reserved" => Some(FulfillmentStrategy::Reserved),
                "hosted" => Some(FulfillmentStrategy::Hosted),
                "auction" => Some(FulfillmentStrategy::Auction),
                _ => None,
            }),
            mode: get("PROOF_MODE").and_then(|s| match s.to_lowercase().as_str() {
                "core" => Some(SP1ProofMode::Core),
                "compressed" => Some(SP1ProofMode::Compressed),
                "plonk" => Some(SP1ProofMode::Plonk),
                "groth16" => Some(SP1ProofMode::Groth16),
                _ => None,
            }),
            cycle_limit: parse_u64("CYCLE_LIMIT"),
            gas_limit: parse_u64("GAS_LIMIT"),
            max_price_per_pgu: parse_u64("MAX_PRICE_PER_PGU"),
            skip_simulation: get("SKIP_SIMULATION").and_then(|s| s.parse().ok()).unwrap_or(false),
            proving_timeout: parse_duration("PROVING_TIMEOUT"),
            min_auction_period: parse_u64("MIN_AUCTION_PERIOD"),
            auction_timeout: parse_duration("AUCTION_TIMEOUT"),
            whitelist: parse_addresses("WHITELIST"),
            auctioneer: parse_address("AUCTIONEER"),
            executor: parse_address("EXECUTOR"),
            verifier: parse_address("VERIFIER"),
        })
    }

    /// Load range proof config from RANGE_* env vars.
    pub fn range_from_env() -> Result<Self> {
        Self::from_env_with_prefix("RANGE")
    }

    /// Load aggregation proof config from AGG_* env vars.
    pub fn agg_from_env() -> Result<Self> {
        Self::from_env_with_prefix("AGG")
    }
}
