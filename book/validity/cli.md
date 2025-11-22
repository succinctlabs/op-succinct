# OP Succinct CLI

## Overview

OP Succinct CLI is a command-line interface for interacting with the OP Succinct proof requests database. It allows to query or update proof requests.

## Installing

```bash
cargo install --path ./cli
```

## Usage

```bash
op-succinct-cli <command>
```

### Commands

op-succinct-cli supports the following commands:

| command | Description |
|-----------|-------------|
| `list` | List proof requests. |
| `split` | Split a proof request in two. |
| `join` | Join 2 proof requests consecutives ranges. |
| `kill` | Set a proof request to failed. |

You can get more details about each command arguments with:

```bash
op-succinct-cli <command> help
```

### Environment Setup

By default, env variables can be retrieved from a `.env` file if it exists. But it is possible specify the path to another file with the `--env-file` argument.

#### Required Environment Variables

The following environment variable is required to query the database.

| Parameter | Description |
|-----------|-------------|
| `DATABASE_URL` | The address of a Postgres database for querying or updating proof requests. |

### Optional Environment Variables

The following environment variables are only required to update the database.

| Parameter | Description |
|-----------|-------------|
| `L1_RPC` | L1 Archive Node. |
| `L2_RPC` | L2 Execution Node (`op-geth`). |
| `L2_NODE_RPC` | L2 Rollup Node (`op-node`). |
