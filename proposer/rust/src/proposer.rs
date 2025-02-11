use crate::config::Config;
use crate::driver::Driver;
use crate::metrics::Metrics;
use ethers::providers::{Http, Provider};
use eyre::Result;
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::{error, info};

pub struct ProposerService {
    config: Config,
    driver: Arc<Driver>,
    metrics: Arc<Metrics>,
    shutdown: broadcast::Sender<()>,
}

impl ProposerService {
    pub async fn new(config: Config) -> Result<Self> {
        config.validate()?;

        let l1_provider = Provider::<Http>::try_from(&config.l1_eth_rpc)?;
        let l2_provider = Provider::<Http>::try_from(&config.rollup_rpc)?;
        
        let metrics = Arc::new(Metrics::new());
        let (shutdown_tx, _) = broadcast::channel(1);

        let driver = Arc::new(Driver::new(
            config.clone(),
            l1_provider,
            l2_provider,
            metrics.clone(),
        ).await?);

        Ok(Self {
            config,
            driver,
            metrics,
            shutdown: shutdown_tx,
        })
    }

    pub async fn run(self) -> Result<()> {
        info!("Starting proposer service");

        let mut shutdown_rx = self.shutdown.subscribe();
        
        tokio::select! {
            result = self.driver.run() => {
                if let Err(e) = result {
                    error!("Driver error: {}", e);
                }
            }
            _ = shutdown_rx.recv() => {
                info!("Received shutdown signal");
            }
        }

        info!("Proposer service stopped");
        Ok(())
    }
} 