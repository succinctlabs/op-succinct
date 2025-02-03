use std::{env, fs, str::FromStr, sync::Arc};

use alloy_primitives::hex;
use alloy_primitives::{Address, B256};
use anyhow::Result;
use kona_proof::BootInfo;
use op_succinct_client_utils::InMemoryOracle;
use op_succinct_host_utils::fetcher::{OPSuccinctDataFetcher, RPCMode};
use serde_json::Value;
use sp1_sdk::{
    network::{proto::network::ProofMode, NetworkClient},
    SP1Stdin,
};

use rusqlite::{params, Connection};

async fn get_agreed_l2_block_number(
    claimed_l2_block_number: u64,
    agreed_l2_output_root: B256,
) -> u64 {
    let mut distance = 60;
    let fetcher = OPSuccinctDataFetcher::default();

    while distance > 0 {
        let val = claimed_l2_block_number - distance;
        let hex = format!("0x{:x}", val);
        let optimism_output_data: Value = fetcher
            .fetch_rpc_data_with_mode(RPCMode::L2Node, "optimism_outputAtBlock", vec![hex.into()])
            .await
            .unwrap();

        let output_root =
            B256::from_str(optimism_output_data["outputRoot"].as_str().unwrap()).unwrap();

        if output_root == agreed_l2_output_root {
            return claimed_l2_block_number - distance;
        }

        distance /= 2;
    }

    claimed_l2_block_number
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let private_key = env::var("NETWORK_PRIVATE_KEY").unwrap();
    let rpc_url = env::var("NETWORK_RPC_URL").unwrap();
    let client = NetworkClient::new(private_key, rpc_url);

    // Get all filtered proof requests from the two accounts that have a created_at in the last 12 hours.

    let account_1 = "0x23185d293a0831f1e018fc41384164b0ed0b7f6a"
        .parse::<Address>()
        .unwrap();
    let account_2 = "0x5a12489380332968e8e76fab2b6c764d15156eec"
        .parse::<Address>()
        .unwrap();

    // Query proof requests with a minimum_deadline in the last 12 hours.
    // Do one query where account_1 is the requester and account_2 is the counterparty...
    let min_deadline = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .saturating_sub(24 * 60 * 60); // current timestamp minus 12h

    let mut proofs_account_1 = vec![];
    let mut page = 1;
    loop {
        let response = client
            .get_filtered_proof_requests(
                None,
                None,
                None,
                Some(min_deadline),
                None,
                Some(account_1.to_vec()),
                None,
                None,
                None,
                Some(100),
                Some(page),
                None,
            )
            .await
            .unwrap();

        if response.requests.is_empty() {
            break;
        }
        proofs_account_1.extend(response.requests);
        page += 1;
    }
    println!("Proof requests account 1: {:?}", proofs_account_1.len());

    let mut proofs_account_2 = vec![];
    let mut page = 1;
    loop {
        let response = client
            .get_filtered_proof_requests(
                None,
                None,
                None,
                Some(min_deadline),
                None,
                Some(account_2.to_vec()),
                None,
                None,
                None,
                Some(100),
                Some(page),
                None,
            )
            .await
            .unwrap();

        if response.requests.is_empty() {
            break;
        }
        proofs_account_2.extend(response.requests);
        page += 1;
    }
    println!("Proof requests account 2: {:?}", proofs_account_2.len());

    // Combine the two lists and sort by created_at.
    let mut proofs = proofs_account_1
        .into_iter()
        .chain(proofs_account_2.into_iter())
        .collect::<Vec<_>>();
    println!("Proof requests: {:?}", proofs.len());

    // Now, get the stdin for each proof request based on the stdin uri and look at how it's downloaded.
    let mut block_ranges = vec![];
    for proof in proofs.clone() {
        if proof.mode == ProofMode::Compressed as i32 {
            let stdin_uri = proof.stdin_uri;
            // let stdin = client.download_artifact(&stdin_uri).await.unwrap();

            // run aws s3 cp command with stdin_uri as input
            let mut cmd = std::process::Command::new("aws");
            let mut child = cmd
                .arg("s3")
                .arg("cp")
                .arg(stdin_uri)
                .arg("temp")
                // .stdin(std::process::Stdio::piped())
                // .stdout(std::process::Stdio::piped())
                .spawn()
                .expect("failed to execute process");

            println!("child pid: {}", child.id());
            let res = child.wait().unwrap();
            println!("child exited with status: {:?}", res.code());
            // Read from temp
            let bytes = fs::read("temp").unwrap();
            let mut stdin: SP1Stdin = bincode::deserialize(&bytes).unwrap();
            // let mut stdin: SP1Stdin =
            //     bincode::deserialize_from(File::open("temp").unwrap()).unwrap();
            println!("slices: {:?}", stdin.buffer.len());

            if stdin.proofs.len() > 0 {
                println!("skipping aggregation");
                continue;
            }

            // Check if the proof mode is Compressed.
            // let mut buf = vec![];
            // stdin.read_slice(&mut buf);
            // stdin.read()
            let buf = stdin.buffer.remove(0);

            let in_memory_oracle = InMemoryOracle::from_raw_bytes(buf);
            let oracle = Arc::new(in_memory_oracle);

            let boot = match BootInfo::load(oracle.as_ref()).await {
                Ok(boot) => boot,
                Err(e) => {
                    panic!("Failed to load boot info: {:?}", e);
                }
            };
            let start_block = get_agreed_l2_block_number(
                boot.claimed_l2_block_number,
                boot.agreed_l2_output_root,
            )
            .await;
            let end_block = boot.claimed_l2_block_number;
            block_ranges.push((start_block, end_block));
        }
    }

    // Open (or create) the SQLite DB.
    let conn = Connection::open("proofs.db").unwrap();

    // Create the table if it doesn't exist.
    conn.execute(
        "CREATE TABLE IF NOT EXISTS proof_request (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                type TEXT,
                start_block INTEGER NOT NULL,
                end_block INTEGER NOT NULL,
                status TEXT,
                request_added_time INTEGER,
                prover_request_id TEXT,
                proof_request_time INTEGER,
                last_updated_time INTEGER,
                l1_block_number INTEGER,
                l1_block_hash TEXT,
                proof BLOB
            )",
        [],
    )
    .unwrap();

    // Insert each proof request into the database
    for (proof, block_range) in proofs.iter().zip(block_ranges) {
        conn.execute(
            "INSERT INTO proof_request (type, start_block, end_block, prover_request_id, request_added_time, status, last_updated_time, l1_block_number, l1_block_hash, proof) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                "SPAN",
                block_range.0,
                block_range.1,
                format!("0x{}", hex::encode(proof.request_id.clone())),
                std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
                "PROVING",
                std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
                Option::<i64>::None,
                Option::<String>::None,
                Option::<Vec<u8>>::None
            ],
        )?;
    }
    Ok(())
}
