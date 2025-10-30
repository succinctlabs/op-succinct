use std::{env, fs, path::PathBuf, str::FromStr, sync::Arc};

use alloy_eips::{BlockId, BlockNumberOrTag};
use alloy_network::EthereumWallet;
use alloy_node_bindings::Anvil;
use alloy_primitives::{address, Address, U256};
use alloy_provider::{Provider, ProviderBuilder};
use alloy_signer_local::PrivateKeySigner;
use alloy_sol_types::SolValue;
use alloy_transport_http::reqwest::Url;
use anyhow::{Context, Result};
use clap::Parser;
use fault_proof::{
    contract::{DisputeGameFactory, OPSuccinctFaultDisputeGame, ProposalStatus},
    FactoryTrait,
};
use op_succinct_client_utils::boot::BootInfoStruct;
use op_succinct_elfs::AGGREGATION_ELF;
use op_succinct_host_utils::{
    fetcher::OPSuccinctDataFetcher,
    get_agg_proof_stdin,
    host::OPSuccinctHost,
    network::{determine_network_mode, get_network_signer, parse_fulfillment_strategy},
    witness_generation::WitnessGenerator,
};
use op_succinct_proof_utils::{get_range_elf_embedded, initialize_host};
use sp1_sdk::{utils, Prover, ProverClient, SP1ProofWithPublicValues};
use tracing::info;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The environment file path.
    #[arg(long, default_value = ".env")]
    env_file: PathBuf,

    /// The range proof.
    #[arg(long)]
    range_proof: Option<String>,

    /// The aggregation proof.
    #[arg(long)]
    agg_proof: Option<String>,
}

/// Preflight check for the OP Succinct Fault Dispute Game.
#[tokio::main]
async fn main() -> Result<()> {
    // 1. Set up the environment.
    utils::setup_logger();

    let args = Args::parse();

    dotenv::from_path(&args.env_file)
        .context(format!("Environment file not found: {}", args.env_file.display()))?;

    let data_fetcher = OPSuccinctDataFetcher::new_with_rollup_config().await?;

    let factory = DisputeGameFactory::new(
        env::var("FACTORY_ADDRESS")?.parse::<Address>().expect("FACTORY_ADDRESS must be set"),
        data_fetcher.l1_provider.clone(),
    );
    info!("Factory at address: {}", factory.address());

    let anchor_l2_block_number = factory
        .get_anchor_l2_block_number(
            env::var("GAME_TYPE")?.parse::<u32>().expect("GAME_TYPE must be set"),
        )
        .await?;
    info!("Anchor L2 block number: {}", anchor_l2_block_number);

    let l2_start_block = anchor_l2_block_number.to::<u64>();
    let l2_end_block = l2_start_block + 10;

    // Use finalized L1 block's hash as the L1 head hash since the factory
    // might have been deployed later than the safe L1 head of the L2 end block.
    let l1_head = {
        data_fetcher
            .get_l1_head(l2_end_block, false)
            .await
            .context("failed to fetch L1 head (pre-check)")?;

        let finalized_l1_header = data_fetcher
            .get_l1_header(BlockId::finalized())
            .await
            .context("failed to fetch finalized L1 header")?;
        info!("Finalized L1 block number: {}", finalized_l1_header.number);

        let set_impl_block_number =
            env::var("SET_IMPL_BLOCK")?.parse::<u64>().expect("SET_IMPL_BLOCK must be set");
        let l1_header_after_set_impl = data_fetcher
            .get_l1_header(BlockId::Number(BlockNumberOrTag::Number(set_impl_block_number + 1)))
            .await?;

        if l1_header_after_set_impl.number < finalized_l1_header.number {
            l1_header_after_set_impl
        } else {
            panic!("Set implementation L1 block number is greater than finalized L1 block number");
        }
    };

    let l1_head_hash = l1_head.hash_slow();
    info!("L1 head number: {:?}", l1_head.number);
    info!("L1 head hash: {:?}", l1_head_hash);

    // 2. Generate the range proof.
    let mut range_proof = if let Some(range_proof_name) = args.range_proof {
        let range_proof_path = format!(
            "data/{}/proofs/range/{}.bin",
            data_fetcher.get_l2_chain_id().await.unwrap(),
            range_proof_name
        );
        if !std::path::Path::new(&range_proof_path).exists() {
            panic!("Range proof file not found at path: {}", range_proof_path);
        }

        let range_proof = SP1ProofWithPublicValues::load(&range_proof_path)?;
        info!("Range proof loaded from path: {}", range_proof_path);

        range_proof
    } else {
        let host = initialize_host(Arc::new(data_fetcher.clone()));
        let host_args = host.fetch(l2_start_block, l2_end_block, Some(l1_head_hash), false).await?;

        info!("Generating range proof witness data...");
        let witness_data = host.run(&host_args).await?;
        info!("Range proof witness data generated successfully");

        info!("Getting range proof stdin...");
        let range_proof_stdin = host.witness_generator().get_sp1_stdin(witness_data)?;
        info!("Range proof stdin generated successfully");

        // Initialize the network prover.
        let network_signer = get_network_signer(
            env::var("USE_KMS_REQUESTER")?.parse::<bool>().expect("USE_KMS_REQUESTER must be set"),
        )
        .await?;
        let network_mode = determine_network_mode(
            parse_fulfillment_strategy(env::var("RANGE_PROOF_STRATEGY")?),
            parse_fulfillment_strategy(env::var("AGG_PROOF_STRATEGY")?),
        )
        .context("failed to determine network mode from range and agg fulfillment strategies")?;
        let network_prover =
            ProverClient::builder().network_for(network_mode).signer(network_signer).build();
        info!("Initialized network prover successfully");

        let (range_pk, _range_vk) = network_prover.setup(get_range_elf_embedded());
        let range_proof =
            network_prover.prove(&range_pk, &range_proof_stdin).compressed().run().unwrap();

        // Save the proof to the proof directory corresponding to the chain ID.
        let range_proof_dir =
            format!("data/{}/proofs/range", data_fetcher.get_l2_chain_id().await.unwrap());
        if !std::path::Path::new(&range_proof_dir).exists() {
            fs::create_dir_all(&range_proof_dir).unwrap();
        }
        range_proof
            .save(format!("{range_proof_dir}/{l2_start_block}-{l2_end_block}.bin"))
            .expect("saving proof failed");
        info!("Range proof saved to {range_proof_dir}/{l2_start_block}-{l2_end_block}.bin");

        range_proof
    };

    // 3. Generate the aggregation proof.
    let boot_info: BootInfoStruct = range_proof.public_values.read();

    let agg_proof = if let Some(agg_proof_name) = args.agg_proof {
        let agg_proof_path = format!(
            "data/{}/proofs/agg/{}.bin",
            data_fetcher.get_l2_chain_id().await.unwrap(),
            agg_proof_name
        );
        if !std::path::Path::new(&agg_proof_path).exists() {
            panic!("Aggregation proof file not found at path: {}", agg_proof_path);
        }

        let agg_proof = SP1ProofWithPublicValues::load(agg_proof_path.clone())?;
        info!("Aggregation proof loaded from path: {}", agg_proof_path);

        agg_proof
    } else {
        assert_eq!(boot_info.l1Head, l1_head_hash, "L1 head hash mismatch");

        // Initialize the network prover.
        let network_signer = get_network_signer(
            env::var("USE_KMS_REQUESTER")?.parse::<bool>().expect("USE_KMS_REQUESTER must be set"),
        )
        .await?;
        let network_mode = determine_network_mode(
            parse_fulfillment_strategy(env::var("RANGE_PROOF_STRATEGY")?),
            parse_fulfillment_strategy(env::var("AGG_PROOF_STRATEGY")?),
        )
        .context("failed to determine network mode from range and agg fulfillment strategies")?;
        let network_prover =
            ProverClient::builder().network_for(network_mode).signer(network_signer).build();
        info!("Initialized network prover successfully");

        let (_, range_vk) = network_prover.setup(get_range_elf_embedded());

        let agg_proof_stdin = get_agg_proof_stdin(
            vec![range_proof.proof],
            vec![boot_info.clone()],
            vec![l1_head.clone()],
            &range_vk,
            boot_info.l1Head,
            address!("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"), /* prover_address: anvil
                                                                     * account 0 */
        )
        .expect("failed to get agg proof stdin");

        let (agg_pk, _) = network_prover.setup(AGGREGATION_ELF);
        let agg_proof = network_prover.prove(&agg_pk, &agg_proof_stdin).plonk().run().unwrap();

        let agg_proof_dir =
            format!("data/{}/proofs/agg", data_fetcher.get_l2_chain_id().await.unwrap());
        if !std::path::Path::new(&agg_proof_dir).exists() {
            fs::create_dir_all(&agg_proof_dir).unwrap();
        }

        agg_proof.save(format!("{agg_proof_dir}/agg.bin")).expect("saving proof failed");
        info!("Agg proof saved to {agg_proof_dir}/agg.bin");

        agg_proof
    };

    // 4. Spin up anvil.
    let l1_head_number =
        data_fetcher.l1_provider.get_block_by_hash(boot_info.l1Head).await?.unwrap().header.number;

    let anvil = Anvil::new()
        .fork(env::var("L1_RPC").expect("L1_RPC must be set"))
        .fork_block_number(l1_head_number)
        .args(["--no-mining"]);
    let anvil_instance = anvil.spawn();
    let endpoint = anvil_instance.endpoint();
    info!("Anvil chain started forked from L1 block number: {} at: {}", l1_head_number, endpoint);

    // 5. Run the preflight check.
    let wallet = PrivateKeySigner::from_str(
        "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80", /* private key of anvil account 0 */
    )?;
    let provider_with_signer = ProviderBuilder::new()
        .wallet(EthereumWallet::from(wallet))
        .connect_http(Url::parse(&endpoint)?);

    let factory = DisputeGameFactory::new(*factory.address(), provider_with_signer.clone());

    let game_type = env::var("GAME_TYPE")?.parse::<u32>().expect("GAME_TYPE must be set");
    let init_bond = factory.initBonds(game_type).call().await?;
    let parent_index = u32::MAX;
    let extra_data = (U256::from(l2_end_block), parent_index).abi_encode_packed();

    let tx = factory
        .create(game_type, boot_info.l2PostRoot, extra_data.into())
        .value(init_bond)
        .send()
        .await?;

    let client = provider_with_signer.client();
    let _: String = client.request("evm_mine", Vec::<serde_json::Value>::new()).await?;

    let block = provider_with_signer.get_block_by_number(BlockNumberOrTag::Latest).await?;
    info!("Mined block: {}", block.unwrap().header.number);

    let receipt = tx.get_receipt().await?;
    info!("Transaction receipt: {:?}", receipt);

    let new_game_count = factory.gameCount().call().await?;
    let game_index = new_game_count - U256::from(1);
    let game_info = factory.gameAtIndex(game_index).call().await?;
    let game_address = game_info.proxy;
    info!("Game address: {}", game_address);

    let game = OPSuccinctFaultDisputeGame::new(game_address, provider_with_signer.clone());
    let tx = game.prove(agg_proof.bytes().into()).send().await?;

    let _: String = client.request("evm_mine", Vec::<serde_json::Value>::new()).await?;

    let block = provider_with_signer.get_block_by_number(BlockNumberOrTag::Latest).await?;
    info!("Mined block: {}", block.unwrap().header.number);

    let receipt = tx.get_receipt().await?;
    info!("Transaction receipt: {:?}", receipt);

    let claim_data = game.claimData().call().await?;
    assert_eq!(claim_data.status, ProposalStatus::UnchallengedAndValidProofProvided);

    info!("Successfully completed preflight check");

    Ok(())
}
