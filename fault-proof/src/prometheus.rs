use metrics::describe_gauge;

pub fn proposer_gauges() {
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

pub fn challenger_gauges() {
    describe_gauge!(
        "op_succinct_fp_challenger_games_challenged",
        "Total number of games challenged by the challenger"
    );
    describe_gauge!(
        "op_succinct_fp_challenger_games_resolved",
        "Total number of games resolved by the challenger"
    );
    describe_gauge!(
        "op_succinct_fp_challenger_errors",
        "Total number of errors encountered by the challenger"
    );
}
