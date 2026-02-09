//! Validium Range Program
//!
//! Verifies Optimism L2 block state transitions with Validium DA.
//!
//! Uses the AltDA protocol:
//! - Batcher posts keccak256(batch_data) to L1 as calldata (34 bytes)
//! - Actual batch data is in the witness (from off-chain storage)
//! - zkVM verifies: keccak256(data) == on-chain commitment

#![no_main]
sp1_zkvm::entrypoint!(main);

use op_succinct_range_utils::run_range_program_with_blob_provider;
use op_succinct_validium_client_utils::{ValidiumWitnessData, ValidiumWitnessExecutor};
use rkyv::rancor::Error;

#[cfg(feature = "tracing-subscriber")]
use op_succinct_range_utils::setup_tracing;

fn main() {
    #[cfg(feature = "tracing-subscriber")]
    setup_tracing();

    kona_proof::block_on(async move {
        // Read validium witness.
        let witness_rkyv_bytes: Vec<u8> = sp1_zkvm::io::read_vec();
        let witness_data = rkyv::from_bytes::<ValidiumWitnessData, Error>(&witness_rkyv_bytes)
            .expect("Failed to deserialize validium witness data.");

        // Get providers: oracle (L1 preimages), beacon (L1 blobs), validium store (off-chain data).
        let (oracle, beacon, validium_store) = witness_data
            .get_providers()
            .await
            .expect("Failed to load providers");

        // Run derivation pipeline with validium executor.
        // ValidiumDADataSource intercepts AltDA commitments from L1 calldata
        // and replaces them with actual data from the validium store,
        // verifying keccak256(data) == commitment.
        run_range_program_with_blob_provider(
            ValidiumWitnessExecutor::new(validium_store),
            oracle,
            beacon,
        )
        .await;
    });
}
