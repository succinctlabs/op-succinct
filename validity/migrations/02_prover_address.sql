-- Add prover_address column to requests table
ALTER TABLE requests ADD COLUMN prover_address BYTEA NOT NULL DEFAULT '\x'::bytea;