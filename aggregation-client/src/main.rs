//! A simple program that aggregates the proofs of multiple programs proven with the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use client_utils::RawBootInfo;
use sha2::{Digest, Sha256};

/// Note: This is the hardcoded program vkey for the multi-block program. Whenever the multi-block
/// program changes, update this.
const MULTI_BLOCK_PROGRAM_VKEY_DIGEST: [u32; 8] = [
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
];

pub fn main() {
    // Read in the public values corresponding to each multi-block proof.
    let boot_infos = sp1_zkvm::io::read::<Vec<RawBootInfo>>();
    assert!(!boot_infos.is_empty());

    // Verify the proofs.
    let mut last_boot_info_opt: Option<&RawBootInfo> = None;
    for boot_info in boot_infos.iter() {
        if let Some(last_boot_info) = last_boot_info_opt {
            // The L2 claim block and output root must be sequential.
            assert_eq!(last_boot_info.l2_claim_block + 1, boot_info.l2_claim_block);
            assert_eq!(last_boot_info.l2_claim, boot_info.l2_output_root);

            // The chain ID and L1 head must be the same for all the boot infos, to ensure they're
            // from the same chain and span batch range.
            assert_eq!(last_boot_info.chain_id, boot_info.chain_id);
            assert_eq!(last_boot_info.l1_head, boot_info.l1_head);
        }
        last_boot_info_opt = Some(boot_info);

        // In the multi-block program, the public values digest is just the hash of the ABI encoded
        // boot info.
        let abi_encoded_boot_info = boot_info.abi_encode();
        let pv_digest = Sha256::digest(abi_encoded_boot_info);

        sp1_lib::verify::verify_sp1_proof(&MULTI_BLOCK_PROGRAM_VKEY_DIGEST, &pv_digest.into());
    }

    // Consolidate the boot info into a single BootInfo struct that represents the range proven.
    let mut final_boot_info = boot_infos[0].clone();
    final_boot_info.l2_claim_block = boot_infos[boot_infos.len() - 1].l2_claim_block;
    final_boot_info.l2_claim = boot_infos[boot_infos.len() - 1].l2_claim;

    // Commit to the aggregated boot info.
    sp1_zkvm::io::commit_slice(&final_boot_info.abi_encode());
}
