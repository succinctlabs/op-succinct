//! Celo EigenDA registry for verifier address fetching.
//!
//! This crate provides Celo-specific implementations for EigenDA cert verification,
//! returning Celo-deployed router addresses instead of the default EigenLabs-deployed ones.

use alloy_primitives::{address, Address};
use canoe_verifier_address_fetcher::{
    CanoeVerifierAddressFetcher, CanoeVerifierAddressFetcherError,
};
use eigenda_cert::EigenDAVersionedCert;

/// Celo-specific EigenDA cert verifier address fetcher.
///
/// Returns Celo-deployed router addresses for EigenDA verification instead of
/// the default EigenLabs-deployed addresses.
#[derive(Clone, Debug, Default)]
pub struct CeloCanoeVerifierAddressFetcher;

impl CanoeVerifierAddressFetcher for CeloCanoeVerifierAddressFetcher {
    // TODO: There is also the L2SpecificCanoeVerifierAddressFetcher trait extension,
    // which would allow us even more control over the routers (e.g. for Chaos) per L2.
    // Use this once the `hokulea_zkvm_verification::eigenda_witness_to_preloaded_provider`
    // used in op-succinct allows that as an argument.
    fn fetch_address(
        &self,
        l1_chain_id: u64,
        _versioned_cert: &EigenDAVersionedCert,
    ) -> Result<Address, CanoeVerifierAddressFetcherError> {
        match l1_chain_id {
            // Sepolia: Celo-deployed router
            11155111 => Ok(address!("f4f934A0b5c09d302d9C6f60040754fEebdd6073")),
            // Mainnet: official EigenDA CertVerifier@v3 (update when Celo mainnet router available)
            1 => Ok(address!("61692e93b6B045c444e942A91EcD1527F23A3FB7")),
            chain_id => Err(CanoeVerifierAddressFetcherError::UnknownL1ChainId(chain_id)),
        }
    }
}
