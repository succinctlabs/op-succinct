use std::{env, time::Duration};

use alloy::{
    eips::BlockNumberOrTag,
    network::Ethereum,
    primitives::{Address, U256},
    providers::{fillers::TxFiller, Provider, ProviderBuilder},
    signers::local::PrivateKeySigner,
    sol_types::SolValue,
    transports::{http::reqwest::Url, Transport},
};
use anyhow::Result;
use op_alloy_network::EthereumWallet;
use tokio::time;

use fault_proof::{
    config::ProposerConfig,
    sol::{DisputeGameFactory, DisputeGameFactory::DisputeGameFactoryInstance},
    utils::setup_logging,
    FactoryTrait, L1Provider, L1ProviderWithWallet, L2Provider, L2ProviderTrait, Mode,
};

struct OPSuccinctProposer<F, P, T>
where
    F: TxFiller<Ethereum>,
    P: Provider<T, Ethereum> + Clone,
    T: Transport + Clone,
{
    config: ProposerConfig,
    l1_provider: L1Provider,
    l2_provider: L2Provider,
    l1_provider_with_wallet: L1ProviderWithWallet<F, P, T>,
    factory: DisputeGameFactoryInstance<T, L1ProviderWithWallet<F, P, T>>,
    init_bond: U256,
}

impl<F, P, T> OPSuccinctProposer<F, P, T>
where
    F: TxFiller<Ethereum>,
    P: Provider<T, Ethereum> + Clone,
    T: Transport + Clone,
{
    /// Creates a new challenger instance with the provided L1 provider with wallet and factory contract instance
    pub async fn new(
        l1_provider_with_wallet: L1ProviderWithWallet<F, P, T>,
        factory: DisputeGameFactoryInstance<T, L1ProviderWithWallet<F, P, T>>,
    ) -> Result<Self> {
        let config = ProposerConfig::from_env()?;

        Ok(Self {
            config: config.clone(),
            l1_provider: ProviderBuilder::default().on_http(config.l1_rpc.clone()),
            l2_provider: ProviderBuilder::default().on_http(config.l2_rpc),
            l1_provider_with_wallet: l1_provider_with_wallet.clone(),
            factory: factory.clone(),
            init_bond: factory.fetch_init_bond(config.game_type).await?,
        })
    }

    /// Creates a new game with the given parameters.
    ///
    /// `l2_block_number`: the L2 block number we are proposing the output root for.
    /// `parent_game_index`: the index of the parent game.
    async fn create_game(&self, l2_block_number: U256, parent_game_index: u32) -> Result<()> {
        let extra_data = <(U256, u32)>::abi_encode_packed(&(l2_block_number, parent_game_index));

        let receipt = self
            .factory
            .create(
                self.config.game_type,
                self.l2_provider
                    .compute_output_root_at_block(l2_block_number)
                    .await?,
                extra_data.into(),
            )
            .value(self.init_bond)
            .send()
            .await?
            .get_receipt()
            .await?;

        let game_address = receipt.inner.logs()[0].address();

        tracing::info!(
            "New game \x1B]8;;https://sepolia.etherscan.io/address/{:?}\x07{:?}\x1B]8;;\x07 created: \x1B]8;;https://sepolia.etherscan.io/tx/{:?}\x07{:?}\x1B]8;;\x07",
            game_address,
            game_address,
            receipt.transaction_hash,
            receipt.transaction_hash
        );

        Ok(())
    }

    /// Runs the proposer indefinitely.
    async fn run(&self) -> Result<()> {
        tracing::info!("OP Succinct Proposer running...");
        let mut interval = time::interval(Duration::from_secs(self.config.fetch_interval));

        loop {
            interval.tick().await;

            let _span = tracing::info_span!("[[Proposing]]").entered();

            let safe_l2_head_block_number = self
                .l2_provider
                .get_l2_block_by_number(BlockNumberOrTag::Safe)
                .await?
                .header
                .number;
            tracing::debug!("Safe L2 head block number: {:?}", safe_l2_head_block_number);

            let latest_valid_proposal = self
                .factory
                .get_latest_valid_proposal(self.l1_provider.clone(), self.l2_provider.clone())
                .await?;

            let (next_l2_block_number_for_proposal, parent_game_index) = match latest_valid_proposal
            {
                Some((latest_block, latest_game_idx)) => (
                    latest_block + U256::from(self.config.proposal_interval_in_blocks),
                    latest_game_idx.to::<u32>(),
                ),
                None => {
                    // For first game, start from genesis L2 block number + proposal interval
                    let genesis_l2_block_number = self
                        .factory
                        .get_genesis_l2_block_number(
                            self.config.game_type,
                            self.l1_provider.clone(),
                        )
                        .await?;

                    (
                        genesis_l2_block_number
                            .checked_add(U256::from(self.config.proposal_interval_in_blocks))
                            .unwrap(),
                        u32::MAX, // Use max value for first game's parent index
                    )
                }
            };

            if U256::from(safe_l2_head_block_number) > next_l2_block_number_for_proposal {
                self.create_game(next_l2_block_number_for_proposal, parent_game_index)
                    .await?;
            }

            drop(_span);

            // Only attempt game resolution if enabled
            if self.config.enable_game_resolution {
                let _span = tracing::info_span!("[[Resolving]]").entered();

                if let Err(e) = self
                    .factory
                    .resolve_games(
                        Mode::Proposer,
                        self.config.max_games_to_check_for_resolution,
                        self.l1_provider_with_wallet.clone(),
                        self.l2_provider.clone(),
                    )
                    .await
                {
                    tracing::warn!("Failed to resolve unchallenged games: {:?}", e);
                }

                drop(_span);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    setup_logging();

    dotenv::from_filename(".env.proposer").ok();

    let wallet = EthereumWallet::from(
        env::var("PRIVATE_KEY")
            .unwrap()
            .parse::<PrivateKeySigner>()
            .unwrap(),
    );

    let l1_provider_with_wallet = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet.clone())
        .on_http(env::var("L1_RPC").unwrap().parse::<Url>().unwrap());

    let factory = DisputeGameFactory::new(
        env::var("FACTORY_ADDRESS")
            .unwrap()
            .parse::<Address>()
            .unwrap(),
        l1_provider_with_wallet.clone(),
    );

    let proposer = OPSuccinctProposer::new(l1_provider_with_wallet, factory)
        .await
        .unwrap();
    proposer.run().await.expect("Runs in an infinite loop");
}
