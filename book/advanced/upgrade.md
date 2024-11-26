# Upgrade OP Succinct

This guide is for upgrading OP Succinct to a new version.

There are two main components that need to be upgraded for a new version of `op-succinct`:

1. The `OPSuccinctL2OutputOracle` contract.
2. The `op-succinct` Docker images.

## Overview

OP Succinct requires an upgrade whenever there is either 1) a new hard fork of the OP Stack or 2) a new version of the `op-succinct` binary.

Some upgrades require upgrading the `OPSuccinctL2OutputOracle` contract (whenever the OP Succinct program has updated/the `OPSuccinctL2OutputOracle` contract has been modified), while other upgrades only require upgrading the `op-succinct` Docker images.

### Upgrade Docker Containers

If you're using Docker, upgrade your containers to use the latest version of `op-succinct` by checking out the [latest release](https://github.com/succinctlabs/op-succinct/releases). We do not build Docker images for releases, but we support a `docker compose` setup for the latest version of `op-succinct`.

### Upgrade Contract

Optimism's contracts make use of an upgradeable proxy pattern.

As of release `v0.1.0`, the `OPSuccinctL2OutputOracle` contract is upgradeable.

To upgrade the contracts, check out the latest release of `op-succinct` and follow the instructions [here](../advanced/l2-output-oracle.md#upgrading-opsuccinctl2outputoracle). The version of the `OPSuccinctL2OutputOracle` contract will be bumped, along with the version in the initializer tag.
