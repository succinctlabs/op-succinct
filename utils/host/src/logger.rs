use std::{env, sync::OnceLock};

use anyhow::{Context, Result};
use opentelemetry::{global, KeyValue};
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_otlp::{Protocol, WithExportConfig};
use opentelemetry_sdk::{logs, propagation::TraceContextPropagator, runtime, Resource};
use tracing_subscriber::{
    layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer, Registry,
};

static INIT: OnceLock<Result<()>> = OnceLock::new();

fn build_env_filter() -> EnvFilter {
    EnvFilter::try_from_default_env()
        .unwrap_or_else(|e| {
            println!("failed to setup env filter: {e:?}");
            EnvFilter::new("info")
        })
        .add_directive("single_hint_handler=error".parse().unwrap())
        .add_directive("execute=error".parse().unwrap())
        .add_directive("sp1_prover=error".parse().unwrap())
        .add_directive("boot_loader=error".parse().unwrap())
        .add_directive("client_executor=error".parse().unwrap())
        .add_directive("client=error".parse().unwrap())
        .add_directive("channel_assembler=error".parse().unwrap())
        .add_directive("attributes_queue=error".parse().unwrap())
        .add_directive("batch_validator=error".parse().unwrap())
        .add_directive("batch_queue=error".parse().unwrap())
        .add_directive("client_derivation_driver=error".parse().unwrap())
        .add_directive("block_builder=error".parse().unwrap())
        .add_directive("host_server=error".parse().unwrap())
        .add_directive("kona_protocol=error".parse().unwrap())
        .add_directive("sp1_core_executor=off".parse().unwrap())
        .add_directive("sp1_core_machine=error".parse().unwrap())
}

/// Set up the logger with optional OpenTelemetry export.
///
/// # Environment Variables
/// - `LOGGER_NAME`: Service name for opentelemetry logs (defaults to `op-succinct`)
/// - `OTLP_ENDPOINT`: OpenTelemetry endpoint (defaults to http://localhost:4317)
/// - `OTLP_ENABLED`: Whether to enable OpenTelemetry export (defaults to false)
/// - `RUST_LOG`: Standard Rust log level configuration
/// - `LOG_FORMAT`: Output format (pretty or json, defaults to pretty)
pub fn setup_logger() {
    INIT.get_or_init(|| {
        let logger_name = std::env::var("LOGGER_NAME").ok();
        let otlp_endpoint =
            std::env::var("OTLP_ENDPOINT").unwrap_or_else(|_| "http://localhost:4317".to_string());

        let otlp_enabled = std::env::var("OTLP_ENABLED")
            .unwrap_or("false".to_string())
            .parse::<bool>()
            .unwrap_or(false);

        let service_name = logger_name.unwrap_or("op-succinct".to_string());

        let params = vec![
            KeyValue::new("service.name", service_name),
            KeyValue::new("service.version", env!("CARGO_PKG_VERSION").to_string()),
        ];

        let resource = Resource::new(params);
        global::set_text_map_propagator(TraceContextPropagator::new());

        let log_format = std::env::var("LOG_FORMAT").unwrap_or("pretty".to_string());
        let fmt_layer: Option<Box<dyn Layer<_> + Send + Sync>> =
            match log_format.to_lowercase().as_str() {
                "json" => {
                    // Initialize with JSON formatting
                    Some(Box::new(
                        tracing_subscriber::fmt::layer()
                            .event_format(tracing_subscriber::fmt::format().json())
                            .with_filter(build_env_filter()),
                    ))
                }
                _ => {
                    // Default to pretty formatting with ANSI colors
                    let ansi = cfg!(feature = "ansi") &&
                        env::var("NO_COLOR").map_or(true, |v| v.is_empty());

                    Some(Box::new(
                        tracing_subscriber::fmt::layer()
                            .with_level(true)
                            .with_target(false)
                            .with_thread_ids(false)
                            .with_thread_names(false)
                            .with_file(false)
                            .with_line_number(false)
                            .with_ansi(ansi)
                            .with_filter(build_env_filter()),
                    ))
                }
            };

        let log_export_layer: Option<Box<dyn Layer<_> + Send + Sync>> = if otlp_enabled {
            let export_layer = opentelemetry_otlp::new_pipeline()
                .logging()
                .with_log_config(logs::config().with_resource(resource))
                .with_exporter(
                    opentelemetry_otlp::new_exporter()
                        .tonic()
                        .with_endpoint(&otlp_endpoint)
                        .with_protocol(Protocol::Grpc),
                )
                .install_batch(runtime::Tokio)
                .context("Failed to install OpenTelemetry logging pipeline")?;
            Some(Box::new(
                OpenTelemetryTracingBridge::new(&export_layer).with_filter(build_env_filter()),
            ))
        } else {
            None
        };

        Registry::default().with(log_export_layer).with(fmt_layer).init();
        if otlp_enabled {
            tracing::info!("OTLP endpoint configured: {}", otlp_endpoint);
        }
        Ok(())
    });
}
