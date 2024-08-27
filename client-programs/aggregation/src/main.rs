//! A simple program that aggregates the proofs of multiple programs proven with the zkVM.

#![cfg_attr(target_os = "zkvm", no_main)]
#[cfg(target_os = "zkvm")]
sp1_zkvm::entrypoint!(main);

use alloy_consensus::Header;
use alloy_primitives::B256;
use alloy_sol_types::SolValue;
use client_utils::{boot::BootInfoStruct, types::AggregationInputs};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

/// Note: This is the hardcoded program vkey for the multi-block program. Whenever the multi-block
/// program changes, update this.
/// TODO: The aggregation program should take in an arbitrary vkey digest, and the smart contract
/// should verify the proof matches the arbitrary vkey digest stored in the contract. This means
/// that the aggregate program would no longer need to update this value.
const MULTI_BLOCK_PROGRAM_VKEY_DIGEST: [u32; 8] = [
    227309663, 1637133225, 136526498, 1878261023, 2013043842, 450616441, 575447582, 1643259779,
];

/// Verify that the L1 heads in the boot infos are in the header chain.
fn verify_l1_heads(agg_inputs: &AggregationInputs, headers: &[Header]) {
    // Create a map of each l1_head in the BootInfo's to booleans
    let mut l1_heads_map: HashMap<B256, bool> = agg_inputs
        .boot_infos
        .iter()
        .map(|boot_info| (boot_info.l1Head, false))
        .collect();

    // Iterate through all headers in the chain.
    let mut current_hash = agg_inputs.latest_l1_checkpoint_head;
    // Iterate through the headers in reverse order. The headers should be sequentially linked and
    // include the L1 head of each boot info.
    for header in headers.iter().rev() {
        assert_eq!(current_hash, header.hash_slow());

        // Mark the l1_head as found if it's in our map
        if let Some(found) = l1_heads_map.get_mut(&current_hash) {
            *found = true;
        }

        current_hash = header.parent_hash;
    }

    // Check if all l1_heads were found in the chain.
    for (l1_head, found) in l1_heads_map.iter() {
        assert!(
            *found,
            "L1 head {:?} not found in the provided header chain",
            l1_head
        );
    }
}

pub fn main() {
    // Read in the public values corresponding to each multi-block proof.
    let agg_inputs = sp1_zkvm::io::read::<AggregationInputs>();
    // Note: The headers are in order from start to end. We use serde_cbor as bincode serialization causes
    // issues with the zkVM.
    let headers_bytes = sp1_zkvm::io::read_vec();
    let headers: Vec<Header> = serde_cbor::from_slice(&headers_bytes).unwrap();
    assert!(!agg_inputs.boot_infos.is_empty());

    // Confirm that the boot infos are sequential.
    agg_inputs.boot_infos.windows(2).for_each(|pair| {
        let (prev_boot_info, boot_info) = (&pair[0], &pair[1]);

        // The claimed block of the previous boot info must be the L2 output root of the current boot.
        assert_eq!(prev_boot_info.l2PostRoot, boot_info.l2PreRoot);

        // The chain ID must be the same for all the boot infos, to ensure they're
        // from the same chain and span batch range.
        assert_eq!(prev_boot_info.chainId, boot_info.chainId);

        // The rollup config must be the same for all the boot infos, to ensure they're
        // from the same chain and span batch range.
        assert_eq!(prev_boot_info.rollupConfigHash, boot_info.rollupConfigHash);
    });

    // Verify each multi-block program proof.
    agg_inputs.boot_infos.iter().for_each(|boot_info| {
        // In the multi-block program, the public values digest is just the hash of the ABI encoded
        // boot info.
        let abi_encoded_boot_info = boot_info.abi_encode();
        let pv_digest = Sha256::digest(abi_encoded_boot_info);

        if cfg!(target_os = "zkvm") {
            sp1_lib::verify::verify_sp1_proof(&MULTI_BLOCK_PROGRAM_VKEY_DIGEST, &pv_digest.into());
        }
    });

    // Verify the L1 heads of each boot info are on the L1.
    verify_l1_heads(&agg_inputs, &headers);

    let first_boot_info = &agg_inputs.boot_infos[0];
    let last_boot_info = &agg_inputs.boot_infos[agg_inputs.boot_infos.len() - 1];
    // Consolidate the boot info into a single BootInfo struct that represents the range proven.
    let final_boot_info = BootInfoStruct {
        // The first boot info's L2 output root is the L2 output root of the range.
        l2PreRoot: first_boot_info.l2PreRoot,
        l2BlockNumber: last_boot_info.l2BlockNumber,
        l2PostRoot: last_boot_info.l2PostRoot,
        l1Head: agg_inputs.latest_l1_checkpoint_head,
        chainId: last_boot_info.chainId,
        rollupConfigHash: last_boot_info.rollupConfigHash,
    };

    // Commit to the aggregated boot info.
    sp1_zkvm::io::commit_slice(&final_boot_info.abi_encode());
}
