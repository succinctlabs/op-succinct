//! Contains the accelerated version of the `ecrecover` precompile.

use alloy_primitives::{keccak256, Address, Bytes};
use revm::{
    precompile::utilities::right_pad,
    precompile::{u64_to_address, Error as PrecompileError, PrecompileWithAddress},
    primitives::{Precompile, PrecompileOutput, PrecompileResult},
};
use sp1_lib::secp256k1::ecrecover;

const ECRECOVER_ADDRESS: Address = u64_to_address(1);

pub(crate) const ZKVM_ECRECOVER: PrecompileWithAddress =
    PrecompileWithAddress(ECRECOVER_ADDRESS, Precompile::Standard(zkvm_ecrecover));

/// Performs an ZKVM-accelerated `ecrecover` precompile call.
/// This should match the functionality of [`revm::precompile::secp256k1::ec_recover_run`].
fn zkvm_ecrecover(input: &Bytes, gas_limit: u64) -> PrecompileResult {
    println!("cycle-tracker-start: precompile-ecrecover");
    const ECRECOVER_BASE: u64 = 3_000;

    if ECRECOVER_BASE > gas_limit {
        return Err(PrecompileError::OutOfGas.into());
    }

    let input = right_pad::<128>(input);

    // `v` must be a 32-byte big-endian integer equal to 27 or 28.
    if !(input[32..63].iter().all(|&b| b == 0) && matches!(input[63], 27 | 28)) {
        return Ok(PrecompileOutput::new(ECRECOVER_BASE, Bytes::new()));
    }

    let msg: [u8; 32] = input[0..32].try_into().unwrap();

    // SP1's ecrecover expects the sig: [u8; 65] = [/* 64 bytes of signature */, /* 1 byte recovery ID */].
    // After SP1's ecrecover, convert the recovered public key to an address.
    let mut sig: [u8; 65] = [0; 65];
    sig[..64].copy_from_slice(&input[64..128]);
    sig[64] = input[63] - 27;
    let out = ecrecover(&sig, &msg).expect("ecrecover failed");
    let mut hash = keccak256(&out[1..]);
    hash[..12].fill(0);

    println!("cycle-tracker-end: precompile-ecrecover");
    Ok(PrecompileOutput::new(ECRECOVER_BASE, hash.into()))
}
