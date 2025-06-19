use std::process::Command;

use anyhow::Context;
use cargo_metadata::MetadataCommand;

fn main() -> anyhow::Result<()> {
    let metadata =
        MetadataCommand::new().no_deps().exec().context("Failed to get cargo metadata")?;

    let workspace_root = metadata.workspace_root;
    let bindings_codegen_path = workspace_root.join("bindings/src/codegen");
    let contracts_package_path = workspace_root.join("contracts");

    // Check if the contracts directory exists.
    if !contracts_package_path.exists() {
        println!("cargo:warning=Contracts directory not found at {:?}", contracts_package_path);
        return Ok(());
    }

    println!("cargo:rerun-if-changed={}", contracts_package_path.join("src"));
    println!("cargo:rerun-if-changed={}", contracts_package_path.join("remappings.txt"));
    println!("cargo:rerun-if-changed={}", contracts_package_path.join("foundry.toml"));

    // Check if solc is available in the PATH.
    let solc_available = Command::new("solc").arg("--version").output().is_ok();

    // Check if forge is available
    if Command::new("forge").arg("--version").output().is_err() {
        println!("cargo:warning=Forge not found in PATH. Skipping bindings generation.");
        return Ok(());
    }

    // Use 'forge bind' to generate the bindings.
    let mut forge_command = Command::new("forge");
    forge_command.args([
        "bind",
        "--bindings-path",
        bindings_codegen_path.as_str(),
        "--module",
        "--overwrite",
    ]);

    forge_command.arg("--alloy-version");
    forge_command.arg("0.15.8");

    forge_command.arg("--skip-cargo-toml");
    forge_command.arg("--force");
    
    
    // // Add --offline flag if solc is available.
    // if solc_available {
    //     forge_command.arg("--offline");
    // }
    
    let status = forge_command.current_dir(&contracts_package_path).status()?;
    
    if !status.success() {
        anyhow::bail!("Forge command failed with exit code: {}", status);
    }

    Ok(())
}
