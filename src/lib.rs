use bs58;

use substreams::errors::Error;
use substreams_solana::pb::sf::solana::r#type::v1::ConfirmedTransaction;
use substreams_solana::pb::sf::solana::r#type::v1::Block;
use substreams_solana_structured_instructions::{
    get_structured_instructions,
    StructuredInstruction,
    StructuredInstructions
};

use spl_token_substream;

use substreams_solana_raydium_amm as raydium_amm;
use raydium_amm::instruction::AmmInstruction;
use raydium_amm::RAYDIUM_LIQUIDITY_POOL;

use substreams_solana_utils::{
    TransactionContext,
    ConfirmedTransactionExt,
};

pub mod pb;
use pb::raydium::{
    RaydiumBlockEvents,
    RaydiumTransactionEvents,
    RaydiumEvent,
    InitializeEvent,
    SwapEvent,
    WithdrawEvent,
    DepositEvent,
};
use pb::raydium::raydium_event::Event;

#[substreams::handlers::map]
fn raydium_block_events(block: Block) -> Result<RaydiumBlockEvents, Error> {
    let transactions = parse_block(&block);
    Ok(RaydiumBlockEvents { transactions })
}

pub fn parse_block(block: &Block) -> Vec<RaydiumTransactionEvents> {
    let mut block_events: Vec<RaydiumTransactionEvents> = Vec::new();
    for transaction in &block.transactions {
        let events = parse_transaction(transaction);
        if !events.is_empty() {
            block_events.push(RaydiumTransactionEvents {
                signature: bs58::encode(transaction.signature()).into_string(),
                events,
            });
        }
    }
    block_events
}

pub fn parse_transaction(transaction: &ConfirmedTransaction) -> Vec<RaydiumEvent> {
    let context = TransactionContext::construct(transaction);
    let mut events: Vec<RaydiumEvent> = Vec::new();
    let instructions = get_structured_instructions(transaction);

    if let Some(_) = transaction.meta.as_ref().unwrap().err {
        return Vec::new();
    }

    for instruction in instructions.flattened() {
        if bs58::encode(context.get_account_from_index(instruction.program_id_index as usize)).into_string() != RAYDIUM_LIQUIDITY_POOL {
            continue;
        }
        match parse_instruction(&instruction, &context) {
            Ok(Some(event)) => {
                events.push(RaydiumEvent { event: Some(event) })
            }
            Ok(None) => (),
            Err(error) => substreams::log::println(format!("Failed to process instruction of transaction {}: {}", &context.signature, error))
        }
    }
    events
}

pub fn parse_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext
) -> Result<Option<Event>, String> {
    if bs58::encode(context.get_account_from_index(instruction.program_id_index as usize)).into_string() != RAYDIUM_LIQUIDITY_POOL {
        return Err("Not a Raydium instruction.".to_string());
    }
    let unpacked = AmmInstruction::unpack(&instruction.data)?;
    match unpacked {
        AmmInstruction::SwapBaseIn(_) |
        AmmInstruction::SwapBaseOut(_) => {
            let event = _parse_swap_instruction(instruction, context)?;
            Ok(Some(Event::Swap(event)))
        },
        AmmInstruction::Initialize2(initialize) => {
            let event = _parse_initialize_instruction(instruction, context, initialize.nonce)?;
            Ok(Some(Event::Initialize(event)))
        },
        AmmInstruction::Deposit(_deposit) => {
            let event = _parse_deposit_instruction(instruction, context)?;
            Ok(Some(Event::Deposit(event)))
        },
        AmmInstruction::Withdraw(_withdraw) => {
            let event = _parse_withdraw_instruction(instruction, context)?;
            Ok(Some(Event::Withdraw(event)))
        }
        _ => Ok(None),
    }
}

fn _parse_swap_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext,
) -> Result<SwapEvent, String> {
    let amm = bs58::encode(context.get_account_from_index(instruction.accounts[1] as usize)).into_string();
    let user = bs58::encode(context.get_account_from_index(*instruction.accounts.last().unwrap() as usize)).into_string();

    let instructions_len = instruction.inner_instructions.len();
    let transfer_in = spl_token_substream::parse_transfer_instruction(&instruction.inner_instructions[instructions_len - 2], context)?;
    let transfer_out = spl_token_substream::parse_transfer_instruction(&instruction.inner_instructions[instructions_len - 1], context)?;

    let amount_in = transfer_in.amount;
    let amount_out = transfer_out.amount;
    let mint_in = transfer_in.source.unwrap().mint;
    let mint_out = transfer_out.source.unwrap().mint;

    Ok(SwapEvent {
        amm,
        user,
        mint_in,
        mint_out,
        amount_in,
        amount_out,
    })
}

fn _parse_initialize_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext,
    nonce: u8,
) -> Result<InitializeEvent, String> {
    let amm = bs58::encode(context.get_account_from_index(instruction.accounts[4] as usize)).into_string();
    let user = bs58::encode(context.get_account_from_index(instruction.accounts[17] as usize)).into_string();

    let instructions_len = instruction.inner_instructions.len();
    let coin_transfer = spl_token_substream::parse_transfer_instruction(&instruction.inner_instructions[instructions_len - 3], context)?;
    let pc_transfer = spl_token_substream::parse_transfer_instruction(&instruction.inner_instructions[instructions_len - 2], context)?;
    let lp_mint_to = spl_token_substream::parse_mint_to_instruction(&instruction.inner_instructions[instructions_len - 1], context)?;

    let pc_init_amount = pc_transfer.amount;
    let coin_init_amount = coin_transfer.amount;
    let lp_init_amount = lp_mint_to.amount;
    let pc_mint = pc_transfer.source.unwrap().mint;
    let coin_mint = coin_transfer.source.unwrap().mint;
    let lp_mint = lp_mint_to.mint;

    Ok(InitializeEvent {
        amm,
        user,
        pc_init_amount,
        coin_init_amount,
        lp_init_amount,
        pc_mint,
        coin_mint,
        lp_mint,
        nonce: nonce as u32,
    })
}

fn _parse_deposit_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext
) -> Result<DepositEvent, String> {
    let amm = bs58::encode(context.get_account_from_index(instruction.accounts[1] as usize)).into_string();
    let user = bs58::encode(context.get_account_from_index(instruction.accounts[12] as usize)).into_string();

    let instructions_len = instruction.inner_instructions.len();
    let pc_transfer = spl_token_substream::parse_transfer_instruction(&instruction.inner_instructions[instructions_len - 2], context)?;
    let coin_transfer = spl_token_substream::parse_transfer_instruction(&instruction.inner_instructions[instructions_len - 3], context)?;
    let lp_mint_to = spl_token_substream::parse_mint_to_instruction(&instruction.inner_instructions[instructions_len - 1], context)?;

    let pc_amount = pc_transfer.amount;
    let coin_amount = coin_transfer.amount;
    let lp_amount = lp_mint_to.amount;
    let pc_mint = pc_transfer.source.unwrap().mint;
    let coin_mint = coin_transfer.source.unwrap().mint;
    let lp_mint = lp_mint_to.mint;

    Ok(DepositEvent {
        amm,
        user,
        pc_amount,
        coin_amount,
        lp_amount,
        pc_mint,
        coin_mint,
        lp_mint,
    })
}

fn _parse_withdraw_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext,
) -> Result<WithdrawEvent, String> {
    let amm = bs58::encode(context.get_account_from_index(instruction.accounts[1] as usize)).into_string();
    let user = bs58::encode(context.get_account_from_index(instruction.accounts[16] as usize)).into_string();

    let instructions_len = instruction.inner_instructions.len();
    let pc_transfer = spl_token_substream::parse_transfer_instruction(&instruction.inner_instructions[instructions_len - 2], context)?;
    let coin_transfer = spl_token_substream::parse_transfer_instruction(&instruction.inner_instructions[instructions_len - 3], context)?;
    let lp_burn = spl_token_substream::parse_burn_instruction(&instruction.inner_instructions[instructions_len - 1], context)?;

    let pc_amount = pc_transfer.amount;
    let coin_amount = coin_transfer.amount;
    let lp_amount = lp_burn.amount;
    let pc_mint = pc_transfer.source.unwrap().mint;
    let coin_mint = coin_transfer.source.unwrap().mint;
    let lp_mint = lp_burn.source.unwrap().mint;

    Ok(WithdrawEvent {
        amm,
        user,
        pc_amount,
        coin_amount,
        lp_amount,
        pc_mint,
        coin_mint,
        lp_mint,
    })
}
