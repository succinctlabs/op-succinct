use std::sync::Arc;

use anyhow::Result;
use op_succinct_elfs::AGGREGATION_ELF;
use op_succinct_host_utils::fetcher::OPSuccinctDataFetcher;
use sp1_cluster_utils::{request_proof_from_env, ClusterElf, ProofRequestResults};
use sp1_sdk::{
    network::proto::types::ProofMode,
    SP1ProofMode, SP1ProofWithPublicValues, SP1Stdin,
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

pub fn to_proto_proof_mode(mode: SP1ProofMode) -> ProofMode {
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
    let timeout_hours = (timeout_secs / 3600).max(1);
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
    let timeout_hours = (timeout_secs / 3600).max(1);
    let proto_mode = to_proto_proof_mode(agg_mode);
    let cluster_elf = ClusterElf::NewElf(AGGREGATION_ELF.to_vec());
    let ProofRequestResults { proof, .. } =
        request_proof_from_env(proto_mode, timeout_hours, cluster_elf, stdin)
            .await
            .map_err(|e| anyhow::anyhow!("cluster agg proof failed: {e}"))?;
    Ok(SP1ProofWithPublicValues::from(proof))
}
