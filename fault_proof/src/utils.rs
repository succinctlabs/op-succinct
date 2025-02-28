use std::net::SocketAddr;
use tracing_subscriber::{fmt, EnvFilter};

pub fn setup_logging() {
    let format = fmt::format()
        .with_level(true)
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_file(false)
        .with_line_number(false)
        .with_ansi(true);

    // Initialize logging using RUST_LOG environment variable, defaulting to INFO level
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_env("RUST_LOG").unwrap_or_else(|_| {
            EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into())
        }))
        .event_format(format)
        .init();
}

/// Setup and start the Prometheus metrics server
pub async fn setup_metrics_server(addr: SocketAddr) -> anyhow::Result<()> {
    tracing::info!("Starting metrics server on {}", addr);
    let builder = prometheus_exporter::Builder::new(addr);
    builder.start()?;
    tracing::info!("Metrics server started");

    Ok(())
}
