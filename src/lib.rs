mod pb;
mod raydium_amm;
mod spl_token;
mod utils;

use std::collections::HashMap;

use substreams::errors::Error;
use substreams_solana::pb::sf::solana::r#type::v1::InnerInstruction;
use substreams_solana::pb::sf::solana::r#type::v1::{Block, InnerInstructions};

use bs58;

use raydium_amm::instruction::AmmInstruction;
use spl_token::instruction::TokenInstruction;
use crate::raydium_amm::RAYDIUM_LIQUIDITY_POOL;

const SOL_MINT: &str = "So11111111111111111111111111111111111111112";

#[substreams::handlers::map]
pub fn events(block: Block) -> Result<pb::raydium::Events, Error> {
    let events = get_raydium_events(block);
    Ok(pb::raydium::Events { events: events })
}

pub fn get_token_events(block: Block) -> Vec<pb::raydium::Event> {
    let mut events: Vec<pb::raydium::Event> = Vec::new();

    for transaction in block.transactions {
        let accounts = transaction.resolved_accounts_as_strings();

        let meta = transaction.meta.unwrap();
        let txn = transaction.transaction.unwrap();

        if let Some(err) = meta.err {
            continue;
        }

        let signature = bs58::encode(&txn.signatures[0]).into_string();
        let inner_instructions = &meta.inner_instructions;
        let message = &txn.message.unwrap();

        let mut owners: HashMap<String, String> = HashMap::new();
        for token_balance in meta.clone().pre_token_balances {
            owners.insert(
                accounts[token_balance.account_index as usize].clone(),
                token_balance.mint
            );
        }

        let signer = accounts[0].clone(); // TODO: There might be more than one signer.

        // Token Program was called directly
        // for (i, instruction) in message.instructions.iter().enumerate() {
        //     if &accounts[instruction.program_id_index as usize] != RAYDIUM_LIQUIDITY_POOL {
        //         continue;
        //     }

        //     match
        // }
    }

    events
}

pub fn get_raydium_events(block: Block) -> Vec<pb::raydium::Event> {
    let mut events: Vec<pb::raydium::Event> = Vec::new();

    for transaction in block.transactions {
        let accounts = transaction.resolved_accounts_as_strings();

        let meta = transaction.meta.unwrap();
        let txn = transaction.transaction.unwrap();

        if let Some(err) = meta.err {
            continue;
        }

        let signature = bs58::encode(&txn.signatures[0]).into_string();
        let inner_instructions = &meta.inner_instructions;
        let message = &txn.message.unwrap();

        let mut owners: HashMap<String, String> = HashMap::new();
        for token_balance in meta.clone().pre_token_balances {
            owners.insert(
                accounts[token_balance.account_index as usize].clone(),
                token_balance.mint,
            );
        }

        let signer = accounts[0].clone(); // TODO: There might be more than one signer.

        // Raydium was called directly
        for (i, instruction) in message.instructions.iter().enumerate() {
            if &accounts[instruction.program_id_index as usize] != RAYDIUM_LIQUIDITY_POOL {
                continue;
            }

            match AmmInstruction::unpack(&instruction.data).unwrap() {
                AmmInstruction::Deposit(deposit) => {}
                AmmInstruction::Initialize2(initialize) => {}
                AmmInstruction::SwapBaseIn(swap_base_in) => {
                    let token_program_key = instruction.accounts[0];
                    let instructions = &find_inner_instructions(inner_instructions, i as u32).unwrap().instructions;
                    let transfer_instructions = find_swap_transfer_instructions(token_program_key, instructions, 0);

                    let amm = &accounts[instruction.accounts[1] as usize];
                    let slot = block.slot;

                    let raydium_event = get_raydium_swap_event(&transfer_instructions, &accounts, &owners, amm);
                    events.push(pb::raydium::Event {
                        program: pb::raydium::Program::Raydium.into(),
                        program_id: RAYDIUM_LIQUIDITY_POOL.to_string(),
                        signer: signer.clone(),
                        signature: signature.clone(),
                        slot,
                        event: Some(pb::raydium::event::Event::Raydium(raydium_event))
                    });
                }
                AmmInstruction::SwapBaseOut(swap_base_out) => {
                    let token_program_key = instruction.accounts[0];
                    let instructions = &find_inner_instructions(inner_instructions, i as u32).unwrap().instructions;
                    let transfer_instructions = find_swap_transfer_instructions(token_program_key, instructions, 0);

                    let amm = &accounts[instruction.accounts[1] as usize];
                    let slot = block.slot;

                    let raydium_event = get_raydium_swap_event(&transfer_instructions, &accounts, &owners, amm);
                    events.push(pb::raydium::Event {
                        program: pb::raydium::Program::Raydium.into(),
                        program_id: RAYDIUM_LIQUIDITY_POOL.to_string(),
                        signer: signer.clone(),
                        signature: signature.clone(),
                        slot,
                        event: Some(pb::raydium::event::Event::Raydium(raydium_event))
                    });
                }
                _ => (),
            }
        }

        // Raydium was invoked from another program
        for instructions in inner_instructions {
            for (i, instruction) in instructions.instructions.iter().enumerate() {
                if &accounts[instruction.program_id_index as usize] != RAYDIUM_LIQUIDITY_POOL {
                    continue;
                }

                match AmmInstruction::unpack(&instruction.data).unwrap() {
                    AmmInstruction::Deposit(deposit) => {}
                    AmmInstruction::Initialize2(initialize) => {}
                    AmmInstruction::SwapBaseIn(swap_base_in) => {
                        let amm = &accounts[instruction.accounts[1] as usize].clone();
                        let token_program_key = instruction.accounts[0];
                        let transfer_instructions = &find_swap_transfer_instructions(token_program_key, &instructions.instructions, i);

                        let raydium_event = get_raydium_swap_event(&transfer_instructions, &accounts, &owners, amm);
                        events.push(pb::raydium::Event {
                            program: pb::raydium::Program::Raydium.into(),
                            program_id: RAYDIUM_LIQUIDITY_POOL.to_string(),
                            signer: signer.clone(),
                            signature: signature.clone(),
                            slot: block.slot,
                            event: Some(pb::raydium::event::Event::Raydium(raydium_event))
                        });
                    }
                    AmmInstruction::SwapBaseOut(swap_base_out) => {
                        let amm = &accounts[instruction.accounts[1] as usize].clone();
                        let token_program_key = instruction.accounts[0];
                        let transfer_instructions = &find_swap_transfer_instructions(token_program_key, &instructions.instructions, i);

                        let raydium_event = get_raydium_swap_event(&transfer_instructions, &accounts, &owners, amm);
                        events.push(pb::raydium::Event {
                            program: pb::raydium::Program::Raydium.into(),
                            program_id: RAYDIUM_LIQUIDITY_POOL.to_string(),
                            signer: signer.clone(),
                            signature: signature.clone(),
                            slot: block.slot,
                            event: Some(pb::raydium::event::Event::Raydium(raydium_event))
                        });
                    }
                    _ => (),
                }
            }
        }
    }

    events
}

fn get_raydium_swap_event<'a>(
    transfer_instructions: &[&InnerInstruction; 2],
    accounts: &Vec<String>,
    owners: &'a HashMap<String, String>,
    amm: &String,
) -> pb::raydium::RaydiumEvent {
    let data = get_swap_data(transfer_instructions, accounts, owners);
    pb::raydium::RaydiumEvent {
        r#type: pb::raydium::RaydiumEventType::Swap.into(),
        amm: amm.clone(),
        data: Some(pb::raydium::raydium_event::Data::Swap(data))
    }
}

fn get_swap_data<'a>(
    transfer_instructions: &[&InnerInstruction; 2],
    accounts: &Vec<String>,
    owners: &'a HashMap<String, String>,
) -> pb::raydium::RaydiumSwapData {
    let in_transfer_instruction = &transfer_instructions[0];
    let out_transfer_instruction = &transfer_instructions[1];

    let amount_in = match TokenInstruction::unpack(&in_transfer_instruction.data).unwrap() {
        TokenInstruction::Transfer { amount } => amount,
        _ => {
            panic!();
        }
    };

    let amount_out = match TokenInstruction::unpack(&out_transfer_instruction.data).unwrap() {
        TokenInstruction::Transfer { amount } => amount,
        _ => {
            panic!();
        }
    };

    let token_in = owners.get(&accounts[in_transfer_instruction.accounts[0] as usize]).unwrap_or(&SOL_MINT.to_string()).clone();
    let token_out = owners.get(&accounts[out_transfer_instruction.accounts[0] as usize]).unwrap_or(&SOL_MINT.to_string()).clone();

    pb::raydium::RaydiumSwapData {
        amount_in,
        token_in,
        amount_out,
        token_out,
    }
}

fn find_inner_instructions<'a>(
    inner_instructions: &'a Vec<InnerInstructions>,
    index: u32,
) -> Option<&'a InnerInstructions> {
    inner_instructions.iter().find(|inner| inner.index == index)
}

fn find_swap_transfer_instructions<'a>(token_program_key: u8, instructions: &'a Vec<InnerInstruction>, from: usize) -> [&'a InnerInstruction; 2] {
    for (j, instruction) in instructions.iter().skip(from as usize).enumerate() {
        if instruction.program_id_index == token_program_key as u32 {
            return [
                &instructions[from + j],
                &instructions[from + j + 1],
            ];
        }
    }
    panic!();
}
