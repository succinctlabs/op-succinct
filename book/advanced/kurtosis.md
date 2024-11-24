# Kurtosis

To test `op-succinct` on a local devnet, you can use Kurtosis.

First, install Kurtosis: https://docs.kurtosis.com/install/

https://github.com/ethpandaops/optimism-package/tree/main/.github/tests

Then, configure the `op-network.yaml` file to use the Kurtosis engine:

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

Run the testnet: `kurtosis run --enclave my-testnet github.com/ethpandaops/optimism-package --args-file op-network.yaml`
