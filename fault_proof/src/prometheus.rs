use metrics::{describe_counter, describe_gauge};
use metrics_exporter_prometheus::PrometheusBuilder;
use metrics_process::Collector;
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    thread,
    time::Duration,
};
use tracing::warn;

pub fn custom_counters() {
    // Game status counters
    describe_counter!(
        "op_succinct_fp_games_created",
        "Number of games created by the proposer"
    );
    describe_counter!(
        "op_succinct_fp_games_challenged",
        "Number of games challenged by the proposer"
    );
    describe_counter!(
        "op_succinct_fp_games_defended",
        "Number of games defended by the proposer"
    );
    describe_counter!(
        "op_succinct_fp_games_resolved",
        "Number of games resolved by the proposer"
    );

    // Error counter
    describe_counter!("op_succinct_fp_errors", "Number of errors");
}

pub fn custom_gauges() {
    // Block number gauges
    describe_gauge!(
        "op_succinct_fp_finalized_l2_block_number",
        "Finalized L2 block number"
    );
    describe_gauge!(
        "op_succinct_fp_latest_game_l2_block_number",
        "Latest game L2 block number"
    );
    describe_gauge!(
        "op_succinct_fp_latest_anchor_l2_block_number",
        "Latest anchor L2 block number"
    );
}

pub fn init_metrics(port: u16) {
    // Initialize custom counters
    custom_counters();

    // Initialize custom gauges
    custom_gauges();

    let builder = PrometheusBuilder::new()
        .with_http_listener(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port));

    if let Err(e) = builder.install() {
        warn!(
            "Failed to start metrics server: {}. Will continue without metrics.",
            e
        );
    }

    // Spawn a thread to collect process metrics
    thread::spawn(move || {
        let collector = Collector::default();
        collector.describe();
        loop {
            // Periodically call `collect()` method to update information.
            collector.collect();
            thread::sleep(Duration::from_millis(750));
        }
    });
}
