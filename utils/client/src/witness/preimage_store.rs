use alloy_primitives::{keccak256, map::HashMap};
use async_trait::async_trait;
use kona_preimage::{
    errors::{PreimageOracleError, PreimageOracleResult},
    HintWriterClient, PreimageKey, PreimageKeyType, PreimageOracleClient,
};
use kona_proof::FlushableCache;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use rust_kzg_bn254_verifier::batch::verify_blob_kzg_proof_batch;
use rust_kzg_bn254_primitives::blob::Blob as EigenBlob;
use ark_bn254::{Fq, G1Affine};
use ark_ff::PrimeField;


#[derive(Default)]
struct EigenDaBlob {
    commitment: Vec<u8>,
    data: Vec<u8>,
    kzg_proof: Vec<u8>,
}

#[derive(
    Clone, Debug, Default, Serialize, Deserialize, rkyv::Serialize, rkyv::Archive, rkyv::Deserialize,
)]
pub struct PreimageStore {
    pub preimage_map: HashMap<PreimageKey, Vec<u8>>,
}

impl PreimageStore {
    pub fn check_preimages(&self) -> PreimageOracleResult<()> {
        for (key, value) in &self.preimage_map {
            check_preimage(key, value)?;
        }
        Ok(())
    }

    pub fn save_preimage(&mut self, key: PreimageKey, value: Vec<u8>) {
        check_preimage(&key, &value).expect("Invalid preimage");
        if let Some(old) = self.preimage_map.insert(key, value.clone()) {
            assert_eq!(old, value, "Cannot overwrite key");
        }
    }
    
    /// The current implementation is limited to EigenDa v1 and will be removed later, 
    /// with improvements made to Eigen blob verification.
    pub fn check_eigenvalues(&self) -> PreimageOracleResult<()> {
        let mut eigenda_blobs: HashMap<[u8; 64], EigenDaBlob> = HashMap::default();
        for (key, value) in &self.preimage_map {
            if key.key_type() == PreimageKeyType::GlobalGeneric {
                let key_value: [u8; 32] = key.key_value().to_be_bytes();
                let blob_key = PreimageKey::new(key_value, PreimageKeyType::Keccak256);
                if let Some(blob_key_data) = self.preimage_map.get(&blob_key) {
                    if blob_key_data.len() <  64 {
                        return Err(PreimageOracleError::Other("eigen da blob key data len is wrong".to_string()))
                    }
                    let commitment = blob_key_data[..64].try_into().unwrap();
                    if blob_key_data.len() == 64 {
                        eigenda_blobs
                            .entry(commitment)
                            .or_default()
                            .kzg_proof
                            .copy_from_slice(value);
                    } else if blob_key_data.len() == 65 {
                        eigenda_blobs
                            .entry(commitment)
                            .or_default()
                            .commitment
                            .copy_from_slice(value);
                    } else {
                        let element_idx_bytes: [u8; 8] = blob_key_data[64..].try_into().unwrap();
                        let element_idx: u64 = u64::from_be_bytes(element_idx_bytes);
                        // Add the 32 bytes of blob data into the correct spot in the blob.
                        eigenda_blobs
                            .entry(commitment)
                            .or_default()
                            .data
                            .get_mut((element_idx as usize) << 5..(element_idx as usize + 1) << 5)
                            .map(|slice| {
                                if slice.iter().all(|&byte| byte == 0) {
                                    slice.copy_from_slice(value);
                                    Ok(())
                                } else {
                                    Err(PreimageOracleError::InvalidPreimageKey)
                                }
                            });
                    }
                }
            }
        }
        println!("Verifying {} eigen da blobs", eigenda_blobs.len());

        if !eigenda_blobs.is_empty() {
            let mut eigen_blobs: Vec<EigenBlob> = Vec::new();
            let mut eigen_commitments: Vec<G1Affine> = Vec::new();
            let mut eigen_proofs: Vec<G1Affine> = Vec::new();
            for (_, value) in eigenda_blobs {
                eigen_blobs.push(EigenBlob::from(value.data));
                let x = Fq::from_be_bytes_mod_order(&value.commitment[..32]);
                let y = Fq::from_be_bytes_mod_order(&value.commitment[32..64]);
                eigen_commitments.push(G1Affine::new(x, y));
                let p_x = Fq::from_be_bytes_mod_order(&value.kzg_proof[..32]);
                let p_y = Fq::from_be_bytes_mod_order(&value.kzg_proof[32..64]);
                eigen_proofs.push(G1Affine::new(p_x, p_y));
            }
            //Verify EigenDa blob
            let e_r = verify_blob_kzg_proof_batch(&eigen_blobs, &eigen_commitments, &eigen_proofs)
                .map_err(|_| PreimageOracleError::Other("blob verification failed for batch".to_string()))?;
            
            if !e_r {
                return Err(PreimageOracleError::Other("EigenDa blob verification failed".to_string()));
            }
        }
        Ok(())
    }
}

/// Check that the preimage matches the expected hash.
pub fn check_preimage(key: &PreimageKey, value: &[u8]) -> PreimageOracleResult<()> {
    if let Some(expected_hash) = match key.key_type() {
        PreimageKeyType::Keccak256 => Some(keccak256(value).0),
        PreimageKeyType::Sha256 => Some(sha2::Sha256::digest(value).into()),
        PreimageKeyType::Local | PreimageKeyType::GlobalGeneric => None,
        PreimageKeyType::Precompile => unimplemented!("Precompile not supported in zkVM"),
        PreimageKeyType::Blob => unreachable!("Blob keys validated in blob witness"),
    } {
        if key != &PreimageKey::new(expected_hash, key.key_type()) {
            return Err(PreimageOracleError::InvalidPreimageKey);
        }
    }
    Ok(())
}

#[async_trait]
impl HintWriterClient for PreimageStore {
    async fn write(&self, _hint: &str) -> PreimageOracleResult<()> {
        Ok(())
    }
}

#[async_trait]
impl PreimageOracleClient for PreimageStore {
    async fn get(&self, key: PreimageKey) -> PreimageOracleResult<Vec<u8>> {
        let Some(value) = self.preimage_map.get(&key) else {
            return Err(PreimageOracleError::InvalidPreimageKey);
        };
        Ok(value.clone())
    }

    async fn get_exact(&self, key: PreimageKey, buf: &mut [u8]) -> PreimageOracleResult<()> {
        buf.copy_from_slice(&self.get(key).await?);
        Ok(())
    }
}

impl FlushableCache for PreimageStore {
    fn flush(&self) {}
}
