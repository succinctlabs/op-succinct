use metrics::{describe_gauge, gauge};
use metrics_exporter_prometheus::PrometheusBuilder;
use metrics_process::Collector;
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    thread,
    time::Duration,
};
use tracing::warn;

// Define an enum for all gauge metrics
#[derive(Debug, Clone, Copy)]
pub enum GaugeMetric {
    // Proof status gauges
    CurrentUnrequestedProofs,
    CurrentProvingProofs,
    CurrentWitnessgenProofs,
    CurrentExecuteProofs,

    // Proposer gauges
    HighestProvenContiguousBlock,
    LatestContractL2Block,
    L2UnsafeHeadBlock,
    L2FinalizedBlock,
    MinBlockToProveToAgg,
    ProofRequestRetryCount,

    // Error gauges
    TotalErrorCount,
    ProofRequestTimeoutErrorCount,
    RetryErrorCount,
    WitnessgenErrorCount,
    ExecutionErrorCount,
    MockRangeProofRequestErrorCount,
    MockAggProofRequestErrorCount,
    RangeProofRequestErrorCount,
    AggProofRequestErrorCount,
    RelayAggProofErrorCount,
}

impl GaugeMetric {
    // Get the metric name as a string
    pub fn name(&self) -> &'static str {
        match self {
            // Proof status gauges
            Self::CurrentUnrequestedProofs => "succinct_current_unrequested_proofs",
            Self::CurrentProvingProofs => "succinct_current_proving_proofs",
            Self::CurrentWitnessgenProofs => "succinct_current_witnessgen_proofs",
            Self::CurrentExecuteProofs => "succinct_current_execute_proofs",

            // Proposer gauges
            Self::HighestProvenContiguousBlock => "succinct_highest_proven_contiguous_block",
            Self::LatestContractL2Block => "succinct_latest_contract_l2_block",
            Self::L2UnsafeHeadBlock => "succinct_l2_unsafe_head_block",
            Self::L2FinalizedBlock => "succinct_l2_finalized_block",
            Self::MinBlockToProveToAgg => "succinct_min_block_to_prove_to_agg",
            Self::ProofRequestRetryCount => "succinct_proof_request_retry_count",

            // Error gauges
            Self::TotalErrorCount => "succinct_total_error_count",
            Self::ProofRequestTimeoutErrorCount => "succinct_proof_request_timeout_error_count",
            Self::RetryErrorCount => "succinct_retry_error_count",
            Self::WitnessgenErrorCount => "succinct_witnessgen_error_count",
            Self::ExecutionErrorCount => "succinct_execution_error_count",
            Self::MockRangeProofRequestErrorCount => {
                "succinct_mock_range_proof_request_error_count"
            }
            Self::MockAggProofRequestErrorCount => "succinct_mock_agg_proof_request_error_count",
            Self::RangeProofRequestErrorCount => "succinct_range_proof_request_error_count",
            Self::AggProofRequestErrorCount => "succinct_agg_proof_request_error_count",
            Self::RelayAggProofErrorCount => "succinct_relay_agg_proof_error_count",
        }
    }

    // Get the description for the metric
    pub fn description(&self) -> &'static str {
        match self {
            // Proof status gauges
            Self::CurrentUnrequestedProofs => "Number of proofs currently unrequested",
            Self::CurrentProvingProofs => "Number of proofs currently being proved",
            Self::CurrentWitnessgenProofs => "Number of proofs currently in witness generation",
            Self::CurrentExecuteProofs => "Number of proofs currently being executed",

            // Proposer gauges
            Self::HighestProvenContiguousBlock => "Highest proven contiguous block",
            Self::LatestContractL2Block => "Latest L2 block number from the contract",
            Self::L2UnsafeHeadBlock => "L2 unsafe head block number",
            Self::L2FinalizedBlock => "L2 finalized block number",
            Self::MinBlockToProveToAgg => "Minimum block number required to prove for aggregation",
            Self::ProofRequestRetryCount => "Number of proof request retries",

            // Error gauges
            Self::TotalErrorCount => "Number of total errors",
            Self::ProofRequestTimeoutErrorCount => "Number of proof request timeout errors",
            Self::RetryErrorCount => "Number of retry errors",
            Self::WitnessgenErrorCount => "Number of witness generation errors",
            Self::ExecutionErrorCount => "Number of execution errors",
            Self::MockRangeProofRequestErrorCount => "Number of mock range proof request errors",
            Self::MockAggProofRequestErrorCount => {
                "Number of mock aggregation proof request errors"
            }
            Self::RangeProofRequestErrorCount => "Number of range proof request errors",
            Self::AggProofRequestErrorCount => "Number of aggregation proof request errors",
            Self::RelayAggProofErrorCount => "Number of relay aggregation proof errors",
        }
    }

    // Helper to describe the gauge
    pub fn describe(&self) {
        describe_gauge!(self.name(), self.description());
    }

    // Helper to set the gauge value
    pub fn set(&self, value: f64) {
        gauge!(self.name()).set(value);
    }

    // Get all metrics
    pub fn all() -> Vec<Self> {
        vec![
            // Proof status gauges
            Self::CurrentUnrequestedProofs,
            Self::CurrentProvingProofs,
            Self::CurrentWitnessgenProofs,
            Self::CurrentExecuteProofs,
            // Proposer gauges
            Self::HighestProvenContiguousBlock,
            Self::LatestContractL2Block,
            Self::L2UnsafeHeadBlock,
            Self::L2FinalizedBlock,
            Self::MinBlockToProveToAgg,
            // Error related gauges
            Self::TotalErrorCount,
            Self::ProofRequestTimeoutErrorCount,
            Self::ProofRequestRetryCount,
            Self::RetryErrorCount,
            Self::WitnessgenErrorCount,
            Self::ExecutionErrorCount,
            Self::MockRangeProofRequestErrorCount,
            Self::MockAggProofRequestErrorCount,
            Self::RangeProofRequestErrorCount,
            Self::AggProofRequestErrorCount,
            Self::RelayAggProofErrorCount,
        ]
    }

    // Get error metrics only
    pub fn error_metrics() -> Vec<Self> {
        vec![
            Self::TotalErrorCount,
            Self::ProofRequestTimeoutErrorCount,
            Self::RetryErrorCount,
            Self::WitnessgenErrorCount,
            Self::ExecutionErrorCount,
            Self::MockRangeProofRequestErrorCount,
            Self::MockAggProofRequestErrorCount,
            Self::RangeProofRequestErrorCount,
            Self::AggProofRequestErrorCount,
            Self::RelayAggProofErrorCount,
        ]
    }
}

pub fn custom_gauges() {
    // Register all gauges
    for metric in GaugeMetric::all() {
        metric.describe();
    }
}

pub fn init_gauges() {
    // Initialize all error gauges to 0.0
    for metric in GaugeMetric::error_metrics() {
        metric.set(0.0);
    }
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
