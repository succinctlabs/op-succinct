/// Set up the logger for the proposer.
pub fn setup_proposer_logger() {
    setup_logger_with_format("pretty");
}

/// Set up the logger with a specific format.
pub fn setup_logger_with_format(format: &str) {
    // Turn off all logging from kona and SP1.
    let env_filter = tracing_subscriber::EnvFilter::from_default_env()
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
        .add_directive("sp1_core_machine=error".parse().unwrap());

    match format.to_lowercase().as_str() {
        "json" => {
            // Initialize with JSON formatting
            tracing_subscriber::fmt().with_env_filter(env_filter).json().init();
        }
        "pretty" | _ => {
            // Default to pretty formatting with ANSI colors
            let pretty_format = tracing_subscriber::fmt::format()
                .with_level(true)
                .with_target(false)
                .with_thread_ids(false)
                .with_thread_names(false)
                .with_file(false)
                .with_line_number(false)
                .with_ansi(true);

            tracing_subscriber::fmt()
                .with_env_filter(env_filter)
                .event_format(pretty_format)
                .init();
        }
    }
}
