//! Validium Witness Generator
//!
//! Reuses the Ethereum DA WitnessGenerator to collect L1 preimages and blobs,
//! then packages the result into ValidiumWitnessData with additional off-chain batch data.

use anyhow::Result;
use async_trait::async_trait;
use kona_proof::l1::OracleBlobProvider;
use op_succinct_client_utils::witness::DefaultWitnessData;
use op_succinct_ethereum_client_utils::executor::ETHDAWitnessExecutor;
use op_succinct_host_utils::witness_generation::{
    online_blob_store::OnlineBlobStore, preimage_witness_collector::PreimageWitnessCollector,
    DefaultOracleBase, WitnessGenerator,
};
use op_succinct_validium_client_utils::{ValidiumBlobData, ValidiumWitnessData};
use rkyv::to_bytes;
use sp1_sdk::SP1Stdin;

/// The native-side executor type (same as Ethereum DA).
/// During witness generation, we run the pipeline natively with L1 data.
type NativeWitnessExecutor = ETHDAWitnessExecutor<
    PreimageWitnessCollector<DefaultOracleBase>,
    OnlineBlobStore<OracleBlobProvider<DefaultOracleBase>>,
>;

/// Validium witness generator.
///
/// Uses the standard Ethereum DA executor for native witness collection
/// (collecting L1 preimages and any L1 blobs). The off-chain validium batch data
/// is added separately by the host.
pub struct ValidiumWitnessGenerator {
    pub executor: NativeWitnessExecutor,
}

impl ValidiumWitnessGenerator {
    pub fn new() -> Self {
        Self { executor: ETHDAWitnessExecutor::new() }
    }
}

impl Default for ValidiumWitnessGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl WitnessGenerator for ValidiumWitnessGenerator {
    type WitnessData = DefaultWitnessData;
    type WitnessExecutor = NativeWitnessExecutor;

    fn get_executor(&self) -> &Self::WitnessExecutor {
        &self.executor
    }

    fn get_sp1_stdin(&self, _witness: Self::WitnessData) -> Result<SP1Stdin> {
        // This is not used directly. Instead, the host calls
        // `create_validium_stdin` which packages the DefaultWitnessData
        // together with the validium batch data.
        Err(anyhow::anyhow!(
            "Use create_validium_stdin() instead of get_sp1_stdin() for validium"
        ))
    }
}

/// Packages a DefaultWitnessData (from L1) + ValidiumBlobData (from off-chain)
/// into SP1Stdin for the validium zkVM program.
pub fn create_validium_stdin(
    l1_witness: DefaultWitnessData,
    validium_data: ValidiumBlobData,
) -> Result<SP1Stdin> {
    let validium_witness = ValidiumWitnessData::from_parts(
        l1_witness.preimage_store,
        l1_witness.blob_data,
        validium_data,
    );
    let mut stdin = SP1Stdin::new();
    let buffer = to_bytes::<rkyv::rancor::Error>(&validium_witness)?;
    stdin.write_slice(&buffer);
    Ok(stdin)
}
