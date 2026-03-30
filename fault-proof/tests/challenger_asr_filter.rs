pub mod common;

#[cfg(feature = "integration")]
mod tests {
    use crate::common::{
        constants::{CHALLENGER_PRIVATE_KEY, TEST_GAME_TYPE},
        new_challenger, TestEnvironment,
    };
    use alloy_primitives::address;
    use anyhow::Result;

    const M: u32 = u32::MAX;

    /// A dummy ASR address that won't match any deployed contract.
    const WRONG_ASR: alloy_primitives::Address =
        address!("0xdead000000000000000000000000000000000001");

    /// Verifies that games are cached when the ASR matches, and filtered when it doesn't.
    /// Uses two challengers: one with the correct ASR, one with a wrong ASR.
    #[tokio::test]
    async fn test_sync_state_filters_different_anchor_state_registry() -> Result<()> {
        let env = TestEnvironment::setup().await?;
        let factory = env.factory()?;
        let init_bond = factory.initBonds(TEST_GAME_TYPE).call().await?;

        // Create a challenger with the correct ASR.
        let correct_challenger = env.init_challenger().await?;

        // Create a challenger with a wrong ASR.
        let wrong_challenger = new_challenger(
            &env.rpc_config,
            CHALLENGER_PRIVATE_KEY,
            &WRONG_ASR,
            &env.deployed.factory,
            env.game_type,
            None,
        )
        .await?;

        let starting_l2_block = env.anvil.starting_l2_block_number;

        // Create a valid game.
        let block = starting_l2_block + 1;
        let root_claim = env.compute_output_root_at_block(block).await?;
        env.create_game(root_claim, block, M, init_bond).await?;

        correct_challenger.sync_state().await?;
        wrong_challenger.sync_state().await?;

        assert_eq!(
            correct_challenger.cached_game_count().await,
            1,
            "Correct ASR challenger should cache the game"
        );
        assert_eq!(
            wrong_challenger.cached_game_count().await,
            0,
            "Wrong ASR challenger should filter out the game"
        );

        Ok(())
    }
}
