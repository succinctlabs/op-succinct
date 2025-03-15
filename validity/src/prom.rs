use metrics::describe_gauge;

pub fn custom_gauges() {
    // Proof status gauges
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

    // Proposer gauges
    describe_gauge!(
        "succinct_highest_proven_contiguous_block",
        "Highest proven contiguous block"
    );
    describe_gauge!(
        "succinct_latest_contract_l2_block",
        "Latest L2 block number from the contract"
    );
    describe_gauge!(
        "succinct_l2_unsafe_head_block",
        "L2 unsafe head block number"
    );
    describe_gauge!("succinct_l2_finalized_block", "L2 finalized block number");
    describe_gauge!(
        "succinct_min_block_to_prove_to_agg",
        "Minimum block number required to prove for aggregation"
    );

    // Error gauges
    describe_gauge!("succinct_error_count", "Number of errors");
}
