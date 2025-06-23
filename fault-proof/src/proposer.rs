use std::{env, sync::Arc, time::Duration};

use alloy_primitives::{Address, TxHash, U256};
use alloy_provider::{Provider, ProviderBuilder};
use alloy_sol_types::{SolEvent, SolValue};
use anyhow::{Context, Result};
use op_succinct_client_utils::boot::BootInfoStruct;
use op_succinct_elfs::AGGREGATION_ELF;
use op_succinct_host_utils::{
    fetcher::OPSuccinctDataFetcher, get_agg_proof_stdin, host::OPSuccinctHost,
    metrics::MetricsGauge, witness_generation::WitnessGenerator,
};
use op_succinct_proof_utils::get_range_elf_embedded;
use op_succinct_signer_utils::Signer;
use sp1_sdk::{
    network::FulfillmentStrategy, NetworkProver, Prover, ProverClient, SP1ProofMode,
    SP1ProofWithPublicValues, SP1ProvingKey, SP1VerifyingKey, SP1_CIRCUIT_VERSION,
};
use tokio::time;

use crate::{
    config::ProposerConfig,
    contract::{
        DisputeGameFactory::{DisputeGameCreated, DisputeGameFactoryInstance},
        OPSuccinctFaultDisputeGame,
    },
    prometheus::ProposerGauge,
    Action, FactoryTrait, L1Provider, L2Provider, L2ProviderTrait, Mode,
};

struct SP1Prover {
    network_prover: Arc<NetworkProver>,
    range_pk: Arc<SP1ProvingKey>,
    range_vk: Arc<SP1VerifyingKey>,
    agg_pk: Arc<SP1ProvingKey>,
}

pub struct OPSuccinctProposer<P, H: OPSuccinctHost>
where
    P: Provider + Clone + Send + Sync,
{
    pub config: ProposerConfig,
    // The address being committed to when generating the aggregation proof to prevent
    // front-running attacks. This should be the same address that is being used to send
    // `prove` transactions.
    pub prover_address: Address,
    pub signer: Signer,
    pub l1_provider: L1Provider,
    pub l2_provider: L2Provider,
    pub factory: Arc<DisputeGameFactoryInstance<P>>,
    pub init_bond: U256,
    pub safe_db_fallback: bool,
    prover: SP1Prover,
    fetcher: Arc<OPSuccinctDataFetcher>,
    host: Arc<H>,
}

impl<P, H: OPSuccinctHost> OPSuccinctProposer<P, H>
where
    P: Provider + Clone + Send + Sync,
{
    /// Creates a new challenger instance with the provided L1 provider with wallet and factory
    /// contract instance.
    pub async fn new(
        prover_address: Address,
        signer: Signer,
        factory: DisputeGameFactoryInstance<P>,
        fetcher: Arc<OPSuccinctDataFetcher>,
        host: Arc<H>,
    ) -> Result<Self> {
        let config = ProposerConfig::from_env()?;

        // Set a default network private key to avoid an error in mock mode.
        let private_key = env::var("NETWORK_PRIVATE_KEY").unwrap_or_else(|_| {
            tracing::warn!(
                "Using default NETWORK_PRIVATE_KEY of 0x01. This is only valid in mock mode."
            );
            "0x0000000000000000000000000000000000000000000000000000000000000001".to_string()
        });

        let network_prover =
            Arc::new(ProverClient::builder().network().private_key(&private_key).build());
        let (range_pk, range_vk) = network_prover.setup(get_range_elf_embedded());
        let (agg_pk, _) = network_prover.setup(AGGREGATION_ELF);

        Ok(Self {
            config: config.clone(),
            prover_address,
            signer,
            l1_provider: ProviderBuilder::default().connect_http(config.l1_rpc.clone()),
            l2_provider: ProviderBuilder::default().connect_http(config.l2_rpc),
            factory: Arc::new(factory.clone()),
            init_bond: factory.fetch_init_bond(config.game_type).await?,
            safe_db_fallback: config.safe_db_fallback,
            prover: SP1Prover {
                network_prover,
                range_pk: Arc::new(range_pk),
                range_vk: Arc::new(range_vk),
                agg_pk: Arc::new(agg_pk),
            },
            fetcher: fetcher.clone(),
            host,
        })
    }

    pub async fn prove_game(&self, game_address: Address) -> Result<TxHash> {
        let fetcher = match OPSuccinctDataFetcher::new_with_rollup_config().await {
            Ok(f) => f,
            Err(e) => {
                tracing::error!("Failed to create data fetcher: {}", e);
                return Err(anyhow::anyhow!("Failed to create data fetcher: {}", e));
            }
        };

        let game = OPSuccinctFaultDisputeGame::new(game_address, self.l1_provider.clone());
        let l1_head_hash = game.l1Head().call().await?.0;
        tracing::debug!("L1 head hash: {:?}", hex::encode(l1_head_hash));
        let l2_block_number = game.l2BlockNumber().call().await?;

        let host_args = self
            .host
            .fetch(
                l2_block_number.to::<u64>() - self.config.proposal_interval_in_blocks,
                l2_block_number.to::<u64>(),
                Some(l1_head_hash.into()),
                self.config.safe_db_fallback,
            )
            .await
            .context("Failed to get host CLI args")?;

        let witness_data = self.host.run(&host_args).await?;

        let sp1_stdin = match self.host.witness_generator().get_sp1_stdin(witness_data) {
            Ok(stdin) => stdin,
            Err(e) => {
                tracing::error!("Failed to get proof stdin: {}", e);
                return Err(anyhow::anyhow!("Failed to get proof stdin: {}", e));
            }
        };

        tracing::info!("Generating Range Proof");
        let range_proof = if self.config.mock_mode {
            tracing::info!("Using mock mode for range proof generation");
            let (public_values, _) =
                self.prover.network_prover.execute(get_range_elf_embedded(), &sp1_stdin).run()?;

            // Create a mock range proof with the public values.
            SP1ProofWithPublicValues::create_mock_proof(
                &self.prover.range_pk,
                public_values,
                SP1ProofMode::Compressed,
                SP1_CIRCUIT_VERSION,
            )
        } else {
            self.prover
                .network_prover
                .prove(&self.prover.range_pk, &sp1_stdin)
                .compressed()
                .strategy(FulfillmentStrategy::Hosted)
                .skip_simulation(true)
                .cycle_limit(1_000_000_000_000)
                .run_async()
                .await?
        };

        tracing::info!("Preparing Stdin for Agg Proof");
        let proof = range_proof.proof.clone();
        let mut public_values = range_proof.public_values.clone();
        let boot_info: BootInfoStruct = public_values.read();

        let headers = match fetcher
            .get_header_preimages(&vec![boot_info.clone()], boot_info.clone().l1Head)
            .await
        {
            Ok(headers) => headers,
            Err(e) => {
                tracing::error!("Failed to get header preimages: {}", e);
                return Err(anyhow::anyhow!("Failed to get header preimages: {}", e));
            }
        };

        let sp1_stdin = match get_agg_proof_stdin(
            vec![proof],
            vec![boot_info.clone()],
            headers,
            &self.prover.range_vk,
            boot_info.l1Head,
            self.prover_address,
        ) {
            Ok(s) => s,
            Err(e) => {
                tracing::error!("Failed to get agg proof stdin: {}", e);
                return Err(anyhow::anyhow!("Failed to get agg proof stdin: {}", e));
            }
        };

        tracing::info!("Generating Agg Proof");
        let agg_proof = if self.config.mock_mode {
            tracing::info!("Using mock mode for aggregation proof generation");
            let (public_values, _) = self
                .prover
                .network_prover
                .execute(AGGREGATION_ELF, &sp1_stdin)
                .deferred_proof_verification(false)
                .run()?;

            // Create a mock aggregation proof with the public values.
            SP1ProofWithPublicValues::create_mock_proof(
                &self.prover.agg_pk,
                public_values,
                SP1ProofMode::Groth16,
                SP1_CIRCUIT_VERSION,
            )
        } else {
            self.prover
                .network_prover
                .prove(&self.prover.agg_pk, &sp1_stdin)
                .groth16()
                .run_async()
                .await?
        };

        let transaction_request = game.prove(agg_proof.bytes().into()).into_transaction_request();

        let receipt = self
            .signer
            .send_transaction_request(self.config.l1_rpc.clone(), transaction_request)
            .await?;

        Ok(receipt.transaction_hash)
    }

    /// Creates a new game with the given parameters.
    ///
    /// `l2_block_number`: the L2 block number we are proposing the output root for.
    /// `parent_game_index`: the index of the parent game.
    pub async fn create_game(
        &self,
        l2_block_number: U256,
        parent_game_index: u32,
    ) -> Result<Address> {
        tracing::info!(
            "Creating game at L2 block number: {:?}, with parent game index: {:?}",
            l2_block_number,
            parent_game_index
        );

        let extra_data = <(U256, u32)>::abi_encode_packed(&(l2_block_number, parent_game_index));

        let transaction_request = self
            .factory
            .create(
                self.config.game_type,
                self.l2_provider.compute_output_root_at_block(l2_block_number).await?,
                extra_data.into(),
            )
            .value(self.init_bond)
            .into_transaction_request();

        let receipt = self
            .signer
            .send_transaction_request(self.config.l1_rpc.clone(), transaction_request)
            .await?;

        let game_address = receipt
            .inner
            .logs()
            .iter()
            .find_map(|log| {
                DisputeGameCreated::decode_log(&log.inner).ok().map(|event| event.disputeProxy)
            })
            .context("Could not find DisputeGameCreated event in transaction receipt logs")?;

        tracing::info!(
            "\x1b[1mNew game at address {:?} created with tx {:?}\x1b[0m",
            game_address,
            receipt.transaction_hash
        );

        if self.config.fast_finality_mode {
            tracing::info!("Fast finality mode enabled: Generating proof for the game immediately");

            let tx_hash = self.prove_game(game_address).await?;
            tracing::info!(
                "\x1b[1mNew game at address {:?} proved with tx {:?}\x1b[0m",
                game_address,
                tx_hash
            );
        }

        Ok(game_address)
    }

    /// Handles the creation of a new game if conditions are met.
    /// Returns the address of the created game, if one was created.
    pub async fn handle_game_creation(&self) -> Result<Option<Address>> {
        let _span = tracing::info_span!("[[Proposing]]").entered();

        // Get the latest valid proposal.
        let latest_valid_proposal =
            self.factory.get_latest_valid_proposal(self.l2_provider.clone()).await?;

        // Determine next block number and parent game index.
        //
        // Two cases based on the result of `get_latest_valid_proposal`:
        // 1. With existing valid proposal:
        //    - Block number = latest valid proposal's block + proposal interval.
        //    - Parent = latest valid game's index.
        //
        // 2. Without valid proposal (first game or all existing games being faulty):
        //    - Block number = anchor L2 block number + proposal interval.
        //    - Parent = u32::MAX (special value indicating no parent).
        let (latest_proposed_block_number, next_l2_block_number_for_proposal, parent_game_index) =
            match latest_valid_proposal {
                Some((latest_block, latest_game_idx)) => (
                    latest_block,
                    latest_block + U256::from(self.config.proposal_interval_in_blocks),
                    latest_game_idx.to::<u32>(),
                ),
                None => {
                    let anchor_l2_block_number =
                        self.factory.get_anchor_l2_block_number(self.config.game_type).await?;
                    tracing::info!("Anchor L2 block number: {:?}", anchor_l2_block_number);
                    (
                        anchor_l2_block_number,
                        anchor_l2_block_number
                            .checked_add(U256::from(self.config.proposal_interval_in_blocks))
                            .unwrap(),
                        u32::MAX,
                    )
                }
            };

        let finalized_l2_head_block_number = self
            .host
            .get_finalized_l2_block_number(&self.fetcher, latest_proposed_block_number.to::<u64>())
            .await?;

        // There's always a new game to propose, as the chain is always moving forward from the
        // genesis block set for the game type. Only create a new game if the finalized L2
        // head block number is greater than the next L2 block number for proposal.
        if let Some(finalized_block) = finalized_l2_head_block_number {
            if U256::from(finalized_block) > next_l2_block_number_for_proposal {
                let game_address =
                    self.create_game(next_l2_block_number_for_proposal, parent_game_index).await?;

                Ok(Some(game_address))
            } else {
                tracing::info!("No new game to propose since proposal interval has not elapsed");

                Ok(None)
            }
        } else {
            tracing::info!("No new finalized block number found since last proposed block");
            Ok(None)
        }
    }

    /// Handles the resolution of all eligible unchallenged games.
    pub async fn handle_game_resolution(&self) -> Result<()> {
        let _span = tracing::info_span!("[[Resolving]]").entered();

        self.factory
            .resolve_games(
                Mode::Proposer,
                self.config.max_games_to_check_for_resolution,
                self.signer.clone(),
                self.config.l1_rpc.clone(),
                self.l1_provider.clone(),
                self.l2_provider.clone(),
            )
            .await
    }

    /// Handles the defense of all eligible games by providing proofs.
    pub async fn handle_game_defense(&self) -> Result<()> {
        let _span = tracing::info_span!("[[Defending]]").entered();

        if let Some(game_address) = self
            .factory
            .get_oldest_defensible_game_address(
                self.config.max_games_to_check_for_defense,
                self.l2_provider.clone(),
            )
            .await?
        {
            tracing::info!("Attempting to defend game {:?}", game_address);

            let tx_hash = self.prove_game(game_address).await?;
            tracing::info!(
                "\x1b[1mSuccessfully defended game {:?} with tx {:?}\x1b[0m",
                game_address,
                tx_hash
            );
        }

        Ok(())
    }

    /// Handles claiming bonds from resolved games.
    pub async fn handle_bond_claiming(&self) -> Result<Action> {
        let _span = tracing::info_span!("[[Claiming Bonds]]").entered();

        if let Some(game_address) = self
            .factory
            .get_oldest_claimable_bond_game_address(
                self.config.game_type,
                self.config.max_games_to_check_for_bond_claiming,
                self.prover_address,
            )
            .await?
        {
            tracing::info!("Attempting to claim bond from game {:?}", game_address);

            // Create a contract instance for the game
            let game = OPSuccinctFaultDisputeGame::new(game_address, self.l1_provider.clone());

            // Create a transaction to claim credit
            let transaction_request =
                game.claimCredit(self.prover_address).into_transaction_request();

            // Sign and send the transaction
            match self
                .signer
                .send_transaction_request(self.config.l1_rpc.clone(), transaction_request)
                .await
            {
                Ok(receipt) => {
                    tracing::info!(
                        "\x1b[1mSuccessfully claimed bond from game {:?} with tx {:?}\x1b[0m",
                        game_address,
                        receipt.transaction_hash
                    );

                    Ok(Action::Performed)
                }
                Err(e) => Err(anyhow::anyhow!(
                    "Failed to claim bond from game {:?}: {:?}",
                    game_address,
                    e
                )),
            }
        } else {
            tracing::info!("No new games to claim bonds from");

            Ok(Action::Skipped)
        }
    }

    /// Fetch the proposer metrics.
    async fn fetch_proposer_metrics(&self) -> Result<()> {
        // Get the latest valid proposal.
        let latest_proposed_block_number =
            match self.factory.get_latest_valid_proposal(self.l2_provider.clone()).await? {
                Some((l2_block_number, _game_index)) => l2_block_number,
                None => {
                    tracing::debug!("No valid proposals found for metrics");
                    self.factory.get_anchor_l2_block_number(self.config.game_type).await?
                }
            };

        // Update metrics for latest game block number.
        ProposerGauge::LatestGameL2BlockNumber.set(latest_proposed_block_number.to::<u64>() as f64);

        // Update metrics for finalized L2 block number.
        if let Some(finalized_l2_block_number) = self
            .host
            .get_finalized_l2_block_number(&self.fetcher, latest_proposed_block_number.to::<u64>())
            .await?
        {
            ProposerGauge::FinalizedL2BlockNumber.set(finalized_l2_block_number as f64);
        }

        // Update metrics for anchor game block number.
        let anchor_game_l2_block_number =
            self.factory.get_anchor_l2_block_number(self.config.game_type).await?;
        ProposerGauge::AnchorGameL2BlockNumber.set(anchor_game_l2_block_number.to::<u64>() as f64);

        Ok(())
    }

    /// Runs the proposer indefinitely.
    pub async fn run(&self) -> Result<()> {
        tracing::info!("OP Succinct Proposer running...");
        let mut interval = time::interval(Duration::from_secs(self.config.fetch_interval));
        let mut metrics_interval = time::interval(Duration::from_secs(15));

        loop {
            tokio::select! {
                _ = interval.tick() => {
                    match self.handle_game_creation().await {
                        Ok(Some(_)) => {
                            ProposerGauge::GamesCreated.increment(1.0);
                        }
                        Ok(None) => {}
                            Err(e) => {
                            tracing::warn!("Failed to handle game creation: {:?}", e);
                            ProposerGauge::GameCreationError.increment(1.0);
                        }
                    }

                    if let Err(e) = self.handle_game_defense().await {
                        tracing::warn!("Failed to handle game defense: {:?}", e);
                        ProposerGauge::GameDefenseError.increment(1.0);
                    }

                    if let Err(e) = self.handle_game_resolution().await {
                        tracing::warn!("Failed to handle game resolution: {:?}", e);
                        ProposerGauge::GameResolutionError.increment(1.0);
                    }

                    match self.handle_bond_claiming().await {
                        Ok(Action::Performed) => {
                            ProposerGauge::GamesBondsClaimed.increment(1.0);
                        }
                        Ok(Action::Skipped) => {}
                        Err(e) => {
                            tracing::warn!("Failed to handle bond claiming: {:?}", e);
                            ProposerGauge::BondClaimingError.increment(1.0);
                        }
                    }
                }
                _ = metrics_interval.tick() => {
                    if let Err(e) = self.fetch_proposer_metrics().await {
                        tracing::warn!("Failed to fetch metrics: {:?}", e);
                        ProposerGauge::MetricsError.increment(1.0);
                    }
                }
            }
        }
    }
}
