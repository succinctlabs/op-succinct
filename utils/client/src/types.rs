use alloy_primitives::{Address, B256};
use alloy_sol_types::sol;
use serde::{Deserialize, Serialize};

use crate::boot::BootInfoStruct;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregationInputs {
    pub boot_infos: Vec<BootInfoStruct>,
    pub latest_l1_checkpoint_head: B256,
    pub multi_block_vkey: [u32; 8],
    pub prover_address: Address,
}

sol! {
    #[derive(Debug, Serialize, Deserialize)]
    struct AggregationOutputs {
        bytes32 l1Head;
        bytes32 l2PreRoot;
        bytes32 l2PostRoot;
        uint64 l2BlockNumber;
        bytes32 rollupConfigHash;
        bytes32 multiBlockVKey;
        address proverAddress;
    }
}
