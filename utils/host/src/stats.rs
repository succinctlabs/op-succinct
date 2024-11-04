use std::fmt;

use crate::fetcher::BlockInfo;
use num_format::{Locale, ToFormattedString};
use serde::{Deserialize, Serialize};
use sp1_sdk::{CostEstimator, ExecutionReport};

/// Statistics for the range execution.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExecutionStats {
    pub batch_start: u64,
    pub batch_end: u64,
    /// The wall clock time to generate the witness.
    pub witness_generation_time_sec: u64,
    /// The wall clock time to execute the range on the machine.
    pub total_execution_time_sec: u64,
    pub total_instruction_count: u64,
    pub oracle_verify_instruction_count: u64,
    pub derivation_instruction_count: u64,
    pub block_execution_instruction_count: u64,
    pub blob_verification_instruction_count: u64,
    pub total_sp1_gas: u64,
    pub nb_blocks: u64,
    pub nb_transactions: u64,
    pub eth_gas_used: u64,
    pub l1_fees: u128,
    pub total_tx_fees: u128,
    pub cycles_per_block: u64,
    pub cycles_per_transaction: u64,
    pub transactions_per_block: u64,
    pub gas_used_per_block: u64,
    pub gas_used_per_transaction: u64,
    pub bn_pair_cycles: u64,
    pub bn_add_cycles: u64,
    pub bn_mul_cycles: u64,
    pub kzg_eval_cycles: u64,
    pub ec_recover_cycles: u64,
}

/// Write a statistic to the formatter.
fn write_stat(f: &mut fmt::Formatter<'_>, label: &str, value: u64) -> fmt::Result {
    writeln!(
        f,
        "| {:<30} | {:>25} |",
        label,
        value.to_formatted_string(&Locale::en)
    )
}

impl fmt::Display for ExecutionStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "+--------------------------------+---------------------------+"
        )?;
        writeln!(f, "| {:<30} | {:<25} |", "Metric", "Value")?;
        writeln!(
            f,
            "+--------------------------------+---------------------------+"
        )?;
        write_stat(f, "Batch Start", self.batch_start)?;
        write_stat(f, "Batch End", self.batch_end)?;
        write_stat(
            f,
            "Witness Generation (seconds)",
            self.witness_generation_time_sec,
        )?;
        write_stat(
            f,
            "Execution Duration (seconds)",
            self.total_execution_time_sec,
        )?;
        write_stat(f, "Total Instruction Count", self.total_instruction_count)?;
        write_stat(
            f,
            "Oracle Verify Cycles",
            self.oracle_verify_instruction_count,
        )?;
        write_stat(f, "Derivation Cycles", self.derivation_instruction_count)?;
        write_stat(
            f,
            "Block Execution Cycles",
            self.block_execution_instruction_count,
        )?;
        write_stat(
            f,
            "Blob Verification Cycles",
            self.blob_verification_instruction_count,
        )?;
        write_stat(f, "Total SP1 Gas", self.total_sp1_gas)?;
        write_stat(f, "Number of Blocks", self.nb_blocks)?;
        write_stat(f, "Number of Transactions", self.nb_transactions)?;
        write_stat(f, "Ethereum Gas Used", self.eth_gas_used)?;
        write_stat(f, "Cycles per Block", self.cycles_per_block)?;
        write_stat(f, "Cycles per Transaction", self.cycles_per_transaction)?;
        write_stat(f, "Transactions per Block", self.transactions_per_block)?;
        write_stat(f, "Gas Used per Block", self.gas_used_per_block)?;
        write_stat(f, "Gas Used per Transaction", self.gas_used_per_transaction)?;
        write_stat(f, "BN Pair Cycles", self.bn_pair_cycles)?;
        write_stat(f, "BN Add Cycles", self.bn_add_cycles)?;
        write_stat(f, "BN Mul Cycles", self.bn_mul_cycles)?;
        write_stat(f, "KZG Eval Cycles", self.kzg_eval_cycles)?;
        write_stat(f, "EC Recover Cycles", self.ec_recover_cycles)?;
        writeln!(
            f,
            "+--------------------------------+---------------------------+"
        )
    }
}

impl ExecutionStats {
    /// Create a new execution stats.
    pub fn new(
        block_data: &[BlockInfo],
        report: &ExecutionReport,
        witness_generation_time_sec: u64,
        total_execution_time_sec: u64,
    ) -> Self {
        // Sort the block data by block number.
        let mut block_data = block_data.to_vec();
        block_data.sort_by_key(|b| b.block_number);

        let get_cycles = |key: &str| *report.cycle_tracker.get(key).unwrap_or(&0);

        let nb_blocks = block_data.len() as u64;
        let nb_transactions = block_data.iter().map(|b| b.transaction_count).sum();
        let total_gas_used: u64 = block_data.iter().map(|b| b.gas_used).sum();

        Self {
            batch_start: block_data[0].block_number,
            batch_end: block_data[block_data.len() - 1].block_number,
            total_instruction_count: report.total_instruction_count(),
            total_sp1_gas: report.estimate_gas(),
            block_execution_instruction_count: get_cycles("block-execution"),
            oracle_verify_instruction_count: get_cycles("oracle-verify"),
            derivation_instruction_count: get_cycles("payload-derivation"),
            blob_verification_instruction_count: get_cycles("blob-verification"),
            bn_add_cycles: get_cycles("precompile-bn-add"),
            bn_mul_cycles: get_cycles("precompile-bn-mul"),
            bn_pair_cycles: get_cycles("precompile-bn-pair"),
            kzg_eval_cycles: get_cycles("precompile-kzg-eval"),
            ec_recover_cycles: get_cycles("precompile-ec-recover"),
            nb_transactions,
            eth_gas_used: block_data.iter().map(|b| b.gas_used).sum(),
            l1_fees: block_data.iter().map(|b| b.total_l1_fees).sum(),
            total_tx_fees: block_data.iter().map(|b| b.total_tx_fees).sum(),
            nb_blocks,
            cycles_per_block: report.total_instruction_count() / nb_blocks,
            cycles_per_transaction: report.total_instruction_count() / nb_transactions,
            transactions_per_block: nb_transactions / nb_blocks,
            gas_used_per_block: total_gas_used / nb_blocks,
            gas_used_per_transaction: total_gas_used / nb_transactions,
            witness_generation_time_sec,
            total_execution_time_sec,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SpanBatchStats {
    pub span_start: u64,
    pub span_end: u64,
    pub total_blocks: u64,
    pub total_transactions: u64,
    pub total_gas_used: u64,
    pub total_cycles: u64,
    pub total_sp1_gas: u64,
    pub cycles_per_block: u64,
    pub cycles_per_transaction: u64,
    pub gas_used_per_block: u64,
    pub gas_used_per_transaction: u64,
    pub total_derivation_cycles: u64,
    pub total_execution_cycles: u64,
    pub total_blob_verification_cycles: u64,
    pub bn_add_cycles: u64,
    pub bn_mul_cycles: u64,
    pub bn_pair_cycles: u64,
    pub kzg_eval_cycles: u64,
    pub ec_recover_cycles: u64,
}

impl fmt::Display for SpanBatchStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "+-------------------------------+---------------------------+"
        )?;
        writeln!(f, "| {:<30} | {:<25} |", "Metric", "Value")?;
        writeln!(
            f,
            "+-------------------------------+---------------------------+"
        )?;
        write_stat(f, "Span Start", self.span_start)?;
        write_stat(f, "Span End", self.span_end)?;
        write_stat(f, "Total Blocks", self.total_blocks)?;
        write_stat(f, "Total Transactions", self.total_transactions)?;
        write_stat(f, "Total Gas Used", self.total_gas_used)?;
        write_stat(f, "Total Cycles", self.total_cycles)?;
        write_stat(f, "Total SP1 Gas", self.total_sp1_gas)?;
        write_stat(f, "Cycles per Block", self.cycles_per_block)?;
        write_stat(f, "Cycles per Transaction", self.cycles_per_transaction)?;
        write_stat(f, "Gas Used per Block", self.gas_used_per_block)?;
        write_stat(f, "Gas Used per Transaction", self.gas_used_per_transaction)?;
        write_stat(f, "Total Derivation Cycles", self.total_derivation_cycles)?;
        write_stat(f, "Total Execution Cycles", self.total_execution_cycles)?;
        write_stat(
            f,
            "Total Blob Verification Cycles",
            self.total_blob_verification_cycles,
        )?;
        write_stat(f, "BN Add Cycles", self.bn_add_cycles)?;
        write_stat(f, "BN Mul Cycles", self.bn_mul_cycles)?;
        write_stat(f, "BN Pair Cycles", self.bn_pair_cycles)?;
        write_stat(f, "KZG Eval Cycles", self.kzg_eval_cycles)?;
        write_stat(f, "EC Recover Cycles", self.ec_recover_cycles)?;
        writeln!(
            f,
            "+-------------------------------+---------------------------+"
        )
    }
}
