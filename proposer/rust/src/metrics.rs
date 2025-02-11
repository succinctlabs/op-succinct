use std::sync::atomic::{AtomicU64, Ordering};

pub struct Metrics {
    pub num_proving: AtomicU64,
    pub num_witness_gen: AtomicU64,
    pub num_unrequested: AtomicU64,
    pub l2_finalized_block: AtomicU64,
    pub latest_contract_l2_block: AtomicU64,
    pub highest_proven_contiguous_l2_block: AtomicU64,
    pub min_block_to_prove_to_agg: AtomicU64,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            num_proving: AtomicU64::new(0),
            num_witness_gen: AtomicU64::new(0),
            num_unrequested: AtomicU64::new(0),
            l2_finalized_block: AtomicU64::new(0),
            latest_contract_l2_block: AtomicU64::new(0),
            highest_proven_contiguous_l2_block: AtomicU64::new(0),
            min_block_to_prove_to_agg: AtomicU64::new(0),
        }
    }

    pub fn record_proving(&self, count: u64) {
        self.num_proving.store(count, Ordering::SeqCst);
    }

    pub fn record_witness_gen(&self, count: u64) {
        self.num_witness_gen.store(count, Ordering::SeqCst);
    }

    pub fn record_unrequested(&self, count: u64) {
        self.num_unrequested.store(count, Ordering::SeqCst);
    }

    pub fn record_l2_finalized_block(&self, block: u64) {
        self.l2_finalized_block.store(block, Ordering::SeqCst);
    }
}
