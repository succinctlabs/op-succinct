use alloy::{hex, sol_types::SolValue};
use alloy_primitives::{Address, B256};
use anyhow::Result;
use clap::Parser;
use futures::StreamExt;
use op_succinct_client_utils::{boot::BootInfoStruct, AGGREGATION_OUTPUTS_SIZE};
use sp1_sdk::{
    network_v2::{
        client::NetworkClient,
        proto::network::{
            prover_network_client::ProverNetworkClient, FulfillmentStatus,
            GetFilteredProofRequestsRequest, ProofMode,
        },
    },
    NetworkProverV2, Prover, SP1ProofWithPublicValues, SP1Stdin,
};
use std::{env, fs, path::Path, str::FromStr};
use tonic::{
    transport::{channel::ClientTlsConfig, Channel},
    Code,
};

pub const RANGE_ELF: &[u8] = include_bytes!("../../../elf/range-elf");

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Requester address.
    #[arg(short, long, required = false)]
    requester: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let args = Args::parse();

    let private_key = env::var("SP1_PRIVATE_KEY")?;
    let rpc_url = env::var("PROVER_NETWORK_RPC")?;
    let mut endpoint = Channel::from_shared(rpc_url.clone())?;

    // Check if the URL scheme is HTTPS and configure TLS.
    if rpc_url.starts_with("https://") {
        let tls_config = ClientTlsConfig::new().with_enabled_roots();
        endpoint = endpoint.tls_config(tls_config)?;
    }

    let channel = endpoint.connect().await?;
    let mut prover_network_client = ProverNetworkClient::new(channel);

    let requester = Address::from_str(&args.requester)?;

    let request = GetFilteredProofRequestsRequest {
        requester: Some(requester.to_vec()),
        fulfillment_status: Some(FulfillmentStatus::Fulfilled as i32),
        limit: Some(10),
        mode: Some(ProofMode::Compressed as i32),
        ..Default::default()
    };

    let response = prover_network_client
        .get_filtered_proof_requests(request)
        .await?;

    let requests = response.into_inner().requests;
    println!("{:?}", requests.len());

    // Loop over all of the proof requests and print the total size of all of the proof data in bytes.
    let mut total_size = 0;
    let mut stdin_size = 0;

    let results = futures::stream::iter(requests)
        .map(|proof_request| async move {
            let private_key = env::var("SP1_PRIVATE_KEY")?;
            let rpc_url = env::var("PROVER_NETWORK_RPC")?;
            let prover = NetworkProverV2::new(&private_key.clone(), Some(rpc_url.clone()), false);
            let (_, range_vk) = prover.setup(RANGE_ELF);
            println!("Fetching proof for request: {:?}", proof_request.request_id);
            let mut proof: SP1ProofWithPublicValues =
                prover.wait_proof(&proof_request.request_id, None).await?;
            println!("Proof fetched for request: {:?}", proof_request.request_id);

            let proof_size = bincode::serialized_size(&proof)?;
            let stdin_size = bincode::serialized_size(&proof.stdin)?;

            proof.stdin = SP1Stdin::default();
            prover.verify(&proof, &range_vk.clone())?;

            Ok::<_, anyhow::Error>((proof_size, stdin_size))
        })
        .buffer_unordered(10)
        .collect::<Vec<_>>()
        .await;

    for result in results {
        let (proof_size, proof_stdin_size) = result?;
        total_size += proof_size;
        stdin_size += proof_stdin_size;
    }

    println!("Total size of all proof data: {} bytes", total_size);
    println!("Total size of all stdin data: {} bytes", stdin_size);

    Ok(())
}
