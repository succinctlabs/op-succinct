use kona_host::HostCli;
use alloy_primitives::{keccak256, B256};
use ethers::{
    providers::{Http, Middleware, Provider},
    types::{BlockNumber, H160, U256},
};
use std::{env, fs, str::FromStr};
use anyhow::Result;
use alloy_sol_types::{sol, SolValue};
use super::BootInfoWithoutRollupConfig;

sol! {
    struct L2Output {
        uint64 num;
        bytes32 l2_state_root;
        bytes32 l2_storage_hash;
        bytes32 l2_head;
    }
}

/// The SP1KonaDataFetcher struct is used to fetch the L2 output data and L2 claim data for a given block number.
/// It is used to generate the boot info for the native host program.
pub struct SP1KonaDataFetcher {
    pub l1_head: Option<B256>,
    pub l2_output_root: Option<B256>,
    pub l2_claim: Option<B256>,
    pub l2_block_number: Option<u64>,
    pub l2_head: Option<B256>,
    pub l2_chain_id: u64,
    pub l1_node_address: String,
    pub l1_beacon_address: String,
    pub l2_node_address: String,
}

impl Default for SP1KonaDataFetcher {
    fn default() -> Self {
        SP1KonaDataFetcher::new()
    }
}

impl SP1KonaDataFetcher {
    pub fn new() -> Self {
        let l1_rpc = env::var("CLABBY_RPC_L1")
            .unwrap_or_else(|_| "http://localhost:8545".to_string());

        let l1_beacon_rpc = env::var("ETH_BEACON_URL")
            .unwrap_or_else(|_| "http://localhost:5052".to_string());

        let l2_rpc = env::var("CLABBY_RPC_L2")
            .unwrap_or_else(|_| "http://localhost:9545".to_string());

        SP1KonaDataFetcher {
            l1_head: None,
            l2_output_root: None,
            l2_claim: None,
            l2_block_number: None,
            l2_head: None,
            // TODO: Determine how we want to handle this.
            l2_chain_id: 10,
            l1_node_address: l1_rpc,
            l1_beacon_address: l1_beacon_rpc,
            l2_node_address: l2_rpc,
        }
    }

    async fn find_block_by_timestamp(&mut self, target_timestamp: U256) -> Result<B256> {
        let l1_provider = Provider::<Http>::try_from(&self.l1_node_address)?;
        let latest_block = l1_provider.get_block(BlockNumber::Latest).await?.unwrap();
        let mut low = 0;
        let mut high = latest_block.number.unwrap().as_u64();

        while low <= high {
            let mid = (low + high) / 2;
            let block = l1_provider.get_block(mid).await?.unwrap();
            let block_timestamp = block.timestamp;

            if block_timestamp == target_timestamp {
                return Ok(block.hash.unwrap().0.into());
            } else if block_timestamp < target_timestamp {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }

        // Return the block hash of the closest block after the target timestamp
        let block = l1_provider.get_block(low).await?.unwrap();
        Ok(block.hash.unwrap().0.into())
    }

    pub async fn pull_block_data(&mut self, l2_block_num: u64) -> Result<()> {
        self.l2_block_number = Some(l2_block_num);

        let l2_provider = Provider::<Http>::try_from(&self.l2_node_address)?;
        let l2_block_safe_head = l2_block_num - 1;

        // Get L2 output data.
        let l2_output_block = l2_provider.get_block(l2_block_safe_head).await?.unwrap();
        let l2_output_state_root = l2_output_block.state_root;
        self.l2_head = Some(l2_output_block.hash.expect("L2 head is missing").0.into());

        let l2_output_storage_hash = l2_provider
            .get_proof(
                H160::from_str("0x4200000000000000000000000000000000000016")?,
                Vec::new(),
                Some(l2_block_safe_head.into()),
            )
            .await?
            .storage_hash;

        let l2_output_encoded = L2Output {
            num: l2_block_num,
            l2_state_root: l2_output_state_root.0.into(),
            l2_storage_hash: l2_output_storage_hash.0.into(),
            l2_head: self.l2_head.unwrap(),
        };
        self.l2_output_root = Some(keccak256(&l2_output_encoded.abi_encode()));

        // Get L2 claim data.
        let l2_claim_block = l2_provider.get_block(l2_block_num).await?.unwrap();
        let l2_claim_state_root = l2_claim_block.state_root;
        let l2_claim_hash = l2_claim_block.hash.expect("L2 claim hash is missing");
        let l2_claim_storage_hash = l2_provider
            .get_proof(
                H160::from_str("0x4200000000000000000000000000000000000016")?,
                Vec::new(),
                Some(l2_block_num.into()),
            )
            .await?
            .storage_hash;

        let l2_claim_encoded = L2Output {
            num: l2_block_num,
            l2_state_root: l2_claim_state_root.0.into(),
            l2_storage_hash: l2_claim_storage_hash.0.into(),
            l2_head: l2_claim_hash.0.into(),
        };
        self.l2_claim = Some(keccak256(&l2_claim_encoded.abi_encode()));

        // Get L1 head.
        let l2_block_timestamp = l2_claim_block.timestamp;
        let target_timestamp = l2_block_timestamp + 300;

        // TODO: Convert target_timestamp to a block number
        self.l1_head = Some(self.find_block_by_timestamp(target_timestamp).await?);

        Ok(())

    }

    pub fn get_boot_info(&self) -> BootInfoWithoutRollupConfig {
        BootInfoWithoutRollupConfig {
            l1_head: self.l1_head.unwrap(),
            l2_output_root: self.l2_output_root.unwrap(),
            l2_claim: self.l2_claim.unwrap(),
            l2_claim_block: self.l2_block_number.unwrap(),
            chain_id: self.l2_chain_id,
        }
    }

    pub fn get_host_cli(&self) -> HostCli {
        let data_directory = format!("./data/{}", self.l2_block_number.unwrap());
        fs::create_dir_all(&data_directory).unwrap();

        HostCli {
            l1_head: self.l1_head.unwrap(),
            l2_output_root: self.l2_output_root.unwrap(),
            l2_claim: self.l2_claim.unwrap(),
            l2_block_number: self.l2_block_number.unwrap(),
            l2_chain_id: self.l2_chain_id,
            l2_head: self.l2_head.unwrap(),
            l2_node_address: Some(self.l2_node_address.clone()),
            l1_node_address: Some(self.l1_node_address.clone()),
            l1_beacon_address: Some(self.l1_beacon_address.clone()),
            data_dir: Some(data_directory.into()),
            exec: Some("./target/release-client-lto/zkvm-client".to_string()),
            server: false,
            v: 4,
        }
    }
}
