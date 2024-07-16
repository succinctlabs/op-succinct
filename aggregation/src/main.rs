//! A simple program that aggregates the proofs of multiple programs proven with the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use sha2::{Digest, Sha256};
use zkvm_common::BootInfoWithoutRollupConfig;

pub fn main() {
    // Read the verification keys.
    let client_vkey = sp1_zkvm::io::read::<[u32; 8]>();

    // Read the public values.
    let boot_infos = sp1_zkvm::io::read::<Vec<BootInfoWithoutRollupConfig>>();

    // Verify the proofs.
    let mut last_boot_info: Option<&BootInfoWithoutRollupConfig> = None;
    for i in 0..boot_infos.len() {
        let boot_info = &boot_infos[i];

        if let Some(last_boot_info_unwrapped) = last_boot_info {
            assert_eq!(last_boot_info_unwrapped.l2_claim_block + 1, boot_info.l2_claim_block);
            assert_eq!(last_boot_info_unwrapped.l2_claim, boot_info.l2_output_root);

            // TODO: Instead of verifying, we can just use different public values struct and pass these once.
            assert_eq!(last_boot_info_unwrapped.chain_id, boot_info.chain_id);
            assert_eq!(last_boot_info_unwrapped.l1_head, boot_info.l1_head);
        }
        last_boot_info = Some(boot_info);

        let serialized_boot_info = bincode::serialize(&boot_info).unwrap();
        let public_values_digest = Sha256::digest(serialized_boot_info);

        sp1_zkvm::precompiles::verify::verify_sp1_proof(&client_vkey, &public_values_digest.into());
    }

    // Commit to the inputs.
    // TODO: Turn this into merkle tree if it's less expensive.
    sp1_zkvm::io::commit::<[u32; 8]>(&client_vkey);
    sp1_zkvm::io::commit::<Vec<BootInfoWithoutRollupConfig>>(&boot_infos);
}
