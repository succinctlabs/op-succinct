use alloy_consensus::Header;
use alloy_primitives::B256;
use serde::{Deserialize, Serialize};

use crate::RawBootInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregationInputs {
    pub boot_infos: Vec<RawBootInfo>,
    pub headers: Vec<Header>,
    pub l1_head: B256,
}
