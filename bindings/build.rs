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
        println!("cargo:warning=Contracts directory not found at {contracts_package_path:?}");
        return Ok(());
    }

    println!("cargo:rerun-if-changed={}", contracts_package_path.join("src"));
    println!("cargo:rerun-if-changed={}", contracts_package_path.join("remappings.txt"));
    println!("cargo:rerun-if-changed={}", contracts_package_path.join("foundry.toml"));

    // Check if forge is available; skip regeneration if not (e.g. Docker builds).
    // CI jobs with forge (cargo-tests, lint) will catch ABI drift.
    let forge_check = Command::new("forge").arg("--version").output();
    match forge_check {
        Err(_) => {
            println!("cargo:warning=Forge not found in PATH. Skipping bindings generation.");
            return Ok(());
        }
        Ok(output) if !output.status.success() => {
            anyhow::bail!(
                "Forge is installed but returned an error. Check your Foundry installation."
            );
        }
        Ok(_) => {} // forge available, continue
    }

    let mut forge_command = Command::new("forge");
    forge_command.args([
        "bind",
        "--bindings-path",
        bindings_codegen_path.as_str(),
        "--module",
        "--overwrite",
        "--skip-extra-derives",
    ]);

    let required_contracts = [
        // E2E test contracts
        "DisputeGameFactory",
        "SuperchainConfig",
        "MockOptimismPortal2",
        "AnchorStateRegistry",
        "AccessManager",
        "SP1MockVerifier",
        "OPSuccinctFaultDisputeGame",
        "ERC1967Proxy",
        "MockPermissionedDisputeGame",
        // Interfaces
        "IDisputeGameFactory",
        "IDisputeGame",
        "IFaultDisputeGame",
        "IAnchorStateRegistry",
        // Production contracts (host utils)
        "OPSuccinctL2OutputOracle",
        "OPSuccinctDisputeGame",
    ];

    // Create a regex pattern that matches any of our required contracts
    let select_pattern = format!("^({})$", required_contracts.join("|"));
    forge_command.args(["--select", &select_pattern]);

    let status = forge_command.current_dir(&contracts_package_path).status()?;

    if !status.success() {
        anyhow::bail!("Forge command failed with exit code: {}", status);
    }

    Ok(())
}
