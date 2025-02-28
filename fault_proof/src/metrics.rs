use lazy_static::lazy_static;
use prometheus::{
    register_counter_vec, register_gauge_vec, register_histogram_vec, CounterVec, GaugeVec,
    HistogramVec,
};

// Define metrics for tracking game-related activities
lazy_static! {
    // Counter for tracking the number of games created
    pub static ref GAMES_CREATED_COUNTER: CounterVec = register_counter_vec!(
        "op_succinct_games_created_total",
        "Total number of games created",
        &["result"] // "success" or "error"
    )
    .unwrap();

    // Counter for tracking game defense attempts
    pub static ref GAMES_DEFENDED_COUNTER: CounterVec = register_counter_vec!(
        "op_succinct_games_defended_total",
        "Total number of games defended",
        &["result"] // "success" or "error"
    )
    .unwrap();

    // Counter for tracking game resolution attempts
    pub static ref GAMES_RESOLVED_COUNTER: CounterVec = register_counter_vec!(
        "op_succinct_games_resolved_total",
        "Total number of games resolved",
        &["result"] // "success" or "error"
    )
    .unwrap();

    // Gauge for tracking the current number of active games
    pub static ref ACTIVE_GAMES_GAUGE: GaugeVec = register_gauge_vec!(
        "op_succinct_active_games",
        "Current number of active games",
        &["status"] // "created", "in_progress", "resolved"
    )
    .unwrap();

    // Histogram for tracking the time taken to prove a game
    pub static ref GAME_PROVING_TIME_HISTOGRAM: HistogramVec = register_histogram_vec!(
        "op_succinct_game_proving_time_seconds",
        "Time taken to prove a game in seconds",
        &["result"], // "success" or "error"
        vec![0.1, 0.5, 1.0, 2.5, 5.0, 10.0, 30.0, 60.0, 120.0, 300.0, 600.0]
    )
    .unwrap();
}

// Helper functions to increment/set metrics
pub fn increment_games_created(result: &str) {
    GAMES_CREATED_COUNTER.with_label_values(&[result]).inc();
}

pub fn increment_games_defended(result: &str) {
    GAMES_DEFENDED_COUNTER.with_label_values(&[result]).inc();
}

pub fn increment_games_resolved(result: &str) {
    GAMES_RESOLVED_COUNTER.with_label_values(&[result]).inc();
}

pub fn set_active_games(status: &str, count: i64) {
    ACTIVE_GAMES_GAUGE
        .with_label_values(&[status])
        .set(count as f64);
}

pub fn observe_game_proving_time(result: &str, seconds: f64) {
    GAME_PROVING_TIME_HISTOGRAM
        .with_label_values(&[result])
        .observe(seconds);
}
