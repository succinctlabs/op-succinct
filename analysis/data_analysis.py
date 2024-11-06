import pandas as pd
import argparse

def analyze_block_data(file_path, block_time=2, eth_price=2400):
    # Read CSV file
    print("Reading CSV file...")
    df = pd.read_csv(file_path)
    print("Done reading file.")

    # Constants
    GAS_PER_BLOCK = 43863

    # Calculations
    number_of_blocks = len(df)
    total_seconds = number_of_blocks * block_time
    total_hours = total_seconds / 3600

    total_gas = df['gas_used'].sum()
    real_gas = total_gas - (number_of_blocks * GAS_PER_BLOCK)

    total_transactions = df['transaction_count'].sum()
    real_transactions = total_transactions - number_of_blocks

    total_l1_fees = (df['total_l1_fees'] / 1e18).sum()  # Convert to Ether
    total_tx_fees = (df['total_tx_fees'] / 1e18).sum()  # Convert to Ether

    total_l1_fees_usd = total_l1_fees * eth_price
    total_tx_fees_usd = total_tx_fees * eth_price

    # Per Second calculations
    mgas_per_sec = (total_gas / 1e6) / total_seconds
    real_mgas_per_sec = (real_gas / 1e6) / total_seconds
    tps = total_transactions / total_seconds
    real_tps = real_transactions / total_seconds

    l1_fees_per_sec = total_l1_fees / total_seconds
    tx_fees_per_sec = total_tx_fees / total_seconds
    l1_fees_per_sec_usd = l1_fees_per_sec * eth_price
    tx_fees_per_sec_usd = tx_fees_per_sec * eth_price
    l1_fees_per_month = l1_fees_per_sec * 24 * 60 * 60 * 30
    tx_fees_per_month = tx_fees_per_sec * 24 * 60 * 60 * 30

    # Per Transaction calculations
    mgas_per_tx = (total_gas / 1e6) / total_transactions
    l1_fees_per_tx = total_l1_fees / total_transactions
    tx_fees_per_tx = total_tx_fees / total_transactions
    l1_fees_per_tx_usd = l1_fees_per_tx * eth_price
    tx_fees_per_tx_usd = tx_fees_per_tx * eth_price

    # Per Real Transaction calculations
    mgas_per_rtx = (real_gas / 1e6) / real_transactions if real_transactions != 0 else 0
    l1_fees_per_rtx = total_l1_fees / real_transactions if real_transactions != 0 else 0
    tx_fees_per_rtx = total_tx_fees / real_transactions if real_transactions != 0 else 0
    l1_fees_per_rtx_usd = l1_fees_per_rtx * eth_price
    tx_fees_per_rtx_usd = tx_fees_per_rtx * eth_price

     # Formatted terminal output
    print("\n--- Blockchain Data Analysis ---\n")
    print(f"{'Number Blocks':<25}{number_of_blocks}")
    print(f"{'Total Seconds':<25}{total_seconds}")
    print(f"{'Total Hours':<25}{total_hours:.4f}\n")

    print(f"{'Total Gas':<25}{total_gas}")
    print(f"{'Total Real Gas':<25}{real_gas}")
    print(f"{'Total Transactions':<25}{total_transactions}")
    print(f"{'Total Real Transactions':<25}{real_transactions}")
    print(f"{'Total L1 Fees':<25}{total_l1_fees:.8f} ETH")
    print(f"{'Total Tx Fees':<25}{total_tx_fees:.8f} ETH")
    print(f"{'Total L1 Fees (USD)':<25}${total_l1_fees_usd:,.2f}")
    print(f"{'Total Tx Fees (USD)':<25}${total_tx_fees_usd:,.2f}\n")

    print(f"{'MGas/sec':<25}{mgas_per_sec:.8f}")
    print(f"{'Real MGas/sec':<25}{real_mgas_per_sec:.8f}")
    print(f"{'TPS':<25}{tps:.8f}")
    print(f"{'Real TPS':<25}{real_tps:.8f}")
    print(f"{'L1 Fees/sec':<25}{l1_fees_per_sec:.8f} ETH")
    print(f"{'Tx Fees/sec':<25}{tx_fees_per_sec:.8f} ETH")
    print(f"{'L1 Fees/sec (USD)':<25}${l1_fees_per_sec_usd:.4f}")
    print(f"{'Tx Fees/sec (USD)':<25}${tx_fees_per_sec_usd:.4f}\n")
    print(f"{'L1 Fees/month':<25}{l1_fees_per_month:.8f} ETH")
    print(f"{'Tx Fees/month':<25}{tx_fees_per_month:.8f} ETH")

    print("\n--- Per Transaction Calculations ---\n")
    print(f"{'MGas/Tx':<25}{mgas_per_tx:.8f}")
    print(f"{'L1 Fees/Tx':<25}{l1_fees_per_tx:.8f} ETH")
    print(f"{'Tx Fees/Tx':<25}{tx_fees_per_tx:.8f} ETH")
    print(f"{'L1 Fees/Tx (USD)':<25}${l1_fees_per_tx_usd:.4f}")
    print(f"{'Tx Fees/Tx (USD)':<25}${tx_fees_per_tx_usd:.4f}\n")

    print("\n--- Per Real Transaction Calculations ---\n")
    print(f"{'MGas/RTx':<25}{mgas_per_rtx:.8f}")
    print(f"{'L1 Fees/RTx':<25}{l1_fees_per_rtx:.8f} ETH")
    print(f"{'Tx Fees/RTx':<25}{tx_fees_per_rtx:.8f} ETH")
    print(f"{'L1 Fees/RTx (USD)':<25}${l1_fees_per_rtx_usd:.4f}")
    print(f"{'Tx Fees/RTx (USD)':<25}${tx_fees_per_rtx_usd:.4f}\n")

# Example usage
if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Analyze blockchain data from a CSV file.")
    parser.add_argument("file_path", type=str, help="Path to the CSV file containing block data.")
    parser.add_argument("--block_time", type=int, default=2, help="Block time in seconds (default: 2).")
    parser.add_argument("--eth_price", type=float, default=2400, help="Ethereum price in USD (default: 2400).")

    args = parser.parse_args()
    analyze_block_data(args.file_path, args.block_time, args.eth_price)

