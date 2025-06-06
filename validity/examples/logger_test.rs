/// Simple test program to verify LOG_FORMAT configuration works
use op_succinct_validity::setup_logger_with_format;
use std::env;
use tracing::{error, info, warn};

fn main() {
    // Read LOG_FORMAT from environment variable, default to "pretty"
    let log_format = env::var("LOG_FORMAT").unwrap_or_else(|_| "pretty".to_string());

    println!("Testing LOG_FORMAT configuration...");
    println!("LOG_FORMAT environment variable: {}", log_format);

    // Initialize logger with the format from environment
    println!("\n=== Initializing logger with format: '{}' ===", log_format);
    setup_logger_with_format(&log_format);

    // Test different log levels
    info!("This is an info message");
    warn!("This is a warning message");
    error!("This is an error message");

    // Test structured logging
    info!(user_id = 12345, action = "login", ip_address = "192.168.1.1", "User login event");

    warn!(error_code = "E001", retry_count = 3, "Retrying failed operation");

    error!(
        exception = "NullPointerException",
        stack_trace = "at main.rs:42",
        "Critical error occurred"
    );

    println!("\nLogger test completed. Log output above shows format: '{}'", log_format);
    println!("\nTo test different formats:");
    println!("  LOG_FORMAT=pretty cargo run --example logger_test");
    println!("  LOG_FORMAT=json cargo run --example logger_test");
    println!("  LOG_FORMAT=invalid cargo run --example logger_test  # should default to pretty");
}
