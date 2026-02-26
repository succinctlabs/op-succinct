//! AltDA hint types for the preimage oracle hint system.
//!
//! When the AltDA data source encounters a commitment in L1 calldata, it sends a hint
//! to the host to fetch the actual batch data from the DA server. The host parses the
//! hint string and resolves the commitment by fetching from the configured DA server URL.

use core::fmt;

/// Hint type for AltDA commitment resolution.
///
/// Used by [`AltDADataSource`](crate::data_source::AltDADataSource) to request batch data
/// from the host when it encounters a commitment in L1 calldata.
///
/// The hint data format is: `[commitment_type_byte][commitment_data...]`
/// - For Keccak256 commitments: `[0x00][32 bytes of keccak256 hash]`
/// - For Generic commitments: `[0x01][variable length opaque bytes]`
///
/// The host (Phase 3) will parse `"altda-commitment"` from the hint string and use
/// the commitment data to fetch batch data from the DA server.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AltDAHintType {
    /// Hint containing an AltDA commitment that needs to be resolved to batch data.
    AltDACommitment,
}

impl fmt::Display for AltDAHintType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AltDAHintType::AltDACommitment => write!(f, "altda-commitment"),
        }
    }
}
