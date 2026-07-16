use async_trait::async_trait;
use kona_preimage::{
    errors::{PreimageOracleError, PreimageOracleResult},
    HintWriterClient, PreimageKey, PreimageKeyType, PreimageOracleClient,
};
use kona_proof::FlushableCache;
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::collections::{hash_map::Entry, HashMap};

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

    pub fn save_preimage(&mut self, key: PreimageKey, value: Vec<u8>) -> PreimageOracleResult<()> {
        check_preimage(&key, &value)?;

        match self.preimage_map.entry(key) {
            Entry::Vacant(e) => {
                e.insert(value);
            }
            Entry::Occupied(e) => {
                if e.get() != &value {
                    return Err(PreimageOracleError::Other("cannot overwrite key".to_string()))
                }
            }
        };

        Ok(())
    }
}

/// Check that the preimage matches the expected hash.
pub fn check_preimage(key: &PreimageKey, value: &[u8]) -> PreimageOracleResult<()> {
    if let Some(expected_hash) = match key.key_type() {
        PreimageKeyType::Keccak256 => Some(keccak256(value)),
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

#[cfg(not(target_os = "zkvm"))]
#[inline]
fn keccak256(value: &[u8]) -> [u8; 32] {
    alloy_primitives::keccak256(value).0
}

#[cfg(target_os = "zkvm")]
#[inline]
fn keccak256(value: &[u8]) -> [u8; 32] {
    const RATE: usize = 136;
    const PAD: u8 = 0x01;

    let mut state = [0u64; 25];
    let mut blocks = value.chunks_exact(RATE);
    for block in &mut blocks {
        xor_block(&mut state, block);
        keccak_permute(&mut state);
    }

    let rem = blocks.remainder();
    let mut block = [0u8; RATE];
    block[..rem.len()].copy_from_slice(rem);
    block[rem.len()] = PAD;
    block[RATE - 1] |= 0x80;

    xor_block(&mut state, &block);
    keccak_permute(&mut state);

    let mut out = [0u8; 32];
    for (o, s) in out.chunks_mut(8).zip(state.iter()) {
        o.copy_from_slice(&s.to_le_bytes()[..o.len()]);
    }
    out
}

#[cfg(target_os = "zkvm")]
#[inline(always)]
fn xor_block(state: &mut [u64; 25], block: &[u8]) {
    let mut chunks = block.chunks_exact(8);
    for (s, chunk) in state.iter_mut().zip(&mut chunks) {
        *s ^= u64::from_le_bytes(chunk.try_into().unwrap());
    }

    let rem = chunks.remainder();
    if !rem.is_empty() {
        let mut buf = [0u8; 8];
        buf[..rem.len()].copy_from_slice(rem);
        state[block.len() / 8] ^= u64::from_le_bytes(buf);
    }
}

#[cfg(target_os = "zkvm")]
extern "C" {
    fn syscall_keccak_permute(state: *mut [u64; 25]);
}

#[cfg(target_os = "zkvm")]
#[inline(always)]
fn keccak_permute(state: &mut [u64; 25]) {
    unsafe {
        syscall_keccak_permute(state);
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    const KECCAK256_EMPTY: [u8; 32] = [
        0xc5, 0xd2, 0x46, 0x01, 0x86, 0xf7, 0x23, 0x3c, 0x92, 0x7e, 0x7d, 0xb2, 0xdc, 0xc7, 0x03,
        0xc0, 0xe5, 0x00, 0xb6, 0x53, 0xca, 0x82, 0x27, 0x3b, 0x7b, 0xfa, 0xd8, 0x04, 0x5d, 0x85,
        0xa4, 0x70,
    ];

    #[test]
    fn check_preimage_accepts_keccak_vector() {
        let key = PreimageKey::new(KECCAK256_EMPTY, PreimageKeyType::Keccak256);

        check_preimage(&key, b"").unwrap();
    }

    #[test]
    fn check_preimage_rejects_invalid_keccak_preimage() {
        let key = PreimageKey::new(KECCAK256_EMPTY, PreimageKeyType::Keccak256);

        assert!(matches!(
            check_preimage(&key, b"not the empty preimage"),
            Err(PreimageOracleError::InvalidPreimageKey)
        ));
    }
}
