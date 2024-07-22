//! A program to verify a Optimism L2 block STF in the zkVM.
#![cfg_attr(target_os = "zkvm", no_main)]

use kona_executor::{StatelessL2BlockExecutor, NoPrecompileOverride};
use kona_client::{
    l1::{OracleBlobProvider, OracleL1ChainProvider},
    BootInfo,
};

mod driver;
pub use driver::MultiBlockDerivationDriver;

mod l2_chain_provider;
use kona_primitives::{OpBlock, L2BlockInfo, SystemConfig};
use l2_chain_provider::MultiblockOracleL2ChainProvider;

use alloc::sync::Arc;
use alloy_consensus::Sealable;
use cfg_if::cfg_if;

extern crate alloc;

cfg_if! {
    // If the target OS is zkVM, set everything up to read input data
    // from SP1 and compile to a program that can be run in zkVM.
    if #[cfg(target_os = "zkvm")] {
        sp1_zkvm::entrypoint!(main);

        use zkvm_common::{
            BootInfoWithoutRollupConfig,
            InMemoryOracle
        };
        use alloc::vec::Vec;
    } else {
        use kona_client::CachingOracle;
    }
}

fn main() {
    zkvm_common::block_on(async move {
        ////////////////////////////////////////////////////////////////
        //                          PROLOGUE                          //
        ////////////////////////////////////////////////////////////////

        cfg_if! {
            // If we are compiling for the zkVM, read inputs from SP1 to generate boot info
            // and in memory oracle.
            if #[cfg(target_os = "zkvm")] {
                let boot = sp1_zkvm::io::read::<BootInfoWithoutRollupConfig>();
                sp1_zkvm::io::commit_slice(&boot.abi_encode());
                let boot: Arc<BootInfo> = Arc::new(boot.into());

                let kv_store_bytes: Vec<u8> = sp1_zkvm::io::read_vec();
                let oracle = Arc::new(InMemoryOracle::from_raw_bytes(kv_store_bytes));

                oracle.verify().expect("key value verification failed");

            // If we are compiling for online mode, create a caching oracle that speaks to the
            // fetcher via hints, and gather boot info from this oracle.
            } else {
                let oracle = Arc::new(CachingOracle::new(1024));
                let boot = Arc::new(BootInfo::load(oracle.as_ref()).await.unwrap());
            }
        }

        let precompile_overrides = NoPrecompileOverride;

        let l1_provider = OracleL1ChainProvider::new(boot.clone(), oracle.clone());
        let mut l2_provider = MultiblockOracleL2ChainProvider::new(boot.clone(), oracle.clone());
        let beacon = OracleBlobProvider::new(oracle.clone());

        ////////////////////////////////////////////////////////////////
        //                   DERIVATION & EXECUTION                   //
        ////////////////////////////////////////////////////////////////

        let mut driver = MultiBlockDerivationDriver::new(
            boot.as_ref(),
            oracle.as_ref(),
            beacon,
            l1_provider,
            l2_provider.clone(),
        )
        .await
        .unwrap();

        let mut executor = StatelessL2BlockExecutor::builder(&boot.rollup_config)
            .with_parent_header(driver.clone_l2_safe_head_header())
            .with_fetcher(l2_provider.clone())
            .with_hinter(l2_provider.clone())
            .with_precompile_overrides(precompile_overrides)
            .build()
            .unwrap();

        let mut last_block_num = driver.l2_safe_head.block_info.number;

        loop {
            let l2_attrs_with_parents = driver.produce_payloads().await.unwrap();
            if l2_attrs_with_parents.is_empty() {
                continue;
            }

            for payload in l2_attrs_with_parents {
                let parent_block_number = payload.parent.block_info.number;

                let header = executor.execute_payload(payload.attributes).unwrap();
                let new_block_number = header.number;
                assert_eq!(new_block_number, parent_block_number + 1);

                // Add all data from this block's execution to the cache.
                l2_provider.add_header_to_cache(header.clone());

                // TODO: Reconstruct this, needs txs which may need to be surfaces from executor.
                let op_block = OpBlock::default();
                l2_provider.add_payload_to_cache(parent_block_number, op_block.into());

                // TODO: Create this out of Header info (also needs L1 origin info, need to find it).
                let l2_block_info = L2BlockInfo::default();
                l2_provider.add_l2_block_info_to_cache(new_block_number, l2_block_info);

                // TODO: Figure out how to derive this.
                let system_config = SystemConfig::default();
                l2_provider.add_system_config_to_cache(new_block_number, system_config);

                // Update data for the next iteration.
                driver.update_safe_head(l2_block_info, header.clone().seal_slow());
                last_block_num = new_block_number;
            }

            if last_block_num == boot.l2_claim_block {
                break;
            }
        }

        let output_root = executor.compute_output_root().unwrap();

        ////////////////////////////////////////////////////////////////
        //                          EPILOGUE                          //
        ////////////////////////////////////////////////////////////////

        assert_eq!(last_block_num, boot.l2_claim_block);
        assert_eq!(output_root, boot.l2_claim);
    });
}
