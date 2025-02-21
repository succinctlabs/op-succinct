-- Add chain_id and contract_address columns to requests table. Has default values for Phala (only chain in the table when migrating).
ALTER TABLE requests
ADD COLUMN l1_chain_id BIGINT NOT NULL DEFAULT 1,
ADD COLUMN l2_chain_id BIGINT NOT NULL DEFAULT 2035,
ADD COLUMN contract_address BYTEA;

-- Create index on chain_id for faster lookups
CREATE INDEX idx_requests_chain_id ON requests (l1_chain_id, l2_chain_id);

-- Create composite index including chain_id
CREATE INDEX idx_requests_chain_id_status ON requests (l1_chain_id, l2_chain_id, status); 