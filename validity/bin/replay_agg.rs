//! Replay an aggregation proof locally with CpuProver for Groth16 debugging.
//!
//! Reads the aggregation request and its constituent range proofs from the proposer DB,
//! reconstructs the exact same SP1Stdin the proposer would build, then generates a real
//! Groth16 proof locally via CpuProver and writes the proof bytes to disk.
//!
//! By default, after saving proof.bin it also relays the proof on-chain like the proposer.
//! Use --skip-relay to only save without relaying.
//!
//! Usage:
//!   cargo run --release --bin replay-agg -- --request-id <ID> --env-file .env
//!   cargo run --release --bin replay-agg -- --request-id <ID> --execute-only  # validate stdin only
//!   cargo run --release --bin replay-agg -- --request-id <ID> --skip-relay    # prove but don't relay

use alloy_primitives::{Address, B256, U256};
use alloy_provider::network::ReceiptResponse;
use anyhow::{anyhow, Context, Result};
use clap::Parser;
use op_succinct_elfs::AGGREGATION_ELF;
use op_succinct_host_utils::{
    fetcher::OPSuccinctDataFetcher,
    get_agg_proof_stdin,
    DisputeGameFactory::DisputeGameFactoryInstance as DisputeGameFactoryContract,
    OPSuccinctL2OutputOracle::OPSuccinctL2OutputOracleInstance as OPSuccinctL2OOContract,
};
use op_succinct_proof_utils::get_range_elf_embedded;
use op_succinct_signer_utils::Signer;
use op_succinct_validity::{CommitmentConfig, DriverDBClient, RequestType};
use sp1_sdk::{Elf, HashableKey, ProveRequest, Prover, ProverClient, ProvingKey, SP1ProofWithPublicValues};
use std::path::PathBuf;
use tracing::info;

#[derive(Parser)]
#[command(about = "Replay an aggregation proof locally with CpuProver for Groth16 debugging")]
struct Args {
    /// Aggregation request ID from the proposer database
    #[arg(long)]
    request_id: i64,

    /// Output file path for on-chain proof bytes
    #[arg(long, default_value = "proof.bin")]
    output: PathBuf,

    /// Path to environment file (needs DATABASE_URL, L1_RPC, L2_RPC, L2_NODE_RPC)
    #[arg(long, default_value = ".env")]
    env_file: String,

    /// Execute only — validate stdin without generating a proof
    #[arg(long)]
    execute_only: bool,

    /// Skip on-chain relay (just save proof.bin)
    #[arg(long)]
    skip_relay: bool,

    /// L2OutputOracle contract address (required for relay)
    #[arg(long, env = "L2OO_ADDRESS")]
    l2oo_address: Option<Address>,

    /// DisputeGameFactory address (optional; if set, uses DGF path)
    #[arg(long, env = "DGF_ADDRESS")]
    dgf_address: Option<Address>,

    /// OP Succinct config name (hashed to bytes32 for the contract call)
    #[arg(long, env = "OP_SUCCINCT_CONFIG_NAME", default_value = "opsuccinct_genesis")]
    op_succinct_config_name: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env early so clap's `env` attributes can read from it.
    // Parse --env-file first from raw args, falling back to ".env".
    let env_file = std::env::args()
        .skip_while(|a| a != "--env-file")
        .nth(1)
        .unwrap_or_else(|| ".env".to_string());
    dotenv::from_filename(&env_file).ok();

    let args = Args::parse();
    sp1_sdk::utils::setup_logger();

    // ── 1. Connect to DB (read-only) ────────────────────────────────────
    let db_url = std::env::var("DATABASE_URL").context("DATABASE_URL env var required")?;
    let db_client = DriverDBClient::new(&db_url).await?;

    // ── 2. Fetch the aggregation request ────────────────────────────────
    let agg_req = db_client
        .fetch_request_by_id(args.request_id)
        .await?
        .ok_or_else(|| anyhow!("Request {} not found", args.request_id))?;

    if agg_req.req_type != RequestType::Aggregation {
        return Err(anyhow!(
            "Request {} is {:?}, not Aggregation",
            args.request_id,
            agg_req.req_type
        ));
    }

    let start_block = agg_req.start_block;
    let end_block = agg_req.end_block;
    let checkpointed_l1_block_hash = B256::from_slice(
        agg_req
            .checkpointed_l1_block_hash
            .as_ref()
            .ok_or_else(|| anyhow!("Aggregation request missing checkpointed_l1_block_hash"))?,
    );
    let checkpointed_l1_block_number = agg_req
        .checkpointed_l1_block_number
        .ok_or_else(|| anyhow!("Aggregation request missing checkpointed_l1_block_number"))?;
    let prover_address = Address::from_slice(
        agg_req
            .prover_address
            .as_ref()
            .ok_or_else(|| anyhow!("Aggregation request missing prover_address"))?,
    );
    let commitments = CommitmentConfig {
        range_vkey_commitment: B256::from_slice(&agg_req.range_vkey_commitment),
        agg_vkey_hash: B256::from_slice(
            agg_req
                .aggregation_vkey_hash
                .as_ref()
                .ok_or_else(|| anyhow!("Aggregation request missing aggregation_vkey_hash"))?,
        ),
        rollup_config_hash: B256::from_slice(&agg_req.rollup_config_hash),
    };

    info!(
        request_id = args.request_id,
        start_block,
        end_block,
        %checkpointed_l1_block_hash,
        checkpointed_l1_block_number,
        %prover_address,
        "Loaded aggregation request"
    );

    // ── 3. Setup CpuProver and keys ─────────────────────────────────────
    let prover = ProverClient::builder().cpu().build().await;
    let range_pk = prover.setup(Elf::Static(get_range_elf_embedded())).await?;
    let range_vk = range_pk.verifying_key().clone();
    let agg_pk = prover.setup(Elf::Static(AGGREGATION_ELF)).await?;
    let agg_vk = agg_pk.verifying_key().clone();
    info!(agg_vkey_hash = %agg_vk.bytes32(), "Prover keys ready");

    // ── 4. Fetch range proofs from DB ───────────────────────────────────
    //       (same query as proof_requester.rs:129-138)
    let range_proofs = db_client
        .get_consecutive_complete_range_proofs(
            start_block,
            end_block,
            &commitments,
            agg_req.l1_chain_id,
            agg_req.l2_chain_id,
        )
        .await?;

    if range_proofs.is_empty() {
        return Err(anyhow!(
            "No completed range proofs found for blocks {start_block}-{end_block}"
        ));
    }
    info!(count = range_proofs.len(), "Fetched range proofs from DB");

    // ── 5. Deserialize range proofs ─────────────────────────────────────
    //       (same as proof_requester.rs:144-164)
    let mut boot_infos = Vec::with_capacity(range_proofs.len());
    let mut proofs = Vec::with_capacity(range_proofs.len());

    for rp in range_proofs.iter() {
        let proof_bytes = rp.proof.as_ref().ok_or_else(|| {
            anyhow!("Range proof {}-{} missing proof data", rp.start_block, rp.end_block)
        })?;

        info!(
            start = rp.start_block,
            end = rp.end_block,
            len = proof_bytes.len(),
            head = %format!("0x{}", hex::encode(&proof_bytes[..std::cmp::min(32, proof_bytes.len())])),
            "Attempting to deserialize range proof"
        );

        let mut proof_with_pv: SP1ProofWithPublicValues =
            bincode::deserialize(proof_bytes).map_err(|e| {
                anyhow!(
                    "Failed to deserialize range proof {}-{} ({} bytes): {e:?}",
                    rp.start_block,
                    rp.end_block,
                    proof_bytes.len()
                )
            })?;

        boot_infos.push(proof_with_pv.public_values.read());
        proofs.push(proof_with_pv.proof.clone());
        info!(start = rp.start_block, end = rp.end_block, "Deserialized range proof");
    }

    // ── 6. Fetch L1 headers ─────────────────────────────────────────────
    let fetcher = OPSuccinctDataFetcher::new_with_rollup_config().await?;
    let headers = fetcher
        .get_header_preimages(&boot_infos, checkpointed_l1_block_hash)
        .await
        .context("Failed to get L1 header preimages")?;
    info!(count = headers.len(), "Fetched L1 headers");

    // ── 7. Build stdin ──────────────────────────────────────────────────
    //       (same as proof_requester.rs:177-185)
    let stdin = get_agg_proof_stdin(
        proofs,
        boot_infos,
        headers,
        &range_vk,
        checkpointed_l1_block_hash,
        prover_address,
    )
    .context("Failed to build aggregation stdin")?;
    info!("Stdin constructed successfully");

    // ── 8a. Execute-only mode ───────────────────────────────────────────
    if args.execute_only {
        info!("Executing aggregation program (no proof generation)...");
        let (_, report) = prover
            .execute(Elf::Static(AGGREGATION_ELF), stdin)
            .deferred_proof_verification(false)
            .await
            .context("Aggregation execution failed")?;
        info!(cycles = report.total_instruction_count(), "Execution successful");
        return Ok(());
    }

    // ── 8b. Generate Groth16 proof ──────────────────────────────────────
    info!("Generating Groth16 proof locally (this will take a while)...");
    let proof = prover
        .prove(&agg_pk, stdin)
        .groth16()
        .await
        .context("Groth16 proof generation failed")?;

    // ── 8c. Verify Groth16 proof locally (BN254 pairing via gnark FFI) ─
    info!("Verifying Groth16 proof locally (BN254 pairing check)...");
    prover
        .verify(&proof, &agg_vk, None)
        .context("Local Groth16 verification FAILED — proof is invalid")?;
    info!("Local Groth16 verification PASSED");

    // ── 9. Save proof bytes ─────────────────────────────────────────────
    let proof_bytes = proof.bytes();
    std::fs::write(&args.output, &proof_bytes)?;

    info!(
        path = %args.output.display(),
        len = proof_bytes.len(),
        hex = %format!("0x{}", hex::encode(&proof_bytes)),
        "Proof saved"
    );

    // Also save the full SP1ProofWithPublicValues for further inspection.
    let full_proof_path = args.output.with_extension("full.bin");
    proof.save(&full_proof_path).context("Failed to save full proof")?;
    info!(path = %full_proof_path.display(), "Full proof saved");

    // ── 10. Relay proof on-chain (like proposer) ────────────────────────
    if args.skip_relay {
        info!("Skipping on-chain relay (--skip-relay)");
        return Ok(());
    }

    let l2oo_address = args
        .l2oo_address
        .ok_or_else(|| anyhow!("L2OO_ADDRESS required for relay (set via --l2oo-address or env)"))?;

    let dgf_address: Option<Address> = args.dgf_address;

    let config_name_hash = alloy_primitives::keccak256(args.op_succinct_config_name.as_bytes());
    info!(%config_name_hash, config_name = %args.op_succinct_config_name, "Config name hash");

    // Get L2 output at the end block.
    let output = fetcher
        .get_l2_output_at_block(end_block as u64)
        .await
        .context("Failed to get L2 output at end block")?;
    info!(end_block, %output.output_root, "Fetched L2 output");

    // Setup signer from env (PRIVATE_KEY, SIGNER_URL+SIGNER_ADDRESS, or GCP KMS).
    let signer = Signer::from_env().await.context("Failed to create signer for relay")?;
    let l1_rpc: reqwest::Url = std::env::var("L1_RPC")
        .context("L1_RPC required for relay")?
        .parse()
        .context("Invalid L1_RPC URL")?;

    info!(signer_address = %signer.address(), "Relay signer ready");

    // Build the contract instance using a plain HTTP provider (no wallet needed here;
    // we only use the instance to build the TransactionRequest, signing happens in Signer).
    let l1_provider = alloy_provider::ProviderBuilder::new()
        .connect_http(l1_rpc.clone());

    let l2oo_contract = OPSuccinctL2OOContract::new(l2oo_address, &l1_provider);

    let use_dgf = dgf_address.is_some_and(|a| a != Address::ZERO);

    let transaction_request = if use_dgf {
        let dgf_addr = dgf_address.unwrap();
        let dgf_contract = DisputeGameFactoryContract::new(dgf_addr, &l1_provider);

        // Validity game type: https://github.com/ethereum-optimism/optimism/blob/develop/packages/contracts-bedrock/src/dispute/lib/Types.sol#L64
        const OP_SUCCINCT_VALIDITY_DISPUTE_GAME_TYPE: u32 = 6;

        let init_bond = dgf_contract
            .initBonds(OP_SUCCINCT_VALIDITY_DISPUTE_GAME_TYPE)
            .call()
            .await
            .context("Failed to query DGF initBonds")?;

        info!(%dgf_addr, init_bond = ?init_bond, "Using DGF path");

        l2oo_contract
            .dgfProposeL2Output(
                config_name_hash,
                output.output_root,
                U256::from(end_block),
                U256::from(checkpointed_l1_block_number),
                proof_bytes.clone().into(),
                signer.address(),
            )
            .value(init_bond)
            .into_transaction_request()
    } else {
        info!("Using direct L2OO proposeL2Output path");

        l2oo_contract
            .proposeL2Output(
                config_name_hash,
                output.output_root,
                U256::from(end_block),
                U256::from(checkpointed_l1_block_number),
                proof_bytes.clone().into(),
                signer.address(),
            )
            .into_transaction_request()
    };

    info!("Sending relay transaction...");
    let receipt = signer
        .send_transaction_request(l1_rpc, transaction_request)
        .await
        .context("Failed to relay aggregation proof on-chain")?;

    if !receipt.status() {
        return Err(anyhow!("Relay transaction reverted: {:?}", receipt));
    }

    info!(
        tx_hash = %receipt.transaction_hash(),
        "Aggregation proof relayed on-chain successfully"
    );

    Ok(())
}
