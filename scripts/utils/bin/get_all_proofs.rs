use std::{env, io::Read, sync::Arc};

use alloy_primitives::{Address, B256};
use anyhow::anyhow;
use kona_proof::BootInfo;
use op_succinct_client_utils::InMemoryOracle;
use sp1_sdk::{
    network::{proto::network::ProofMode, NetworkClient},
    ProverClient, SP1Stdin,
};

fn get_agreed_l2_block_number(claimed_l2_block_number: u64, agreed_l2_output_root: B256) -> u64 {
    const DEFAULT_DISTANCE: u64 = 60;

    claimed_l2_block_number
}

#[tokio::main]
async fn main() {
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
    for proof in proofs {
        if proof.mode == ProofMode::Compressed as i32 {
            let stdin_uri = proof.stdin_uri;
            // let stdin = client.download_artifact(&stdin_uri).await.unwrap();

            // run aws s3 cp command with stdin_uri as input
            let mut cmd = std::process::Command::new("aws");
            let mut child = cmd
                .arg("s3")
                .arg("cp")
                .arg(stdin_uri)
                .arg("-")
                // .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .spawn()
                .expect("failed to execute process");

            println!("child pid: {}", child.id());
            child.wait().unwrap();
            println!("child exited with status: {:?}", child);
            let mut stdout = child.stdout.unwrap();
            let mut stdout_bytes = vec![];
            stdout.read_to_end(&mut stdout_bytes).unwrap();
            println!("downloaded {} bytes", stdout_bytes.len());

            let mut stdin: SP1Stdin = bincode::deserialize(&stdout_bytes).unwrap();
            // Check if the proof mode is Compressed.
            let mut buf = vec![];
            stdin.read_slice(&mut buf);

            let in_memory_oracle = InMemoryOracle::from_raw_bytes(buf);
            let oracle = Arc::new(in_memory_oracle);

            let boot = match BootInfo::load(oracle.as_ref()).await {
                Ok(boot) => boot,
                Err(e) => {
                    panic!("Failed to load boot info: {:?}", e);
                }
            };
            println!("Boot Claim: {:?}", boot.claimed_l2_block_number);

            // println!("Boot info: {:?}", boot);
        }
    }
}
