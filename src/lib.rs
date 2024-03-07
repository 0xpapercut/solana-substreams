mod pb;

use std::collections::HashMap;

use substreams_solana::pb::sf::solana::r#type::v1::Block;
use pb::swap::{Swap, Swaps};
use bs58;

const RAYDIUM_LIQUIDITY_POOL: &str = "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8";
const SOL_MINT: &str = "So11111111111111111111111111111111111111112";

#[substreams::handlers::map]
fn swaps(block: Block) -> Result<Swaps, substreams::errors::Error> {
    let mut swaps: Vec<Swap> = Vec::new();

    for successful_txn in block.transactions {
        let accounts = successful_txn.resolved_accounts_as_strings();
        let signature = bs58::encode(successful_txn.clone().transaction.unwrap().signatures[0].clone()).into_string();
        let meta = successful_txn.clone().meta.unwrap();

        if meta.clone().err.is_some() {
            continue;
        }

        let mut mints: HashMap<String, String> = HashMap::new();
        for token_balance in meta.clone().pre_token_balances {
            mints.insert(accounts[token_balance.account_index as usize].clone(), token_balance.mint);
        }

        for (index, instruction) in successful_txn.instructions().enumerate() {
            let program_id = accounts[instruction.instruction.program_id_index as usize].as_str();
            if program_id != RAYDIUM_LIQUIDITY_POOL {
                continue;
            }
            if instruction.instruction.accounts.len() <= 15 {
                break;
            }
            let filtered = meta.inner_instructions.iter().find(|inner| inner.index == index as u32);
            if filtered.is_none() {
                break;
            }
            let inner_instructions = filtered.unwrap();
            if inner_instructions.instructions.len() == 1 {
                break;
            }

            let data = inner_instructions.instructions[0].data.clone();
            if data.len() == 1 {
                break; // remove later
            }
            let amount_in = u64::from_le_bytes(data[1..9].try_into().expect("Slice with incorrect length."));

            let data = inner_instructions.instructions[1].data.clone();
            if data.len() == 1 {
                break; // remove later
            }
            let amount_out = u64::from_le_bytes(data[1..9].try_into().expect("Slice with incorrect length."));

            let token_in = mints.get(&accounts[inner_instructions.instructions[0].accounts[0] as usize]).unwrap_or(&SOL_MINT.to_string()).clone();
            let token_out = mints.get(&accounts[inner_instructions.instructions[1].accounts[0] as usize]).unwrap_or(&SOL_MINT.to_string()).clone();

            let amm = accounts[instruction.instruction.accounts[1] as usize].clone();

            swaps.push(Swap {
                signer: accounts[0].clone(),
                token_in,
                token_out,
                amount_in,
                amount_out,
                signature: signature.clone(),
                amm,
                slot: block.slot,
            });
        }

        for inner_instructions in meta.clone().inner_instructions {
            for (i, inner_instruction) in inner_instructions.instructions.clone().iter().enumerate() {
                let program_id = &accounts[inner_instruction.program_id_index as usize];
                if program_id != RAYDIUM_LIQUIDITY_POOL {
                    continue;
                }

                let data = &inner_instructions.instructions.get(i + 1)
                    .ok_or(substreams::errors::Error::msg(format!("{}:{} - Failed to process transaction {}", file!(), line!(), signature)))?
                    .data;
                let amount_in = u64::from_le_bytes(data[1..9].try_into().expect("Slice with incorrect length."));

                let data = &inner_instructions.instructions.get(i + 2)
                    .ok_or(substreams::errors::Error::msg(format!("{}:{} - Failed to process transaction {}", file!(), line!(), signature)))?
                    .data;
                let amount_out = u64::from_le_bytes(data[1..9].try_into().expect("Slice with incorrect length."));

                let token_in = mints.get(&accounts[inner_instructions.instructions[i + 1].accounts[0] as usize]).unwrap_or(&SOL_MINT.to_string()).clone();
                let token_out = mints.get(&accounts[inner_instructions.instructions[i + 2].accounts[0] as usize]).unwrap_or(&SOL_MINT.to_string()).clone();

                let amm = accounts[inner_instruction.accounts[1] as usize].clone();

                swaps.push(Swap {
                    signer: accounts[0].clone(),
                    token_in,
                    token_out,
                    amount_in,
                    amount_out,
                    signature: signature.clone(),
                    amm,
                    slot: block.slot,
                })
            }
        }
    }

    Ok(Swaps {swaps})
}
