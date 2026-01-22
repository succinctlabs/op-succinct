#!/usr/bin/env bash
set -euo pipefail

# Script to generate SP1Stdin files for hypercube benchmarks and upload to S3.
#
# Usage:
#   ./collect-data.sh --sp1-version v5
#
# Prerequisites:
#   - .env file with L1_RPC, L2_RPC, L2_NODE_RPC configured
#   - AWS CLI configured with credentials

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OUTPUT_DIR="${SCRIPT_DIR}/hypercube-benches"
S3_BUCKET="sp1-testing-suite"
NETWORK="op-mainnet"

# Parse arguments
SP1_VERSION=""
ENV_FILE=".env"

while [[ $# -gt 0 ]]; do
    case $1 in
        --sp1-version)
            SP1_VERSION="$2"
            shift 2
            ;;
        --env-file)
            ENV_FILE="$2"
            shift 2
            ;;
        --help|-h)
            echo "Usage: $0 --sp1-version <version> [--env-file <path>]"
            echo ""
            echo "Options:"
            echo "  --sp1-version    SP1 version for S3 path (e.g., v5)"
            echo "  --env-file       Path to .env file (default: .env)"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Validate required arguments
if [[ -z "$SP1_VERSION" ]]; then
    echo "Error: --sp1-version is required"
    echo "Usage: $0 --sp1-version <version>"
    exit 1
fi

# Check .env file exists
if [[ ! -f "$ENV_FILE" ]]; then
    echo "Error: Environment file not found: $ENV_FILE"
    echo "Please create a .env file with L1_RPC, L2_RPC, and L2_NODE_RPC configured."
    exit 1
fi

# Check AWS credentials
if ! aws sts get-caller-identity &>/dev/null; then
    echo "Error: AWS credentials not configured or invalid."
    echo "Please run 'aws configure' or set AWS_ACCESS_KEY_ID and AWS_SECRET_ACCESS_KEY."
    exit 1
fi

echo "Configuration:"
echo "  SP1 Version: $SP1_VERSION"
echo "  Env File: $ENV_FILE"
echo "  Output Dir: $OUTPUT_DIR"
echo "  S3 Target: s3://$S3_BUCKET/hypercube-benches/$SP1_VERSION/$NETWORK/"
echo ""

# Clean up previous output
rm -rf "$OUTPUT_DIR"
mkdir -p "$OUTPUT_DIR"

# Run the Rust binary to generate SP1Stdin files
echo "Generating SP1Stdin files..."
RUST_LOG=info cargo run --release -p op-succinct-scripts --bin gen-hypercube-benches -- \
    --env-file "$ENV_FILE" \
    --output-dir "$OUTPUT_DIR"

# Check if files were generated
if [[ ! "$(ls -A "$OUTPUT_DIR"/*.bin 2>/dev/null)" ]]; then
    echo "Error: No .bin files generated in $OUTPUT_DIR"
    exit 1
fi

# Upload to S3
S3_PATH="s3://$S3_BUCKET/hypercube-benches/$SP1_VERSION/$NETWORK/"
echo ""
echo "Uploading files to $S3_PATH..."

for file in "$OUTPUT_DIR"/*.bin; do
    filename=$(basename "$file")
    echo "  Uploading $filename..."
    aws s3 cp "$file" "$S3_PATH$filename"
done

echo ""
echo "Upload complete!"
echo ""
echo "Verify with:"
echo "  aws s3 ls $S3_PATH"
