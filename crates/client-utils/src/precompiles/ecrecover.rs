//! Contains the accelerated version of the `ecrecover` precompile.

use alloy_primitives::{Address, Bytes};
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

    // TODO: figure out whether the sp1-lib signature for `ecrecover` should match this logic more closely.
    // let msg = <&B256>::try_from(&input[0..32]).unwrap();
    // let recid = input[63] - 27;
    // let sig = <&B512>::try_from(&input[64..128]).unwrap();

    let msg: [u8; 32] = input[0..32].try_into().unwrap();
    let sig: [u8; 65] = input[63..128].try_into().unwrap();

    let out = ecrecover(&sig, &msg)
        .map(|o| o.to_vec().into())
        .unwrap_or_default();
    println!("cycle-tracker-end: precompile-ecrecover");
    Ok(PrecompileOutput::new(ECRECOVER_BASE, out))
}
