# Upgrade OP Succinct

This guide is for upgrading OP Succinct to a new version.

## Overview

OP Succinct requires an upgrade whenever there is either 1) a new hard fork of the OP Stack or 2) a new version of the `op-succinct` binary.

Check out the latest release of `op-succinct` [here](https://github.com/succinctlabs/op-succinct/releases).

### Upgrade Docker Containers

If you're using Docker, first upgrade your containers to use the latest version of `op-succinct`.

### Upgrade Contract

Optimism's contracts make use of an upgradeable proxy pattern where a modification for a storage variable requires a contract upgrade. 

To upgrade the contracts, check out the latest release of `op-succinct` and follow the instructions [here](../advanced/l2-output-oracle.md#upgrading-opsuccinctl2outputoracle). The version of the `OPSuccinctL2OutputOracle` contract will be bumped, along with the version in the initializer tag.
