-- Migration script to create the requests and eth_metrics tables

-- Create requests table
CREATE TABLE IF NOT EXISTS requests (
    id BIGSERIAL PRIMARY KEY,
    status SMALLINT NOT NULL,
    req_type SMALLINT NOT NULL,
    mode SMALLINT NOT NULL,
    start_block BIGINT NOT NULL,
    end_block BIGINT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    proof_request_id BYTEA,
    checkpointed_l1_block_number BIGINT,
    checkpointed_l1_block_hash BYTEA,
    execution_statistics JSONB NOT NULL DEFAULT 'null'::jsonb,
    witnessgen_duration BIGINT,
    execution_duration BIGINT,
    proof_request_time TIMESTAMP,
    prove_duration BIGINT,
    range_vkey_commitment BYTEA NOT NULL,
    aggregation_vkey_hash BYTEA,
    rollup_config_hash BYTEA NOT NULL,
    relay_tx_hash BYTEA,
    proof BYTEA,
    total_nb_transactions BIGINT NOT NULL,
    total_eth_gas_used BIGINT NOT NULL,
    total_l1_fees NUMERIC(38,0) NOT NULL,
    total_tx_fees NUMERIC(38,0) NOT NULL,
    l1_chain_id BIGINT NOT NULL,
    l2_chain_id BIGINT NOT NULL,
    contract_address BYTEA
);
