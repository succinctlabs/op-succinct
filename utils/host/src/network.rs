use anyhow::{anyhow, Result};
use sp1_sdk::network::{FulfillmentStrategy, NetworkMode};

/// Parse a fulfillment strategy from a string.
pub fn parse_fulfillment_strategy(value: String) -> FulfillmentStrategy {
    match value.to_ascii_lowercase().as_str() {
        "reserved" => FulfillmentStrategy::Reserved,
        "hosted" => FulfillmentStrategy::Hosted,
        "auction" => FulfillmentStrategy::Auction,
        _ => FulfillmentStrategy::UnspecifiedFulfillmentStrategy,
    }
}

/// Try to determine the network mode from the provided fulfillment strategies.
pub fn determine_network_mode(
    range_proof_strategy: FulfillmentStrategy,
    agg_proof_strategy: FulfillmentStrategy,
) -> Result<NetworkMode> {
    match (range_proof_strategy, agg_proof_strategy) {
            (FulfillmentStrategy::Auction, FulfillmentStrategy::Auction) => {
                Ok(NetworkMode::Mainnet)
            }
            (
                FulfillmentStrategy::Hosted | FulfillmentStrategy::Reserved,
                FulfillmentStrategy::Hosted | FulfillmentStrategy::Reserved,
            ) => Ok(NetworkMode::Reserved),
            (FulfillmentStrategy::UnspecifiedFulfillmentStrategy, _) |
            (_, FulfillmentStrategy::UnspecifiedFulfillmentStrategy) => Err(anyhow!(
                "The range and agg fulfillment Strategies must be specified"
            )),
            _ => Err(anyhow!(
                "The range fulfillment Strategy '{}' and agg fulfillment Strategy '{}' are incompatible",
                range_proof_strategy.as_str_name().to_ascii_lowercase(),
                agg_proof_strategy.as_str_name().to_ascii_lowercase()
            )),
        }
}
