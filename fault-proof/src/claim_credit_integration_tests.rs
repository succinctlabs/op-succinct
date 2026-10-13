//! Executes bond claims against local L1 contracts without an L2 node or proof setup.

use std::{fs, path::Path, process::Command};

use alloy_eips::BlockId;
use alloy_node_bindings::{Anvil, AnvilInstance};
use alloy_primitives::{address, Address, B256, U256};
use alloy_provider::{Provider, ProviderBuilder, RootProvider};
use anyhow::{ensure, Context, Result};
use op_succinct_bindings::op_succinct_fault_dispute_game::OPSuccinctFaultDisputeGame as GameWithClose;
use op_succinct_signer_utils::Signer;
use serde_json::{json, Value};

use crate::{
    claim_credit_transaction,
    contract::{AnchorStateRegistry, DisputeGameFactory, GameStatus, OPSuccinctFaultDisputeGame},
};

#[tokio::test]
async fn claim_credit_pays_both_roles_and_updates_only_eligible_anchors() -> Result<()> {
    let (anvil, provider, factory_address, registry_address) = setup_local_claims().await?;
    let rpc = anvil.endpoint_url();
    let proposer = anvil.addresses()[0];
    let challenger = anvil.addresses()[1];
    let initial_bond = U256::from(10_000_000_000_000_000u64);
    let challenger_bond = U256::from(1_000_000_000_000_000_000u64);
    let factory = DisputeGameFactory::new(factory_address, provider.clone());
    let registry = AnchorStateRegistry::new(registry_address, provider.clone());
    let mut expected_anchor = Address::ZERO;

    // Each game is independent so an invalid proposal cannot invalidate another case's parent.
    for (index, (challenged, already_closed)) in
        [(false, false), (false, true), (true, false), (true, true)].into_iter().enumerate()
    {
        provider.client().request::<_, Value>("evm_increaseTime", [1]).await?;
        provider.client().request::<_, Value>("evm_mine", ()).await?;
        let mut extra_data = U256::from((index + 1) * 1000).to_be_bytes::<32>().to_vec();
        extra_data.extend_from_slice(&u32::MAX.to_be_bytes());
        let receipt = factory
            .create(42, B256::repeat_byte(index as u8 + 1), extra_data.into())
            .from(proposer)
            .value(initial_bond)
            .send()
            .await?
            .get_receipt()
            .await?;
        assert!(receipt.status());
        let game_address = factory.gameAtIndex(U256::from(index)).call().await?.proxy;
        let game = OPSuccinctFaultDisputeGame::new(game_address, provider.clone());
        if challenged {
            assert!(game
                .challenge()
                .from(challenger)
                .value(challenger_bond)
                .send()
                .await?
                .get_receipt()
                .await?
                .status());
        }
        provider.client().request::<_, Value>("evm_increaseTime", [43201]).await?;
        provider.client().request::<_, Value>("evm_mine", ()).await?;
        assert!(game.resolve().from(proposer).send().await?.get_receipt().await?.status());
        assert_eq!(
            game.status().call().await?,
            if challenged { GameStatus::CHALLENGER_WINS } else { GameStatus::DEFENDER_WINS }
        );
        provider.client().request::<_, Value>("evm_increaseTime", [11]).await?;
        provider.client().request::<_, Value>("evm_mine", ()).await?;
        if already_closed {
            assert!(GameWithClose::new(game_address, provider.clone())
                .closeGame()
                .from(proposer)
                .send()
                .await?
                .get_receipt()
                .await?
                .status());
        }
        let key_index = usize::from(challenged);
        let signer = Signer::new_local_signer(&format!(
            "0x{}",
            hex::encode(anvil.keys()[key_index].to_bytes())
        ))?;
        let claimant = signer.address();
        let credit = game.credit(claimant).call().await?;
        assert_eq!(credit, if challenged { initial_bond + challenger_bond } else { initial_bond });
        let balance_before = provider.get_balance(claimant).await?;
        let estimate = game.claimCredit(claimant).from(claimant).estimate_gas().await?;
        if !challenged {
            expected_anchor = game_address;
        }
        let request = claim_credit_transaction(&game, claimant).await?;
        assert_eq!(request.from, Some(claimant));
        assert_eq!(request.gas, Some(estimate));
        let gas_limit = request.gas.unwrap();
        let receipt =
            signer.send_transaction_request_with_timeout(rpc.clone(), request, 30).await?;
        let sent: Value = provider
            .client()
            .request("eth_getTransactionByHash", (receipt.transaction_hash,))
            .await?;
        assert_eq!(sent["gas"], format!("{gas_limit:#x}"));
        let trace: Value = provider
            .client()
            .request(
                "debug_traceTransaction",
                (receipt.transaction_hash, json!({"tracer": "callTracer"})),
            )
            .await?;
        let mut errors = Vec::new();
        collect_call_errors(&trace, &mut errors);
        let actual_anchor = registry.anchorGame().call().await?;
        eprintln!(
            "production claim {}",
            json!({
                "challenged": challenged, "already_closed": already_closed,
                "estimate": estimate, "limit": gas_limit,
                "used": receipt.gas_used, "payout": credit.to_string(), "status": receipt.status(),
                "anchor": actual_anchor, "expected_anchor": expected_anchor,
                "anchor_correct": actual_anchor == expected_anchor, "errors": errors
            })
        );
        assert!(receipt.status());
        let fee = U256::from(receipt.gas_used) * U256::from(receipt.effective_gas_price);
        assert_eq!(provider.get_balance(claimant).await?, balance_before + credit - fee);
        assert_eq!(provider.get_balance(game_address).await?, U256::ZERO);
        assert_eq!(game.credit(claimant).call().await?, U256::ZERO);
        assert_eq!(actual_anchor, expected_anchor);
        assert!(errors.iter().all(|error| error["error"] != "out of gas"));
        // A spent credit must fail during estimation before a second paid submission.
        let nonce = provider.get_transaction_count(claimant).await?;
        assert!(claim_credit_transaction(&game, claimant).await.is_err());
        assert_eq!(provider.get_transaction_count(claimant).await?, nonce);
    }
    Ok(())
}

fn collect_call_errors(trace: &Value, errors: &mut Vec<Value>) {
    if let Some(error) = trace.get("error") {
        errors.push(json!({"to": trace["to"], "error": error, "output": trace["output"]}));
    }
    if let Some(calls) = trace["calls"].as_array() {
        for call in calls {
            collect_call_errors(call, errors);
        }
    }
}

async fn setup_local_claims() -> Result<(AnvilInstance, RootProvider, Address, Address)> {
    let anvil = Anvil::new().block_time(1).arg("--disable-code-size-limit").spawn();
    deploy_local_claims(anvil).await
}

async fn deploy_local_claims(
    anvil: AnvilInstance,
) -> Result<(AnvilInstance, RootProvider, Address, Address)> {
    let rpc = anvil.endpoint_url();
    let provider = ProviderBuilder::new().disable_recommended_fillers().connect_http(rpc.clone());
    let proposer = anvil.addresses()[0];
    let challenger = anvil.addresses()[1];
    let contracts_dir = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().join("contracts");
    let config = tempfile::NamedTempFile::new()?;
    fs::write(
        config.path(),
        serde_json::to_vec(&json!({
            "aggregationVkey": B256::ZERO,
            "challengerAddresses": [challenger],
            "challengerBondWei": 1_000_000_000_000_000_000u64,
            "disputeGameFinalityDelaySeconds": 10,
            "existingAnchorStateRegistry": Address::ZERO,
            "existingDisputeGameFactoryProxy": Address::ZERO,
            "fallbackTimeoutFpSecs": 3600,
            "gameType": 42,
            "initialBondWei": 10_000_000_000_000_000u64,
            "maxChallengeDuration": 3600,
            "maxProveDuration": 43200,
            "optimismPortal2Address": Address::ZERO,
            "permissionlessMode": false,
            "proposerAddresses": [proposer],
            "rangeVkeyCommitment": B256::ZERO,
            "rollupConfigHash": B256::ZERO,
            "startingL2BlockNumber": 0,
            "startingRoot": B256::repeat_byte(0x11),
            "useSp1MockVerifier": true,
            "verifierAddress": Address::ZERO
        }))?,
    )?;
    let deploy = Command::new("forge")
        .args(["script", "script/fp/DeployOPSuccinctFDG.s.sol", "--broadcast", "--json"])
        .arg("--rpc-url")
        .arg(rpc.as_str())
        .arg("--private-key")
        .arg(format!("0x{}", hex::encode(anvil.keys()[0].to_bytes())))
        .env("OP_SUCCINCT_FAULT_DISPUTE_GAME_CONFIG_PATH", config.path())
        .env("RUST_LOG", "off")
        .current_dir(contracts_dir)
        .output()?;
    ensure!(
        deploy.status.success(),
        "forge deployment failed: {}",
        String::from_utf8_lossy(&deploy.stderr)
    );
    let stdout = String::from_utf8(deploy.stdout)?;
    let deployment: Value =
        serde_json::from_str(stdout.lines().next().context("missing forge deployment output")?)?;
    ensure!(deployment["success"] == true, "forge script failed: {deployment}");
    let factory_address = deployment["returns"]["factoryProxy"]["value"]
        .as_str()
        .context("missing factory address")?
        .parse()?;
    let registry_address = deployment["returns"]["anchorStateRegistry"]["value"]
        .as_str()
        .context("missing registry address")?
        .parse()?;
    Ok((anvil, provider, factory_address, registry_address))
}

#[tokio::test]
#[ignore = "Requires L1_RPC pointing to an archive Sepolia node with debug_traceCall."]
async fn claim_credit_replays_reported_sepolia_failure() -> Result<()> {
    let block = BlockId::from(11_865_092);
    let provider = ProviderBuilder::new()
        .disable_recommended_fillers()
        .with_default_block(block)
        .connect_http(std::env::var("L1_RPC").context("L1_RPC is required")?.parse()?);
    assert_eq!(provider.get_chain_id().await?, 11_155_111);
    let claimant = address!("0x736e68af2cbf2ab0e46e4310fe5ae568b3642ff6");
    let game_address = address!("0x595827e0fdddaada1ff463553888dde08a8927fd");
    let contract = OPSuccinctFaultDisputeGame::new(game_address, provider.clone());
    let credit = contract.credit(claimant).call().await?;
    assert!(credit > U256::ZERO);

    // Replay only simulations at the original parent block; never broadcast a claim.
    let old_request =
        contract.claimCredit(claimant).from(claimant).gas(200_000).into_transaction_request();
    let old_trace: Value = provider
        .client()
        .request("debug_traceCall", (old_request, block, json!({"tracer": "callTracer"})))
        .await?;
    assert_eq!(old_trace["calls"][0]["error"], "out of gas");

    let estimate = contract.claimCredit(claimant).from(claimant).estimate_gas().await?;
    let request = claim_credit_transaction(&contract, claimant).await?;
    assert_eq!(request.gas, Some(estimate));
    let gas_limit = request.gas.unwrap();
    assert!(gas_limit > 200_000);
    let trace: Value = provider
        .client()
        .request("debug_traceCall", (&request, block, json!({"tracer": "callTracer"})))
        .await?;
    assert!(trace.get("error").is_none(), "claim reverted: {trace}");
    let payout = trace["calls"][0]["calls"]
        .as_array()
        .unwrap()
        .iter()
        .find(|call| call["to"] == claimant.to_string().to_lowercase())
        .unwrap();
    assert_eq!(payout["value"], format!("{credit:#x}"));
    assert!(payout.get("error").is_none());

    let diff: Value = provider
        .client()
        .request(
            "debug_traceCall",
            (
                request,
                block,
                json!({"tracer": "prestateTracer", "tracerConfig": {"diffMode": true}}),
            ),
        )
        .await?;
    let game_key = game_address.to_string().to_lowercase();
    assert_eq!(diff["post"][&game_key]["balance"], "0x0");
    let pre_storage = diff["pre"][&game_key]["storage"].as_object().unwrap();
    let credit_slots: Vec<_> = pre_storage
        .iter()
        .filter(|(_, value)| value.as_str().and_then(|v| v.parse::<U256>().ok()) == Some(credit))
        .map(|(slot, _)| slot)
        .collect();
    assert_eq!(credit_slots.len(), 2);
    for slot in credit_slots {
        assert!(diff["post"][&game_key]["storage"].get(slot).is_none());
    }
    let mut errors = Vec::new();
    collect_call_errors(&trace, &mut errors);
    assert!(errors.iter().all(|error| error["error"] != "out of gas"));
    eprintln!(
        "production archive claim {}",
        json!({"estimate": estimate, "limit": gas_limit, "payout": credit.to_string(), "errors": errors})
    );
    Ok(())
}

#[tokio::test]
#[ignore = "Requires L1_RPC pointing to an archive Sepolia node."]
async fn claim_credit_rejects_historical_recipient_and_recovers_without_fallback() -> Result<()> {
    let anvil = Anvil::new()
        .block_time(1)
        .fork(std::env::var("L1_RPC").context("L1_RPC is required")?)
        .fork_block_number(9_130_971)
        .arg("--disable-code-size-limit")
        .spawn();
    let (anvil, provider, factory_address, registry_address) = deploy_local_claims(anvil).await?;
    let signer =
        Signer::new_local_signer(&format!("0x{}", hex::encode(anvil.keys()[0].to_bytes())))?;
    let claimant = signer.address();
    let inherited_code = provider.get_code_at(claimant).await?;
    assert_eq!(inherited_code.to_string(), "0xef010083dc91a00bc9902a58eba53e9bc26b1b7be789ff");
    let factory = DisputeGameFactory::new(factory_address, provider.clone());
    // Games created at the registry's initialization timestamp are retired.
    provider.client().request::<_, Value>("evm_increaseTime", [1]).await?;
    provider.client().request::<_, Value>("evm_mine", ()).await?;
    let mut extra = U256::from(1000).to_be_bytes::<32>().to_vec();
    extra.extend_from_slice(&u32::MAX.to_be_bytes());
    assert!(factory
        .create(42, B256::repeat_byte(0x33), extra.into())
        .from(claimant)
        .value(U256::from(10_000_000_000_000_000u64))
        .send()
        .await?
        .get_receipt()
        .await?
        .status());
    let game_address = factory.gameAtIndex(U256::ZERO).call().await?.proxy;
    let game = OPSuccinctFaultDisputeGame::new(game_address, provider.clone());
    provider.client().request::<_, Value>("evm_increaseTime", [3601]).await?;
    provider.client().request::<_, Value>("evm_mine", ()).await?;
    assert!(game.resolve().from(claimant).send().await?.get_receipt().await?.status());
    provider.client().request::<_, Value>("evm_increaseTime", [11]).await?;
    provider.client().request::<_, Value>("evm_mine", ()).await?;
    let credit = game.credit(claimant).call().await?;
    assert!(credit > U256::ZERO);
    let nonce = provider.get_transaction_count(claimant).await?;
    let balance = provider.get_balance(claimant).await?;
    let registry = AnchorStateRegistry::new(registry_address, provider.clone());
    let anchor = registry.anchorGame().call().await?;

    for _ in 0..2 {
        let error = claim_credit_transaction(&game, claimant).await.unwrap_err();
        assert!(format!("{error:#}").contains("0x83e6cc6b"), "unexpected error: {error:#}");
        assert_eq!(provider.get_transaction_count(claimant).await?, nonce);
        assert_eq!(provider.get_balance(claimant).await?, balance);
        assert_eq!(game.credit(claimant).call().await?, credit);
        assert_eq!(registry.anchorGame().call().await?, anchor);
    }

    // Clear only the inherited recipient code to verify the same claim can recover.
    provider.client().request::<_, Value>("anvil_setCode", (claimant, "0x")).await?;
    let request = claim_credit_transaction(&game, claimant).await?;
    let receipt =
        signer.send_transaction_request_with_timeout(anvil.endpoint_url(), request, 30).await?;
    assert!(receipt.status());
    let fee = U256::from(receipt.gas_used) * U256::from(receipt.effective_gas_price);
    assert_eq!(provider.get_balance(claimant).await?, balance + credit - fee);
    assert_eq!(game.credit(claimant).call().await?, U256::ZERO);
    assert_eq!(provider.get_balance(game_address).await?, U256::ZERO);
    assert_eq!(registry.anchorGame().call().await?, game_address);
    eprintln!("historical recipient: real estimation rejection, no paid submission, recovered payout={credit} gas_used={}", receipt.gas_used);
    Ok(())
}
