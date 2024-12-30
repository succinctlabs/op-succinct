use std::process::Command;

use chrono::Local;
use sp1_build::{build_program_with_args, BuildArgs};

const ZKVM_TAG_VERSION: &str = "v3.0.0";

#[derive(Clone, Copy)]
pub enum ProgramBuildArgs {
    Default,
    WithTracing,
}

/// Configuration for build process
pub struct BuildConfig {
    pub program_args: ProgramBuildArgs,
    pub build_zkvm: bool,
}

/// Build a native program.
fn build_native_program(program: &str, program_args: ProgramBuildArgs) {
    let mut args = vec![
        "build",
        "--workspace",
        "--bin",
        program,
        "--profile",
        "release-client-lto",
    ];

    match program_args {
        ProgramBuildArgs::Default => {}
        ProgramBuildArgs::WithTracing => {
            args.extend(&["--features", "tracing-subscriber"]);
        }
    }

    let status = Command::new("cargo")
        .args(&args)
        .status()
        .expect("Failed to execute cargo build command");

    if !status.success() {
        panic!("Failed to build {}", program);
    }

    println!(
        "cargo:warning={} built with release-client-lto profile at {}",
        program,
        current_datetime()
    );
}

/// Build the native host runner to a separate target directory to avoid build lockups.
fn build_native_host_runner() {
    let metadata = cargo_metadata::MetadataCommand::new()
        .exec()
        .expect("Failed to get cargo metadata");
    let target_dir = metadata.target_directory.join("native_host_runner");

    let status = Command::new("cargo")
        .args([
            "build",
            "--workspace",
            "--bin",
            "native_host_runner",
            "--release",
            "--target-dir",
            target_dir.as_ref(),
        ])
        .status()
        .expect("Failed to execute cargo build command");
    if !status.success() {
        panic!("Failed to build native_host_runner");
    }

    println!(
        "cargo:warning=native_host_runner built with release profile at {}",
        current_datetime()
    );
}

pub(crate) fn current_datetime() -> String {
    let now = Local::now();
    now.format("%Y-%m-%d %H:%M:%S").to_string()
}

/// Build a zkVM program.
#[allow(dead_code)]
fn build_zkvm_program(program: &str) -> Result<(), String> {
    let metadata = cargo_metadata::MetadataCommand::new()
        .exec()
        .map_err(|e| format!("Failed to get cargo metadata: {}", e))?;
    
    build_program_with_args(
        &format!("{}/{}", metadata.workspace_root.join("programs"), program),
        BuildArgs {
            elf_name: format!("{}-elf", program),
            docker: true,
            tag: ZKVM_TAG_VERSION.to_string(),
            ..Default::default()
        },
    );
    Ok(())
}

/// Build all the native programs and the native host runner.
pub fn build_all(config: BuildConfig) {
    build_native_program("range", config.program_args);
    
    if config.build_zkvm {
        if let Err(e) = build_zkvm_program("range") {
            println!("cargo:warning=Failed to build zkVM range program: {}", e);
        }
        
        if let Err(e) = build_zkvm_program("aggregation") {
            println!("cargo:warning=Failed to build zkVM aggregation program: {}", e);
        }
    }

    build_native_host_runner();
}
