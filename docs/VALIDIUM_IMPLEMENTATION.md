# Validium Implementation for OP Succinct

## Overview

Validium uses the OP Stack's **AltDA protocol** to store batch data off-chain
while posting only a **keccak256 commitment** (34 bytes) to L1 as calldata.

```
BATCHER                              L1 (Ethereum)
   │                                  │
   │   L1 Transaction calldata:       │
   │   [0x01] [0x00] [keccak256...]   │
   │    │      │      │               │
   │    │      │      └── 32-byte hash of batch_data
   │    │      └── commitment type (keccak256)
   │    └── AltDA version byte
   │                                  │
   │   Total: 34 bytes!               │
   │   (vs 128KB+ for blobs)          │
   └──────────────────────────────────►
```

## How It Works

```
1. SEQUENCER produces batch_data
   ├── Stores batch_data OFF-CHAIN
   └── Posts keccak256(batch_data) to L1 as calldata (34 bytes)

2. HOST prepares witness
   ├── Fetches L1 data (headers, receipts, batcher txs)
   ├── Fetches batch_data from OFF-CHAIN storage
   └── Creates ValidiumWitnessData

3. ZKVM runs derivation
   ├── Reads batcher txs from L1
   ├── Sees AltDA commitment (0x01 prefix)
   ├── Looks up data in ValidiumBlobStore
   ├── VERIFIES: keccak256(data) == commitment
   ├── Feeds data into derivation pipeline
   └── Executes blocks, verifies state
```

## Architecture

### Files (only in utils/validium/ and programs/range/validium/)

```
utils/validium/
├── client/src/
│   ├── blob_store.rs    # ValidiumBlobStore: hash-keyed batch data store
│   ├── da_source.rs     # ValidiumDADataSource: intercepts AltDA commitments
│   ├── executor.rs      # ValidiumWitnessExecutor
│   ├── witness.rs       # ValidiumWitnessData
│   └── lib.rs
└── host/src/
    ├── witness_generator.rs  # Host helpers
    └── lib.rs

programs/range/validium/src/
    └── main.rs               # zkVM entry point
```

### Data Flow

```
ValidiumDADataSource wraps EthereumDataSource:

EthereumDataSource.next()
        │
        ▼
  Read batcher tx calldata from L1
        │
        ▼
  data[0] == 0x01?  ──── NO ──── Pass through (normal data)
        │
       YES
        │
        ▼
  Extract commitment = data[2..34]
        │
        ▼
  ValidiumBlobStore.get_by_commitment(commitment)
        │
        ▼
  VERIFY: keccak256(batch_data) == commitment
        │
        ▼
  Return batch_data to pipeline
```

## Key Components

### ValidiumBlobStore (`blob_store.rs`)

Stores batch data keyed by keccak256 hash:

```rust
pub struct ValidiumBlobStore {
    store: BTreeMap<[u8; 32], Vec<u8>>,  // keccak256(data) → data
}

impl ValidiumBlobStore {
    pub fn get_by_commitment(&self, commitment: &B256) -> Option<Vec<u8>> {
        let data = self.store.get(&commitment.0)?;
        // Verify the commitment matches
        assert!(keccak256(data) == *commitment);
        Some(data.clone())
    }
}
```

### ValidiumDADataSource (`da_source.rs`)

Wraps `EthereumDataSource` and intercepts AltDA commitments:

```rust
impl DataAvailabilityProvider for ValidiumDADataSource<C, B> {
    async fn next(&mut self, block_ref, batcher_address) -> PipelineResult<Bytes> {
        let data = self.ethereum_source.next(block_ref, batcher_address).await?;

        // Check for AltDA commitment: [0x01] [0x00] [32 bytes keccak256]
        if data.len() == 34 && data[0] == 0x01 && data[1] == 0x00 {
            let commitment = B256::from_slice(&data[2..34]);
            let batch_data = self.blob_store.get_by_commitment(&commitment)?;
            return Ok(Bytes::from(batch_data));
        }

        Ok(data)  // Pass through normal data
    }
}
```

## Comparison

| | Ethereum DA | Celestia | Validium |
|---|---|---|---|
| L1 data | Full blobs (128KB) | Commitment pointer | keccak256 hash (34 bytes) |
| Off-chain | None | Celestia network | Your storage |
| Verification | KZG proofs | Blobstream | keccak256 |
| Cost | High | Medium | Low |

## Usage

### Host Side

```rust
use op_succinct_validium_host_utils::{
    create_validium_witness, create_validium_blob_data, witness_to_stdin,
};

// 1. Collect L1 preimages (same as Ethereum DA)
let preimage_store = /* from L1 */;
let l1_blob_data = /* L1 blobs, may be empty */;

// 2. Collect off-chain batch data
let batches: Vec<Vec<u8>> = fetch_from_offchain_storage();
let validium_data = create_validium_blob_data(batches);

// 3. Create witness and stdin
let witness = create_validium_witness(preimage_store, l1_blob_data, validium_data);
let stdin = witness_to_stdin(witness)?;
```

### Building

```bash
# Build the validium zkVM program
cd programs/range/validium
cargo prove build

# Build the proposer with validium feature
cargo build --release --bin proposer --features validium
```

## Deployment

### Prerequisites

1. **OP Stack chain** with AltDA enabled in the rollup config:
   ```json
   {
     "alt_da_config": {
       "da_challenge_window": 160,
       "da_resolve_window": 160,
       "commitment_type": "KeccakCommitment"
     }
   }
   ```

2. **Batcher** configured to post AltDA commitments (calldata with keccak256 hashes) instead of blobs.

3. **op-alt-da server** running to store/serve batch data.

### Environment Variables

```bash
# Standard OP Succinct variables
L1_RPC=https://...
L2_RPC=https://...
L2_NODE_RPC=https://...

# Validium-specific
ALT_DA_SERVER=http://localhost:3100
```

### Docker

```bash
cd fault-proof

# Start with docker-compose
docker compose -f docker-compose-validium.yml up -d
```

This starts:
- `op-alt-da-server` - stores/serves batch data
- `op-succinct-lite-proposer-validium` - proposes with validium DA
- `op-succinct-lite-challenger-validium` - challenges invalid proposals

### Manual

```bash
# 1. Start the op-alt-da server
docker run -p 3100:3100 us-docker.pkg.dev/oplabs-tools-artifacts/images/op-alt-da:latest

# 2. Export env vars
export ALT_DA_SERVER=http://localhost:3100
export L1_RPC=...
export L2_RPC=...

# 3. Run the proposer
cargo run --release --bin proposer --features validium
```
