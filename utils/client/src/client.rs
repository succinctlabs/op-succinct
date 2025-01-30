use alloy_primitives::{Sealed, B256};
use anyhow::anyhow;
use anyhow::Result;
use kona_driver::Driver;
use kona_executor::{KonaHandleRegister, TrieDBProvider};
use kona_preimage::{CommsClient, PreimageKeyType};
use kona_proof::errors::OracleProviderError;
use kona_proof::executor::KonaExecutor;
use kona_proof::l1::{OracleL1ChainProvider, OraclePipeline};
use kona_proof::l2::OracleL2ChainProvider;
use kona_proof::sync::new_pipeline_cursor;
use kona_proof::{BootInfo, FlushableCache, HintType};
use std::fmt::Debug;
use std::sync::Arc;
use tracing::info;

use crate::oracle::OPSuccinctOracleBlobProvider;

// Sourced from https://github.com/op-rs/kona/tree/main/bin/client/src/single.rs
pub async fn run_opsuccinct_client<O>(
    oracle: Arc<O>,
    handle_register: Option<KonaHandleRegister<OracleL2ChainProvider<O>, OracleL2ChainProvider<O>>>,
) -> Result<BootInfo>
where
    O: CommsClient + FlushableCache + Send + Sync + Debug,
{
    ////////////////////////////////////////////////////////////////
    //                          PROLOGUE                          //
    ////////////////////////////////////////////////////////////////

    let boot = match BootInfo::load(oracle.as_ref()).await {
        Ok(boot) => boot,
        Err(e) => {
            return Err(anyhow!("Failed to load boot info: {:?}", e));
        }
    };

    let boot_arc = Arc::new(boot.clone());

    let safe_head_hash = fetch_safe_head_hash(oracle.as_ref(), boot_arc.as_ref()).await?;

    let mut l1_provider = OracleL1ChainProvider::new(boot.l1_head, oracle.clone());
    let mut l2_provider =
        OracleL2ChainProvider::new(safe_head_hash, boot.rollup_config.clone(), oracle.clone());
    let beacon = OPSuccinctOracleBlobProvider::new(oracle.clone());

    // Fetch the safe head's block header.
    let safe_head = l2_provider
        .header_by_hash(safe_head_hash)
        .map(|header| Sealed::new_unchecked(header, safe_head_hash))?;

    // If the claimed L2 block number is less than the safe head of the L2 chain, the claim is
    // invalid.
    if boot.claimed_l2_block_number < safe_head.number {
        return Err(anyhow!(
            "Claimed L2 block number {claimed} is less than the safe head {safe}",
            claimed = boot.claimed_l2_block_number,
            safe = safe_head.number
        ));
    }

    // In the case where the agreed upon L2 output root is the same as the claimed L2 output root,
    // trace extension is detected and we can skip the derivation and execution steps.
    if boot.agreed_l2_output_root == boot.claimed_l2_output_root {
        info!(
            target: "client",
            "Trace extension detected. State transition is already agreed upon.",
        );
        return Ok(boot.clone());
    }
    ////////////////////////////////////////////////////////////////
    //                   DERIVATION & EXECUTION                   //
    ////////////////////////////////////////////////////////////////

    // Create a new derivation driver with the given boot information and oracle.
    let cursor = new_pipeline_cursor(
        &boot.rollup_config,
        safe_head,
        &mut l1_provider,
        &mut l2_provider,
    )
    .await?;
    l2_provider.set_cursor(cursor.clone());

    let cfg = Arc::new(boot.rollup_config.clone());
    let pipeline = OraclePipeline::new(
        cfg.clone(),
        cursor.clone(),
        oracle.clone(),
        beacon,
        l1_provider.clone(),
        l2_provider.clone(),
    );
    let executor = KonaExecutor::new(
        &cfg,
        l2_provider.clone(),
        l2_provider,
        handle_register,
        None,
    );
    let mut driver = Driver::new(cursor, executor, pipeline);
    // Run the derivation pipeline until we are able to produce the output root of the claimed
    // L2 block.

    // TODO: Replace advance_to_target to get a more refined cycle count.
    #[cfg(target_os = "zkvm")]
    println!("cycle-tracker-report-start: block-execution-and-derivation");
    let (safe_head, output_root) = driver
        .advance_to_target(&boot.rollup_config, Some(boot.claimed_l2_block_number))
        .await?;
    #[cfg(target_os = "zkvm")]
    println!("cycle-tracker-report-end: block-execution-and-derivation");

    ////////////////////////////////////////////////////////////////
    //                          EPILOGUE                          //
    ////////////////////////////////////////////////////////////////

    if output_root != boot.claimed_l2_output_root {
        return Err(anyhow!(
            "Failed to validate L2 block #{number} with output root {output_root}",
            number = safe_head.block_info.number,
            output_root = output_root
        ));
    }

    info!(
        target: "client",
        "Successfully validated L2 block #{number} with output root {output_root}",
        number = safe_head.block_info.number,
        output_root = output_root
    );

    Ok(boot)
}

/// Fetches the safe head hash of the L2 chain based on the agreed upon L2 output root in the
/// [BootInfo].
async fn fetch_safe_head_hash<O>(
    caching_oracle: &O,
    boot_info: &BootInfo,
) -> Result<B256, OracleProviderError>
where
    O: CommsClient,
{
    let mut output_preimage = [0u8; 128];
    HintType::StartingL2Output
        .get_exact_preimage(
            caching_oracle,
            boot_info.agreed_l2_output_root,
            PreimageKeyType::Keccak256,
            &mut output_preimage,
        )
        .await?;

    output_preimage[96..128]
        .try_into()
        .map_err(OracleProviderError::SliceConversion)
}
