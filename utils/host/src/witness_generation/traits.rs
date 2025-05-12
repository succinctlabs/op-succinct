use anyhow::Result;
use async_trait::async_trait;
use kona_preimage::NativeChannel;
use op_succinct_client_utils::witness::WitnessData;
use sp1_sdk::SP1Stdin;

#[async_trait]
pub trait WitnessGenerator {
    type WitnessData: WitnessData;

    async fn run(
        &self,
        preimage_chan: NativeChannel,
        hint_chan: NativeChannel,
    ) -> Result<Self::WitnessData>;

    fn get_sp1_stdin(&self, witness: Self::WitnessData) -> Result<SP1Stdin>;
}
