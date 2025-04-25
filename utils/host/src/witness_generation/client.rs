use anyhow::Result;
use async_trait::async_trait;
use kona_preimage::NativeChannel;
use op_succinct_client_utils::witness::WitnessData;

#[async_trait]
pub trait WitnessGenClient {
    async fn run(
        &self,
        preimage_chan: NativeChannel,
        hint_chan: NativeChannel,
    ) -> Result<WitnessData>;
}
