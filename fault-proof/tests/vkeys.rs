//! Verify test-environment keys against the embedded programs across runtime lifetimes.
#![recursion_limit = "256"]

pub mod common;

use alloy_primitives::B256;
use op_succinct_elfs::AGGREGATION_ELF;
use op_succinct_proof_utils::get_range_elf_embedded;
use sp1_sdk::{Elf, HashableKey, Prover, ProverClient, ProvingKey};

#[test]
fn test_environment_keys_match_embedded_programs_across_runtimes() -> anyhow::Result<()> {
    let started = std::time::Instant::now();
    let expected = tokio::runtime::Runtime::new()?.block_on(async {
        let prover = ProverClient::builder().cpu().build().await;
        let aggregation = prover.setup(Elf::Static(AGGREGATION_ELF)).await?;
        let range = prover.setup(Elf::Static(get_range_elf_embedded())).await?;
        anyhow::Ok((
            B256::from(aggregation.verifying_key().bytes32_raw()),
            B256::from(range.verifying_key().hash_bytes()),
        ))
    })?;

    eprintln!("uncached SDK VK derivation: {:?}; keys: {expected:?}", started.elapsed());

    for attempt in 0..2 {
        let started = std::time::Instant::now();
        let actual = tokio::runtime::Runtime::new()?.block_on(common::env::compute_vkeys())?;
        assert_eq!(actual, expected);
        eprintln!("test-environment VK call {attempt}: {:?}", started.elapsed());
    }
    Ok(())
}
