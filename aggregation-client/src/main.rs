//! A simple program that aggregates the proofs of multiple programs proven with the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use client_utils::RawBootInfo;
// use kona_client::{
//     l1::{OracleBlobProvider, OracleL1ChainProvider},
//     BootInfo,
// };
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

    // Confirm that the boot infos are sequential and match correctly.
    boot_infos.windows(2).for_each(|pair| {
        let (last_boot_info, boot_info) = (&pair[0], &pair[1]);

        // The L2 claim block and output root must be sequential.
        assert_eq!(last_boot_info.l2_claim, boot_info.l2_output_root);

        // The chain ID and L1 head must be the same for all the boot infos, to ensure they're
        // from the same chain and span batch range.
        assert_eq!(last_boot_info.chain_id, boot_info.chain_id);
    });

    // TODO: Show that the L1 heads for each boot info can be proven from the L1 head of the
    // last boot info. This ensures that we only need to checkpoint the L1 head of the last boot info.

    // Verify each multi-block program proof.
    boot_infos.iter().for_each(|boot_info| {
        // In the multi-block program, the public values digest is just the hash of the ABI encoded
        // boot info.
        let abi_encoded_boot_info = boot_info.abi_encode();
        let pv_digest = Sha256::digest(abi_encoded_boot_info);

        sp1_lib::verify::verify_sp1_proof(&MULTI_BLOCK_PROGRAM_VKEY_DIGEST, &pv_digest.into());
    });

    // Consolidate the boot info into a single BootInfo struct that represents the range proven.
    let first_boot_info = &boot_infos[0];
    let last_boot_info = &boot_infos[boot_infos.len() - 1];
    let final_boot_info = RawBootInfo {
        // The first boot info's L2 output root is the L2 output root of the range.
        l2_output_root: first_boot_info.l2_output_root,
        l2_claim_block: last_boot_info.l2_claim_block,
        l2_claim: last_boot_info.l2_claim,
        l1_head: last_boot_info.l1_head,
        chain_id: last_boot_info.chain_id,
    };

    // Commit to the aggregated boot info.
    sp1_zkvm::io::commit_slice(&final_boot_info.abi_encode());
}
