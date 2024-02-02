CREATE TABLE IF NOT EXISTS block (
    "hash" VARCHAR(64),
    "parent_hash" VARCHAR(64),
    "block_height" int,
    "transaction_count" int,
    PRIMARY KEY(hash)
);
