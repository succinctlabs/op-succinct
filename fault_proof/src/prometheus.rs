use metrics::describe_gauge;
use metrics_exporter_prometheus::PrometheusBuilder;
use metrics_process::Collector;
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    thread,
    time::Duration,
};
use tracing::warn;

pub fn custom_gauges() {
    // Proposer metrics
    describe_gauge!(
        "op_succinct_fp_finalized_l2_block_number",
        "Finalized L2 block number"
    );
    describe_gauge!(
        "op_succinct_fp_latest_game_l2_block_number",
        "Latest game L2 block number"
    );
    describe_gauge!(
        "op_succinct_fp_anchor_game_l2_block_number",
        "Anchor game L2 block number"
    );
    describe_gauge!(
        "op_succinct_fp_games_created",
        "Total number of games created by the proposer"
    );
    describe_gauge!(
        "op_succinct_fp_games_resolved",
        "Total number of games resolved by the proposer"
    );
    describe_gauge!(
        "op_succinct_fp_games_bonds_claimed",
        "Total number of games that bonds were claimed by the proposer"
    );

    // Error metrics
    describe_gauge!(
        "op_succinct_fp_errors",
        "Total number of errors encountered by the proposer"
    );
}

pub fn init_metrics(port: &u16) {
    custom_gauges();

    let builder = PrometheusBuilder::new().with_http_listener(SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
        port.to_owned(),
    ));

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
