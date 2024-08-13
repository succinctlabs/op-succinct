# Run OP Proposer

## Instructions 

Once you've deployed an `L2OutputOracle` contract, you can run the OP Proposer to generate proofs.

First, create a `.env.server` file that matches the `.env.server.example` file.

```bash
cp .env.server.example .env.server
```

Then, run the OP Proposer + SP1 server:

1. Start a server and an OP Proposer.
2. The OP Proposer will:
   a. Query the current state of the `L2OutputOracle` contract.
   b. Request proofs from the server.
3. The server will:
   a. Send requests to the prover network.
   b. Relay the responses back to the OP Proposer.
4. Then, the OP Proposer will:
   a. Verify the proofs.
   b. Submit the proofs to the `L2OutputOracle` contract.

To build the server, run:
```bash
docker compose build
```

To start the OP Proposer, run:
```bash
docker compose up -d
```

To stop the OP Proposer, run:
```bash
docker compose down
```

