use anyhow::Result;
use sp1_sdk::{HashableKey, ProverClient};

pub const AGG_ELF: &[u8] = include_bytes!("../../elf/aggregation-client-elf");
pub const MULTI_BLOCK_ELF: &[u8] = include_bytes!("../../elf/validity-client-elf");

// Get the verification keys for the ELFs.
#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let prover = ProverClient::new();

    let (_, vkey) = prover.setup(MULTI_BLOCK_ELF);

    println!(
        "Multi-block ELF Verification Key U32 Hash: {:?}",
        vkey.vk.hash_u32()
    );

    let (_, agg_vk) = prover.setup(AGG_ELF);
    println!("Aggregate ELF Verification Key: {:?}", agg_vk.vk.bytes32());

    Ok(())
}
