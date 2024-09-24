use regex;

use substreams::errors::Error;
use substreams_solana::pb::sf::solana::r#type::v1::ConfirmedTransaction;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

pub mod raydium_amm;
use raydium_amm::instruction::AmmInstruction;
use raydium_amm::constants::RAYDIUM_AMM_PROGRAM_ID;
use raydium_amm::log::{decode_ray_log, RayLog};

use substreams_solana_utils as utils;
use utils::instruction::{get_structured_instructions, StructuredInstruction, StructuredInstructions};
use utils::transaction::{get_context, TransactionContext};
use utils::pubkey::Pubkey;
use utils::log::Log;

use spl_token_substream;

pub mod pb;
use pb::raydium_amm::*;
use pb::raydium_amm::raydium_amm_event::Event;

#[substreams::handlers::map]
fn raydium_amm_block_events(block: Block) -> Result<RaydiumAmmBlockEvents, Error> {
    let transactions = parse_block(&block);
    Ok(RaydiumAmmBlockEvents { transactions})
}

pub fn parse_block(block: &Block) -> Vec<RaydiumAmmTransactionEvents> {
    let mut block_events: Vec<RaydiumAmmTransactionEvents> = Vec::new();
    for transaction in block.transactions.iter() {
        if let Ok(events) = parse_transaction(transaction) {
            if !events.is_empty() {
                block_events.push(RaydiumAmmTransactionEvents {
                    signature: utils::transaction::get_signature(&transaction),
                    events,
                });
            }
        }
    }
    block_events
}

pub fn parse_transaction(transaction: &ConfirmedTransaction) -> Result<Vec<RaydiumAmmEvent>, Error> {
    if let Some(_) = transaction.meta.as_ref().unwrap().err {
        return Ok(Vec::new());
    }

    let mut events: Vec<RaydiumAmmEvent> = Vec::new();

    let context = get_context(transaction)?;
    let instructions = get_structured_instructions(transaction)?;

    for instruction in instructions.flattened().iter() {
        if instruction.program_id() != RAYDIUM_AMM_PROGRAM_ID {
            continue;
        }

        match parse_instruction(&instruction, &context) {
            Ok(Some(event)) => {
                events.push(RaydiumAmmEvent {
                    event: Some(event),
                })
            }
            Ok(None) => (),
            Err(error) => substreams::log::println(format!("Failed to process instruction of transaction {}: {}", &context.signature, error))
        }
    }
    Ok(events)
}

pub fn parse_instruction<'a>(
    instruction: &StructuredInstruction<'a>,
    context: &TransactionContext
) -> Result<Option<Event>, String> {
    if instruction.program_id() != RAYDIUM_AMM_PROGRAM_ID {
        return Err("Instruction does not originate from Raydium AMM Program.".into());
    }
    let unpacked = AmmInstruction::unpack(&instruction.data())?;
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
        },
        AmmInstruction::WithdrawPnl => {
            let event = _parse_withdraw_pnl_instruction(instruction, context)?;
            Ok(Some(Event::WithdrawPnl(event)))
        }
        _ => Ok(None),
    }
}

fn _parse_swap_instruction<'a>(
    instruction: &StructuredInstruction<'a>,
    context: &TransactionContext,
) -> Result<SwapEvent, String> {
    let amm = instruction.accounts()[1].to_string();
    let user = instruction.accounts().last().unwrap().to_string();

    let instructions_len = instruction.inner_instructions().len();
    let transfer_in = spl_token_substream::parse_transfer_instruction(&instruction.inner_instructions()[instructions_len - 2], context)?;
    let transfer_out = spl_token_substream::parse_transfer_instruction(&instruction.inner_instructions()[instructions_len - 1], context)?;

    let amount_in = transfer_in.amount;
    let amount_out = transfer_out.amount;
    let mint_in = transfer_in.source.unwrap().mint;
    let mint_out = transfer_out.source.unwrap().mint;

    let delta = if instruction.accounts().len() == 17 { 0 } else { 1 };
    let coin_mint = context.get_token_account(&instruction.accounts()[4 + delta]).unwrap().mint.to_string();
    let pc_mint = context.get_token_account(&instruction.accounts()[5 + delta]).unwrap().mint.to_string();

    let direction = (if mint_out == coin_mint { "coin" } else { "pc" }).to_string();

    let (pool_coin_amount, pool_pc_amount) = match parse_log(instruction) {
        Ok(RayLog::SwapBaseIn(swap_base_in)) => {
            (Some(swap_base_in.pool_coin), Some(swap_base_in.pool_pc))
        },
        Ok(RayLog::SwapBaseOut(swap_base_out)) => {
            (Some(swap_base_out.pool_coin), Some(swap_base_out.pool_pc))
        },
        _ => (None, None),
    };

    Ok(SwapEvent {
        amm,
        user,
        mint_in,
        mint_out,
        amount_in,
        amount_out,
        direction,
        pool_coin_amount,
        pool_pc_amount,
        coin_mint,
        pc_mint,
    })
}

fn _parse_initialize_instruction<'a>(
    instruction: &StructuredInstruction<'a>,
    context: &TransactionContext,
    nonce: u8,
) -> Result<InitializeEvent, String> {
    let amm = instruction.accounts()[4].to_string();
    let user = instruction.accounts()[17].to_string();

    let instructions_len = instruction.inner_instructions().len();
    let coin_transfer = spl_token_substream::parse_transfer_instruction(&instruction.inner_instructions()[instructions_len - 3], context)?;
    let pc_transfer = spl_token_substream::parse_transfer_instruction(&instruction.inner_instructions()[instructions_len - 2], context)?;
    let lp_mint_to = spl_token_substream::parse_mint_to_instruction(&instruction.inner_instructions()[instructions_len - 1], context)?;

    let pc_init_amount = pc_transfer.amount;
    let coin_init_amount = coin_transfer.amount;
    let lp_init_amount = lp_mint_to.amount;
    let pc_mint = pc_transfer.source.unwrap().mint;
    let coin_mint = coin_transfer.source.unwrap().mint;
    let lp_mint = lp_mint_to.mint;

    let market = match parse_log(instruction) {
        Ok(RayLog::Init(init)) => Some(Pubkey(init.market).to_string()),
        _ => None,
    };

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
        market,
    })
}

fn _parse_deposit_instruction<'a>(
    instruction: &StructuredInstruction<'a>,
    context: &TransactionContext
) -> Result<DepositEvent, String> {
    let amm = instruction.accounts()[1].to_string();
    let user = instruction.accounts()[12].to_string();

    let instructions_len = instruction.inner_instructions().len();
    let pc_transfer = spl_token_substream::parse_transfer_instruction(&instruction.inner_instructions()[instructions_len - 2], context)?;
    let coin_transfer = spl_token_substream::parse_transfer_instruction(&instruction.inner_instructions()[instructions_len - 3], context)?;
    let lp_mint_to = spl_token_substream::parse_mint_to_instruction(&instruction.inner_instructions()[instructions_len - 1], context)?;

    let pc_amount = pc_transfer.amount;
    let coin_amount = coin_transfer.amount;
    let lp_amount = lp_mint_to.amount;
    let pc_mint = pc_transfer.source.unwrap().mint;
    let coin_mint = coin_transfer.source.unwrap().mint;
    let lp_mint = lp_mint_to.mint;

    let (pool_pc_amount, pool_coin_amount, pool_lp_amount) = match parse_log(instruction) {
        Ok(RayLog::Deposit(deposit)) => {
            (Some(deposit.pool_pc), Some(deposit.pool_coin), Some(deposit.pool_lp))
        },
        _ => (None, None, None)
    };

    Ok(DepositEvent {
        amm,
        user,
        pc_amount,
        coin_amount,
        lp_amount,
        pc_mint,
        coin_mint,
        lp_mint,
        pool_pc_amount,
        pool_coin_amount,
        pool_lp_amount,
    })
}

fn _parse_withdraw_instruction<'a>(
    instruction: &StructuredInstruction<'a>,
    context: &TransactionContext,
) -> Result<WithdrawEvent, String> {
    let amm = instruction.accounts()[1].to_string();
    let user = instruction.accounts()[16].to_string();

    let instructions_len = instruction.inner_instructions().len();
    let pc_transfer = spl_token_substream::parse_transfer_instruction(&instruction.inner_instructions()[instructions_len - 2], context)?;
    let coin_transfer = spl_token_substream::parse_transfer_instruction(&instruction.inner_instructions()[instructions_len - 3], context)?;
    let lp_burn = spl_token_substream::parse_burn_instruction(&instruction.inner_instructions()[instructions_len - 1], context)?;

    let pc_amount = pc_transfer.amount;
    let coin_amount = coin_transfer.amount;
    let lp_amount = lp_burn.amount;
    let pc_mint = pc_transfer.source.unwrap().mint;
    let coin_mint = coin_transfer.source.unwrap().mint;
    let lp_mint = lp_burn.source.unwrap().mint;

    let (pool_pc_amount, pool_coin_amount, pool_lp_amount) = match parse_log(instruction) {
        Ok(RayLog::Withdraw(withdraw)) => {
            (Some(withdraw.pool_pc), Some(withdraw.pool_coin), Some(withdraw.pool_lp))
        },
        _ => (None, None, None)
    };

    Ok(WithdrawEvent {
        amm,
        user,
        pc_amount,
        coin_amount,
        lp_amount,
        pc_mint,
        coin_mint,
        lp_mint,
        pool_pc_amount,
        pool_coin_amount,
        pool_lp_amount,
    })
}

fn _parse_withdraw_pnl_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext,
) -> Result<WithdrawPnlEvent, String> {
    let amm = instruction.accounts()[1].to_string();
    let user = instruction.accounts()[9].to_string();

    let instructions_len = instruction.inner_instructions().len();
    if instructions_len == 2 || instructions_len == 3 {
        let pc_transfer = spl_token_substream::parse_transfer_instruction(&instruction.inner_instructions()[instructions_len - 1], context)?;
        let coin_transfer = spl_token_substream::parse_transfer_instruction(&instruction.inner_instructions()[instructions_len - 2], context)?;

        let pc_amount = Some(pc_transfer.amount);
        let coin_amount = Some(coin_transfer.amount);
        let pc_mint = Some(pc_transfer.source.unwrap().mint);
        let coin_mint = Some(coin_transfer.source.unwrap().mint);

        return Ok(WithdrawPnlEvent {
            amm,
            user,
            pc_amount,
            coin_amount,
            pc_mint,
            coin_mint
        });
    } else {
        return Ok(WithdrawPnlEvent {
            amm,
            user,
            pc_amount: None,
            coin_amount: None,
            pc_mint: None,
            coin_mint: None,
        })
    }
}

fn parse_log(instruction: &StructuredInstruction) -> Result<RayLog, String> {
    let re = regex::Regex::new(r"ray_log: (.+)").unwrap();
    let log_message = instruction.logs().iter().rev().find_map(|log| {
        if let Log::Program(program_log) = log {
            Some(program_log.message().unwrap())
        } else {
            None
        }
    });
    match log_message {
        Some(message) => match re.captures(message.as_str()) {
            Some(captures) => Ok(decode_ray_log(&captures[1])),
            None => return Err("Failed to capture log message".to_string()),
        },
        None => return Err("Log message not found".to_string()),
    }
}
