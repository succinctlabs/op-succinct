# Validium Data Availability

Validium uses the OP Stack's **AltDA protocol**: the batcher posts only a
**keccak256 hash** (34 bytes of calldata) to L1, while batch data is stored off-chain.

## L1 Calldata Format

```
[0x01] [0x00] [32 bytes keccak256 hash]
  │      │      └── keccak256(batch_data)
  │      └── commitment type (keccak256)
  └── AltDA version byte

Total: 34 bytes on L1
```

## How It Works

1. Batcher stores batch data off-chain
2. Batcher posts `keccak256(batch_data)` to L1 as calldata (34 bytes)
3. zkVM reads batcher txs from L1, sees the commitment
4. zkVM looks up the actual data from the witness
5. zkVM **verifies**: `keccak256(data) == commitment`
6. Data fed into the derivation pipeline, blocks executed

## Security

- L1 anchoring preserved (headers, deposits, ordering, finality)
- Commitment verified inside zkVM: `keccak256(data) == on-chain hash`
- Prover cannot use fake data (hash wouldn't match)

## Crates

| Crate | Purpose |
|-------|---------|
| `op-succinct-validium-client-utils` | ValidiumDADataSource, ValidiumBlobStore |
| `op-succinct-validium-host-utils` | Host utilities |
| `validium-program` | zkVM program |

## Building

```bash
cd programs/range/validium
cargo prove build
```

## Cost Comparison

| Mode | L1 Data | Approximate Cost |
|------|---------|-----------------|
| Ethereum DA (blobs) | 128KB blob | ~131K gas |
| Calldata | ~100KB calldata | ~1.6M gas |
| **Validium** | **34 bytes calldata** | **~550 gas** |
