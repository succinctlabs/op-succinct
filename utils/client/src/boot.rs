//! This module contains the prologue phase of the client program, pulling in the boot
//! information, which is passed to the zkVM a public inputs to be verified on chain.

use alloy_primitives::B256;
use alloy_sol_types::sol;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

// ABI encoding of BootInfo is 6 * 32 bytes.
pub const BOOT_INFO_SIZE: usize = 6 * 32;

/// Hash the serialized rollup config using SHA256. Note: The rollup config is never unrolled on-chain,
/// so switching to a different hash function is not a concern, as long as the config hash is
/// consistent with the one on the contract.
pub fn hash_rollup_config(serialized_config: &Vec<u8>) -> B256 {
    // Create a SHA256 hasher
    let mut hasher = Sha256::new();

<<<<<<< HEAD:crates/client-utils/src/boot.rs
    // Hash the serialized config
    hasher.update(serialized_config.as_slice());

    // Finalize and convert to B256
    let hash = hasher.finalize();
    B256::from_slice(hash.as_slice())
=======
impl From<RawBootInfo> for BootInfo {
    /// Convert the BootInfoWithoutRollupConfig into BootInfo by deriving the RollupConfig.
    fn from(boot_info_without_rollup_config: RawBootInfo) -> Self {
        let RawBootInfo { l1_head, l2_output_root, l2_claim, l2_claim_block, chain_id } =
            boot_info_without_rollup_config;
        let rollup_config = RollupConfig::from_l2_chain_id(chain_id).unwrap();

        Self { l1_head, l2_output_root, l2_claim, l2_claim_block, chain_id, rollup_config }
    }
>>>>>>> origin/main:utils/client/src/boot.rs
}

sol! {
    #[derive(Debug, Serialize, Deserialize)]
    struct BootInfoStruct {
        bytes32 l1Head;
        bytes32 l2PreRoot;
        bytes32 l2PostRoot;
        uint64 l2BlockNumber;
        uint64 chainId;
        bytes32 rollupConfigHash;
    }
}

impl From<BootInfoWithBytesConfig> for BootInfoStruct {
    /// Convert a `BootInfoWithBytesConfig` to a `BootInfoStruct`.
    fn from(boot_info: BootInfoWithBytesConfig) -> Self {
        BootInfoStruct {
            l1Head: boot_info.l1_head,
            l2PreRoot: boot_info.l2_output_root,
            l2PostRoot: boot_info.l2_claim,
            l2BlockNumber: boot_info.l2_claim_block,
            chainId: boot_info.chain_id,
            rollupConfigHash: hash_rollup_config(&boot_info.rollup_config_bytes),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootInfoWithBytesConfig {
    pub l1_head: B256,
    pub l2_output_root: B256,
    pub l2_claim: B256,
    pub l2_claim_block: u64,
    pub chain_id: u64,
    pub rollup_config_bytes: Vec<u8>,
}
