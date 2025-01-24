//! A program to verify a Optimism L2 block STF in the zkVM.
//!
//! This binary contains the client program for executing the Optimism rollup state transition
//! across a range of blocks, which can be used to generate an on chain validity proof. Depending on
//! the compilation pipeline, it will compile to be run either in native mode or in zkVM mode. In
//! native mode, the data for verifying the batch validity is fetched from RPC, while in zkVM mode,
//! the data is supplied by the host binary to the verifiable program.

#![cfg_attr(target_os = "zkvm", no_main)]

extern crate alloc;

use alloc::sync::Arc;

use alloy_consensus::{BlockBody, Sealable};
use alloy_primitives::B256;
use alloy_rlp::Decodable;
use cfg_if::cfg_if;
use core::fmt::Debug;
use kona_driver::{Driver, DriverError};
use kona_executor::{ExecutorError, KonaHandleRegister, TrieDBProvider};
use kona_preimage::{CommsClient, HintWriterClient, PreimageKeyType, PreimageOracleClient};
use kona_proof::{
    errors::OracleProviderError,
    executor::KonaExecutor,
    l1::{OracleBlobProvider, OracleL1ChainProvider, OraclePipeline},
    l2::OracleL2ChainProvider,
    sync::new_pipeline_cursor,
    BootInfo, CachingOracle, HintType,
};
use maili_genesis::RollupConfig;
use maili_protocol::L2BlockInfo;
use op_alloy_consensus::{OpBlock, OpTxEnvelope, OpTxType};
use op_alloy_rpc_types_engine::OpAttributesWithParent;
use op_succinct_client_utils::precompiles::zkvm_handle_register;
use tracing::{error, info, warn};

cfg_if! {
    if #[cfg(target_os = "zkvm")] {
        sp1_zkvm::entrypoint!(main);

        use op_succinct_client_utils::{
            BootInfoWithBytesConfig, boot::BootInfoStruct,
            InMemoryOracle
        };
        use alloc::vec::Vec;
        use serde_json;
    } else {
        use op_succinct_client_utils::pipes::{ORACLE_READER, HINT_WRITER};
    }
}

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

    op_succinct_client_utils::block_on(async move {
        ////////////////////////////////////////////////////////////////
        //                          PROLOGUE                          //
        ////////////////////////////////////////////////////////////////

        cfg_if! {
            // If we are compiling for the zkVM, read inputs from SP1 to generate boot info
            // and in memory oracle.
            if #[cfg(target_os = "zkvm")] {
                println!("cycle-tracker-start: boot-load");
                let boot_info_with_bytes_config = sp1_zkvm::io::read::<BootInfoWithBytesConfig>();

                // BootInfoStruct is identical to BootInfoWithBytesConfig, except it replaces
                // the rollup_config_bytes with a hash of those bytes (rollupConfigHash). Securely
                // hashes the rollup config bytes.
                let boot_info_struct = BootInfoStruct::from(boot_info_with_bytes_config.clone());
                sp1_zkvm::io::commit::<BootInfoStruct>(&boot_info_struct);

                let rollup_config: RollupConfig = serde_json::from_slice(&boot_info_with_bytes_config.rollup_config_bytes).expect("failed to parse rollup config");
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
            } else {
                const ORACLE_LRU_SIZE: usize = 1024;
                let oracle = Arc::new(CachingOracle::new(ORACLE_LRU_SIZE, oracle_client, hint_client));
                let boot = Arc::new(BootInfo::load(oracle.as_ref()).await.unwrap());
            }
        }

        let safe_head_hash = fetch_safe_head_hash(oracle.as_ref(), boot.as_ref()).await.unwrap();

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
            return Err(FaultProofProgramError::InvalidClaim(
                boot.agreed_l2_output_root,
                boot.claimed_l2_output_root,
            ));
        }

        // In the case where the agreed upon L2 output root is the same as the claimed L2 output root,
        // trace extension is detected and we can skip the derivation and execution steps.
        if boot.agreed_l2_output_root == boot.claimed_l2_output_root {
            info!(
                target: "client",
                "Trace extension detected. State transition is already agreed upon.",
            );
            return Ok(());
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
            handle_register,
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
            return Err(FaultProofProgramError::InvalidClaim(
                output_root,
                boot.claimed_l2_output_root,
            ));
        }

        info!(
            target: "client",
            "Successfully validated L2 block #{number} with output root {output_root}",
            number = number,
            output_root = output_root
        );

        // // Manually forget large objects to avoid allocator overhead
        // std::mem::forget(pipeline);
        // std::mem::forget(executor);
        // std::mem::forget(l2_provider);
        // std::mem::forget(l1_provider);
        // std::mem::forget(oracle);
        // std::mem::forget(cfg);
        // std::mem::forget(cursor);
        // std::mem::forget(boot);
    });
}
