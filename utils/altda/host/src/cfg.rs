//! Configuration types for the AltDA host.
//!
//! Contains the [`AltDAChainHost`] configuration struct, [`AltDAExtendedHintType`] hint type
//! wrapper, and [`AltDAChainProviders`] provider set. Follows the pattern established by
//! `CelestiaChainHost` (hana-host) and `SingleChainHostWithEigenDA` (hokulea-host).

use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use kona_host::{
    single::{SingleChainHost, SingleChainHostError, SingleChainProviders},
    OfflineHostBackend, OnlineHostBackend, OnlineHostBackendCfg, PreimageServer,
};
use kona_preimage::{Channel, HintReader, OracleServer};
use kona_proof::{errors::HintParsingError, HintType};
use op_succinct_host_utils::host::PreimageServerStarter;
use serde::Serialize;
use tokio::task::{self, JoinHandle};

use crate::handler::AltDAHintHandler;

/// Extended hint type that wraps kona's [`HintType`] and adds the `AltDACommitment` variant.
///
/// The client-side [`AltDAHintType`](op_succinct_altda_client_utils::hint::AltDAHintType) produces
/// the hint string `"altda-commitment"` via its `Display` impl. This type parses that string back
/// via `FromStr`, routing it to the AltDA-specific hint handler on the host side.
///
/// Standard kona hint types (L1BlockHeader, L1Transactions, etc.) are wrapped in the `Standard`
/// variant and delegated to kona's [`SingleChainHintHandler`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AltDAExtendedHintType {
    /// A standard kona hint type.
    Standard(HintType),
    /// An AltDA commitment hint. The hint data contains the encoded commitment:
    /// `[commitment_type_byte][commitment_data...]`
    AltDACommitment,
}

impl core::str::FromStr for AltDAExtendedHintType {
    type Err = HintParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "altda-commitment" => Ok(Self::AltDACommitment),
            _ => Ok(Self::Standard(HintType::from_str(s)?)),
        }
    }
}

impl core::fmt::Display for AltDAExtendedHintType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            AltDAExtendedHintType::Standard(hint) => write!(f, "{hint}"),
            AltDAExtendedHintType::AltDACommitment => write!(f, "altda-commitment"),
        }
    }
}

/// The host configuration for AltDA-backed OP Stack chains.
///
/// Wraps kona's [`SingleChainHost`] and adds the AltDA server URL. The DA server is the
/// standard OP Stack `op-alt-da` server that stores batch data and serves it by commitment.
#[derive(Default, Parser, Serialize, Clone, Debug)]
pub struct AltDAChainHost {
    /// The inner kona single-chain host configuration.
    #[clap(flatten)]
    pub single_host: SingleChainHost,

    /// URL of the AltDA server (e.g., `http://127.0.0.1:8080`).
    ///
    /// The host fetches batch data from this server using the endpoint:
    /// `GET {altda_server_url}/get/0x{hex(encoded_commitment)}`
    #[clap(long, env = "ALTDA_SERVER_URL")]
    pub altda_server_url: Option<String>,
}

impl AltDAChainHost {
    /// Starts the preimage server, communicating with the client over the provided channels.
    ///
    /// In online mode, creates an [`OnlineHostBackend`] with [`AltDAHintHandler`] to handle
    /// both standard kona hints and AltDA commitment hints. In offline mode, uses kona's
    /// [`OfflineHostBackend`] which serves preimages from the key-value store only.
    pub async fn start_server<C>(
        &self,
        hint: C,
        preimage: C,
    ) -> Result<JoinHandle<Result<(), SingleChainHostError>>, SingleChainHostError>
    where
        C: Channel + Send + Sync + 'static,
    {
        let kv_store = self.single_host.create_key_value_store()?;

        let task_handle = if self.is_offline() {
            task::spawn(async {
                PreimageServer::new(
                    OracleServer::new(preimage),
                    HintReader::new(hint),
                    Arc::new(OfflineHostBackend::new(kv_store)),
                )
                .start()
                .await
                .map_err(SingleChainHostError::from)
            })
        } else {
            let providers = self.create_providers().await?;
            let backend =
                OnlineHostBackend::new(self.clone(), kv_store.clone(), providers, AltDAHintHandler);

            task::spawn(async {
                PreimageServer::new(
                    OracleServer::new(preimage),
                    HintReader::new(hint),
                    Arc::new(backend),
                )
                .start()
                .await
                .map_err(SingleChainHostError::from)
            })
        };

        Ok(task_handle)
    }

    /// Returns `true` if the host is running in offline mode.
    pub const fn is_offline(&self) -> bool {
        self.single_host.is_offline()
    }

    /// Creates the providers required for the host backend.
    ///
    /// Creates the standard kona providers (L1, L2, beacon) plus an HTTP client and DA server
    /// URL for fetching AltDA commitment data.
    async fn create_providers(&self) -> Result<AltDAChainProviders, SingleChainHostError> {
        let inner_providers = self.single_host.create_providers().await?;

        let da_server_url = self
            .altda_server_url
            .clone()
            .ok_or(SingleChainHostError::Other("AltDA server URL must be set"))?;

        Ok(AltDAChainProviders {
            inner_providers,
            da_server_url,
            http_client: reqwest::Client::new(),
        })
    }
}

impl OnlineHostBackendCfg for AltDAChainHost {
    type HintType = AltDAExtendedHintType;
    type Providers = AltDAChainProviders;
}

/// The providers required for the AltDA host.
///
/// Extends kona's [`SingleChainProviders`] with an HTTP client for fetching batch data from
/// the DA server.
#[derive(Debug, Clone)]
pub struct AltDAChainProviders {
    /// The standard kona providers (L1, L2, beacon).
    pub inner_providers: SingleChainProviders,
    /// The URL of the AltDA server.
    pub da_server_url: String,
    /// HTTP client for making requests to the DA server.
    pub http_client: reqwest::Client,
}

#[async_trait]
impl PreimageServerStarter for AltDAChainHost {
    async fn start_server<C>(
        &self,
        hint: C,
        preimage: C,
    ) -> Result<JoinHandle<Result<(), SingleChainHostError>>, SingleChainHostError>
    where
        C: Channel + Send + Sync + 'static,
    {
        self.start_server(hint, preimage).await
    }
}
