use ethers::{
    abi::{Function, Param, ParamType, StateMutability, Token},
    core::types::transaction::eip2718::TypedTransaction,
    middleware::SignerMiddleware,
    prelude::*,
    providers::{Http, Provider},
    signers::{LocalWallet, Signer},
    types::{Address, Bytes, TransactionRequest, H256, U256},
};
use eyre::Result;
use serde_json;
use std::{str::FromStr, sync::Arc, time::Duration};
use tokio::time;

// GameType(42) for OP Succinct Fault Dispute games
const GAME_TYPE: u32 = 42;

#[derive(Debug)]
struct OutputRoot {
    root: H256,
    l2_block_number: U256,
}

#[derive(Debug)]
struct GameInfo {
    challenged: bool,
    clock_expired: bool,
}

struct ContractAddresses {
    factory_proxy: Address,
    registry_proxy: Address,
    game_impl: Address,
    sp1_verifier: Address,
}

impl ContractAddresses {
    fn from_env() -> Result<Self> {
        Ok(Self {
            factory_proxy: std::env::var("FACTORY_PROXY")
                .expect("FACTORY_PROXY must be set")
                .parse()?,
            registry_proxy: std::env::var("REGISTRY_PROXY")
                .expect("REGISTRY_PROXY must be set")
                .parse()?,
            game_impl: std::env::var("GAME_IMPL")
                .expect("GAME_IMPL must be set")
                .parse()?,
            sp1_verifier: std::env::var("SP1_VERIFIER")
                .expect("SP1_VERIFIER must be set")
                .parse()?,
        })
    }
}

struct OPSuccinctProposer {
    eth_client: Arc<SignerMiddleware<Arc<Provider<Http>>, LocalWallet>>,
    l2_client: Arc<Provider<Http>>,
    l2_node_client: Arc<Provider<Http>>,
    proposal_interval: Duration,
    last_proposed_block: U256,
    addresses: ContractAddresses,
    proposal_interval_in_blocks: u64,
}

impl OPSuccinctProposer {
    fn new(
        eth_client: Arc<SignerMiddleware<Arc<Provider<Http>>, LocalWallet>>,
        l2_client: Arc<Provider<Http>>,
        l2_node_client: Arc<Provider<Http>>,
        proposal_interval: Duration,
        addresses: ContractAddresses,
        proposal_interval_in_blocks: u64,
    ) -> Self {
        Self {
            eth_client,
            l2_client,
            l2_node_client,
            proposal_interval,
            last_proposed_block: U256::zero(),
            addresses,
            proposal_interval_in_blocks,
        }
    }

    async fn run(&mut self) -> Result<()> {
        let mut interval = time::interval(self.proposal_interval);

        loop {
            interval.tick().await;

            // Get latest finalized root and safe L2 head
            let latest_root = self.poll_latest_finalized_root().await?;
            let safe_l2_head = self.poll_safe_l2_head().await?;

            tracing::info!(
                "Latest finalized root: {:?}, Safe L2 head: {}",
                latest_root,
                safe_l2_head
            );

            // Check if enough blocks have passed since last proposal
            if safe_l2_head > self.last_proposed_block + self.proposal_interval_in_blocks {
                tracing::info!("Creating new game for L2 block {}", safe_l2_head);
                match self.create_new_game(safe_l2_head).await {
                    Ok(_) => {
                        self.last_proposed_block = safe_l2_head;
                        tracing::info!("Successfully created new game");
                    }
                    Err(e) => tracing::error!("Failed to create new game: {}", e),
                }
            }

            // Check for resolvable games
            match self.check_and_resolve_games().await {
                Ok(_) => tracing::info!("Checked for resolvable games"),
                Err(e) => tracing::error!("Failed to check/resolve games: {}", e),
            }
        }
    }

    async fn poll_latest_finalized_root(&self) -> Result<OutputRoot> {
        let function = Function {
            name: "anchors".to_string(),
            inputs: vec![Param {
                name: "".to_string(),
                kind: ParamType::Uint(32),
                internal_type: None,
            }],
            outputs: vec![
                Param {
                    name: "".to_string(),
                    kind: ParamType::FixedBytes(32),
                    internal_type: None,
                },
                Param {
                    name: "".to_string(),
                    kind: ParamType::Uint(256),
                    internal_type: None,
                },
            ],
            constant: Some(true),
            state_mutability: StateMutability::View,
        };

        let params = vec![Token::Uint(GAME_TYPE.into())];
        let data = function.encode_input(&params)?;

        let result = self
            .eth_client
            .call(
                &TypedTransaction::Legacy(
                    TransactionRequest::new()
                        .to(self.addresses.registry_proxy)
                        .data(Bytes::from(data)),
                ),
                None,
            )
            .await?;

        let decoded = function.decode_output(&result)?;
        let root = H256::from_slice(&decoded[0].clone().into_fixed_bytes().unwrap());
        let l2_block_number = decoded[1].clone().into_uint().unwrap();

        Ok(OutputRoot {
            root,
            l2_block_number,
        })
    }

    async fn poll_safe_l2_head(&self) -> Result<U256> {
        let response: serde_json::Value = self
            .l2_client
            .request(
                "eth_getBlockByNumber",
                vec![serde_json::json!("safe"), serde_json::json!(false)],
            )
            .await?;

        let block_number = response
            .get("number")
            .and_then(|v| v.as_str())
            .ok_or_else(|| eyre::eyre!("Invalid block response format"))?;

        // Remove "0x" prefix and parse as hex
        let block_number = U256::from_str(&block_number.replace("0x", ""))?;
        Ok(block_number)
    }

    async fn get_output_root_at_block(&self, block_number: U256) -> Result<H256> {
        let params = vec![serde_json::json!(format!("0x{:x}", block_number))];

        let response: serde_json::Value = self
            .l2_node_client
            .request("optimism_outputAtBlock", params)
            .await?;

        tracing::debug!("RPC Response: {:?}", response);

        // The outputRoot is directly in the response, not under "result"
        let output_root = response
            .get("outputRoot")
            .and_then(|v| v.as_str())
            .ok_or_else(|| eyre::eyre!("Invalid output root response format: {:?}", response))?;

        tracing::debug!("Extracted output root: {}", output_root);

        // Remove "0x" prefix and parse as H256
        let output_root = output_root.strip_prefix("0x").unwrap_or(output_root);
        Ok(H256::from_str(output_root)?)
    }

    async fn create_new_game(&self, l2_block_number: U256) -> Result<()> {
        let function = Function {
            name: "create".to_string(),
            inputs: vec![
                Param {
                    name: "_gameType".to_string(),
                    kind: ParamType::Uint(32),
                    internal_type: None,
                },
                Param {
                    name: "_rootClaim".to_string(),
                    kind: ParamType::FixedBytes(32),
                    internal_type: None,
                },
                Param {
                    name: "_extraData".to_string(),
                    kind: ParamType::Bytes,
                    internal_type: None,
                },
            ],
            outputs: vec![],
            constant: None,
            state_mutability: StateMutability::Payable,
        };

        let root_claim = self.get_output_root_at_block(l2_block_number).await?;
        let extra_data = ethers::abi::encode(&[Token::Uint(l2_block_number)]);

        let params = vec![
            Token::Uint(GAME_TYPE.into()),
            Token::FixedBytes(root_claim.as_bytes().to_vec()),
            Token::Bytes(extra_data),
        ];
        let data = function.encode_input(&params)?;

        // TODO: Get actual bond amount from factory
        let bond_amount = U256::from(10_000_000_000_000_000u64); // 0.01 ETH

        let tx = TransactionRequest::new()
            .to(self.addresses.factory_proxy)
            .data(Bytes::from(data))
            .value(bond_amount);

        // Send transaction using SignerMiddleware
        let pending_tx = self.eth_client.send_transaction(tx, None).await?;
        let receipt = pending_tx.await?;
        tracing::info!(
            "Game created in tx: {:?}",
            receipt.unwrap().transaction_hash
        );

        Ok(())
    }

    async fn check_and_resolve_games(&self) -> Result<()> {
        // Get total games count
        let games_count = self.get_games_count().await?;

        // Iterate through all games
        for game_id in 0..games_count.as_u64() {
            let game_id = U256::from(game_id);

            // Get game status
            if let Ok(game_info) = self.get_game_info(game_id).await {
                // Check if game is unchallenged and clock expired
                if !game_info.challenged && game_info.clock_expired {
                    tracing::info!("Found resolvable game: {}", game_id);
                    match self.resolve_game(game_id).await {
                        Ok(_) => tracing::info!("Successfully resolved game {}", game_id),
                        Err(e) => tracing::error!("Failed to resolve game {}: {}", game_id, e),
                    }
                }
            }
        }
        Ok(())
    }

    async fn get_games_count(&self) -> Result<U256> {
        let function = Function {
            name: "gameCount".to_string(),
            inputs: vec![],
            outputs: vec![Param {
                name: "".to_string(),
                kind: ParamType::Uint(256),
                internal_type: None,
            }],
            constant: Some(true),
            state_mutability: StateMutability::View,
        };

        let data = function.encode_input(&[])?;

        let result = self
            .eth_client
            .call(
                &TypedTransaction::Legacy(
                    TransactionRequest::new()
                        .to(self.addresses.factory_proxy)
                        .data(Bytes::from(data)),
                ),
                None,
            )
            .await?;

        let decoded = function.decode_output(&result)?;
        Ok(decoded[0].clone().into_uint().unwrap())
    }

    async fn get_game_info(&self, game_id: U256) -> Result<GameInfo> {
        let function = Function {
            name: "games".to_string(),
            inputs: vec![Param {
                name: "".to_string(),
                kind: ParamType::Uint(256),
                internal_type: None,
            }],
            outputs: vec![
                // Add all game struct fields here, but we only care about status for now
                Param {
                    name: "status".to_string(),
                    kind: ParamType::Uint(8),
                    internal_type: None,
                },
                // Add other fields as needed
            ],
            constant: Some(true),
            state_mutability: StateMutability::View,
        };

        let params = vec![Token::Uint(game_id)];
        let data = function.encode_input(&params)?;

        let result = self
            .eth_client
            .call(
                &TypedTransaction::Legacy(
                    TransactionRequest::new()
                        .to(self.addresses.factory_proxy)
                        .data(Bytes::from(data)),
                ),
                None,
            )
            .await?;

        let decoded = function.decode_output(&result)?;
        let status = decoded[0].clone().into_uint().unwrap().as_u64();

        // Status 1 means IN_PROGRESS
        // If game is in progress, check if clock is expired
        let clock_expired = if status == 1 {
            self.check_clock_expired(game_id).await?
        } else {
            false
        };

        // Game is challenged if there are moves (status > 1)
        Ok(GameInfo {
            challenged: status > 1,
            clock_expired,
        })
    }

    async fn check_clock_expired(&self, game_id: U256) -> Result<bool> {
        let function = Function {
            name: "clockExpired".to_string(),
            inputs: vec![Param {
                name: "gameId".to_string(),
                kind: ParamType::Uint(256),
                internal_type: None,
            }],
            outputs: vec![Param {
                name: "".to_string(),
                kind: ParamType::Bool,
                internal_type: None,
            }],
            constant: Some(true),
            state_mutability: StateMutability::View,
        };

        let params = vec![Token::Uint(game_id)];
        let data = function.encode_input(&params)?;

        let result = self
            .eth_client
            .call(
                &TypedTransaction::Legacy(
                    TransactionRequest::new()
                        .to(self.addresses.factory_proxy)
                        .data(Bytes::from(data)),
                ),
                None,
            )
            .await?;

        let decoded = function.decode_output(&result)?;
        Ok(decoded[0].clone().into_bool().unwrap())
    }

    async fn resolve_game(&self, game_id: U256) -> Result<()> {
        let function = Function {
            name: "resolve".to_string(),
            inputs: vec![Param {
                name: "gameId".to_string(),
                kind: ParamType::Uint(256),
                internal_type: None,
            }],
            outputs: vec![],
            constant: None,
            state_mutability: StateMutability::NonPayable,
        };

        let params = vec![Token::Uint(game_id)];
        let data = function.encode_input(&params)?;

        let tx = TransactionRequest::new()
            .to(self.addresses.factory_proxy)
            .data(Bytes::from(data));

        let pending_tx = self.eth_client.send_transaction(tx, None).await?;
        let receipt = pending_tx.await?;
        tracing::info!(
            "Game {} resolved in tx: {:?}",
            game_id,
            receipt.unwrap().transaction_hash
        );

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Load environment variables
    dotenv::dotenv().ok();

    // Load contract addresses
    let addresses = ContractAddresses::from_env()?;

    // Connect to ETH Sepolia
    let eth_rpc_url =
        std::env::var("ETH_SEPOLIA_RPC_URL").expect("ETH_SEPOLIA_RPC_URL must be set");
    let provider = Provider::<Http>::try_from(eth_rpc_url)?;
    let eth_client = Arc::new(provider);

    // Connect to L2 RPC
    let l2_rpc = std::env::var("L2_RPC").expect("L2_RPC must be set");
    let l2_provider = Provider::<Http>::try_from(l2_rpc)?;
    let l2_client = Arc::new(l2_provider);

    // Connect to L2 Node RPC
    let l2_node_rpc = std::env::var("L2_NODE_RPC").expect("L2_NODE_RPC must be set");
    let l2_node_provider = Provider::<Http>::try_from(l2_node_rpc)?;
    let l2_node_client = Arc::new(l2_node_provider);

    // Get proposal interval from env or use default
    let proposal_interval_in_blocks = std::env::var("PROPOSAL_INTERVAL_IN_BLOCKS")
        .map(|v| {
            v.parse()
                .expect("PROPOSAL_INTERVAL_IN_BLOCKS must be a valid number")
        })
        .unwrap_or(100);

    // Get private key from environment variable
    let private_key = std::env::var("PRIVATE_KEY")
        .map_err(|_| eyre::eyre!("PRIVATE_KEY environment variable not set"))?
        .strip_prefix("0x")
        .unwrap_or(&std::env::var("PRIVATE_KEY").unwrap())
        .to_string();

    // Create wallet from private key
    let wallet = private_key
        .parse::<LocalWallet>()?
        .with_chain_id(11155111u64);

    // Create SignerMiddleware
    let eth_client = SignerMiddleware::new(eth_client, wallet);
    let eth_client = Arc::new(eth_client);

    // Create proposer with 5 minute interval
    let mut proposer = OPSuccinctProposer::new(
        eth_client,
        l2_client,
        l2_node_client,
        Duration::from_secs(5 * 60),
        addresses,
        proposal_interval_in_blocks,
    );

    proposer.run().await
}
