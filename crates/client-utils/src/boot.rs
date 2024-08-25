//! This module contains the prologue phase of the client program, pulling in the boot
//! information, which is passed to the zkVM a public inputs to be verified on chain.

use alloy_primitives::B256;
use alloy_sol_types::{sol, SolValue};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tiny_keccak::{Hasher, Keccak};

// ABI encoding of BootInfo is 6 * 32 bytes.
pub const BOOT_INFO_SIZE: usize = 6 * 32;

pub fn hash_rollup_config(serialized_config: &Vec<u8>) -> B256 {
    // Create a Keccak256 hasher
    let mut keccak = Keccak::v256();
    let mut hash = [0u8; 32];

    // Hash the serialized string
    keccak.update(serialized_config.as_slice());
    keccak.finalize(&mut hash);

    hash.into()
}

sol! {
    struct BootInfoStruct {
        bytes32 l1Head;
        bytes32 l2PreRoot;
        bytes32 l2PostRoot;
        uint64 l2BlockNumber;
        uint64 chainId;
        bytes32 rollupConfigHash;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootInfoWithNoConfig {
    pub l1_head: B256,
    pub l2_output_root: B256,
    pub l2_claim: B256,
    pub l2_claim_block: u64,
    pub chain_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootInfoWithHashedConfig {
    pub l1_head: B256,
    pub l2_output_root: B256,
    pub l2_claim: B256,
    pub l2_claim_block: u64,
    pub chain_id: u64,
    pub rollup_config_hash: B256,
}

impl BootInfoWithHashedConfig {
    pub fn new(boot_info: &BootInfoWithNoConfig, config: &Vec<u8>) -> Self {
        Self {
            l1_head: boot_info.l1_head,
            l2_output_root: boot_info.l2_output_root,
            l2_claim: boot_info.l2_claim,
            l2_claim_block: boot_info.l2_claim_block,
            chain_id: boot_info.chain_id,
            rollup_config_hash: hash_rollup_config(config),
        }
    }

    pub fn abi_encode(&self) -> Vec<u8> {
        BootInfoStruct {
            l1Head: self.l1_head,
            l2PreRoot: self.l2_output_root,
            l2PostRoot: self.l2_claim,
            l2BlockNumber: self.l2_claim_block,
            chainId: self.chain_id,
            rollupConfigHash: self.rollup_config_hash,
        }
        .abi_encode()
    }

    pub fn abi_decode(bytes: &[u8]) -> Result<Self> {
        let boot_info = BootInfoStruct::abi_decode(bytes, true)?;
        Ok(Self {
            l1_head: boot_info.l1Head,
            l2_output_root: boot_info.l2PreRoot,
            l2_claim: boot_info.l2PostRoot,
            l2_claim_block: boot_info.l2BlockNumber,
            chain_id: boot_info.chainId,
            rollup_config_hash: boot_info.rollupConfigHash,
        })
    }
}
