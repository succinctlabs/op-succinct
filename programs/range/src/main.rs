//! A program to verify a Optimism L2 block STF in the zkVM.
//!
//! This binary contains the client program for executing the Optimism rollup state transition
//! across a range of blocks, which can be used to generate an on chain validity proof. Depending on
//! the compilation pipeline, it will compile to be run either in native mode or in zkVM mode. In
//! native mode, the data for verifying the batch validity is fetched from RPC, while in zkVM mode,
//! the data is supplied by the host binary to the verifiable program.

#![no_main]
sp1_zkvm::entrypoint!(main);

extern crate alloc;

use alloc::sync::Arc;

use alloy_consensus::Sealed;
use alloy_primitives::B256;
use kona_driver::Driver;
use kona_executor::TrieDBProvider;
use kona_preimage::{CommsClient, PreimageKeyType};
use kona_proof::{
    errors::OracleProviderError,
    executor::KonaExecutor,
    l1::{OracleBlobProvider, OracleL1ChainProvider, OraclePipeline},
    l2::OracleL2ChainProvider,
    sync::new_pipeline_cursor,
    BootInfo, HintType,
};
use maili_genesis::RollupConfig;
use op_succinct_client_utils::precompiles::zkvm_handle_register;
use tracing::{error, info};

use alloc::vec::Vec;
use op_succinct_client_utils::{boot::BootInfoStruct, BootInfoWithBytesConfig, InMemoryOracle};
use serde_json;

fn main() {
    #[cfg(feature = "tracing-subscriber")]
    {
        use anyhow::anyhow;
        use tracing::Level;

        let subscriber = tracing_subscriber::fmt()
            .with_max_level(Level::INFO)
            .finish();
        tracing::subscriber::set_global_default(subscriber)
            .map_err(|e| anyhow!(e))
            .unwrap();
    }

    kona_proof::block_on(async move {
        ////////////////////////////////////////////////////////////////
        //                          PROLOGUE                          //
        ////////////////////////////////////////////////////////////////

        println!("cycle-tracker-start: boot-load");
        let boot_info_with_bytes_config = sp1_zkvm::io::read::<BootInfoWithBytesConfig>();

        // BootInfoStruct is identical to BootInfoWithBytesConfig, except it replaces
        // the rollup_config_bytes with a hash of those bytes (rollupConfigHash). Securely
        // hashes the rollup config bytes.
        let boot_info_struct = BootInfoStruct::from(boot_info_with_bytes_config.clone());
        sp1_zkvm::io::commit::<BootInfoStruct>(&boot_info_struct);

        let rollup_config: RollupConfig =
            serde_json::from_slice(&boot_info_with_bytes_config.rollup_config_bytes)
                .expect("failed to parse rollup config");
        let boot: Arc<BootInfo> = Arc::new(BootInfo {
            l1_head: boot_info_with_bytes_config.l1_head,
            agreed_l2_output_root: boot_info_with_bytes_config.l2_output_root,
            claimed_l2_output_root: boot_info_with_bytes_config.l2_claim,
            claimed_l2_block_number: boot_info_with_bytes_config.l2_claim_block,
            chain_id: boot_info_with_bytes_config.chain_id,
            rollup_config,
        });
        println!("cycle-tracker-end: boot-load");

        println!("cycle-tracker-start: oracle-load");
        let in_memory_oracle_bytes: Vec<u8> = sp1_zkvm::io::read_vec();
        let oracle = Arc::new(InMemoryOracle::from_raw_bytes(in_memory_oracle_bytes));
        println!("cycle-tracker-end: oracle-load");

        println!("cycle-tracker-report-start: oracle-verify");
        oracle.verify().expect("key value verification failed");
        println!("cycle-tracker-report-end: oracle-verify");

        let safe_head_hash = fetch_safe_head_hash(oracle.as_ref(), boot.as_ref())
            .await
            .unwrap();

        let mut l1_provider = OracleL1ChainProvider::new(boot.l1_head, oracle.clone());
        let mut l2_provider =
            OracleL2ChainProvider::new(safe_head_hash, boot.rollup_config.clone(), oracle.clone());
        let beacon = OracleBlobProvider::new(oracle.clone());

        // Fetch the safe head's block header.
        let safe_head = l2_provider
            .header_by_hash(safe_head_hash)
            .map(|header| Sealed::new_unchecked(header, safe_head_hash))
            .expect("Failed to fetch safe head");

        // If the claimed L2 block number is less than the safe head of the L2 chain, the claim is
        // invalid.
        if boot.claimed_l2_block_number < safe_head.number {
            error!(
                target: "client",
                "Claimed L2 block number {claimed} is less than the safe head {safe}",
                claimed = boot.claimed_l2_block_number,
                safe = safe_head.number
            );
            panic!(
                "Invalid claim. Expected {:?} actual {:?}",
                boot.claimed_l2_output_root, boot.agreed_l2_output_root
            );
        }

        // In the case where the agreed upon L2 output root is the same as the claimed L2 output root,
        // trace extension is detected and we can skip the derivation and execution steps.
        if boot.agreed_l2_output_root == boot.claimed_l2_output_root {
            info!(
                target: "client",
                "Trace extension detected. State transition is already agreed upon.",
            );
            return;
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
        .await
        .expect("Failed to create pipeline cursor");
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
            Some(zkvm_handle_register),
            None,
        );
        let mut driver = Driver::new(cursor, executor, pipeline);

        // Run the derivation pipeline until we are able to produce the output root of the claimed
        // L2 block.
        let (number, _, output_root) = driver
            .advance_to_target(&boot.rollup_config, Some(boot.claimed_l2_block_number))
            .await
            .expect("Failed to advance to target");

        ////////////////////////////////////////////////////////////////
        //                          EPILOGUE                          //
        ////////////////////////////////////////////////////////////////

        if output_root != boot.claimed_l2_output_root {
            error!(
                target: "client",
                "Failed to validate L2 block #{number} with output root {output_root}",
                number = number,
                output_root = output_root
            );
            panic!(
                "Invalid claim. Expected {:?} actual {:?}",
                boot.claimed_l2_output_root, output_root
            );
        }

        info!(
            target: "client",
            "Successfully validated L2 block #{number} with output root {output_root}",
            number = number,
            output_root = output_root
        );

        // Manually forget large objects to avoid allocator overhead
        std::mem::forget(l1_provider);
        std::mem::forget(oracle);
        std::mem::forget(cfg);
        std::mem::forget(boot);
    });
}

/// Fetches the safe head hash of the L2 chain based on the agreed upon L2 output root in the
/// [BootInfo].
/// 
/// Sourced from Kona until it's exposed nicely from a crate that doesn't depend on kona-std-fpvm, which can compile in zkVM mode.
/// https://github.com/op-rs/kona/blob/a59f643d0627320efff49f40f4803741ae9194f1/bin/client/src/single.rs#L153-L155.
pub async fn fetch_safe_head_hash<O>(
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
