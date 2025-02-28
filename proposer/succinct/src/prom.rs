use metrics::describe_gauge;
use metrics_exporter_prometheus::PrometheusBuilder;
use metrics_process::Collector;
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    thread,
    time::Duration,
};
use tracing::warn;

pub fn init_metrics(port: &u16) {
    describe_gauge!(
        "succinct_current_unrequested_proofs",
        "Number of proofs currently unrequested"
    );
    describe_gauge!(
        "succinct_current_proving_proofs",
        "Number of proofs currently being proved"
    );
    describe_gauge!(
        "succinct_current_witnessgen_proofs",
        "Number of proofs currently in witness generation"
    );
    describe_gauge!(
        "succinct_current_execute_proofs",
        "Number of proofs currently being executed"
    );

    // Note: Metrics only are exported with the process collector on Linux.
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
