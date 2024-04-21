mod pb;
mod raydium_amm;
mod spl_token;
mod utils;
mod transaction;

use std::collections::HashMap;

use pb::event::{Event, RaydiumEvent};
use substreams::errors::Error;
use substreams::log::println;
use substreams_solana::pb::sf::solana::r#type::v1::{ConfirmedTransaction, InnerInstruction};
use substreams_solana::pb::sf::solana::r#type::v1::{Block, InnerInstructions};

use bs58;

use raydium_amm::instruction::AmmInstruction;
use spl_token::instruction::TokenInstruction;
use substreams_solana_program_instructions::pubkey;
use utils::{get_token_accounts, TokenAccount};
use crate::raydium_amm::RAYDIUM_LIQUIDITY_POOL;
use crate::spl_token::{SOL_MINT, TOKEN_PROGRAM};

#[substreams::handlers::map]
pub fn events(block: Block) -> Result<pb::event::Events, Error> {
    let events = parse_block(block);
    Ok(pb::event::Events { events: events })
}

pub struct TokenAccountInfo {
    pub owner: String,
    pub mint: String
}

pub fn parse_block(block: Block) -> Vec<pb::event::Event> {
    let mut events: Vec<pb::event::Event> = Vec::new();
    for transaction in block.transactions {
        events.extend(parse_transaction(&transaction, block.slot));
    }
    events
}

fn parse_transaction(transaction: &ConfirmedTransaction, slot: u64) -> Vec<pb::event::Event> {
    let mut events: Vec<pb::event::Event> = Vec::new();

    let accounts = transaction.resolved_accounts_as_strings();
    let meta = transaction.meta.as_ref().unwrap();
    let txn = transaction.transaction.as_ref().unwrap();

    if let Some(err) = meta.err.as_ref() {
        return Vec::new();
    }

    let message = txn.message.as_ref().unwrap();
    let signature = bs58::encode(&txn.signatures[0]).into_string();

    let token_accounts = get_token_accounts(transaction);

    // Raydium was called directly
    for (i, instruction) in message.instructions.iter().enumerate() {
        if accounts[instruction.program_id_index as usize] != RAYDIUM_LIQUIDITY_POOL {
            continue;
        }
        let instructions = &meta.inner_instructions.iter().find(|x| x.index == i as u32).unwrap().instructions;
        let inner_instructions = fetch_inner_instructions(&instructions, None);
        let event = parse_event(&instruction.data, &instruction.accounts, inner_instructions, &accounts, &token_accounts).unwrap();
        events.push(pb::event::Event {
            event: Some(event),
            signer: accounts[0].clone(),
            signature: signature.clone(),
            slot,
        });
    }

    // Raydium was invoked from another program
    for instructions in &meta.inner_instructions {
        for (i, instruction) in instructions.instructions.iter().enumerate() {
            let inner_instructions = fetch_inner_instructions(&instructions.instructions, Some(i));
            let event = parse_event(&instruction.data, &instruction.accounts, inner_instructions, &accounts, &token_accounts).unwrap();
            events.push(pb::event::Event {
                event: Some(event),
                signer: accounts[0].clone(),
                signature: signature.clone(),
                slot,
            });
        }
    }

    events
}

fn fetch_inner_instructions(instructions: &Vec<InnerInstruction>, index: Option<usize>) -> Vec<&InnerInstruction> {
    if let Some(idx) = index {
        let stack_height = instructions[idx].stack_height();
        let mut inner_instructions: Vec<&InnerInstruction> = Vec::new();
        for instruction in instructions[idx + 1..].iter() {
            if instruction.stack_height() == stack_height + 1 {
                inner_instructions.push(instruction);
            } else if instruction.stack_height() == stack_height {
                break;
            }
        }
        inner_instructions
    } else {
        instructions.iter().filter(|x| x.stack_height() == 2).collect()
    }
}

fn parse_event(
    instruction_data: &Vec<u8>,
    instructions_accounts: &Vec<u8>,
    inner_instructions: Vec<&InnerInstruction>,
    accounts: &Vec<String>,
    token_accounts: &HashMap<String, TokenAccount>
) -> Result<RaydiumEvent, &'static str> {
    let unpacked = AmmInstruction::unpack(&instruction_data);
    if unpacked.is_err() { return Err("Not a Raydium event."); }

    match unpacked.unwrap() {
        AmmInstruction::SwapBaseIn(swap_base_in) => Ok(parse_swap_event(instructions_accounts, inner_instructions, accounts, token_accounts)),
        AmmInstruction::SwapBaseOut(swap_base_out) => Ok(parse_swap_event(instructions_accounts, inner_instructions, accounts, token_accounts)),
        _ => Err("Unsupported Raydium event."),
    }
}

fn parse_swap_event(
    instruction_accounts: &Vec<u8>,
    inner_instructions: Vec<&InnerInstruction>,
    accounts: &Vec<String>,
    token_accounts: &HashMap<String, TokenAccount>,
) -> pb::event::RaydiumEvent {
    let amm = accounts[instruction_accounts[1] as usize].clone();

    let transfer_in = parse_token_transfer(inner_instructions[0], accounts).unwrap();
    let transfer_out = parse_token_transfer(inner_instructions[1], accounts).unwrap();

    let amount_in = transfer_in.amount;
    let mint_in: String;
    if let Some(token_account) = token_accounts.get(&transfer_in.source) {
        mint_in = token_account.mint.clone()
    } else {
        let token_account = token_accounts.get(&transfer_in.destination).unwrap();
        mint_in = token_account.mint.clone();
    }

    let amount_out = transfer_out.amount;
    let mint_out: String;
    if let Some(token_account) = token_accounts.get(&transfer_out.source) {
        mint_out = token_account.mint.clone();
    } else {
        let token_account = token_accounts.get(&transfer_out.destination).unwrap();
        mint_out = token_account.mint.clone();
    }

    let data = pb::event::SwapData {
        amount_in,
        amount_out,
        mint_in,
        mint_out
    };
    pb::event::RaydiumEvent {
        amm,
        r#type: pb::event::EventType::Swap.into(),
        data: Some(pb::event::raydium_event::Data::Swap(data))
    }
}

pub struct TokenTransfer {
    pub source: String,
    pub destination: String,
    pub amount: u64,
}

fn parse_token_transfer(
    instruction: &InnerInstruction,
    accounts: &Vec<String>,
) -> Result<TokenTransfer, &'static str> {
    match TokenInstruction::unpack(&instruction.data).unwrap() {
        TokenInstruction::Transfer { amount } => {
            let source = &accounts[instruction.accounts[0] as usize];
            let destination = &accounts[instruction.accounts[1] as usize];
            Ok(TokenTransfer {
                source: source.clone(),
                destination: destination.clone(),
                amount,
            })
        },
        TokenInstruction::TransferChecked { amount, decimals } => {
            let source = &accounts[instruction.accounts[0] as usize];
            let destination = &accounts[instruction.accounts[1] as usize];
            Ok(TokenTransfer {
                source: source.clone(),
                destination: destination.clone(),
                amount,
            })
        },
        _ => Err("Not an SplToken transfer")
    }
}

// fn get_raydium_swap_event<'a>(
//     transfer_instructions: &[&InnerInstruction; 2],
//     accounts: &Vec<String>,
//     owners: &'a HashMap<String, String>,
//     amm: &String,
// ) -> pb::raydium::RaydiumEvent {
//     let data = get_swap_data(transfer_instructions, accounts, owners);
//     pb::raydium::RaydiumEvent {
//         r#type: pb::raydium::RaydiumEventType::Swap.into(),
//         amm: amm.clone(),
//         data: Some(pb::raydium::raydium_event::Data::Swap(data))
//     }
// }

// fn get_swap_data<'a>(
//     transfer_instructions: &[&InnerInstruction; 2],
//     accounts: &Vec<String>,
//     owners: &'a HashMap<String, String>,
// ) -> pb::raydium::RaydiumSwapData {
//     let in_transfer_instruction = &transfer_instructions[0];
//     let out_transfer_instruction = &transfer_instructions[1];

//     let amount_in = match TokenInstruction::unpack(&in_transfer_instruction.data).unwrap() {
//         TokenInstruction::Transfer { amount } => amount,
//         _ => {
//             panic!();
//         }
//     };

//     let amount_out = match TokenInstruction::unpack(&out_transfer_instruction.data).unwrap() {
//         TokenInstruction::Transfer { amount } => amount,
//         _ => {
//             panic!();
//         }
//     };

//     let token_in = owners.get(&accounts[in_transfer_instruction.accounts[0] as usize]).unwrap_or(&SOL_MINT.to_string()).clone();
//     let token_out = owners.get(&accounts[out_transfer_instruction.accounts[0] as usize]).unwrap_or(&SOL_MINT.to_string()).clone();

//     pb::raydium::RaydiumSwapData {
//         amount_in,
//         token_in,
//         amount_out,
//         token_out,
//     }
// }
