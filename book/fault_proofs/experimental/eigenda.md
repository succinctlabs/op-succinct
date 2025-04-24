# EigenDA Data Availability

This section describes the requirements to use OP Succinct Lite for a chain with EigenDA. The requirements are additive to the ones required for the OP Succinct Lite. Please refer to the [Proposer](../proposer.md) section for the base configuration. Also, please refer to the [Docker Setup](../docker.md) section for details on how to run the OP Succinct Lite with Docker.

## Environment Setup

To use EigenDA, you need additional environment variables in the `.env` file:

| Parameter | Description |
|-----------|-------------|
| `EIGENDA_PROXY_ADDRESS` | Address of the EigenDA proxy, a sidecar server ran as part of a rollup node cluster for communication with the EigenDA network. For more details, see [EigenDA's documentation on the proxy](https://docs.eigenda.xyz/integrations-guides/eigenda-proxy). |

## Run Services with EigenDA

```bash
# Navigate to the fault_proof directory
cd fault_proof

# Start both proposer and challenger
docker compose -f docker-compose-eigenda.yml up -d
```

To see the logs, run:

```bash
docker compose -f docker-compose-eigenda.yml logs -f
```

To stop the services, run:

```bash
docker compose -f docker-compose-eigenda.yml down
```
