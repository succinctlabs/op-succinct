# Self-Hosted Proving Cluster

By default, op-succinct uses the [Succinct Prover Network](https://docs.succinct.xyz/docs/sp1/prover-network/intro) to generate proofs. If you want to run your own proving infrastructure instead, you can deploy a self-hosted [SP1 cluster](https://github.com/succinctlabs/sp1-cluster) and point op-succinct at it.

## Prerequisites

- A deployed SP1 cluster passing the fibonacci smoke test. Follow the [SP1 Cluster Kubernetes Deployment Guide](https://docs.succinct.xyz/docs/provers/setup/deployment/kubernetes) to set one up.
- RPC endpoints for your OP Stack chain (L1, L1 beacon, L2, L2 node).
- A gRPC endpoint for the cluster API. Exposing `api-grpc` via a LoadBalancer or Ingress is recommended — `kubectl port-forward` works for quick tests but is unreliable for long-running proofs.

## Quick Test: Range Proof

Before integrating with the full proposer, verify that your cluster can generate op-succinct proofs using the `multi` script.

### 1. Set up environment

Create a `.env` file in the op-succinct root:

```env
L1_RPC=<YOUR_L1_RPC>
L1_BEACON_RPC=<YOUR_L1_BEACON_RPC>
L2_RPC=<YOUR_L2_RPC>
L2_NODE_RPC=<YOUR_L2_NODE_RPC>
```

### 2. Connect to cluster services

**Recommended:** Expose `api-grpc` via a LoadBalancer for stable, long-lived connections:

```bash
# Patch the service to use a LoadBalancer (e.g., AWS NLB)
kubectl patch svc api-grpc -n sp1-cluster -p '{"spec":{"type":"LoadBalancer"}}'

# Get the external endpoint
kubectl get svc api-grpc -n sp1-cluster -o jsonpath='{.status.loadBalancer.ingress[0].hostname}'
```

**Alternative (quick local testing only):** Use port-forward. Note that `kubectl port-forward` can drop under sustained load — the client polls the cluster every 50ms for the entire proving duration.

```bash
kubectl port-forward svc/api-grpc 50051:50051 -n sp1-cluster
```

### 3. Validate witness generation

Run without `--prove` first to verify RPC connectivity:

```bash
RUST_LOG=info cargo run --bin multi --release -- --env-file .env
```

This auto-detects the latest finalized L2 block range (default: 5 blocks).
Use `--default-range <N>` to change the range size, or `--start <BLOCK> --end <BLOCK>` for an exact range.

This executes locally and prints execution stats. If this fails, fix RPC issues before using cluster time.

### 4. Generate a range proof

**With S3 artifacts (recommended):**

```bash
SP1_PROVER=cluster \
CLI_CLUSTER_RPC=http://<CLUSTER_LB_ENDPOINT>:50051 \
CLI_S3_BUCKET=<YOUR_S3_BUCKET> \
CLI_S3_REGION=<YOUR_REGION> \
RUST_LOG=info \
cargo run --bin multi --release -- --prove --env-file .env
```

**With Redis artifacts:**

```bash
SP1_PROVER=cluster \
CLI_CLUSTER_RPC=http://<CLUSTER_LB_ENDPOINT>:50051 \
CLI_REDIS_NODES="redis://:<YOUR_REDIS_PASSWORD>@<REDIS_HOST>:6379/0" \
RUST_LOG=info \
cargo run --bin multi --release -- --prove --env-file .env
```

```admonish warning
Redis artifacts have a hardcoded 4-hour TTL. For large proofs that take longer, use S3 instead to avoid artifacts expiring mid-prove.
```

```admonish info
Start with a small range (5 blocks). A 5-block range proof typically completes in ~8 minutes on a single GPU worker.
```

A successful run produces output like:

```
INFO using s3 artifact store
INFO connecting to http://<CLUSTER_LB_ENDPOINT>:50051
INFO upload took 182ms, size: 2307656
INFO Successfully created proof request cli_<timestamp>
INFO Proof request for proof id cli_<timestamp> completed after ~475s
INFO Completed after ~475s
```

Proofs are saved to `data/<chain_id>/proofs/<start_block>-<end_block>.bin`.

## Running the Proposer

Before running a proposer, complete the relevant setup guide to deploy contracts and configure your environment:

- **Fault proofs**: [Quick Start Guide](../fault_proofs/quick_start.md)
- **Validity proofs**: [Contract Deployment](../validity/contracts/deploy.md)

Then add the following cluster variables to your proposer environment file (`.env` for [validity](../validity/proposer.md), `.env.proposer` for [fault proofs](../fault_proofs/proposer.md)):

```env
SP1_PROVER=cluster
CLI_CLUSTER_RPC=http://<CLUSTER_LB_ENDPOINT>:50051
CLI_S3_BUCKET=<YOUR_S3_BUCKET>
CLI_S3_REGION=<YOUR_REGION>
RUST_LOG=info
```

If using Redis instead of S3, replace `CLI_S3_BUCKET` + `CLI_S3_REGION` with `CLI_REDIS_NODES`.
You must set exactly one artifact store — setting both (or neither) will panic.

```admonish note
Self-hosted cluster mode is an alternative to the Succinct Prover Network. You do **not** need a `NETWORK_PRIVATE_KEY`.
Also there's no need to set any of `OP_SUCCINCT_MOCK=true` or `MOCK_MODE=true` — cluster mode uses real proving.
```

### Tuning for large proofs

For proofs that may take longer than 4 hours, increase the proving timeout:

```env
PROVING_TIMEOUT=21600  # 6 hours (default: 14400 = 4 hours)
```

This controls both the client-side timeout and the deadline sent to the cluster coordinator.

## Troubleshooting

### Proof request hangs

1. Verify `RUST_LOG=info` is set — without it, the CLI produces no output.
2. Check coordinator and worker logs:
   ```bash
   kubectl logs -l app=coordinator -n sp1-cluster
   kubectl logs -l app=gpu-node -n sp1-cluster
   ```
3. Verify the artifact store (S3 or Redis) is reachable from workers.

### "cluster proof failed" error

Check that `CLI_CLUSTER_RPC` is reachable and the API pod is running:

```bash
kubectl get pods -n sp1-cluster
kubectl logs -l app=api -n sp1-cluster
```

### Proof times out

The default `PROVING_TIMEOUT` is 14400 seconds (4 hours). If your proof needs more time (large block ranges, few GPU workers), increase it. The cluster also has a per-task timeout of 6 hours (`TASK_TIMEOUT`) on the worker node — individual shard tasks that exceed this are retried on another worker.
