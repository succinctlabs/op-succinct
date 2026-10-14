-- Records the L1 identity used by range proofs and logical invalidation after an L1 reorg.
ALTER TABLE requests
ADD COLUMN l1_head_block_hash BYTEA,
ADD COLUMN invalidated_at TIMESTAMP;
