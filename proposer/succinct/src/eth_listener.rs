use alloy_provider::{Provider, ProviderBuilder, WsConnect};
use anyhow::Result;
use futures_util::StreamExt;
use op_alloy_network::Optimism;
use std::sync::Arc;

// Import our DB client and EthMetrics struct
use crate::db::{DriverDBClient, EthMetrics};

pub struct EthListener<P>
where
    P: Provider<Optimism> + 'static,
{
    db_client: Arc<DriverDBClient>,
    provider: Arc<P>,
}

impl<P> EthListener<P>
where
    P: Provider<Optimism> + 'static,
{
    pub fn new(db_client: Arc<DriverDBClient>, l2_provider: Arc<P>) -> Self {
        Self {
            db_client,
            provider: l2_provider,
        }
    }

    pub async fn listen(&self) -> Result<()> {
        // Subscribe to new blocks.
        let sub = self.provider.subscribe_blocks().await?;
        println!("Listening for new blocks on alloy provider");

        // Convert the subscription to a stream.
        let mut stream = sub.into_stream();

        while let Some(header) = stream.next().await {
            // Fetch the full block.
            let receipts = self
                .provider
                .get_block_receipts(header.number.into())
                .await?;

            if let Some(receipts) = receipts {
                let nb_transactions = receipts.len() as i64;
                let eth_gas_used = header.gas_used as i64;

                let mut tx_fees = 0;
                let mut l1_fees = 0;
                // Collect the transaction fees.
                for receipt in receipts {
                    tx_fees += receipt.inner.effective_gas_price * receipt.inner.gas_used as u128;

                    if let Some(l1_fee) = receipt.l1_block_info.l1_fee {
                        l1_fees += l1_fee;
                    }
                }

                let metrics = EthMetrics {
                    nb_transactions,
                    eth_gas_used,
                    l1_fees: l1_fees.into(),
                    tx_fees: tx_fees.into(),
                };

                match self.db_client.insert_eth_metrics(&metrics).await {
                    Ok(_) => {
                        tracing::info!("Inserted metrics for block: {}", header.number);
                    }
                    Err(e) => {
                        tracing::error!(
                            "Error inserting metrics for block {}: {:?}",
                            header.number,
                            e
                        )
                    }
                }
            }
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let l2_rpc = "wss://your-l2-rpc-endpoint";
    let database_url = "postgres://user:password@localhost/dbname";

    // Initialize DB client and wrap it in an Arc for shared ownership
    let db_client = Arc::new(DriverDBClient::new(database_url).await?);

    // Create a provider for the L2 network
    let ws = WsConnect::new(l2_rpc);
    let provider = ProviderBuilder::default().on_ws(ws).await?;

    // Create an EthListener struct with a reference to our Arc'd DB client

    // Create an EthListener struct with a reference to our Arc'd DB client
    let eth_listener = EthListener::new(db_client, Arc::new(provider));

    eth_listener.listen().await?;

    Ok(())
}
