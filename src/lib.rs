use substreams::errors::Error;
use substreams_solana::pb::sf::solana::r#type::v1::ConfirmedTransaction;
use substreams_solana::pb::sf::solana::r#type::v1::Block;
use structured_instructions::{get_structured_instructions, StructuredInstruction, StructuredInstructions};

use substreams_solana_raydium_amm as raydium_amm;
use raydium_amm::instruction::AmmInstruction;
use raydium_amm::RAYDIUM_LIQUIDITY_POOL;

use spl_token_substream;
use bs58;
mod pb;

use substreams_solana_utils::{
    TransactionContext,
    ConfirmedTransactionExt,
};

#[substreams::handlers::map]
pub fn raydium_events(block: Block) -> Result<pb::raydium::BlockRaydiumEvents, Error> {
    let slot = block.slot;
    let transactions = parse_block(block);
    Ok(pb::raydium::BlockRaydiumEvents {
        transactions,
        slot,
    })
}

pub fn parse_block(block: Block) -> Vec<pb::raydium::TransactionRaydiumEvents> {
    let mut block_events: Vec<pb::raydium::TransactionRaydiumEvents> = Vec::new();
    for transaction in block.transactions {
        let events = parse_transaction(&transaction);
        if !events.is_empty() {
            block_events.push(pb::raydium::TransactionRaydiumEvents {
                signature: bs58::encode(transaction.signature()).into_string(),
                events,
            });
        }
    }
    block_events
}

fn parse_transaction(transaction: &ConfirmedTransaction) -> Vec<pb::raydium::RaydiumEvent> {
    let context = TransactionContext::construct(transaction);
    let mut events: Vec<pb::raydium::RaydiumEvent> = Vec::new();
    let instructions = get_structured_instructions(transaction);

    if let Some(_) = transaction.meta.as_ref().unwrap().err {
        return Vec::new();
    }

    for instruction in instructions.flattened() {
        if bs58::encode(context.get_account_from_index(instruction.program_id_index as usize)).into_string() != RAYDIUM_LIQUIDITY_POOL {
            continue;
        }
        match parse_instruction(&instruction, &context) {
            Ok(Some(event)) => events.push(event),
            Ok(None) => (),
            Err(error) => substreams::log::println(format!("Failed to process instruction of transaction {}: {}", &context.signature, error))
        }
    }
    events
}

pub fn parse_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext
) -> Result<Option<pb::raydium::RaydiumEvent>, String> {
    if bs58::encode(context.get_account_from_index(instruction.program_id_index as usize)).into_string() != RAYDIUM_LIQUIDITY_POOL {
        return Err("Not a Raydium instruction.".to_string());
    }
    let unpacked = AmmInstruction::unpack(&instruction.data)?;
    match unpacked {
        AmmInstruction::SwapBaseIn(_) |
        AmmInstruction::SwapBaseOut(_) => {
            parse_swap_instruction(instruction, context).map(Some)
        },
        AmmInstruction::Initialize2(initialize) => {
            parse_initialize_instruction(instruction, context, initialize.nonce).map(Some)
        },
        AmmInstruction::Deposit(_deposit) => {
            parse_deposit_instruction(instruction, context).map(Some)
        },
        AmmInstruction::Withdraw(_withdraw) => {
            parse_withdraw_instruction(instruction, context).map(Some)
        }
        _ => Ok(None),
    }
}

fn parse_swap_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext,
) -> Result<pb::raydium::RaydiumEvent, String> {
    let amm = bs58::encode(context.get_account_from_index(instruction.accounts[1] as usize)).into_string();
    let user = bs58::encode(context.get_account_from_index(*instruction.accounts.last().unwrap() as usize)).into_string();

    let instructions_len = instruction.inner_instructions.len();
    let transfer_in = parse_token_transfer_instruction(&instruction.inner_instructions[instructions_len - 2], context)?;
    let transfer_out = parse_token_transfer_instruction(&instruction.inner_instructions[instructions_len - 1], context)?;

    let data = pb::raydium::SwapData {
        amount_in: transfer_in.amount,
        mint_in: transfer_in.source.unwrap().mint,
        amount_out: transfer_out.amount,
        mint_out: transfer_out.source.unwrap().mint,
    };
    Ok(pb::raydium::RaydiumEvent {
        amm,
        user,
        data: Some(pb::raydium::raydium_event::Data::Swap(data)),
    })
}

fn parse_initialize_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext,
    nonce: u8,
) -> Result<pb::raydium::RaydiumEvent, String> {
    let amm = bs58::encode(context.get_account_from_index(instruction.accounts[4] as usize)).into_string();
    let user = bs58::encode(context.get_account_from_index(instruction.accounts[17] as usize)).into_string();

    let instructions_len = instruction.inner_instructions.len();
    let coin_transfer = parse_token_transfer_instruction(&instruction.inner_instructions[instructions_len - 3], context)?;
    let pc_transfer = parse_token_transfer_instruction(&instruction.inner_instructions[instructions_len - 2], context)?;
    let lp_mint_to = parse_token_mint_to_instruction(&instruction.inner_instructions[instructions_len - 1], context)?;

    let data = pb::raydium::InitializeData {
        pc_init_amount: pc_transfer.amount,
        coin_init_amount: coin_transfer.amount,
        lp_init_amount: lp_mint_to.amount,
        pc_mint: pc_transfer.source.unwrap().mint,
        coin_mint: coin_transfer.source.unwrap().mint,
        lp_mint: lp_mint_to.mint,
        nonce: nonce as u32,
    };
    Ok(pb::raydium::RaydiumEvent {
        amm,
        user,
        data: Some(pb::raydium::raydium_event::Data::Initialize(data)),
    })
}

fn parse_deposit_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext
) -> Result<pb::raydium::RaydiumEvent, String> {
    let amm = bs58::encode(context.get_account_from_index(instruction.accounts[1] as usize)).into_string();
    let user = bs58::encode(context.get_account_from_index(instruction.accounts[12] as usize)).into_string();

    let instructions_len = instruction.inner_instructions.len();
    let coin_transfer = parse_token_transfer_instruction(&instruction.inner_instructions[instructions_len - 3], context)?;
    let pc_transfer = parse_token_transfer_instruction(&instruction.inner_instructions[instructions_len - 2], context)?;
    let lp_mint_to = parse_token_mint_to_instruction(&instruction.inner_instructions[instructions_len - 1], context)?;

    let data = pb::raydium::DepositData {
        pc_amount: pc_transfer.amount,
        coin_amount: coin_transfer.amount,
        lp_amount: lp_mint_to.amount,
    };
    Ok(pb::raydium::RaydiumEvent {
        amm,
        user,
        data: Some(pb::raydium::raydium_event::Data::Deposit(data)),
    })
}

fn parse_withdraw_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext,
) -> Result<pb::raydium::RaydiumEvent, String> {
    let amm = bs58::encode(context.get_account_from_index(instruction.accounts[1] as usize)).into_string();
    let user = bs58::encode(context.get_account_from_index(instruction.accounts[16] as usize)).into_string();

    let instructions_len = instruction.inner_instructions.len();
    let coin_transfer = parse_token_transfer_instruction(&instruction.inner_instructions[instructions_len - 3], context)?;
    let pc_transfer = parse_token_transfer_instruction(&instruction.inner_instructions[instructions_len - 2], context)?;
    let lp_mint_to = parse_token_burn_instruction(&instruction.inner_instructions[instructions_len - 1], context)?;

    let data = pb::raydium::WithdrawData {
        pc_amount: pc_transfer.amount,
        coin_amount: coin_transfer.amount,
        lp_amount: lp_mint_to.amount,
    };
    Ok(pb::raydium::RaydiumEvent {
        amm,
        user,
        data: Some(pb::raydium::raydium_event::Data::Withdraw(data)),
    })
}

fn parse_token_transfer_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext,
) -> Result<spl_token_substream::pb::spl_token::TransferEvent, String> {
    match spl_token_substream::parse_instruction(instruction, context) {
        Ok(Some(event)) => match event {
            spl_token_substream::pb::spl_token::spl_token_event::Event::Transfer(transfer) => Ok(transfer),
            _ => Err("Not an SPL Token transfer event.".to_string())
        },
        _ => Err("Failed to parse instruction.".to_string())
    }
}

fn parse_token_mint_to_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext,
) -> Result<spl_token_substream::pb::spl_token::MintToEvent, String> {
    match spl_token_substream::parse_instruction(instruction, context) {
        Ok(Some(event)) => match event {
            spl_token_substream::pb::spl_token::spl_token_event::Event::MintTo(mint_to) => Ok(mint_to),
            _ => Err("Not an SPL Token mint to event.".to_string())
        },
        _ => Err("Failed to parse instruction.".to_string())
    }
}

fn parse_token_burn_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext,
) -> Result<spl_token_substream::pb::spl_token::BurnEvent, String> {
    match spl_token_substream::parse_instruction(instruction, context) {
        Ok(Some(event)) => match event {
            spl_token_substream::pb::spl_token::spl_token_event::Event::Burn(burn) => Ok(burn),
            _ => Err("Not an SPL Token burn event.".to_string())
        },
        _ => Err("Failed to parse instruction".to_string())
    }
}
