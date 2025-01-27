use alloy::hex;
use alloy_primitives::B256;
use anyhow::Result;
use op_succinct_client_utils::boot::BootInfoStruct;
use rusqlite::Connection;
use sp1_sdk::{SP1Proof, SP1ProofWithPublicValues};

const TARGET_L1_HEAD: &str = "0x728d0041623da3bd1deedf41a94eb5bc2ad20ff15158fe9e0093a07511e8bc5a";

fn main() -> Result<()> {
    let conn = Connection::open_with_flags(
        "/home/ubuntu/op-succinct-deployments/db/480/proofs.db",
        rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY,
    )?;

    let mut stmt = conn.prepare(
        "SELECT id, start_block, end_block, proof FROM proof_requests 
         WHERE proof IS NOT NULL 
         AND status = 'COMPLETE'
         AND last_updated_time >= strftime('%s', 'now') - 43200;",
    )?;
    // let mut stmt = conn.prepare(
    //     "SELECT id, start_block, end_block, proof FROM proof_requests;"
    // )?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, i64>(1)?,
            row.get::<_, i64>(2)?,
            row.get::<_, Option<Vec<u8>>>(3)?,
        ))
    })?;
    println!("Query complete.");

    let mut count = 0;
    for row in rows {
        count += 1;
        println!("Processed {} rows", count);
        println!("Processing row...");
        let (id, start_block, end_block, proof_bytes) = row?;

        // Skip if proof is null
        let Some(proof_bytes) = proof_bytes else {
            continue;
        };
        // Deserialize the proof
        if let Ok(mut proof_with_pv) =
            bincode::deserialize::<SP1ProofWithPublicValues>(&proof_bytes)
        {
            // Read the boot info
            let boot_info: BootInfoStruct = proof_with_pv.public_values.read();

            // Convert boot_info.l1_head to hex string for comparison
            let l1_head = format!("0x{}", hex::encode(boot_info.l1Head));

            if l1_head.to_lowercase() == TARGET_L1_HEAD.to_lowercase() {
                println!("Found matching proof!");
                println!("ID: {}", id);
                println!("Start Block: {}", start_block);
                println!("End Block: {}", end_block);
                println!("L1 Head: {}", l1_head);
                return Ok(())
            }
        }
    }

    println!("No matching proof found");
    Ok(())
}
