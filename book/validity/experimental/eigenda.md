# EigenDA

This section describes the requirements to use OP Succinct for a chain with EigenDA. The requirements are additive to the ones required for the `op-succinct` service. Please refer to the [Proposer](../proposer.md) section for the base configuration.

## Environment Setup

To use EigenDA, you need additional environment variables in the `.env` file:

| Parameter | Description |
|-----------|-------------|
| `EIGENDA_PROXY_ADDRESS` | Address of the EigenDA proxy, a sidecar server ran as part of a rollup node cluster for communication with the EigenDA network. For more details, see [EigenDA's documentation on the proxy](https://docs.eigenda.xyz/integrations-guides/eigenda-proxy). |

## Run the EigenDA Proposer Service

Run the `op-succinct-eigenda` service.

```bash
docker compose -f docker-compose-eigenda.yml up -d
```

To see the logs of the `op-succinct-eigenda` service, run:

```bash
docker compose -f docker-compose-eigenda.yml logs -f
```

To stop the `op-succinct-eigenda` service, run:

```bash
docker compose -f docker-compose-eigenda.yml down
```

## Deploying `OPSuccinctL2OutputOracle` with EigenDA features

```bash
just deploy-oracle .env eigenda
```

## Updating `OPSuccinctL2OutputOracle` Parameters

```bash
just update-parameters .env eigenda
```

For more details on the `just update-parameters` command, see the [Updating `OPSuccinctL2OutputOracle` Parameters](../contracts/update-parameters.md) section.
