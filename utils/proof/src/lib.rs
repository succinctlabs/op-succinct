use std::sync::Arc;

use anyhow::{Context, Result};
use op_succinct_elfs::AGGREGATION_ELF;
use op_succinct_host_utils::fetcher::OPSuccinctDataFetcher;
use sp1_cluster_utils::{request_proof_from_env, ClusterElf, ProofRequestResults};
use sp1_sdk::{
    blocking::{CpuProver, Prover as BlockingProver},
    network::proto::types::ProofMode,
    Elf, ProvingKey, SP1ProofMode, SP1ProofWithPublicValues, SP1ProvingKey, SP1Stdin,
    SP1VerifyingKey,
};

/// Get the range ELF depending on the feature flag.
pub fn get_range_elf_embedded() -> &'static [u8] {
    cfg_if::cfg_if! {
        if #[cfg(feature = "celestia")] {
            use op_succinct_elfs::CELESTIA_RANGE_ELF_EMBEDDED;

            CELESTIA_RANGE_ELF_EMBEDDED
        } else if #[cfg(feature = "eigenda")] {
            use op_succinct_elfs::EIGENDA_RANGE_ELF_EMBEDDED;

            EIGENDA_RANGE_ELF_EMBEDDED
        } else {
            use op_succinct_elfs::RANGE_ELF_EMBEDDED;

            RANGE_ELF_EMBEDDED
        }
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "celestia")] {
        use op_succinct_celestia_host_utils::host::CelestiaOPSuccinctHost;

        /// Initialize the Celestia host.
        pub fn initialize_host(
            fetcher: Arc<OPSuccinctDataFetcher>,
        ) -> Arc<CelestiaOPSuccinctHost> {
            tracing::info!("Initializing host with Celestia DA");
            Arc::new(CelestiaOPSuccinctHost::new(fetcher))
        }
    } else if #[cfg(feature = "eigenda")] {
        use op_succinct_eigenda_host_utils::host::EigenDAOPSuccinctHost;

        /// Initialize the EigenDA host.
        pub fn initialize_host(
            fetcher: Arc<OPSuccinctDataFetcher>,
        ) -> Arc<EigenDAOPSuccinctHost> {
            tracing::info!("Initializing host with EigenDA");
            Arc::new(EigenDAOPSuccinctHost::new(fetcher))
        }
    } else {
        use op_succinct_ethereum_host_utils::host::SingleChainOPSuccinctHost;

        /// Initialize the default (ETH-DA) host.
        pub fn initialize_host(
            fetcher: Arc<OPSuccinctDataFetcher>,
        ) -> Arc<SingleChainOPSuccinctHost> {
            tracing::info!("Initializing host with Ethereum DA");
            Arc::new(SingleChainOPSuccinctHost::new(fetcher))
        }
    }
}

/// Returns true if `SP1_PROVER` is set to `"cluster"` (self-hosted cluster mode).
pub fn is_cluster_mode() -> bool {
    std::env::var("SP1_PROVER").unwrap_or_default() == "cluster"
}

/// Set up range and aggregation proving/verifying keys via blocking CpuProver.
///
/// Runs in `spawn_blocking` because `CpuProver` creates its own tokio runtime
/// internally, which would panic if called directly from an async context.
pub async fn cluster_setup_keys(
) -> Result<(SP1ProvingKey, SP1VerifyingKey, SP1ProvingKey, SP1VerifyingKey)> {
    tokio::task::spawn_blocking(|| {
        let cpu_prover = CpuProver::new();
        let range_pk = cpu_prover
            .setup(Elf::Static(get_range_elf_embedded()))
            .context("range ELF setup failed")?;
        let range_vk = range_pk.verifying_key().clone();
        let agg_pk =
            cpu_prover.setup(Elf::Static(AGGREGATION_ELF)).context("agg ELF setup failed")?;
        let agg_vk = agg_pk.verifying_key().clone();
        anyhow::Ok((range_pk, range_vk, agg_pk, agg_vk))
    })
    .await?
}

fn to_proto_proof_mode(mode: SP1ProofMode) -> ProofMode {
    match mode {
        SP1ProofMode::Core => ProofMode::Core,
        SP1ProofMode::Compressed => ProofMode::Compressed,
        SP1ProofMode::Plonk => ProofMode::Plonk,
        SP1ProofMode::Groth16 => ProofMode::Groth16,
    }
}

/// Generate a compressed range proof via a self-hosted SP1 cluster.
pub async fn cluster_range_proof(
    timeout_secs: u64,
    stdin: SP1Stdin,
) -> Result<SP1ProofWithPublicValues> {
    tracing::info!("Generating range proof via cluster");
    let timeout_hours = timeout_secs.div_ceil(3600).max(1);
    let cluster_elf = ClusterElf::NewElf(get_range_elf_embedded().to_vec());
    let ProofRequestResults { proof, .. } =
        request_proof_from_env(ProofMode::Compressed, timeout_hours, cluster_elf, stdin)
            .await
            .map_err(|e| anyhow::anyhow!("cluster range proof failed: {e}"))?;
    Ok(SP1ProofWithPublicValues::from(proof))
}

/// Generate an aggregation proof via a self-hosted SP1 cluster.
pub async fn cluster_agg_proof(
    timeout_secs: u64,
    agg_mode: SP1ProofMode,
    stdin: SP1Stdin,
) -> Result<SP1ProofWithPublicValues> {
    tracing::info!("Generating aggregation proof via cluster");
    let timeout_hours = timeout_secs.div_ceil(3600).max(1);
    let proto_mode = to_proto_proof_mode(agg_mode);
    let cluster_elf = ClusterElf::NewElf(AGGREGATION_ELF.to_vec());
    let ProofRequestResults { proof, .. } =
        request_proof_from_env(proto_mode, timeout_hours, cluster_elf, stdin)
            .await
            .map_err(|e| anyhow::anyhow!("cluster agg proof failed: {e}"))?;
    Ok(SP1ProofWithPublicValues::from(proof))
}
