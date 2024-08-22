use crate::BootInfoWithHashedConfig;
use alloy_primitives::B256;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregationInputs {
    pub boot_infos: Vec<BootInfoWithHashedConfig>,
    pub latest_l1_checkpoint_head: B256,
}
