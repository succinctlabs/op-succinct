use alloy_primitives::B256;
use anyhow::Result;
use comfy_table::{presets::UTF8_FULL, ContentArrangement, Table};
use op_succinct_validity::{DriverDBClient, OPSuccinctRequest, RequestStatus};

pub async fn list(status: RequestStatus, client: DriverDBClient) -> Result<Table> {
    let requests = client.fetch_all_requests_by_status(status).await?;

    Ok(build_requests_table(requests))
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
