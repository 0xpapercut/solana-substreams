use std::collections::HashMap;
use bs58;

use substreams::errors::Error;
use substreams_solana::pb::sf::solana::r#type::v1::ConfirmedTransaction;
use substreams_solana::pb::sf::solana::r#type::v1::Block;
use structured_instructions::{get_structured_instructions, StructuredInstruction};

use substreams_solana_raydium_amm as raydium_amm;
use raydium_amm::instruction::AmmInstruction;
use raydium_amm::RAYDIUM_LIQUIDITY_POOL;

use spl_token_substream;

mod spl_token;
use spl_token::instruction::TokenInstruction;
use spl_token::TOKEN_PROGRAM;

mod utils;
use utils::{get_token_accounts, TokenAccount};

mod pb;

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
    let mut transactions_events: Vec<pb::raydium::TransactionRaydiumEvents> = Vec::new();
    for transaction in block.transactions {
        let events = parse_transaction(&transaction);
        if !events.is_empty() {
            let signature = bs58::encode(transaction.transaction.as_ref().unwrap().signatures.get(0).unwrap()).into_string();
            transactions_events.push(pb::raydium::TransactionRaydiumEvents {
                signature,
                events,
            });
        }
    }
    transactions_events
}

fn parse_transaction(transaction: &ConfirmedTransaction) -> Vec<pb::raydium::RaydiumEvent> {
    let mut events: Vec<pb::raydium::RaydiumEvent> = Vec::new();

    let instructions = get_structured_instructions(transaction);
    let accounts = transaction.resolved_accounts_as_strings();
    let txn = transaction.transaction.as_ref().unwrap();
    let meta = transaction.meta.as_ref().unwrap();
    let token_accounts = get_token_accounts(transaction);
    let signature = bs58::encode(&txn.signatures[0]).into_string();

    if let Some(_) = meta.err.as_ref() {
        return Vec::new();
    }

    for instr in instructions {
        for event in parse_instruction(&instr, &accounts, &token_accounts) {
            events.push(event);
        }
    }

    events
}

fn parse_instruction(instruction: &StructuredInstruction, accounts: &Vec<String>, token_accounts: &HashMap<String, TokenAccount>) -> Vec<pb::raydium::RaydiumEvent> {
    let mut events: Vec<pb::raydium::RaydiumEvent> = Vec::new();
    if accounts[instruction.program_id_index as usize] != RAYDIUM_LIQUIDITY_POOL {
        for instr in &instruction.inner_instructions {
            events.extend(parse_instruction(instr, accounts, token_accounts));
        }
    } else if let Ok(event) = parse_raydium_instruction(instruction, accounts, token_accounts) {
        events.push(event);
    }
    events
}

fn parse_raydium_instruction(instruction: &StructuredInstruction, accounts: &Vec<String>, token_accounts: &HashMap<String, TokenAccount>) -> Result<pb::raydium::RaydiumEvent, String> {
    match AmmInstruction::unpack(&instruction.data) {
        Ok(unpacked) => match unpacked {
            AmmInstruction::SwapBaseIn(_) |
            AmmInstruction::SwapBaseOut(_) => {
                Ok(parse_raydium_swap_instruction(instruction, accounts, token_accounts))
            },
            AmmInstruction::Initialize2(initialize) => {
                unimplemented!()
            }
            AmmInstruction::Deposit(deposit) => {
                unimplemented!()
            },
            AmmInstruction::Withdraw(withdraw) => {
                unimplemented!()
            }
            _ => Err(format!("Unsupported instruction {:#?}.", unpacked)),
        },
        Err(_) => Err("Not a Raydium event.".to_string())
    }
}

fn parse_raydium_swap_instruction(instruction: &StructuredInstruction, accounts: &Vec<String>, token_accounts: &HashMap<String, TokenAccount>) -> pb::raydium::RaydiumEvent {
    let amm = accounts[instruction.accounts[1] as usize].clone();

    // Sometimes OpenBook is also invoked, so we filter the inner instructions
    let inner_instructions: Vec<_> = instruction.inner_instructions.iter().filter(|x| accounts[x.program_id_index as usize] == TOKEN_PROGRAM).collect();
    let transfer_in = parse_token_transfer_instruction(inner_instructions[0], accounts);
    let transfer_out = parse_token_transfer_instruction(inner_instructions[1], accounts);

    let amount_in = transfer_in.amount;
    let amount_out = transfer_out.amount;

    let mint_in: String;
    if let Some(token_account) = token_accounts.get(&transfer_in.source) {
        mint_in = token_account.mint.clone()
    } else {
        let token_account = token_accounts.get(&transfer_in.destination).unwrap();
        mint_in = token_account.mint.clone();
    }

    let mint_out: String;
    if let Some(token_account) = token_accounts.get(&transfer_out.source) {
        mint_out = token_account.mint.clone()
    } else {
        let token_account = token_accounts.get(&transfer_out.destination).unwrap();
        mint_out = token_account.mint.clone();
    }

    pb::raydium::RaydiumEvent {
        amm,
        data: Some(pb::raydium::raydium_event::Data::Swap(pb::raydium::SwapData {
            mint_in,
            mint_out,
            amount_in,
            amount_out,
        })),
    }
}

fn parse_token_transfer_instruction(instruction: &StructuredInstruction, accounts: &Vec<String>) -> TokenTransfer {
    match TokenInstruction::unpack(&instruction.data).unwrap() {
        TokenInstruction::Transfer { amount } => {
            let source = accounts[instruction.accounts[0] as usize].clone();
            let destination = accounts[instruction.accounts[1] as usize].clone();
            TokenTransfer {
                amount,
                source,
                destination,
            }
        },
        TokenInstruction::TransferChecked { amount, decimals: _ } => {
            let source = accounts[instruction.accounts[0] as usize].clone();
            let destination = accounts[instruction.accounts[1] as usize].clone();
            TokenTransfer {
                amount,
                source,
                destination,
            }
        }
        _ => panic!(),
    }
}

pub struct TokenTransfer {
    pub source: String,
    pub destination: String,
    pub amount: u64,
}
