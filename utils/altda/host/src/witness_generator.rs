//! [`WitnessGenerator`] implementation for AltDA-backed OP Stack chains.
//!
//! Follows the same pattern as [`ETHDAWitnessGenerator`] since AltDA uses [`DefaultWitnessData`]
//! (no extra DA-specific witness data needed — resolved batch data flows through the standard
//! preimage collection).

use anyhow::Result;
use async_trait::async_trait;
use kona_proof::l1::OracleBlobProvider;
use op_succinct_altda_client_utils::executor::AltDAWitnessExecutor;
use op_succinct_client_utils::witness::DefaultWitnessData;
use op_succinct_host_utils::witness_generation::{
    online_blob_store::OnlineBlobStore, preimage_witness_collector::PreimageWitnessCollector,
    DefaultOracleBase, WitnessGenerator,
};
use rkyv::to_bytes;
use sp1_sdk::SP1Stdin;

type AltDAWitnessExec = AltDAWitnessExecutor<
    PreimageWitnessCollector<DefaultOracleBase>,
    OnlineBlobStore<OracleBlobProvider<DefaultOracleBase>>,
>;

pub struct AltDAWitnessGenerator {
    pub executor: AltDAWitnessExec,
}

#[async_trait]
impl WitnessGenerator for AltDAWitnessGenerator {
    type WitnessData = DefaultWitnessData;
    type WitnessExecutor = AltDAWitnessExec;

    fn get_executor(&self) -> &Self::WitnessExecutor {
        &self.executor
    }

    fn get_sp1_stdin(&self, witness: Self::WitnessData) -> Result<SP1Stdin> {
        let mut stdin = SP1Stdin::default();
        let buffer = to_bytes::<rkyv::rancor::Error>(&witness)?;
        stdin.write_slice(&buffer);
        Ok(stdin)
    }
}
