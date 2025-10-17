use std::sync::Arc;

use alloy_primitives::B256;
use anyhow::{anyhow, bail, Result};
use comfy_table::{presets::UTF8_FULL, ContentArrangement, Table};
use op_succinct_host_utils::fetcher::OPSuccinctDataFetcher;
use op_succinct_validity::{DriverDBClient, OPSuccinctRequest, RequestStatus};

pub async fn list(status: RequestStatus, db_client: DriverDBClient) -> Result<Table> {
    let requests = db_client.fetch_all_requests_by_status(status).await?;

    Ok(build_requests_table(requests))
}

pub async fn split(
    id: u64,
    at: u64,
    db_client: DriverDBClient,
    fetcher: Arc<OPSuccinctDataFetcher>,
) -> Result<()> {
    let at = at as i64;
    let request = db_client
        .fetch_request(id as i64)
        .await?
        .ok_or_else(|| anyhow!("The proof request '{id}' wasn't found in the DB"))?;

    if request.start_block >= at || request.end_block <= at {
        bail!("The proof request '{id}' is not valid for splitting at block {at}");
    }

    let range_vkey_commitment = B256::from_slice(&request.range_vkey_commitment);
    let rollup_config_hash = B256::from_slice(&request.rollup_config_hash);

    let a = OPSuccinctRequest::create_range_request(
        request.mode,
        request.start_block,
        at,
        range_vkey_commitment,
        rollup_config_hash,
        request.l1_chain_id,
        request.l2_chain_id,
        fetcher.clone(),
    )
    .await?;

    let b = OPSuccinctRequest::create_range_request(
        request.mode,
        at,
        request.end_block,
        range_vkey_commitment,
        rollup_config_hash,
        request.l1_chain_id,
        request.l2_chain_id,
        fetcher,
    )
    .await?;

    db_client.update_request_status(request.id, RequestStatus::Failed).await?;

    println!("Marked {request} as failed");

    db_client.insert_request(&a).await?;

    println!("Inserted {a}");

    db_client.insert_request(&b).await?;

    println!("Inserted {b}");

    Ok(())
}

fn build_requests_table(requests: Vec<OPSuccinctRequest>) -> Table {
    let mut table = Table::new();

    table.load_preset(UTF8_FULL).set_content_arrangement(ContentArrangement::Dynamic).set_header(
        vec![
            "Id",
            "Request Id",
            "Start Block",
            "End Block",
            "Created at",
            "Execution Duration",
            "Prove Duration",
        ],
    );

    for r in requests {
        table.add_row(vec![
            r.id.to_string(),
            r.proof_request_id.map(|r| B256::from_slice(&r).to_string()).unwrap_or_default(),
            r.start_block.to_string(),
            r.end_block.to_string(),
            r.created_at.to_string(),
            r.execution_duration.map(|d| d.to_string()).unwrap_or_default(),
            r.prove_duration.map(|d| d.to_string()).unwrap_or_default(),
        ]);
    }

    table
}
