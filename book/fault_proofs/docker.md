# OP Succinct Fault Proof Docker Setup

This guide explains how to run the OP Succinct fault proof system using Docker Compose.

## Prerequisites

- Docker and Docker Compose installed
- L1 and L2 RPC endpoints available
- Private key with sufficient ETH for bonds and transaction fees
- Deployed DisputeGameFactory contract address (see [Deploy Guide](./deploy.md))

## Docker Setup Overview

The fault proof system consists of two main components:
- **Proposer**: Creates and defends claims about L2 state. See `fault-proof/Dockerfile.proposer`.
- **Challenger**: Monitors and challenges invalid claims. See `fault-proof/Dockerfile.challenger`.

## Quick Start

1. Create environment files under the `fault-proof` directory for both components:

    ### Proposer Configuration (.env.proposer)

    See [Proposer Configuration](./proposer.md#configuration) for more details.

    ### Challenger Configuration (.env.challenger)

    See [Challenger Configuration](./challenger.md#configuration) for more details.

2. Start the services:

```bash
# Navigate to the fault-proof directory
cd fault-proof

# Start both proposer and challenger
docker compose up -d

# Or start them individually
docker compose up -d proposer
docker compose up -d challenger
```

## Monitoring

The fault proof system includes a Grafana dashboard for monitoring the proposer and challenger.
To access the dashboard, open your browser and navigate to `http://localhost:3000`. Use the following credentials:
- Username: `admin`
- Password: `admin`

If the default port 9090 and 3000 for Prometheus and Grafana are already in use, you can change the port by setting the `FP_PROMETHEUS_PORT` and `FP_GRAFANA_PORT` environment variables in fault-proof/.env file.

View logs for the services:

```bash
# View logs for both services.
docker compose logs -f

# View logs for a specific service.
docker logs -f op-succinct-fp-proposer
docker logs -f op-succinct-fp-challenger
```

## Stopping the Services

```bash
docker compose down
```

## Building Images Manually

If you need to build the Docker images manually:

```bash
# Navigate to the fault-proof directory
cd fault-proof

# Build the proposer image
docker build -f Dockerfile.proposer -t op-succinct-fp-proposer:latest ..

# Build the challenger image
docker build -f Dockerfile.challenger -t op-succinct-fp-challenger:latest ..
```

## Troubleshooting

If you encounter issues:

1. Check the logs for error messages.
2. Verify your RPC endpoints are accessible.
3. Ensure your private key has sufficient ETH.
4. Confirm the factory address and contract deployment environment variables were set as expected.
5. Check that the Docker images built successfully with `docker images`.

For more detailed information, refer to the documentation:
- [Deployment Guide](./deploy.md)
- [Proposer Documentation](./proposer.md)
- [Challenger Documentation](./challenger.md)