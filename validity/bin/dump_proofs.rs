//! Dump range proof BLOBs from the proposer DB for local aggregation proof reproduction.
//!
//! Usage:
//!   cargo run --bin dump_proofs --release -- \
//!     --database-url $PROPOSER_DB_URL \
//!     --start-block 1000 \
//!     --end-block 2000

use anyhow::Result;
use clap::Parser;
use sqlx::PgPool;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about = "Dump range proof BLOBs from the proposer DB")]
struct Args {
    /// Postgres connection string for the proposer database.
    #[arg(long)]
    database_url: String,

    /// Aggregation request start block.
    #[arg(long)]
    start_block: i64,

    /// Aggregation request end block.
    #[arg(long)]
    end_block: i64,

    /// Output directory for proof files.
    #[arg(long, default_value = "data/fetched_proofs")]
    output_dir: String,
}

#[derive(sqlx::FromRow)]
struct RangeProofRow {
    id: i64,
    start_block: i64,
    end_block: i64,
    proof: Option<Vec<u8>>,
}

#[derive(sqlx::FromRow)]
#[allow(dead_code)]
struct AggRequestInfo {
    start_block: i64,
    end_block: i64,
    checkpointed_l1_block_hash: Option<Vec<u8>>,
    prover_address: Option<Vec<u8>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let pool = PgPool::connect(&args.database_url).await?;
    println!(
        "Connected to database. Querying range proofs for blocks {}..{}",
        args.start_block, args.end_block
    );

    // Query completed range proofs in the block range.
    let range_proofs = sqlx::query_as::<_, RangeProofRow>(
        "SELECT id, start_block, end_block, proof \
         FROM requests \
         WHERE req_type = 0 AND status = 4 \
           AND start_block >= $1 AND end_block <= $2 \
         ORDER BY start_block ASC",
    )
    .bind(args.start_block)
    .bind(args.end_block)
    .fetch_all(&pool)
    .await?;

    if range_proofs.is_empty() {
        println!("No completed range proofs found in block range {}..{}", args.start_block, args.end_block);
        return Ok(());
    }

    fs::create_dir_all(&args.output_dir)?;

    let mut proof_names = Vec::new();
    for row in &range_proofs {
        let proof_bytes = match &row.proof {
            Some(bytes) => bytes,
            None => {
                println!(
                    "  SKIP id={} blocks {}-{}: proof column is NULL",
                    row.id, row.start_block, row.end_block
                );
                continue;
            }
        };

        let filename = format!("{}-{}.bin", row.start_block, row.end_block);
        let path = format!("{}/{}", args.output_dir, filename);
        fs::write(&path, proof_bytes)?;
        println!(
            "  Saved id={} blocks {}-{} ({} bytes) -> {}",
            row.id,
            row.start_block,
            row.end_block,
            proof_bytes.len(),
            path
        );
        proof_names.push(format!("{}-{}", row.start_block, row.end_block));
    }

    println!("\nDumped {} range proofs to {}/", proof_names.len(), args.output_dir);

    // Query the most recent failed aggregation request for this range to get metadata.
    let agg_info = sqlx::query_as::<_, AggRequestInfo>(
        "SELECT start_block, end_block, checkpointed_l1_block_hash, prover_address \
         FROM requests \
         WHERE req_type = 1 AND status = 6 \
           AND start_block = $1 AND end_block = $2 \
         ORDER BY created_at DESC LIMIT 1",
    )
    .bind(args.start_block)
    .bind(args.end_block)
    .fetch_optional(&pool)
    .await?;

    if let Some(info) = agg_info {
        println!("\n=== Failed aggregation request metadata ===");
        if let Some(hash) = &info.checkpointed_l1_block_hash {
            println!("  checkpointed_l1_block_hash: 0x{}", hex::encode(hash));
        }
        if let Some(addr) = &info.prover_address {
            println!("  prover_address: 0x{}", hex::encode(addr));
        }
    } else {
        println!("\nNo failed aggregation request found for blocks {}..{}", args.start_block, args.end_block);
    }

    // Print the agg command for convenience.
    println!("\n=== Next step: run agg with --save-artifacts ===");
    println!(
        "cargo run --bin agg --release -- \\\n  --proofs {} \\\n  --prover 0x<PROVER_ADDRESS> \\\n  --env-file .env.sepolia \\\n  --save-artifacts",
        proof_names.join(",")
    );

    Ok(())
}
