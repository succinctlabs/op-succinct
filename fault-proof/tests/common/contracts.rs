//! Contract deployment utilities for E2E tests.

use alloy_primitives::Address;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use tracing::info;

/// Container for deployed contracts
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeployedContracts {
    pub factory: Address,
    pub portal: Address,
    pub access_manager: Address,
    pub game_implementation: Address,
}

/// Typed structure for forge script output
#[derive(Debug, Deserialize)]
struct ForgeOutput {
    returns: ForgeReturns,
    success: bool,
}

/// Forge returns structure containing contract addresses
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ForgeReturns {
    game_implementation: AddressField,
    #[allow(dead_code)]
    sp1_verifier: AddressField,
    #[allow(dead_code)]
    anchor_state_registry: AddressField,
    access_manager: AddressField,
    optimism_portal2: AddressField,
    factory_proxy: AddressField,
}

/// Address field structure
#[derive(Debug, Deserialize)]
struct AddressField {
    #[allow(dead_code)]
    internal_type: String,
    value: String,
}

/// Parse forge script output to extract contract addresses
fn parse_forge_output(output: &str) -> Result<DeployedContracts> {
    // Get the first line which should be valid JSON
    let json_line = output.lines().next().ok_or_else(|| anyhow!("No output from forge script"))?;

    // Parse the forge output structure
    let forge_output: ForgeOutput = serde_json::from_str(json_line)
        .map_err(|e| anyhow!("Failed to parse forge output JSON: {}\n\n{}", e, json_line))?;

    if !forge_output.success {
        return Err(anyhow!("Forge script execution was not successful"));
    }

    // Parse individual addresses directly from returns
    let factory = forge_output
        .returns
        .factory_proxy
        .value
        .parse::<Address>()
        .map_err(|e| anyhow!("Invalid factoryProxy address: {}", e))?;

    let portal = forge_output
        .returns
        .optimism_portal2
        .value
        .parse::<Address>()
        .map_err(|e| anyhow!("Invalid optimismPortal2 address: {}", e))?;

    let access_manager = forge_output
        .returns
        .access_manager
        .value
        .parse::<Address>()
        .map_err(|e| anyhow!("Invalid accessManager address: {}", e))?;

    let game_implementation = forge_output
        .returns
        .game_implementation
        .value
        .parse::<Address>()
        .map_err(|e| anyhow!("Invalid gameImplementation address: {}", e))?;

    Ok(DeployedContracts { factory, portal, access_manager, game_implementation })
}

/// Deploy all contracts required for E2E testing
pub async fn deploy_test_contracts(rpc_url: &str, private_key: &str) -> Result<DeployedContracts> {
    info!("Deploying test contracts using forge script");

    // Run the forge script to deploy contracts
    let output = std::process::Command::new("forge")
        .arg("script")
        .arg("script/fp/DeployOPSuccinctFDG.s.sol")
        .arg("--broadcast")
        .arg("--rpc-url")
        .arg(rpc_url)
        .arg("--private-key")
        .arg(private_key)
        .arg("--json")
        .env("RUST_LOG", "off")
        .current_dir("../contracts")
        .output()
        .map_err(|e| anyhow!("Failed to execute forge script: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("Forge script failed: {}", stderr));
    }

    // Parse the JSON output to extract contract addresses
    let stdout = String::from_utf8_lossy(&output.stdout);
    let deployed_contracts = parse_forge_output(&stdout)?;

    info!("✓ Contracts deployed successfully");
    info!("  Factory: {}", deployed_contracts.factory);
    info!("  Portal: {}", deployed_contracts.portal);
    info!("  Access Manager: {}", deployed_contracts.access_manager);
    info!("  Game Implementation: {}", deployed_contracts.game_implementation);

    Ok(deployed_contracts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_forge_output() {
        // Using a complete JSON structure
        let sample_output = r#"{"logs":["Using existing OptimismPortal2: 0x0000000000000000000000000000000000000280","Anchor state registry: 0x4ae99c90E5F5d21bBc2bD946F7a50Feea0640662","Access manager: 0xf940F099a47F4613C1C6bc5810f9223442ce0f48","Permissionless fallback timeout (seconds): 1209600","Added proposer: 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266","Added challenger: 0x70997970C51812dc3A010C7d01b50e0d17dc79C8","Using SP1 Verifier Gateway: 0x00000000000000000000000000000000000002e0","Factory Proxy: 0x00CFfc825F160ede7EF2dD3188Cc546698361937","Game Implementation: 0x6699E889831e647c768537ad3D4167C11d710B92","SP1 Verifier: 0x00000000000000000000000000000000000002e0"],"returns":{"gameImplementation":{"internal_type":"address","value":"0x18685de6E1616bdFc63C5a2d566596130FA0fFf9"},"sp1Verifier":{"internal_type":"address","value":"0x74660714fF363D1FA5162a114b6F9aAdCbD9FB05"},"anchorStateRegistry":{"internal_type":"address","value":"0x8A837723abF6dF9E36d450e2529083e101f9Bac7"},"accessManager":{"internal_type":"address","value":"0xAcb8a69AE424032a239650B67744B09807508144"},"optimismPortal2":{"internal_type":"address","value":"0xe4B0df555585243eADf67B96827e5044Bf2Db8Ef"},"factoryProxy":{"internal_type":"address","value":"0xf718cc377680f9E2bD79789451c189023dF872aC"}},"success":true}"#;

        let result = parse_forge_output(sample_output);
        assert!(result.is_ok(), "Failed to parse forge output: {:?}", result.err());

        let deployed = result.unwrap();
        assert_eq!(
            deployed.factory.to_string().to_lowercase(),
            "0xf718cc377680f9e2bd79789451c189023df872ac"
        );
        assert_eq!(
            deployed.portal.to_string().to_lowercase(),
            "0xe4b0df555585243eadf67b96827e5044bf2db8ef"
        );
        assert_eq!(
            deployed.access_manager.to_string().to_lowercase(),
            "0xacb8a69ae424032a239650b67744b09807508144"
        );
        assert_eq!(
            deployed.game_implementation.to_string().to_lowercase(),
            "0x18685de6e1616bdfc63c5a2d566596130fa0fff9"
        );
    }
}
