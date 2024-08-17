use std::collections::HashMap;
use std::rc::Rc;
use bs58;

use substreams::errors::Error;
use substreams_solana::pb::sf::solana::r#type::v1::ConfirmedTransaction;
use substreams_solana::pb::sf::solana::r#type::v1::Block;
use substreams_database_change::tables::Tables;
use substreams_database_change::pb::database::TableChange;

use substreams_solana_raydium_amm as raydium_amm;
use raydium_amm::instruction::AmmInstruction;
use raydium_amm::RAYDIUM_LIQUIDITY_POOL;

use substreams_solana_utils as utils;
pub use utils::instruction::{StructuredInstruction, StructuredInstructions};
pub use utils::transaction::TransactionContext;

use spl_token_substream;

pub mod pb;
use pb::raydium::{
    RaydiumBlockEvents,
    RaydiumTransactionEvents,
    RaydiumEvent,
    InitializeEvent,
    SwapEvent,
    WithdrawEvent,
    WithdrawPnlEvent,
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
    for (i, transaction) in block.transactions.iter().enumerate() {
        if let Ok(events) = parse_transaction(transaction) {
            if !events.is_empty() {
                block_events.push(RaydiumTransactionEvents {
                    signature: utils::transaction::get_signature(&transaction),
                    transaction_index: i as u32,
                    events,
                });
            }
        }
    }
    block_events
}

pub fn parse_transaction(transaction: &ConfirmedTransaction) -> Result<Vec<RaydiumEvent>, String> {
    if let Some(_) = transaction.meta.as_ref().unwrap().err {
        return Err("Cannot parse failed transaction.".to_string());
    }

    let mut events: Vec<RaydiumEvent> = Vec::new();

    let context = utils::transaction::get_context(transaction);
    let instructions = utils::instruction::get_structured_instructions(transaction).unwrap();
    let flattened_instructions = instructions.flattened();
    let instruction_index: HashMap<_, _> = flattened_instructions.iter().enumerate().map(|(x, y)| (Rc::as_ptr(y), x)).collect();

    for (i, instruction) in flattened_instructions.iter().enumerate() {
        if bs58::encode(context.get_account_from_index(instruction.program_id_index() as usize)).into_string() != RAYDIUM_LIQUIDITY_POOL {
            continue;
        }

        match parse_instruction(&instruction, &context) {
            Ok(Some(event)) => {
                let parent_instruction = instruction.parent_instruction();
                let top_instruction = instruction.top_instruction();
                let parent_instruction_program_id = parent_instruction.as_ref().map(|x| bs58::encode(context.get_account_from_index(x.program_id_index() as usize)).into_string());
                let top_instruction_program_id = top_instruction.as_ref().map(|x| bs58::encode(context.get_account_from_index(x.program_id_index() as usize)).into_string());
                let parent_instruction_index = parent_instruction.as_ref().map(|x| instruction_index[&Rc::as_ptr(x)] as u32);
                let top_instruction_index = top_instruction.as_ref().map(|x| instruction_index[&Rc::as_ptr(x)] as u32);

                events.push(RaydiumEvent {
                    instruction_index: i as u32,
                    event: Some(event),
                    top_instruction_program_id,
                    parent_instruction_program_id,
                    top_instruction_index,
                    parent_instruction_index,
                })
            }
            Ok(None) => (),
            Err(error) => substreams::log::println(format!("Failed to process instruction of transaction {}: {}", &context.signature, error))
        }
    }
    Ok(events)
}

pub fn parse_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext
) -> Result<Option<Event>, String> {
    if bs58::encode(context.get_account_from_index(instruction.program_id_index() as usize)).into_string() != RAYDIUM_LIQUIDITY_POOL {
        return Err("Not a Raydium instruction.".to_string());
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

fn _parse_swap_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext,
) -> Result<SwapEvent, String> {
    let amm = bs58::encode(context.get_account_from_index(instruction.accounts()[1] as usize)).into_string();
    let user = bs58::encode(context.get_account_from_index(*instruction.accounts().last().unwrap() as usize)).into_string();

    let instructions_len = instruction.inner_instructions().len();
    let transfer_in = spl_token_substream::parse_transfer_instruction(&instruction.inner_instructions()[instructions_len - 2], context)?;
    let transfer_out = spl_token_substream::parse_transfer_instruction(&instruction.inner_instructions()[instructions_len - 1], context)?;

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
    let amm = bs58::encode(context.get_account_from_index(instruction.accounts()[4] as usize)).into_string();
    let user = bs58::encode(context.get_account_from_index(instruction.accounts()[17] as usize)).into_string();

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
    let amm = bs58::encode(context.get_account_from_index(instruction.accounts()[1] as usize)).into_string();
    let user = bs58::encode(context.get_account_from_index(instruction.accounts()[12] as usize)).into_string();

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
    let amm = bs58::encode(context.get_account_from_index(instruction.accounts()[1] as usize)).into_string();
    let user = bs58::encode(context.get_account_from_index(instruction.accounts()[16] as usize)).into_string();

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

fn _parse_withdraw_pnl_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext,
) -> Result<WithdrawPnlEvent, String> {
    let amm = bs58::encode(context.get_account_from_index(instruction.accounts()[1] as usize)).into_string();
    let user = bs58::encode(context.get_account_from_index(instruction.accounts()[9] as usize)).into_string();

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

pub fn tables_changes(block: &Block) -> Result<Vec<TableChange>, substreams::errors::Error> {
    let mut tables = Tables::new();
    for transaction in parse_block(block) {
        for event in transaction.events.iter() {
            match &event.event {
                Some(Event::Swap(swap)) => {
                    tables.create_row("raydium_swap_events", [("signature", transaction.signature.clone()), ("instruction_index", event.instruction_index.to_string())])
                        .set("transaction_index", transaction.transaction_index)
                        .set("parent_instruction_program_id", event.parent_instruction_program_id.as_ref().unwrap())
                        .set("top_instruction_program_id", event.top_instruction_program_id.as_ref().unwrap())
                        .set("slot", block.slot)
                        .set("amm", &swap.amm)
                        .set("user", &swap.user)
                        .set("amount_in", swap.amount_in)
                        .set("amount_out", swap.amount_out)
                        .set("mint_in", &swap.mint_in)
                        .set("mint_out", &swap.mint_out);
                }
                Some(Event::Initialize(initialize)) => {
                    tables.create_row("raydium_initialize_events", [("signature", transaction.signature.clone()), ("instruction_index", event.instruction_index.to_string())])
                        .set("transaction_index", transaction.transaction_index)
                        .set("parent_instruction_program_id", event.parent_instruction_program_id.as_ref().unwrap())
                        .set("top_instruction_program_id", event.top_instruction_program_id.as_ref().unwrap())
                        .set("slot", block.slot)
                        .set("amm", &initialize.amm)
                        .set("user", &initialize.user)
                        .set("pc_init_amount", initialize.pc_init_amount)
                        .set("coin_init_amount", initialize.coin_init_amount)
                        .set("lp_init_amount", initialize.lp_init_amount)
                        .set("pc_mint", &initialize.pc_mint)
                        .set("coin_mint", &initialize.coin_mint)
                        .set("lp_mint", &initialize.lp_mint);
                },
                Some(Event::Deposit(deposit)) => {
                    tables.create_row("raydium_deposit_events", [("signature", transaction.signature.clone()), ("instruction_index", event.instruction_index.to_string())])
                        .set("transaction_index", transaction.transaction_index)
                        .set("parent_instruction_program_id", event.parent_instruction_program_id.as_ref().unwrap())
                        .set("top_instruction_program_id", event.top_instruction_program_id.as_ref().unwrap())
                        .set("slot", block.slot)
                        .set("amm", &deposit.amm)
                        .set("user", &deposit.user)
                        .set("pc_amount", deposit.pc_amount)
                        .set("coin_amount", deposit.coin_amount)
                        .set("lp_amount", deposit.lp_amount)
                        .set("pc_mint", &deposit.pc_mint)
                        .set("coin_mint", &deposit.coin_mint)
                        .set("lp_mint", &deposit.lp_mint);
                },
                Some(Event::Withdraw(withdraw)) => {
                    tables.create_row("raydium_withdraw_events", [("signature", transaction.signature.clone()), ("instruction_index", event.instruction_index.to_string())])
                        .set("transaction_index", transaction.transaction_index)
                        .set("parent_instruction_program_id", event.parent_instruction_program_id.as_ref().unwrap())
                        .set("top_instruction_program_id", event.top_instruction_program_id.as_ref().unwrap())
                        .set("slot", block.slot)
                        .set("amm", &withdraw.amm)
                        .set("user", &withdraw.user)
                        .set("pc_amount", withdraw.pc_amount)
                        .set("coin_amount", withdraw.coin_amount)
                        .set("lp_amount", withdraw.lp_amount)
                        .set("pc_mint", &withdraw.pc_mint)
                        .set("coin_mint", &withdraw.coin_mint)
                        .set("lp_mint", &withdraw.lp_mint);
                }
                Some(Event::WithdrawPnl(withdraw_pnl)) => {
                    tables.create_row("raydium_withdraw_pnl_events", [("signature", transaction.signature.clone()), ("instruction_index", event.instruction_index.to_string())])
                        .set("transaction_index", transaction.transaction_index)
                        .set("parent_instruction_program_id", event.parent_instruction_program_id.as_ref().unwrap())
                        .set("top_instruction_program_id", event.top_instruction_program_id.as_ref().unwrap())
                        .set("slot", block.slot)
                        .set("amm", &withdraw_pnl.amm)
                        .set("user", &withdraw_pnl.user)
                        .set("pc_amount", withdraw_pnl.pc_amount.unwrap_or(0))
                        .set("coin_amount", withdraw_pnl.coin_amount.unwrap_or(0))
                        .set("pc_mint", withdraw_pnl.pc_mint.as_deref().unwrap_or(""))
                        .set("coin_mint", withdraw_pnl.coin_mint.as_deref().unwrap_or(""));
                }
                None => ()
            }
        }
    }
    Ok(tables.to_database_changes().table_changes)
}
