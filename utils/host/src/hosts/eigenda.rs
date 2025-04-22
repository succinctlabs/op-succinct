use std::sync::Arc;

use alloy_primitives::B256;
use async_trait::async_trait;
use kona_preimage::BidirectionalChannel;
use alloy_eips::BlockId;
use op_succinct_client_utils::witness::WitnessData;

use crate::fetcher::OPSuccinctDataFetcher;
use crate::hosts::OPSuccinctHost;
use anyhow::Result;

use hokulea_host_bin::cfg::SingleChainHostWithEigenDA;

use kona_preimage::{HintWriter, NativeChannel, OracleReader};




use crate::witness_generation::eigenda_witness_gen::generate_opsuccinct_eigenda_witness;
use hokulea_proof::eigenda_provider::OracleEigenDAProvider;


use kona_proof::{l1::OracleBlobProvider, CachingOracle};

#[derive(Clone)]
pub struct EigenDAOPSuccinctHost {
    pub fetcher: Arc<OPSuccinctDataFetcher>,    
}

#[async_trait]
impl OPSuccinctHost for EigenDAOPSuccinctHost {
    type Args = SingleChainHostWithEigenDA;

    async fn run(&self, args: &Self::Args) -> Result<WitnessData> {
        
        let hint = BidirectionalChannel::new()?;
        let preimage = BidirectionalChannel::new()?;

        let server_task = args.start_server(hint.host, preimage.host).await?;
        let witness = Self::run_eigenda_witnessgen_client(preimage.client, hint.client).await?;
        // Unlike the upstream, manually abort the server task, as it will hang if you wait for both tasks to complete.
        server_task.abort();

        Ok(witness)
    }

    async fn fetch(
        &self,
        l2_start_block: u64,
        l2_end_block: u64,
        l1_head_hash: Option<B256>,
        safe_db_fallback: Option<bool>,
    ) -> Result<SingleChainHostWithEigenDA> {
        let host = self
            .fetcher
            .get_host_args(
                l2_start_block,
                l2_end_block,
                l1_head_hash,
                safe_db_fallback.expect("`safe_db_fallback` must be set"),
            )
            .await?;

        let eigenda_proxy_address = std::env::var("EIGENDA_PROXY_ADDRESS").ok();
        Ok(SingleChainHostWithEigenDA{
            kona_cfg: host,
            eigenda_proxy_address: eigenda_proxy_address,
            verbose: 1,
        })
    }

    async fn get_finalized_l2_block_number(
        &self,
        fetcher: &OPSuccinctDataFetcher,
        _: u64,
    ) -> Result<Option<u64>> {
        let finalized_l2_block_number = fetcher.get_l2_header(BlockId::finalized()).await?;
        Ok(Some(finalized_l2_block_number.number))
    }

    fn get_l1_head_hash(&self, args: &Self::Args) -> Option<B256> {
        Some(args.kona_cfg.l1_head)
    }
}

impl EigenDAOPSuccinctHost {
    pub fn new(fetcher: Arc<OPSuccinctDataFetcher>) -> Self {
        Self { fetcher }
    }

    /// Run the witness generation client.
    async fn run_eigenda_witnessgen_client(        
        preimage_chan: NativeChannel,
        hint_chan: NativeChannel,
    ) -> Result<WitnessData> {        
        // Instantiate oracles
        let preimage_oracle = Arc::new(CachingOracle::new(
            2048,
            OracleReader::new(preimage_chan),
            HintWriter::new(hint_chan),
        ));
        let blob_provider = OracleBlobProvider::new(preimage_oracle.clone());        

        let eigenda_blob_provider = OracleEigenDAProvider::new(preimage_oracle.clone());        

        let (_, witness) = generate_opsuccinct_eigenda_witness(preimage_oracle.clone(), blob_provider, eigenda_blob_provider).await?;    
                    
        Ok(witness)
    }
}



/*
/// Generate a witness with the given oracle and blob provider.
pub async fn generate_opsuccinct_witness<O, B, E>(
    preimage_oracle: Arc<O>,
    blob_provider: B,    
) -> Result<(BootInfo, WitnessData)>
where
    O: CommsClient + FlushableCache + Send + Sync + Debug,
    B: BlobProvider + Send + Sync + Debug + Clone,
    E: OracleEigenDAProvider + Send + Sync + Debug + Clone,
{
    let preimage_witness_store = Arc::new(Mutex::new(PreimageStore::default()));
    let blob_data = Arc::new(Mutex::new(BlobData::default()));

    let oracle = Arc::new(PreimageWitnessCollector {
        preimage_oracle: preimage_oracle.clone(),
        preimage_witness_store: preimage_witness_store.clone(),
    });
    let beacon = OnlineBlobStore { provider: blob_provider.clone(), store: blob_data.clone() };

    let boot = run_opsuccinct_eigenda_client(oracle, beacon, eigenda_blob_provider).await?;

    let witness = WitnessData {
        preimage_store: preimage_witness_store.lock().unwrap().clone(),
        blob_data: blob_data.lock().unwrap().clone(),
        eigenda_data: vec![],
    };

    Ok((boot, witness))
}
*/