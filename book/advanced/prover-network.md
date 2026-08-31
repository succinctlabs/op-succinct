# Prover Network Connection

OP Succinct uses HTTPS to connect to the [Succinct Prover Network](https://docs.succinct.xyz/docs/sp1/prover-network/intro).

## Mutual TLS Client Authentication

Some private endpoints require mutual authentication with Transport Layer Security (mTLS).
With mTLS, the server and client verify certificates during connection setup.
These settings apply only when OP Succinct uses the Prover Network.

Set `NETWORK_RPC_URL` to the HTTPS endpoint.
Then set both client identity variables:

| Variable | Description |
|---|---|
| `NETWORK_MTLS_CERT_PATH` | Path to the PEM-encoded client certificate chain. |
| `NETWORK_MTLS_KEY_PATH` | Path to the matching PEM-encoded private key. |

Set both variables, or leave both variables unset.
OP Succinct stops if a path is empty or unreadable.
It also stops if the certificate and private key are invalid.

Store the private key in a mounted secret.
Do not commit the key or copy it into a container image.

When mTLS is enabled, OP Succinct tests the connection with a 20-second timeout.
The proposer runs this test during startup, before proof setup and witness generation.
The prove scripts run this test when they create the network client.

When both variables are unset, OP Succinct uses its standard HTTPS connection.
