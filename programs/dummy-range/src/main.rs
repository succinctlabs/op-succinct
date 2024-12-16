//! A program to verify a Optimism L2 block STF in the zkVM.
//!
//! This binary contains the client program for executing the Optimism rollup state transition
//! across a range of blocks, which can be used to generate an on chain validity proof. Depending on
//! the compilation pipeline, it will compile to be run either in native mode or in zkVM mode. In
//! native mode, the data for verifying the batch validity is fetched from RPC, while in zkVM mode,
//! the data is supplied by the host binary to the verifiable program.

#![no_main]
sp1_zkvm::entrypoint!(main);

use op_succinct_client_utils::boot::BootInfoStruct;
use op_succinct_client_utils::BootInfoWithBytesConfig;

pub fn main() {
    let boot_info_with_bytes_config = sp1_zkvm::io::read::<BootInfoWithBytesConfig>();

    // BootInfoStruct is identical to BootInfoWithBytesConfig, except it replaces
    // the rollup_config_bytes with a hash of those bytes (rollupConfigHash). Securely
    // hashes the rollup config bytes.
    let boot_info_struct = BootInfoStruct::from(boot_info_with_bytes_config.clone());
    sp1_zkvm::io::commit::<BootInfoStruct>(&boot_info_struct);
}
