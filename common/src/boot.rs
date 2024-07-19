//! This module contains the prologue phase of the client program, pulling in the boot
//! information, which is passed to the zkVM a public inputs to be verified on chain.

#[cfg(feature = "io")]
use crate::SP1KonaDataFetcher;

use alloy_primitives::{B256, U256};
use alloy_sol_types::{sol, SolValue};
use kona_client::BootInfo;
use kona_primitives::RollupConfig;
use serde::{Deserialize, Serialize};

/// Boot information that is passed to the zkVM as public inputs.
/// This struct contains all information needed to generate BootInfo,
/// as the RollupConfig can be derived from the `chain_id`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootInfoWithoutRollupConfig {
    pub l1_head: B256,
    pub l2_output_root: B256,
    pub l2_claim: B256,
    pub l2_claim_block: u64,
    pub chain_id: u64,
}

impl Into<BootInfo> for BootInfoWithoutRollupConfig{
    /// Convert the BootInfoWithoutRollupConfig into BootInfo by deriving the RollupConfig.
    fn into(self) -> BootInfo {
        let rollup_config = RollupConfig::from_l2_chain_id(self.chain_id).unwrap();

        BootInfo {
            l1_head: self.l1_head,
            l2_output_root: self.l2_output_root,
            l2_claim: self.l2_claim,
            l2_claim_block: self.l2_claim_block,
            chain_id: self.chain_id,
            rollup_config,
        }
    }
}

#[cfg(feature = "io")]
impl From<SP1KonaDataFetcher> for BootInfoWithoutRollupConfig {
    fn from(data_fetcher: SP1KonaDataFetcher) -> Self {
        BootInfoWithoutRollupConfig {
            l1_head: data_fetcher.l1_head.unwrap(),
            l2_output_root: data_fetcher.l2_output_root.unwrap(),
            l2_claim: data_fetcher.l2_claim.unwrap(),
            l2_claim_block: data_fetcher.l2_block_number.unwrap(),
            chain_id: data_fetcher.l2_chain_id,
        }
    }
}

impl BootInfoWithoutRollupConfig {
    /// ABI encode the boot info. This is used to commit to in the zkVM,
    /// so that we can verify on chain that the correct values were used in
    /// the proof.
    pub fn abi_encode(&self) -> Vec<u8> {
        sol! {
            struct PublicValuesStruct {
                bytes32 l1Head;
                bytes32 l2PreRoot;
                bytes32 l2PostRoot;
                uint256 l2BlockNumber;
                uint256 chainId;
            }
        }

        let public_values = PublicValuesStruct {
            l1Head: self.l1_head,
            l2PreRoot: self.l2_output_root,
            l2PostRoot: self.l2_claim,
            l2BlockNumber: U256::from(self.l2_claim_block),
            chainId: U256::from(self.chain_id),
        };

        public_values.abi_encode()
    }
}
