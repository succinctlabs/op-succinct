use std::{fmt::Debug, sync::Arc};

use alloy_primitives::Sealed;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use kona_derive::traits::{Pipeline, SignalReceiver};
use kona_driver::{Driver, DriverPipeline, PipelineCursor};
use kona_executor::TrieDBProvider;
use kona_preimage::CommsClient;
use kona_proof::{
    executor::KonaExecutor, l1::OracleL1ChainProvider, l2::OracleL2ChainProvider,
    sync::new_pipeline_cursor, BootInfo, FlushableCache,
};
use spin::RwLock;
use tracing::info;

use crate::{
    client::{advance_to_target, fetch_safe_head_hash},
    precompiles::zkvm_handle_register,
};

#[async_trait]
pub trait WitnessExecutor {
    // Gets the inputs for constructing the derivation pipeline.
    async fn get_inputs_for_pipeline<O>(
        &self,
        oracle: Arc<O>,
    ) -> Result<(
        BootInfo,
        Option<(Arc<RwLock<PipelineCursor>>, OracleL1ChainProvider<O>, OracleL2ChainProvider<O>)>,
    )>
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

        let boot_clone = boot.clone();

        let rollup_config = Arc::new(boot.rollup_config);
        let safe_head_hash =
            fetch_safe_head_hash(oracle.as_ref(), boot.agreed_l2_output_root).await?;

        let mut l1_provider = OracleL1ChainProvider::new(boot.l1_head, oracle.clone());
        let mut l2_provider =
            OracleL2ChainProvider::new(safe_head_hash, rollup_config.clone(), oracle.clone());

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

        // In the case where the agreed upon L2 output root is the same as the claimed L2 output
        // root, trace extension is detected and we can skip the derivation and execution
        // steps.
        if boot.agreed_l2_output_root == boot.claimed_l2_output_root {
            info!(
                target: "client",
                "Trace extension detected. State transition is already agreed upon.",
            );
            return Ok((boot_clone, None));
        }
        ////////////////////////////////////////////////////////////////
        //                   DERIVATION & EXECUTION                   //
        ////////////////////////////////////////////////////////////////

        // Create a new derivation driver with the given boot information and oracle.
        let cursor = new_pipeline_cursor(
            rollup_config.as_ref(),
            safe_head,
            &mut l1_provider,
            &mut l2_provider,
        )
        .await?;
        l2_provider.set_cursor(cursor.clone());

        Ok((boot_clone, Some((cursor, l1_provider, l2_provider))))
    }

    // Sourced from https://github.com/op-rs/kona/tree/main/bin/client/src/single.rs
    // Runs the OP Succinct witness executor using the given derivation pipeline,
    async fn run<O, DP, P>(
        &self,
        boot: BootInfo,
        pipeline: DP,
        cursor: Arc<RwLock<PipelineCursor>>,
        l2_provider: OracleL2ChainProvider<O>,
    ) -> Result<BootInfo>
    where
        O: CommsClient + FlushableCache + Send + Sync + Debug,
        DP: DriverPipeline<P> + Send + Sync + Debug,
        P: Pipeline + SignalReceiver + Send + Sync + Debug,
    {
        let boot_clone = boot.clone();

        let rollup_config = Arc::new(boot.rollup_config);

        let executor = KonaExecutor::new(
            rollup_config.as_ref(),
            l2_provider.clone(),
            l2_provider,
            Some(zkvm_handle_register),
            None,
        );
        let mut driver = Driver::new(cursor, executor, pipeline);
        // Run the derivation pipeline until we are able to produce the output root of the claimed
        // L2 block.

        // Use custom advance to target with cycle tracking.
        #[cfg(target_os = "zkvm")]
        println!("cycle-tracker-report-start: block-execution-and-derivation");
        let (safe_head, output_root) = advance_to_target(
            &mut driver,
            rollup_config.as_ref(),
            Some(boot.claimed_l2_block_number),
        )
        .await?;
        #[cfg(target_os = "zkvm")]
        println!("cycle-tracker-report-end: block-execution-and-derivation");

        ////////////////////////////////////////////////////////////////
        //                          EPILOGUE                          //
        ////////////////////////////////////////////////////////////////

        if output_root != boot.claimed_l2_output_root {
            return Err(anyhow!(
            "Failed to validate L2 block #{number} with claimed output root {claimed_output_root}. Got {output_root} instead",
            number = safe_head.block_info.number,
            output_root = output_root,
            claimed_output_root = boot.claimed_l2_output_root,
        ));
        }

        info!(
            target: "client",
            "Successfully validated L2 block #{number} with output root {output_root}",
            number = safe_head.block_info.number,
            output_root = output_root
        );

        #[cfg(target_os = "zkvm")]
        {
            std::mem::forget(driver);
            std::mem::forget(rollup_config);
        }

        Ok(boot_clone)
    }
}

pub struct ETHDAWitnessExecutor;

impl WitnessExecutor for ETHDAWitnessExecutor {}

pub struct CelestiaDAWitnessExecutor;

impl WitnessExecutor for CelestiaDAWitnessExecutor {}

pub struct EigenDAWitnessExecutor;

impl WitnessExecutor for EigenDAWitnessExecutor {}
