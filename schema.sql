CREATE TABLE spl_token_initialize_mint_events
(
    signature VARCHAR(88) CODEC(LZ4),
    instruction_index UInt32,
    transaction_index UInt32,
    parent_instruction_program_id VARCHAR(44) CODEC(LZ4),
    top_instruction_program_id VARCHAR(44) CODEC(LZ4),
    slot UInt64,
    mint VARCHAR(44) CODEC(LZ4),
    decimals UInt32,
    mint_authority VARCHAR(44) CODEC(LZ4),
    freeze_authority VARCHAR(44) CODEC(LZ4),
)
ENGINE = MergeTree
PRIMARY KEY (signature, instruction_index);

CREATE TABLE spl_token_initialize_account_events
(
    signature VARCHAR(88) CODEC(LZ4),
    instruction_index UInt32,
    transaction_index UInt32,
    parent_instruction_program_id VARCHAR(44) CODEC(LZ4),
    top_instruction_program_id VARCHAR(44) CODEC(LZ4),
    slot UInt64,
    account_address VARCHAR(44) CODEC(LZ4),
    account_owner VARCHAR(44) CODEC(LZ4),
    mint VARCHAR(44) CODEC(LZ4),
)
ENGINE = MergeTree
PRIMARY KEY (signature, instruction_index);

CREATE TABLE spl_token_initialize_multisig_events
(
    signature VARCHAR(88) CODEC(LZ4),
    instruction_index UInt32,
    transaction_index UInt32,
    parent_instruction_program_id VARCHAR(44) CODEC(LZ4),
    top_instruction_program_id VARCHAR(44) CODEC(LZ4),
    multisig VARCHAR(44) CODEC(LZ4),
    signers Array(VARCHAR(44)) CODEC(LZ4),
)
ENGINE = MergeTree
PRIMARY KEY (signature, instruction_index);

CREATE TABLE spl_token_transfer_events
(
    signature VARCHAR(88) CODEC(LZ4),
    instruction_index UInt32,
    transaction_index UInt32,
    parent_instruction_program_id VARCHAR(44) CODEC(LZ4),
    top_instruction_program_id VARCHAR(44) CODEC(LZ4),
    slot UInt64,
    source_address VARCHAR(44) CODEC(LZ4),
    source_owner VARCHAR(44) CODEC(LZ4),
    destination_address VARCHAR(44) CODEC(LZ4),
    destination_owner VARCHAR(44) CODEC(LZ4),
    mint VARCHAR(44) CODEC(LZ4),
    amount UInt64,
    authority VARCHAR(44) CODEC(LZ4),
)
ENGINE = MergeTree
PRIMARY KEY (signature, instruction_index);

CREATE TABLE spl_token_approve_events
(
    signature VARCHAR(88) CODEC(LZ4),
    instruction_index UInt32,
    transaction_index UInt32,
    parent_instruction_program_id VARCHAR(44) CODEC(LZ4),
    top_instruction_program_id VARCHAR(44) CODEC(LZ4),
    slot UInt64,
    source_address VARCHAR(44) CODEC(LZ4),
    source_owner VARCHAR(44) CODEC(LZ4),
    mint VARCHAR(44) CODEC(LZ4),
    delegate VARCHAR(44) CODEC(LZ4),
    amount UInt64,
)
ENGINE = MergeTree
PRIMARY KEY (signature, instruction_index);

CREATE TABLE spl_token_revoke_events
(
    signature VARCHAR(88) CODEC(LZ4),
    instruction_index UInt32,
    transaction_index UInt32,
    parent_instruction_program_id VARCHAR(44) CODEC(LZ4),
    top_instruction_program_id VARCHAR(44) CODEC(LZ4),
    slot UInt64,
    source_address VARCHAR(44) CODEC(LZ4),
    source_owner VARCHAR(44) CODEC(LZ4),
    mint VARCHAR(44) CODEC(LZ4),
)
ENGINE = MergeTree
PRIMARY KEY (signature, instruction_index);

CREATE TABLE spl_token_set_authority_events
(
    signature VARCHAR(88) CODEC(LZ4),
    instruction_index UInt32,
    transaction_index UInt32,
    parent_instruction_program_id VARCHAR(44) CODEC(LZ4),
    top_instruction_program_id VARCHAR(44) CODEC(LZ4),
    slot UInt64,
    mint VARCHAR(44) CODEC(LZ4),
    authority_type VARCHAR(14) CODEC(LZ4),
    new_authority VARCHAR(44) CODEC(LZ4),
)
ENGINE = MergeTree
PRIMARY KEY (signature, instruction_index);

CREATE TABLE spl_token_mint_to_events
(
    signature VARCHAR(88) CODEC(LZ4),
    instruction_index UInt32,
    transaction_index UInt32,
    parent_instruction_program_id VARCHAR(44) CODEC(LZ4),
    top_instruction_program_id VARCHAR(44) CODEC(LZ4),
    slot UInt64,
    destination_address VARCHAR(44) CODEC(LZ4),
    destination_owner VARCHAR(44) CODEC(LZ4),
    mint VARCHAR(44) CODEC(LZ4),
    mint_authority VARCHAR(44) CODEC(LZ4),
    amount UInt64,
)
ENGINE = MergeTree
PRIMARY KEY (signature, instruction_index);

CREATE TABLE spl_token_burn_events
(
    signature VARCHAR(88) CODEC(LZ4),
    instruction_index UInt32,
    transaction_index UInt32,
    parent_instruction_program_id VARCHAR(44) CODEC(LZ4),
    top_instruction_program_id VARCHAR(44) CODEC(LZ4),
    slot UInt64,
    source_address VARCHAR(44) CODEC(LZ4),
    source_owner VARCHAR(44) CODEC(LZ4),
    mint VARCHAR(44) CODEC(LZ4),
    authority VARCHAR(44) CODEC(LZ4),
    amount UInt64,
)
ENGINE = MergeTree
PRIMARY KEY (signature, instruction_index);

CREATE TABLE spl_token_close_account_events
(
    signature VARCHAR(88) CODEC(LZ4),
    instruction_index UInt32,
    transaction_index UInt32,
    parent_instruction_program_id VARCHAR(44) CODEC(LZ4),
    top_instruction_program_id VARCHAR(44) CODEC(LZ4),
    slot UInt64,
    source_address VARCHAR(44) CODEC(LZ4),
    source_owner VARCHAR(44) CODEC(LZ4),
    destination VARCHAR(44) CODEC(LZ4),
    mint VARCHAR(44),
)
ENGINE = MergeTree
PRIMARY KEY (signature, instruction_index);

CREATE TABLE spl_token_freeze_account_events
(
    signature VARCHAR(88) CODEC(LZ4),
    instruction_index UInt32,
    transaction_index UInt32,
    parent_instruction_program_id VARCHAR(44) CODEC(LZ4),
    top_instruction_program_id VARCHAR(44) CODEC(LZ4),
    slot UInt64,
    source_address VARCHAR(44) CODEC(LZ4),
    source_owner VARCHAR(44) CODEC(LZ4),
    mint VARCHAR(44) CODEC(LZ4),
    freeze_authority VARCHAR(44) CODEC(LZ4),
)
ENGINE = MergeTree
PRIMARY KEY (signature, instruction_index);

CREATE TABLE spl_token_thaw_account_events
(
    signature VARCHAR(88) CODEC(LZ4),
    instruction_index UInt32,
    transaction_index UInt32,
    parent_instruction_program_id VARCHAR(44) CODEC(LZ4),
    top_instruction_program_id VARCHAR(44) CODEC(LZ4),
    slot UInt64,
    source_address VARCHAR(44) CODEC(LZ4),
    source_owner VARCHAR(44) CODEC(LZ4),
    mint VARCHAR(44) CODEC(LZ4),
    freeze_authority VARCHAR(44) CODEC(LZ4),
)
ENGINE = MergeTree
PRIMARY KEY (signature, instruction_index);

CREATE TABLE spl_token_initialize_immutable_owner_events
(
    signature VARCHAR(88) CODEC(LZ4),
    instruction_index UInt32,
    transaction_index UInt32,
    parent_instruction_program_id VARCHAR(44) CODEC(LZ4),
    top_instruction_program_id VARCHAR(44) CODEC(LZ4),
    slot UInt64,
    account_address VARCHAR(44) CODEC(LZ4),
    account_owner VARCHAR(44) CODEC(LZ4),
    mint VARCHAR(44) CODEC(LZ4),
)
ENGINE = MergeTree
PRIMARY KEY (signature, instruction_index);
