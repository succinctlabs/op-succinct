# Kurtosis

## What is Kurtosis?

Kurtosis is a tool that allows you to test `op-succinct` on a local devnet.

## Install Kurtosis

First, install Kurtosis by following the instructions here: [Kurtosis Installation Guide](https://docs.kurtosis.com/install/).

## How to configure Kurtosis

Configure the `op-network.yaml` file to use the Kurtosis engine:

```yaml
optimism_package:
  chains:
    - participants:
        - el_type: op-geth
          cl_type: op-node
      network_params:
        fjord_time_offset: 0
        granite_time_offset: 0
        holocene_time_offset: 0
      additional_services:
        - blockscout
ethereum_package:
  participants:
    - el_type: geth
    - el_type: reth
  network_params:
    preset: minimal
  additional_services:
    - blockscout
```

## How to run Kurtosis?

Run the testnet using the following command:

```bash
kurtosis run --enclave my-testnet github.com/ethpandaops/optimism-package@bbusa/add-miner --args-file op-network.yaml --image-download always
```

Note: We're currently using the `bbusa/add-miner` branch of the `optimism-package` repo because it has a fix for the `op-batcher` container.

## How to get the relevant RPC's from Kurtosis?

To get the relevant RPC endpoints (`L1_RPC`, `L2_RPC`, `L1_BEACON_RPC`, `L2_NODE_RPC`) from Kurtosis for `op-succinct`, you can use the following commands:

```bash
kurtosis logs -a
```

## Clean up

Remove the testnet with:

```bash
kurtosis clean -a
```