use metrics::{describe_gauge, gauge};
use strum::{EnumMessage, IntoEnumIterator};
use strum_macros::{Display, EnumIter};

// Define an enum for all proposer gauge metrics.
#[derive(Debug, Clone, Copy, Display, EnumIter, EnumMessage)]
pub enum ProposerGauge {
    // Proposer metrics
    #[strum(
        serialize = "op_succinct_fp_finalized_l2_block_number",
        message = "Finalized L2 block number"
    )]
    FinalizedL2BlockNumber,
    #[strum(
        serialize = "op_succinct_fp_latest_game_l2_block_number",
        message = "Latest game L2 block number"
    )]
    LatestGameL2BlockNumber,
    #[strum(
        serialize = "op_succinct_fp_anchor_game_l2_block_number",
        message = "Anchor game L2 block number"
    )]
    AnchorGameL2BlockNumber,
    #[strum(
        serialize = "op_succinct_fp_games_created",
        message = "Total number of games created by the proposer"
    )]
    GamesCreated,
    #[strum(
        serialize = "op_succinct_fp_games_resolved",
        message = "Total number of games resolved by the proposer"
    )]
    GamesResolved,
    #[strum(
        serialize = "op_succinct_fp_games_bonds_claimed",
        message = "Total number of games that bonds were claimed by the proposer"
    )]
    GamesBondsClaimed,
    // Error metrics
    #[strum(
        serialize = "op_succinct_fp_errors",
        message = "Total number of errors encountered by the proposer"
    )]
    Errors,
}

impl ProposerGauge {
    // Helper to describe the proposer gauge.
    pub fn describe(&self) {
        describe_gauge!(self.to_string(), self.get_message().unwrap());
    }

    // Helper to set the proposer gauge value.
    pub fn set(&self, value: f64) {
        gauge!(self.to_string()).set(value);
    }

    // Helper to increment the proposer gauge value.
    pub fn increment(&self, value: f64) {
        gauge!(self.to_string()).increment(value);
    }
}

pub fn proposer_gauges() {
    // Register all proposer gauges.
    for metric in ProposerGauge::iter() {
        metric.describe();
    }
}

pub fn init_proposer_gauges() {
    // Initialize all proposer gauges to 0.0.
    for metric in ProposerGauge::iter() {
        metric.set(0.0);
    }
}

// Define an enum for all challenger gauge metrics.
#[derive(Debug, Clone, Copy, Display, EnumIter, EnumMessage)]
pub enum ChallengerGauge {
    // Challenger metrics
    #[strum(
        serialize = "op_succinct_fp_challenger_games_challenged",
        message = "Total number of games challenged by the challenger"
    )]
    GamesChallenged,
    #[strum(
        serialize = "op_succinct_fp_challenger_games_resolved",
        message = "Total number of games resolved by the challenger"
    )]
    GamesResolved,
    // Error metrics
    #[strum(
        serialize = "op_succinct_fp_challenger_errors",
        message = "Total number of errors encountered by the challenger"
    )]
    Errors,
}

impl ChallengerGauge {
    // Helper to describe the challenger gauge.
    pub fn describe(&self) {
        describe_gauge!(self.to_string(), self.get_message().unwrap());
    }

    // Helper to set the challenger gauge value.
    pub fn set(&self, value: f64) {
        gauge!(self.to_string()).set(value);
    }

    // Helper to increment the challenger gauge value.
    pub fn increment(&self, value: f64) {
        gauge!(self.to_string()).increment(value);
    }
}

pub fn challenger_gauges() {
    // Register all challenger gauges.
    for metric in ChallengerGauge::iter() {
        metric.describe();
    }
}

pub fn init_challenger_gauges() {
    // Initialize all challenger gauges to 0.0.
    for metric in ChallengerGauge::iter() {
        metric.set(0.0);
    }
}
