pub mod block_range;
pub mod fetcher;
pub mod helpers;
pub mod rollup_config;
pub mod stats;

use alloy_consensus::Header;
use alloy_primitives::{map::HashMap, B256};
use alloy_provider::ReqwestProvider;
use alloy_sol_types::sol;
use anyhow::anyhow;
use anyhow::Result;
use async_trait::async_trait;
use kona_host::eth::http_provider;
use kona_host::single::SingleChainFetcher;
use kona_host::single::SingleChainProviders;
use kona_host::Fetcher;
use kona_host::HostOrchestrator;
use kona_host::KeyValueStore;
use kona_host::PreimageServer;
use kona_host::SharedKeyValueStore;
use kona_host::{single::SingleChainHostCli, DiskKeyValueStore, MemoryKeyValueStore};
use kona_preimage::BidirectionalChannel;
use kona_preimage::HintReader;
use kona_preimage::HintWriter;
use kona_preimage::NativeChannel;
use kona_preimage::OracleReader;
use kona_preimage::OracleServer;
use kona_providers_alloy::OnlineBeaconClient;
use kona_providers_alloy::OnlineBlobProvider;
use log::info;
use maili_genesis::RollupConfig;
use op_succinct_client_utils::{
    boot::BootInfoStruct, types::AggregationInputs, BootInfoWithBytesConfig, BytesHasherBuilder,
    InMemoryOracleData,
};
use rkyv::to_bytes;
use sp1_sdk::{HashableKey, SP1Proof, SP1Stdin};
use std::sync::Arc;
use std::{fs::File, io::Read};
use tokio::sync::RwLock;
use tokio::task;

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    contract L2OutputOracle {
        bytes32 public aggregationVkey;
        bytes32 public rangeVkeyCommitment;
        bytes32 public rollupConfigHash;

        function updateAggregationVKey(bytes32 _aggregationVKey) external onlyOwner;

        function updateRangeVkeyCommitment(bytes32 _rangeVkeyCommitment) external onlyOwner;
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ProgramType {
    Single,
    Multi,
}

sol! {
    struct L2Output {
        uint64 zero;
        bytes32 l2_state_root;
        bytes32 l2_storage_hash;
        bytes32 l2_claim_hash;
    }
}

/// Get the stdin to generate a proof for the given L2 claim.
pub fn get_proof_stdin(host_cli: &SingleChainHostCli) -> Result<SP1Stdin> {
    let mut stdin = SP1Stdin::new();

    // Read the rollup config.
    let mut rollup_config_file = File::open(host_cli.rollup_config_path.as_ref().unwrap())?;
    let mut rollup_config_bytes = Vec::new();
    rollup_config_file.read_to_end(&mut rollup_config_bytes)?;

    let ser_config = std::fs::read_to_string(host_cli.rollup_config_path.as_ref().unwrap())?;
    let rollup_config: RollupConfig = serde_json::from_str(&ser_config)?;

    let boot_info = BootInfoWithBytesConfig {
        l1_head: host_cli.l1_head,
        l2_output_root: host_cli.agreed_l2_output_root,
        l2_claim: host_cli.claimed_l2_output_root,
        l2_claim_block: host_cli.claimed_l2_block_number,
        chain_id: rollup_config.l2_chain_id,
        rollup_config_bytes,
    };
    stdin.write(&boot_info);

    println!("boot_info: {:?}", boot_info);

    // Get the disk KV store.
    let disk_kv_store = DiskKeyValueStore::new(host_cli.data_dir.clone().unwrap());

    println!("disk_kv_store: {:?}", disk_kv_store);

    // Convert the disk KV store to a memory KV store.
    let mem_kv_store: MemoryKeyValueStore = disk_kv_store.try_into().map_err(|_| {
        anyhow::anyhow!("Failed to convert DiskKeyValueStore to MemoryKeyValueStore")
    })?;

    // Convert the memory KV store to a HashMap<[u8;32], Vec<u8>>.
    let mut kv_store_map = HashMap::with_hasher(BytesHasherBuilder);
    for (k, v) in mem_kv_store.store {
        kv_store_map.insert(k.0, v);
    }

    // Serialize the underlying KV store.
    let buffer = to_bytes::<rkyv::rancor::Error>(&InMemoryOracleData { map: kv_store_map })?;

    let kv_store_bytes = buffer.into_vec();
    stdin.write_slice(&kv_store_bytes);

    Ok(stdin)
}

/// Get the stdin for the aggregation proof.
pub fn get_agg_proof_stdin(
    proofs: Vec<SP1Proof>,
    boot_infos: Vec<BootInfoStruct>,
    headers: Vec<Header>,
    multi_block_vkey: &sp1_sdk::SP1VerifyingKey,
    latest_checkpoint_head: B256,
) -> Result<SP1Stdin> {
    let mut stdin = SP1Stdin::new();
    for proof in proofs {
        let SP1Proof::Compressed(compressed_proof) = proof else {
            panic!();
        };
        stdin.write_proof(*compressed_proof, multi_block_vkey.vk.clone());
    }

    // Write the aggregation inputs to the stdin.
    stdin.write(&AggregationInputs {
        boot_infos,
        latest_l1_checkpoint_head: latest_checkpoint_head,
        multi_block_vkey: multi_block_vkey.hash_u32(),
    });
    // The headers have issues serializing with bincode, so use serde_json instead.
    let headers_bytes = serde_cbor::to_vec(&headers).unwrap();
    stdin.write_vec(headers_bytes);

    Ok(stdin)
}

/// The [OPSuccinctFetcher] struct is responsible for fetching preimages from a remote source.
#[derive(Debug)]
pub struct OPSuccinctFetcher<KV>
where
    KV: KeyValueStore + ?Sized,
{
    /// Key-value store for preimages.
    kv_store: Arc<RwLock<KV>>,
    /// L1 chain provider.
    l1_provider: ReqwestProvider,
    /// The blob provider
    blob_provider: OnlineBlobProvider<OnlineBeaconClient>,
    /// L2 chain provider.
    l2_provider: ReqwestProvider,
    /// L2 head
    l2_head: B256,
    /// The last hint that was received. [None] if no hint has been received yet.
    last_hint: Arc<RwLock<Option<String>>>,
}

/// Starts the [PreimageServer] and the client program in separate threads. The client program is
/// ran natively in this mode.
///
/// ## Takes
/// - `cfg`: The host configuration.
///
/// ## Returns
/// - `Ok(exit_code)` if the client program exits successfully.
/// - `Err(_)` if the client program failed to execute, was killed by a signal, or the host program
///   exited first.
pub async fn start_server_and_native_client(
    cfg: SingleChainHostCli,
) -> Result<MemoryKeyValueStore, anyhow::Error> {
    let hint_chan = BidirectionalChannel::new().map_err(|e| anyhow!(e))?;
    let preimage_chan = BidirectionalChannel::new().map_err(|e| anyhow!(e))?;

    let host_cli = OPSuccinctHostCli { inner: cfg };
    let disk_kv_store = DiskKeyValueStore::new(cfg.data_dir.clone().unwrap());
    let kv_store = Arc::new(RwLock::new(disk_kv_store));
    let providers = host_cli.create_providers().await?;
    let fetcher = host_cli.create_fetcher_with_disk_kv_store().await;

    let server_task = task::spawn(
        PreimageServer::new(
            OracleServer::new(preimage_chan.host),
            HintReader::new(hint_chan.host),
            kv_store,
            fetcher,
        )
        .start(),
    );

    let program_task = tokio::spawn(SingleChainHostCli::run_client_native(
        HintWriter::new(hint_chan.client),
        OracleReader::new(preimage_chan.client),
    ));

    info!("Starting preimage server and client program.");
    let (_, client_result) =
        tokio::try_join!(server_task, program_task,).map_err(|e| anyhow!(e))?;
    info!(target: "kona_host", "Preimage server and client program have joined.");

    // Loop over all of the keys in the KV store and convert to MemoryKeyValueStore.
    let mem_kv_store: MemoryKeyValueStore = kv_store.read().await.try_into()?;

    // Convert to MemoryKeyValueStore and return it
    Ok(mem_kv_store)
}

struct OPSuccinctHostCli {
    inner: SingleChainHostCli,
}

/// The providers required for the single chain host.
#[derive(Debug)]
struct OPSuccinctProviders {
    /// The L1 EL provider.
    pub l1_provider: ReqwestProvider,
    /// The L1 beacon node provider.
    pub blob_provider: OnlineBlobProvider<OnlineBeaconClient>,
    /// The L2 EL provider.
    pub l2_provider: ReqwestProvider,
}

#[async_trait]
impl HostOrchestrator for OPSuccinctHostCli {
    type Providers = OPSuccinctProviders;

    async fn create_providers(&self) -> Result<Option<Self::Providers>> {
        if self.inner.is_offline() {
            return Ok(None);
        }

        let l1_provider = http_provider(
            self.inner
                .l1_node_address
                .as_ref()
                .ok_or(anyhow!("Provider must be set"))?,
        );
        let blob_provider = OnlineBlobProvider::init(OnlineBeaconClient::new_http(
            self.inner
                .l1_beacon_address
                .clone()
                .ok_or(anyhow!("Beacon API URL must be set"))?,
        ))
        .await;
        let l2_provider = http_provider(
            self.inner
                .l2_node_address
                .as_ref()
                .ok_or(anyhow!("L2 node address must be set"))?,
        );

        Ok(Some(OPSuccinctProviders {
            l1_provider,
            blob_provider,
            l2_provider,
        }))
    }

    fn create_key_value_store(&self) -> Result<SharedKeyValueStore> {
        self.inner.create_key_value_store()
    }

    fn create_fetcher(
        &self,
        providers: Option<Self::Providers>,
        kv_store: SharedKeyValueStore,
    ) -> Option<Arc<RwLock<impl Fetcher + Send + Sync + 'static>>> {
        providers.map(|providers| {
            Arc::new(RwLock::new(SingleChainFetcher::new(
                kv_store,
                providers.l1_provider,
                providers.blob_provider,
                providers.l2_provider,
                self.inner.agreed_l2_head_hash,
            )))
        })
    }

    async fn run_client_native(
        hint_reader: HintWriter<NativeChannel>,
        oracle_reader: OracleReader<NativeChannel>,
    ) -> Result<()> {
        kona_client::single::run(oracle_reader, hint_reader, None)
            .await
            .map_err(Into::into)
    }
}

impl OPSuccinctHostCli {
    async fn create_fetcher_with_disk_kv_store(
        &self,
    ) -> Option<Arc<RwLock<impl Fetcher + Send + Sync + 'static>>> {
        let disk_kv_store = self.create_disk_kv_store().unwrap();
        let providers = self.create_providers().await.unwrap();
        providers.map(|providers| {
            Arc::new(RwLock::new(SingleChainFetcher::new(
                Arc::new(RwLock::new(disk_kv_store)),
                providers.l1_provider,
                providers.blob_provider,
                providers.l2_provider,
                self.inner.agreed_l2_head_hash,
            )))
        })
    }

    fn create_disk_kv_store(&self) -> Result<DiskKeyValueStore> {
        let disk_kv_store = DiskKeyValueStore::new(self.inner.data_dir.clone().unwrap());

        Ok(disk_kv_store)
    }
}
