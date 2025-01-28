use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use kona_host::{
    single::SingleChainHostCli, DetachedHostOrchestrator, Fetcher, HostOrchestrator,
    PreimageServer, SharedKeyValueStore,
};
use kona_preimage::{
    BidirectionalChannel, HintReader, HintWriter, NativeChannel, OracleReader, OracleServer,
};
use op_succinct_host_utils::{
    block_range::get_validated_block_range,
    fetcher::{CacheMode, OPSuccinctDataFetcher, RunContext},
    get_proof_stdin,
    stats::ExecutionStats,
    ProgramType,
};
use op_succinct_prove::{execute_multi, DEFAULT_RANGE, RANGE_ELF};
use op_succinct_scripts::HostExecutorArgs;
use sp1_sdk::{utils, ProverClient};
use std::{fs, sync::Arc};
use tokio::{sync::RwLock, task};

struct OPSuccinctHost {
    cli: SingleChainHostCli,
}

/// The host<->client communication channels. The client channels are optional, as the client may
/// not be running in the same process as the host.
#[derive(Debug)]
struct HostComms {
    /// The host<->client hint channel.
    pub hint: BidirectionalChannel,
    /// The host<->client preimage channel.
    pub preimage: BidirectionalChannel,
}

#[async_trait]
impl HostOrchestrator for OPSuccinctHost {
    type Providers = <SingleChainHostCli as HostOrchestrator>::Providers;

    async fn create_providers(&self) -> Result<Option<Self::Providers>> {
        self.cli.create_providers().await
    }

    fn create_key_value_store(&self) -> Result<SharedKeyValueStore> {
        self.cli.create_key_value_store()
    }

    fn create_fetcher(
        &self,
        providers: Option<Self::Providers>,
        kv_store: SharedKeyValueStore,
    ) -> Option<Arc<RwLock<impl Fetcher + Send + Sync + 'static>>> {
        self.cli.create_fetcher(providers, kv_store)
    }

    async fn run_client_native(
        hint_reader: HintWriter<NativeChannel>,
        oracle_reader: OracleReader<NativeChannel>,
    ) -> Result<()> {
        SingleChainHostCli::run_client_native(hint_reader, oracle_reader).await
    }

    /// Starts the host and client program in-process.
    async fn start(&self) -> Result<()> {
        let comms = HostComms {
            hint: BidirectionalChannel::new()?,
            preimage: BidirectionalChannel::new()?,
        };
        let kv_store = self.create_key_value_store()?;
        let providers = self.create_providers().await?;
        let fetcher = self.create_fetcher(providers, kv_store.clone());

        let server_task = task::spawn(
            PreimageServer::new(
                OracleServer::new(comms.preimage.host),
                HintReader::new(comms.hint.host),
                kv_store,
                fetcher,
            )
            .start(),
        );
        let client_task = task::spawn(Self::run_client_native(
            HintWriter::new(comms.hint.client),
            OracleReader::new(comms.preimage.client),
        ));

        let (_, client_result) = tokio::try_join!(server_task, client_task)?;

        Ok(())
    }
}

/// Execute the OP Succinct program for multiple blocks.
#[tokio::main]
async fn main() -> Result<()> {
    let args = HostExecutorArgs::parse();

    dotenv::from_path(&args.env_file)?;
    utils::setup_logger();

    let data_fetcher = OPSuccinctDataFetcher::new_with_rollup_config(RunContext::Dev).await?;

    let cache_mode = if args.use_cache {
        CacheMode::KeepCache
    } else {
        CacheMode::DeleteCache
    };

    // If the end block is provided, check that it is less than the latest finalized block. If the end block is not provided, use the latest finalized block.
    let (l2_start_block, l2_end_block) =
        get_validated_block_range(&data_fetcher, args.start, args.end, DEFAULT_RANGE).await?;

    let host_cli = data_fetcher
        .get_host_cli_args(l2_start_block, l2_end_block, ProgramType::Multi, cache_mode)
        .await?;

    let host = OPSuccinctHost {
        cli: host_cli.clone(),
    };

    println!("Running host CLI");

    host.start().await?;

    println!("Host CLI finished");
    drop(host);

    // Get the stdin for the block.
    let sp1_stdin = get_proof_stdin(&host_cli)?;

    let prover = ProverClient::from_env();

    if args.prove {
        // If the prove flag is set, generate a proof.
        let (pk, _) = prover.setup(RANGE_ELF);

        // Generate proofs in compressed mode for aggregation verification.
        let proof = prover.prove(&pk, &sp1_stdin).compressed().run().unwrap();

        // Create a proof directory for the chain ID if it doesn't exist.
        let proof_dir = format!(
            "data/{}/proofs",
            data_fetcher.get_l2_chain_id().await.unwrap()
        );
        if !std::path::Path::new(&proof_dir).exists() {
            fs::create_dir_all(&proof_dir).unwrap();
        }
        // Save the proof to the proof directory corresponding to the chain ID.
        proof
            .save(format!(
                "{}/{}-{}.bin",
                proof_dir, l2_start_block, l2_end_block
            ))
            .expect("saving proof failed");
    } else {
        let l2_chain_id = data_fetcher.get_l2_chain_id().await?;

        let (block_data, report, execution_duration) =
            execute_multi(&data_fetcher, sp1_stdin, l2_start_block, l2_end_block).await?;

        let stats = ExecutionStats::new(&block_data, &report, 0, execution_duration.as_secs());

        println!("Execution Stats: \n{:?}", stats);

        // Create the report directory if it doesn't exist.
        let report_dir = format!("execution-reports/multi/{}", l2_chain_id);
        if !std::path::Path::new(&report_dir).exists() {
            fs::create_dir_all(&report_dir)?;
        }

        let report_path = format!(
            "execution-reports/multi/{}/{}-{}.csv",
            l2_chain_id, l2_start_block, l2_end_block
        );

        // Write to CSV.
        let mut csv_writer = csv::Writer::from_path(report_path)?;
        csv_writer.serialize(&stats)?;
        csv_writer.flush()?;
    }

    Ok(())
}
