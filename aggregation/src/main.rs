//! A simple program that aggregates the proofs of multiple programs proven with the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use sha2::{Digest, Sha256};

pub fn main() {
    // Read the verification keys.
    let client_vkey = sp1_zkvm::io::read::<[u32; 8]>();

    // Read the public values.
    let public_values = sp1_zkvm::io::read::<Vec<Vec<u8>>>();

    // Verify the proofs.
    let num_blocks = public_values.len();
    for i in 0..num_blocks {
        let public_values = &public_values[i];
        let public_values_digest = Sha256::digest(public_values);
        sp1_zkvm::precompiles::verify::verify_sp1_proof(&client_vkey, &public_values_digest.into());
    }

    // Commit to the inputs.
    // TODO: Turn this into merkle tree if it's less expensive.
    sp1_zkvm::io::commit::<[u32; 8]>(&client_vkey);
    sp1_zkvm::io::commit::<Vec<Vec<u8>>>(&public_values);
}
