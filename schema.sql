CREATE TABLE raydium_swap_events
(
    signature VARCHAR(88) CODEC(LZ4),
    instruction_index UInt32,
    transaction_index UInt32,
    slot UInt64,
    amm VARCHAR(44) CODEC(LZ4),
    user VARCHAR(44) CODEC(LZ4),
    amount_in UInt64,
    amount_out UInt64,
    mint_in VARCHAR(44) CODEC(LZ4),
    mint_out VARCHAR(44) CODEC(LZ4),
)
ENGINE = MergeTree
PRIMARY KEY (signature, instruction_index);

CREATE TABLE raydium_initialize_events
(
    signature VARCHAR(88) CODEC(LZ4),
    instruction_index UInt32,
    transaction_index UInt32,
    slot UInt64,
    amm VARCHAR(44) CODEC(LZ4),
    user VARCHAR(44) CODEC(LZ4),
    pc_init_amount UInt64,
    coin_init_amount UInt64,
    lp_init_amount UInt64,
    pc_mint VARCHAR(44) CODEC(LZ4),
    coin_mint VARCHAR(44) CODEC(LZ4),
    lp_mint VARCHAR(44) CODEC(LZ4),
)
ENGINE = MergeTree
PRIMARY KEY (signature, instruction_index);

CREATE TABLE raydium_deposit_events
(
    signature VARCHAR(88) CODEC(LZ4),
    instruction_index UInt32,
    transaction_index UInt32,
    slot UInt64,
    amm VARCHAR(44) CODEC(LZ4),
    user VARCHAR(44) CODEC(LZ4),
    pc_amount UInt64,
    coin_amount UInt64,
    lp_amount UInt64,
    pc_mint VARCHAR(44) CODEC(LZ4),
    coin_mint VARCHAR(44) CODEC(LZ4),
    lp_mint VARCHAR(44) CODEC(LZ4),
)
ENGINE = MergeTree
PRIMARY KEY (signature, instruction_index);

CREATE TABLE raydium_withdraw_events
(
    signature VARCHAR(88) CODEC(LZ4),
    instruction_index UInt32,
    transaction_index UInt32,
    slot UInt64,
    amm VARCHAR(44) CODEC(LZ4),
    user VARCHAR(44) CODEC(LZ4),
    pc_amount UInt64,
    coin_amount UInt64,
    lp_amount UInt64,
    pc_mint VARCHAR(44) CODEC(LZ4),
    coin_mint VARCHAR(44) CODEC(LZ4),
    lp_mint VARCHAR(44) CODEC(LZ4),
)
ENGINE = MergeTree
PRIMARY KEY (signature, instruction_index);
