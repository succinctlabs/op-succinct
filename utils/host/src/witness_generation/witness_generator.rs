use std::sync::Arc;

use anyhow::Result;
use hokulea_proof::eigenda_provider::OracleEigenDAProvider;
use kona_preimage::{HintWriter, NativeChannel, OracleReader};
use kona_proof::{l1::OracleBlobProvider, CachingOracle};

use crate::witness_generation::{generate_opsuccinct_eigenda_witness, generate_opsuccinct_witness};
use op_succinct_client_utils::witness::WitnessData;

pub trait WitnessGenerator {
    async fn run_witnessgen_client(
        &self,
        preimage_chan: NativeChannel,
        hint_chan: NativeChannel,
    ) -> Result<WitnessData>;
}

pub struct DefaultWitnessGenerator;

impl WitnessGenerator for DefaultWitnessGenerator {
    async fn run_witnessgen_client(
        &self,
        preimage_chan: NativeChannel,
        hint_chan: NativeChannel,
    ) -> Result<WitnessData> {
        // Instantiate oracles
        let preimage_oracle = Arc::new(CachingOracle::new(
            2048,
            OracleReader::new(preimage_chan),
            HintWriter::new(hint_chan),
        ));
        let blob_provider = OracleBlobProvider::new(preimage_oracle.clone());

        let (_, witness) =
            generate_opsuccinct_witness(preimage_oracle.clone(), blob_provider).await?;

        Ok(witness)
    }
}

pub struct EigenDAWitnessGenerator;

impl WitnessGenerator for EigenDAWitnessGenerator {
    async fn run_witnessgen_client(
        &self,
        preimage_chan: NativeChannel,
        hint_chan: NativeChannel,
    ) -> Result<WitnessData> {
        // Instantiate oracles
        let preimage_oracle = Arc::new(CachingOracle::new(
            2048,
            OracleReader::new(preimage_chan),
            HintWriter::new(hint_chan),
        ));
        let blob_provider = OracleBlobProvider::new(preimage_oracle.clone());

        let eigenda_blob_provider = OracleEigenDAProvider::new(preimage_oracle.clone());

        let (_, witness) = generate_opsuccinct_eigenda_witness(
            preimage_oracle.clone(),
            blob_provider,
            eigenda_blob_provider,
        )
        .await?;

        Ok(witness)
    }
}
