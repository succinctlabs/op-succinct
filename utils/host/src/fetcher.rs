use alloy::{
    eips::{BlockId, BlockNumberOrTag},
    primitives::{Address, B256},
    providers::{Provider, ProviderBuilder, RootProvider},
    transports::http::{reqwest::Url, Client, Http},
};
use alloy_consensus::Header;
use alloy_sol_types::SolValue;
use anyhow::anyhow;
use anyhow::Result;
use cargo_metadata::MetadataCommand;
use kona_host::HostCli;
use op_alloy_genesis::RollupConfig;
use op_alloy_network::{
    primitives::{BlockTransactions, BlockTransactionsKind},
    Optimism,
};
use op_alloy_protocol::calculate_tx_l1_cost_fjord;
use op_alloy_rpc_types::{
    output::OutputResponse, safe_head::SafeHeadResponse, OpTransactionReceipt,
};
use op_succinct_client_utils::boot::BootInfoStruct;
use serde_json::{json, Value};
use sp1_sdk::block_on;
use std::{
    cmp::Ordering, collections::HashMap, env, fs, path::Path, str::FromStr, sync::Arc,
    time::Duration,
};
use tokio::time::sleep;

use alloy_primitives::{keccak256, Bytes, U256};

use crate::{
    rollup_config::{get_rollup_config_path, merge_rollup_config, save_rollup_config},
    L2Output, ProgramType,
};

#[derive(Clone)]
/// The OPSuccinctDataFetcher struct is used to fetch the L2 output data and L2 claim data for a
/// given block number. It is used to generate the boot info for the native host program.
/// TODO: Add retries for all requests (3 retries).
/// TODO: We can generify some of these methods based on the Network (Ethereum, Optimism, etc.) types.
pub struct OPSuccinctDataFetcher {
    pub rpc_config: RPCConfig,
    pub l1_provider: Arc<RootProvider<Http<Client>>>,
    pub l2_provider: Arc<RootProvider<Http<Client>, Optimism>>,
    pub rollup_config: RollupConfig,
    pub l1_block_time_secs: u64,
}

impl Default for OPSuccinctDataFetcher {
    fn default() -> Self {
        block_on(OPSuccinctDataFetcher::new())
    }
}

#[derive(Debug, Clone, Default)]
pub struct RPCConfig {
    pub l1_rpc: String,
    pub l1_beacon_rpc: String,
    pub l2_rpc: String,
    pub l2_node_rpc: String,
}

/// The mode corresponding to the chain we are fetching data for.
#[derive(Clone, Copy)]
pub enum RPCMode {
    L1,
    L1Beacon,
    L2,
    L2Node,
}

/// Whether to keep the cache or delete the cache.
#[derive(Clone, Copy)]
pub enum CacheMode {
    KeepCache,
    DeleteCache,
}

fn get_rpcs() -> RPCConfig {
    RPCConfig {
        l1_rpc: env::var("L1_RPC").unwrap_or_else(|_| "http://localhost:8545".to_string()),
        l1_beacon_rpc: env::var("L1_BEACON_RPC")
            .unwrap_or_else(|_| "http://localhost:5052".to_string()),
        l2_rpc: env::var("L2_RPC").unwrap_or_else(|_| "http://localhost:9545".to_string()),
        l2_node_rpc: env::var("L2_NODE_RPC")
            .unwrap_or_else(|_| "http://localhost:5058".to_string()),
    }
}

/// The info to fetch for a block.
pub struct BlockInfo {
    pub block_number: u64,
    pub transaction_count: u64,
    pub gas_used: u64,
    pub l1_gas_cost: U256,
}

/// The fee data for a block.
pub struct FeeData {
    pub block_number: u64,
    pub tx_index: u64,
    pub tx_hash: B256,
    pub l1_gas_cost: U256,
}

impl OPSuccinctDataFetcher {
    /// Gets the RPC URL's and saves the rollup config for the chain to the rollup config file.
    pub async fn new() -> Self {
        let rpc_config = get_rpcs();

        let l1_provider = Arc::new(
            ProviderBuilder::default().on_http(Url::from_str(&rpc_config.l1_rpc).unwrap()),
        );
        let l2_provider = Arc::new(
            ProviderBuilder::default().on_http(Url::from_str(&rpc_config.l2_rpc).unwrap()),
        );

        let mut fetcher = OPSuccinctDataFetcher {
            rpc_config,
            l1_provider,
            l2_provider,
            rollup_config: RollupConfig::default(),
            // Default L1 block time for most Ethereum chains.
            l1_block_time_secs: 12,
        };

        // Get the L1 block time.
        let l1_block_time_secs = fetcher
            .get_l1_block_time()
            .await
            .expect("Failed to get L1 block time. Make sure that the L1 RPC is active.");
        fetcher.l1_block_time_secs = l1_block_time_secs;

        // Load and save the rollup config.
        let rollup_config = fetcher
            .fetch_rollup_config()
            .await
            .expect("Failed to fetch rollup config");
        save_rollup_config(&rollup_config).expect("Failed to save rollup config");
        fetcher.rollup_config = rollup_config;

        fetcher
    }

    pub async fn get_l2_chain_id(&self) -> Result<u64> {
        Ok(self.l2_provider.get_chain_id().await?)
    }

    pub async fn get_l2_head(&self) -> Header {
        self.l2_provider
            .get_block_by_number(BlockNumberOrTag::Latest, false)
            .await
            .unwrap()
            .unwrap()
            .header
            .try_into()
            .unwrap()
    }

    pub async fn get_l2_header_by_number(&self, block_number: u64) -> Header {
        self.l2_provider
            .get_block_by_number(block_number.into(), false)
            .await
            .unwrap()
            .unwrap()
            .header
            .try_into()
            .unwrap()
    }

    /// Manually calculate the L1 fee data for a range of blocks. Allows for modifying the L1 fee scalar.
    pub async fn get_l2_fee_data_with_modified_l1_fee_scalar(
        &self,
        start: u64,
        end: u64,
        custom_l1_fee_scalar: Option<U256>,
    ) -> Result<Vec<FeeData>> {
        use futures::stream::{self, StreamExt};

        // Fetch all tranasctions in parallel.
        // Return a tuple of the block number and the transactions.
        let transactions: Vec<(u64, Vec<B256>)> = stream::iter(start..=end)
            .map(|block_number| async move {
                let block = self
                    .l2_provider
                    .get_block(block_number.into(), BlockTransactionsKind::Hashes)
                    .await?
                    .unwrap();
                match block.transactions {
                    BlockTransactions::Hashes(txs) => Ok((block_number, txs)),
                    _ => Err(anyhow::anyhow!("Unsupported transaction type")),
                }
            })
            .buffered(100)
            .collect::<Vec<Result<(u64, Vec<B256>)>>>()
            .await
            .into_iter()
            .filter_map(Result::ok)
            .collect();

        // Create a map of the block number to the transactions.
        let block_number_to_transactions: HashMap<u64, Vec<B256>> =
            transactions.into_iter().collect();

        // Fetch all of the L1 block receipts in parallel.
        let block_receipts: Vec<(u64, Vec<OpTransactionReceipt>)> = stream::iter(start..=end)
            .map(|block_number| async move {
                (
                    block_number,
                    self.l2_provider
                        .get_block_receipts(block_number.into())
                        .await
                        .unwrap()
                        .unwrap(),
                )
            })
            .buffered(100)
            .collect::<Vec<(u64, Vec<OpTransactionReceipt>)>>()
            .await;

        // Get all the encoded transactions for each block number in parallel.
        let block_number_to_encoded_transactions = stream::iter(block_number_to_transactions)
            .map(|(block_number, transactions)| async move {
                let encoded_transactions = stream::iter(transactions)
                    .map(|tx_hash| async move {
                        self.l2_provider
                            .client()
                            .request::<&[B256; 1], Bytes>("debug_getRawTransaction", &[tx_hash])
                            .await
                            .map_err(|e| anyhow!("Error fetching transaction: {e}"))
                            .unwrap()
                    })
                    .buffered(100)
                    .collect::<Vec<Bytes>>()
                    .await;
                (block_number, encoded_transactions)
            })
            .buffered(100)
            .collect::<HashMap<u64, Vec<Bytes>>>()
            .await;

        // Zip the block number to encoded transactions with the block number to receipts.
        let block_number_to_receipts_and_transactions: HashMap<
            u64,
            (Vec<OpTransactionReceipt>, Vec<Bytes>),
        > = block_receipts
            .into_iter()
            .filter_map(|(block_number, receipts)| {
                block_number_to_encoded_transactions
                    .get(&block_number)
                    .map(|transactions| (block_number, (receipts, transactions.clone())))
            })
            .collect();

        let mut fee_data = Vec::new();
        for (block_number, (receipts, transactions)) in block_number_to_receipts_and_transactions {
            for (transaction, receipt) in transactions.iter().zip(receipts) {
                let l1_fee_scalar = if let Some(custom_l1_fee_scalar) = custom_l1_fee_scalar {
                    custom_l1_fee_scalar
                } else {
                    U256::from(receipt.l1_block_info.l1_base_fee_scalar.unwrap_or(0))
                };
                // Get the Fjord L1 cost of the transaction.
                let l1_gas_cost = calculate_tx_l1_cost_fjord(
                    transaction.as_ref(),
                    U256::from(receipt.l1_block_info.l1_gas_price.unwrap_or(0)),
                    l1_fee_scalar,
                    U256::from(receipt.l1_block_info.l1_blob_base_fee.unwrap_or(0)),
                    U256::from(receipt.l1_block_info.l1_blob_base_fee_scalar.unwrap_or(0)),
                );

                fee_data.push(FeeData {
                    block_number,
                    tx_index: receipt.inner.transaction_index.unwrap(),
                    tx_hash: receipt.inner.transaction_hash,
                    l1_gas_cost,
                });
            }
        }
        Ok(fee_data)
    }

    /// Get the fee data for a range of blocks. Extracts the l1 fee data from the receipts.
    pub async fn get_l2_fee_data_range(&self, start: u64, end: u64) -> Result<Vec<FeeData>> {
        let l2_provider = self.l2_provider.clone();

        use futures::stream::{self, StreamExt};

        // Only fetch 100 receipts at a time to better use system resources. Increases stability.
        let fee_data = stream::iter(start..=end)
            .map(|block_number| {
                let l2_provider = l2_provider.clone();
                async move {
                    let receipt = l2_provider
                        .get_block_receipts(block_number.into())
                        .await
                        .unwrap();
                    let transactions = receipt.unwrap();
                    let block_fee_data: Vec<FeeData> = transactions
                        .iter()
                        .enumerate()
                        .map(|(tx_index, tx)| FeeData {
                            block_number,
                            tx_index: tx_index as u64,
                            tx_hash: tx.inner.transaction_hash,
                            l1_gas_cost: U256::from(tx.l1_block_info.l1_fee.unwrap_or(0)),
                        })
                        .collect();
                    block_fee_data
                }
            })
            .buffered(100)
            .collect::<Vec<_>>()
            .await
            .into_iter()
            .flatten()
            .collect();
        Ok(fee_data)
    }

    /// Get the aggregate block statistics for a range of blocks.
    pub async fn get_l2_block_data_range(&self, start: u64, end: u64) -> Result<Vec<BlockInfo>> {
        use futures::stream::{self, StreamExt};

        let l2_provider = self.l2_provider.clone();
        let block_data = stream::iter(start..=end)
            .map(|block_number| {
                let l2_provider = l2_provider.clone();
                async move {
                    let block = l2_provider
                        .get_block_by_number(block_number.into(), false)
                        .await?
                        .unwrap();
                    let receipts = l2_provider
                        .get_block_receipts(block_number.into())
                        .await?
                        .unwrap();
                    let l1_gas_cost: U256 = receipts
                        .iter()
                        .map(|tx| U256::from(tx.l1_block_info.l1_fee.unwrap_or(0)))
                        .sum();
                    Ok(BlockInfo {
                        block_number,
                        transaction_count: block.transactions.len() as u64,
                        gas_used: block.header.gas_used,
                        l1_gas_cost,
                    })
                }
            })
            .buffered(100)
            .collect::<Vec<Result<BlockInfo>>>()
            .await;

        block_data.into_iter().collect()
    }

    pub async fn get_l1_header(&self, block_number: BlockId) -> Result<Header> {
        Ok(self
            .l1_provider
            .get_block(block_number, alloy::rpc::types::BlockTransactionsKind::Full)
            .await?
            .unwrap()
            .header
            .try_into()
            .unwrap())
    }

    pub async fn get_l2_header(&self, block_number: BlockId) -> Result<Header> {
        Ok(self
            .l2_provider
            .get_block(block_number, BlockTransactionsKind::Full)
            .await?
            .unwrap()
            .header
            .try_into()
            .unwrap())
    }

    pub async fn find_l1_block_hash_by_timestamp(&self, target_timestamp: u64) -> Result<B256> {
        let latest_block = self
            .l1_provider
            .get_block(BlockId::latest(), BlockTransactionsKind::Hashes)
            .await?
            .unwrap();
        let mut low = 0;
        let mut high = latest_block.header.number;

        while low <= high {
            let mid = (low + high) / 2;
            let block = self
                .l1_provider
                .get_block(mid.into(), BlockTransactionsKind::Hashes)
                .await?
                .unwrap();
            let block_timestamp = block.header.timestamp;

            match block_timestamp.cmp(&target_timestamp) {
                Ordering::Equal => return Ok(block.header.hash.0.into()),
                Ordering::Less => low = mid + 1,
                Ordering::Greater => high = mid - 1,
            }
        }

        // Return the block hash of the closest block after the target timestamp
        let block = self
            .l1_provider
            .get_block(low.into(), BlockTransactionsKind::Hashes)
            .await?
            .unwrap();
        Ok(block.header.hash.0.into())
    }

    /// Get the RPC URL for the given RPC mode.
    pub fn get_rpc_url(&self, rpc_mode: RPCMode) -> String {
        match rpc_mode {
            RPCMode::L1 => self.rpc_config.l1_rpc.clone(),
            RPCMode::L2 => self.rpc_config.l2_rpc.clone(),
            RPCMode::L1Beacon => self.rpc_config.l1_beacon_rpc.clone(),
            RPCMode::L2Node => self.rpc_config.l2_node_rpc.clone(),
        }
    }

    /// Fetch the rollup config. Combines the rollup config from `optimism_rollupConfig` and the
    /// chain config from `debug_chainConfig`.
    pub async fn fetch_rollup_config(&self) -> Result<RollupConfig> {
        let rollup_config = self
            .fetch_rpc_data(RPCMode::L2Node, "optimism_rollupConfig", vec![])
            .await?;
        let chain_config = self
            .fetch_rpc_data(RPCMode::L2, "debug_chainConfig", vec![])
            .await?;
        merge_rollup_config(&rollup_config, &chain_config)
    }

    /// Fetch arbitrary data from the RPC.
    pub async fn fetch_rpc_data<T>(
        &self,
        rpc_mode: RPCMode,
        method: &str,
        params: Vec<Value>,
    ) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let client = reqwest::Client::new();
        let response = client
            .post(self.get_rpc_url(rpc_mode))
            .json(&json!({
                "jsonrpc": "2.0",
                "method": method,
                "params": params,
                "id": 1
            }))
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        // Check for RPC error from the JSON RPC response.
        if let Some(error) = response.get("error") {
            let error_message = error["message"].as_str().unwrap_or("Unknown error");
            return Err(anyhow::anyhow!("RPC error: {}", error_message));
        }

        serde_json::from_value(response["result"].clone()).map_err(Into::into)
    }

    /// Get the earliest L1 header in a batch of boot infos.
    pub async fn get_earliest_l1_head_in_batch(
        &self,
        boot_infos: &Vec<BootInfoStruct>,
    ) -> Result<Header> {
        let mut earliest_block_num: u64 = u64::MAX;
        let mut earliest_l1_header: Option<Header> = None;

        for boot_info in boot_infos {
            let l1_block_header = self.get_l1_header(boot_info.l1Head.into()).await?;
            if l1_block_header.number < earliest_block_num {
                earliest_block_num = l1_block_header.number;
                earliest_l1_header = Some(l1_block_header);
            }
        }
        Ok(earliest_l1_header.unwrap())
    }

    /// Fetch headers for a range of blocks inclusive.
    pub async fn fetch_headers_in_range(&self, start: u64, end: u64) -> Result<Vec<Header>> {
        let mut headers: Vec<Header> = Vec::with_capacity((end - start + 1).try_into().unwrap());

        // Note: Node rate limits at 300 requests per second.
        let batch_size = 200;
        let mut block_number = start;
        while block_number <= end {
            let batch_end = block_number + batch_size - 1;
            let batch_headers: Vec<Header> = futures::future::join_all(
                (block_number..=batch_end.min(end)).map(|num| self.get_l1_header(num.into())),
            )
            .await
            .into_iter()
            .map(|header| header.unwrap())
            .collect();

            headers.extend(batch_headers);
            block_number += batch_size;
            sleep(Duration::from_millis(1500)).await;
        }
        Ok(headers)
    }

    /// Get the preimages for the headers corresponding to the boot infos. Specifically, fetch the
    /// headers corresponding to the boot infos and the latest L1 head.
    pub async fn get_header_preimages(
        &self,
        boot_infos: &Vec<BootInfoStruct>,
        checkpoint_block_hash: B256,
    ) -> Result<Vec<Header>> {
        // Get the earliest L1 Head from the boot_infos.
        let start_header = self.get_earliest_l1_head_in_batch(boot_infos).await?;

        // Fetch the full header for the latest L1 Head (which is validated on chain).
        let latest_header = self.get_l1_header(checkpoint_block_hash.into()).await?;

        // Create a vector of futures for fetching all headers
        let headers = self
            .fetch_headers_in_range(start_header.number, latest_header.number)
            .await?;

        Ok(headers)
    }

    /// Get the L2 output data for a given block number and save the boot info to a file in the data
    /// directory with block_number. Return the arguments to be passed to the native host for
    /// datagen.
    pub async fn get_host_cli_args(
        &self,
        l2_start_block: u64,
        l2_end_block: u64,
        multi_block: ProgramType,
        cache_mode: CacheMode,
    ) -> Result<HostCli> {
        if l2_start_block >= l2_end_block {
            return Err(anyhow::anyhow!(
                "L2 start block is greater than or equal to L2 end block. Start: {}, End: {}",
                l2_start_block,
                l2_end_block
            ));
        }

        let l2_provider = self.l2_provider.clone();

        // Get L2 output data.
        let l2_output_block = l2_provider
            .get_block_by_number(l2_start_block.into(), false)
            .await?
            .ok_or_else(|| {
                anyhow::anyhow!("Block not found for block number {}", l2_start_block)
            })?;
        let l2_output_state_root = l2_output_block.header.state_root;
        let agreed_l2_head_hash = l2_output_block.header.hash;
        let l2_output_storage_hash = l2_provider
            .get_proof(
                Address::from_str("0x4200000000000000000000000000000000000016")?,
                Vec::new(),
            )
            .block_id(l2_start_block.into())
            .await?
            .storage_hash;

        let l2_output_encoded = L2Output {
            zero: 0,
            l2_state_root: l2_output_state_root.0.into(),
            l2_storage_hash: l2_output_storage_hash.0.into(),
            l2_claim_hash: agreed_l2_head_hash.0.into(),
        };
        let agreed_l2_output_root = keccak256(l2_output_encoded.abi_encode());

        // Get L2 claim data.
        let l2_claim_block = l2_provider
            .get_block_by_number(l2_end_block.into(), false)
            .await?
            .unwrap();
        let l2_claim_state_root = l2_claim_block.header.state_root;
        let l2_claim_hash = l2_claim_block.header.hash;
        let l2_claim_storage_hash = l2_provider
            .get_proof(
                Address::from_str("0x4200000000000000000000000000000000000016")?,
                Vec::new(),
            )
            .block_id(l2_end_block.into())
            .await?
            .storage_hash;

        let l2_claim_encoded = L2Output {
            zero: 0,
            l2_state_root: l2_claim_state_root.0.into(),
            l2_storage_hash: l2_claim_storage_hash.0.into(),
            l2_claim_hash: l2_claim_hash.0.into(),
        };
        let claimed_l2_output_root = keccak256(l2_claim_encoded.abi_encode());

        let l1_head = self.get_l1_head(l2_end_block).await?;

        // Get the workspace root, which is where the data directory is.
        let metadata = MetadataCommand::new().exec().unwrap();
        let workspace_root = metadata.workspace_root;
        let data_directory = match multi_block {
            ProgramType::Single => {
                let proof_dir = format!(
                    "{}/data/{}/single/{}",
                    workspace_root, self.rollup_config.l2_chain_id, l2_end_block
                );
                proof_dir
            }
            ProgramType::Multi => {
                let proof_dir = format!(
                    "{}/data/{}/multi/{}-{}",
                    workspace_root, self.rollup_config.l2_chain_id, l2_start_block, l2_end_block
                );
                proof_dir
            }
        };

        // The native programs are built with profile release-client-lto in build.rs
        let exec_directory = match multi_block {
            ProgramType::Single => {
                format!("{}/target/release-client-lto/fault-proof", workspace_root)
            }
            ProgramType::Multi => format!("{}/target/release-client-lto/range", workspace_root),
        };

        // Delete the data directory if the cache mode is DeleteCache.
        match cache_mode {
            CacheMode::KeepCache => (),
            CacheMode::DeleteCache => {
                if Path::new(&data_directory).exists() {
                    fs::remove_dir_all(&data_directory)?;
                }
            }
        }

        // Create the path to the rollup config file.
        let rollup_config_path = get_rollup_config_path(self.rollup_config.l2_chain_id)?;

        // Creates the data directory if it doesn't exist, or no-ops if it does. Used to store the
        // witness data.
        fs::create_dir_all(&data_directory)?;

        Ok(HostCli {
            l1_head,
            agreed_l2_output_root,
            agreed_l2_head_hash,
            claimed_l2_output_root,
            claimed_l2_block_number: l2_end_block,
            l2_chain_id: None,
            l2_node_address: Some(self.rpc_config.l2_rpc.clone()),
            l1_node_address: Some(self.rpc_config.l1_rpc.clone()),
            l1_beacon_address: Some(self.rpc_config.l1_beacon_rpc.clone()),
            data_dir: Some(data_directory.into()),
            exec: Some(exec_directory),
            server: false,
            rollup_config_path: Some(rollup_config_path),
            v: std::env::var("VERBOSITY")
                .unwrap_or("0".to_string())
                .parse()
                .unwrap(),
        })
    }

    /// Get the L1 block time in seconds.
    async fn get_l1_block_time(&self) -> Result<u64> {
        let l1_head = self.get_l1_header(BlockId::latest()).await?;

        let l1_head_minus_1 = l1_head.number - 1;
        let l1_block_minus_1 = self.get_l1_header(l1_head_minus_1.into()).await?;
        Ok(l1_head.timestamp - l1_block_minus_1.timestamp)
    }

    /// Get the L1 block from which the `l2_end_block` can be derived.
    async fn get_l1_head_with_safe_head(&self, l2_end_block: u64) -> Result<B256> {
        let latest_l1_header = self.get_l1_header(BlockId::latest()).await?;

        // Get the l1 origin of the l2 end block.
        let l2_end_block_hex = format!("0x{:x}", l2_end_block);
        let optimism_output_data: OutputResponse = self
            .fetch_rpc_data(
                RPCMode::L2Node,
                "optimism_outputAtBlock",
                vec![l2_end_block_hex.into()],
            )
            .await?;

        let l1_origin = optimism_output_data.block_ref.l1_origin;

        // Search forward from the l1Origin, skipping forward in 5 minute increments until an L1 block with an L2 safe head greater than the l2_end_block is found.
        let mut current_l1_block_number = l1_origin.number;
        loop {
            // If the current L1 block number is greater than the latest L1 header number, then return an error.
            if current_l1_block_number > latest_l1_header.number {
                return Err(anyhow::anyhow!(
                    "Could not find an L1 block with an L2 safe head greater than the L2 end block."
                ));
            }

            let l1_block_number_hex = format!("0x{:x}", current_l1_block_number);
            let result: SafeHeadResponse = self
                .fetch_rpc_data(
                    RPCMode::L2Node,
                    "optimism_safeHeadAtL1Block",
                    vec![l1_block_number_hex.into()],
                )
                .await?;
            let l2_safe_head = result.safe_head.number;
            if l2_safe_head > l2_end_block {
                return Ok(result.l1_block.hash);
            }

            // Move forward in 5 minute increments.
            const SKIP_MINS: u64 = 5;
            current_l1_block_number += SKIP_MINS * (60 / self.l1_block_time_secs);
        }
    }

    /// For OP Sepolia, OP Mainnet and Base, the batcher posts at least every 10 minutes. Otherwise,
    /// the batcher may post as infrequently as every couple hours. The l1Head is set as the l1 block from which all of the
    /// relevant L2 block data can be derived.
    /// E.g. Origin Advance Error: BlockInfoFetch(Block number past L1 head.).
    async fn get_l1_head(&self, l2_end_block: u64) -> Result<B256> {
        // See if optimism_safeHeadAtL1Block is available. If there's an error, then estimate the L1 block necessary based on the chain config.
        let result = self.get_l1_head_with_safe_head(l2_end_block).await;

        if let Ok(safe_head_at_l1_block) = result {
            Ok(safe_head_at_l1_block)
        } else {
            // Estimate the L1 block necessary based on the chain config. This is based on the maximum
            // delay between batches being posted on the L2 chain.
            let max_batch_post_delay_minutes = match self.rollup_config.l2_chain_id {
                11155420 => 10,
                10 => 10,
                8453 => 10,
                _ => 60,
            };

            // Get L1 head.
            let l2_block_timestamp = self.get_l2_header(l2_end_block.into()).await?.timestamp;

            let target_timestamp = l2_block_timestamp + (max_batch_post_delay_minutes * 60);
            Ok(self
                .find_l1_block_hash_by_timestamp(target_timestamp)
                .await?)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::fetcher::OPSuccinctDataFetcher;

    #[tokio::test]
    #[cfg(test)]
    async fn test_get_l1_head() {
        use alloy::eips::BlockId;

        dotenv::dotenv().ok();
        let fetcher = OPSuccinctDataFetcher::new().await;
        let latest_l2_block = fetcher.get_l2_header(BlockId::latest()).await.unwrap();

        // Get the L2 block number from 1 hour ago.
        let l2_end_block = latest_l2_block.number - ((60 * 60) / fetcher.rollup_config.block_time);

        let _ = fetcher.get_l1_head(l2_end_block).await.unwrap();
    }
}
