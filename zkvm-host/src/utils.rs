use std::fmt;

use num_format::{Locale, ToFormattedString};

/// Statistics for the multi-block execution.
#[derive(Debug)]
pub struct ExecutionStats {
    pub total_instruction_count: u64,
    pub nb_blocks: u64,
    pub nb_transactions: u64,
    pub total_gas_used: u64,
}

impl fmt::Display for ExecutionStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cycles_per_block = self.total_instruction_count / self.nb_blocks;
        let cycles_per_transaction = self.total_instruction_count / self.nb_transactions;
        let transactions_per_block = self.nb_transactions / self.nb_blocks;
        let gas_used_per_block = self.total_gas_used / self.nb_blocks;
        let gas_used_per_transaction = self.total_gas_used / self.nb_transactions;

        writeln!(
            f,
            "+--------------------------------+---------------------------+"
        )?;
        writeln!(f, "| {:<30} | {:<25} |", "Metric", "Value")?;
        writeln!(
            f,
            "+--------------------------------+---------------------------+"
        )?;
        writeln!(
            f,
            "| {:<30} | {:>25} |",
            "Total Instruction Count",
            self.total_instruction_count
                .to_formatted_string(&Locale::en)
        )?;
        writeln!(
            f,
            "| {:<30} | {:>25} |",
            "Total Blocks",
            self.nb_blocks.to_formatted_string(&Locale::en)
        )?;
        writeln!(
            f,
            "| {:<30} | {:>25} |",
            "Total Transactions",
            self.nb_transactions.to_formatted_string(&Locale::en)
        )?;
        writeln!(
            f,
            "| {:<30} | {:>25} |",
            "Cycles per Block",
            cycles_per_block.to_formatted_string(&Locale::en)
        )?;
        writeln!(
            f,
            "| {:<30} | {:>25} |",
            "Cycles per Transaction",
            cycles_per_transaction.to_formatted_string(&Locale::en)
        )?;
        writeln!(
            f,
            "| {:<30} | {:>25} |",
            "Transactions per Block",
            transactions_per_block.to_formatted_string(&Locale::en)
        )?;
        writeln!(
            f,
            "| {:<30} | {:>25} |",
            "Total Gas Used",
            self.total_gas_used.to_formatted_string(&Locale::en)
        )?;
        writeln!(
            f,
            "| {:<30} | {:>25} |",
            "Gas Used per Block",
            gas_used_per_block.to_formatted_string(&Locale::en)
        )?;
        writeln!(
            f,
            "| {:<30} | {:>25} |",
            "Gas Used per Transaction",
            gas_used_per_transaction.to_formatted_string(&Locale::en)
        )?;
        writeln!(
            f,
            "+--------------------------------+---------------------------+"
        )
    }
}
