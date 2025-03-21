# OP Succinct

<a href="https://github.com/succinctlabs/op-succinct/actions/workflows/docker-build.yaml"><img src="https://img.shields.io/github/actions/workflow/status/succinctlabs/op-succinct/docker-build.yaml?style=flat&labelColor=1C2C2E&label=ci&color=BEC5C9&logo=GitHub%20Actions&logoColor=BEC5C9" alt="CI"></a>
   <a href="https://github.com/succinctlabs/op-succinct/blob/main/LICENSE-MIT"><img src="https://img.shields.io/badge/License-MIT-d1d1f6.svg?style=flat&labelColor=1C2C2E&color=BEC5C9&logo=googledocs&label=license&logoColor=BEC5C9" alt="License"></a>
   <a href="https://succinctlabs.github.io/op-succinct"><img src="https://img.shields.io/badge/Book-854a15?style=flat&labelColor=1C2C2E&color=BEC5C9&logo=mdBook&logoColor=BEC5C9" alt="Book"></a>

## Overview

OP Succinct is the production-grade proving engine for the OP Stack, powered by Succinct.

### Key Benefits

- **1 hour finality** secured by ZKPs, a dramatic improvement over the 7-day withdrawal window of standard OP Stack rollups.
- **Unlimited customization** for rollup modifications in pure Rust and easy maintainability.
- **Cost-effective proving** with an average cost of proving only fractions of cent per transaction (with an expected 5-10x improvement by EOY), thanks to SP1's blazing fast performance.

## OP Stack Proving Options

| Feature | OP Succinct | OP Succinct Lite | Standard OP Stack |
|---------|-------------|------------------|-------------------|
| Proof System | Full validity proofs with SP1 | ZK fault proofs (with optional fast finality) | Interactive fraud proofs |
| Decentralization | Stage 1 - Stage 2 | Configurable: Stage 0 or Stage 1 | Stage 1 |
| Finality Time | 1 hour | 1 day | 7 days |
| Security Level | Highest | High | High |
| Alt DA Support | ✅ | ✅ | ❌ |
| Dispute Process | Prove every transcation | Prove when there is a dispute | Replay transactions on L1 with FPVM |
| Dispute Capital Requirements | 0 | Configurable: 5-15 ETH (Less than worst-case prover cost) | Scales with TVL; up to 1000s of ETH |
| Ongoing proving costs | Less than a tenth of a cent per transaction; can be paid by users | $0; Proofs only generated with disputes | $0 |
| Designed for | Large L2s, DeFi Chains, and other high-value chains | Cost-sensitive L2s, appchains and gaming chains | 🐢 |


## Getting Started

1. Check the [Prerequisites](./quick-start/prerequisites.md) to ensure your environment is ready.
2. Run the [Cost Estimator](./quick-start/cost-estimator.md) to understand the resource requirements for proving.
3. Try [OP Succinct in Mock Mode](./quick-start/mock.md) for development.
4. Deploy [OP Succinct in Full Mode](./quick-start/full.md) for production.

## Support and Community

All of this has been possible thanks to close collaboration with the core team at [OP Labs](https://www.oplabs.co/).

**Ready to upgrade your rollup? [Contact us](https://docs.google.com/forms/d/e/1FAIpQLSd2Yil8TrU54cIuohH1WvDvbxTusyqh5rsDmMAtGC85-Arshg/viewform?ref=https://succinctlabs.github.io/op-succinct/) to get started with a Type-1 zkEVM rollup powered by SP1.**

## Documentation Structure

- **Quick Start**: Get up and running quickly with basic setup and configuration.
- **Advanced**: Detailed guides for production deployment and maintenance.
- **Contracts**: `OPSuccinctL2OutputOracle` contract documentation and deployment guides.
- **FAQ & Troubleshooting**: Common issues and their solutions.
