mod pb;

use std::collections::HashMap;

use pb::sf::solana::block_meta::v1::BlockMeta;
use substreams_solana::pb::sf::solana::r#type::v1::Block;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_database_change::tables::Tables as DatabaseChangeTables;
use pb::swap::Swap;
use pb::swap::Swaps;
use bs58;

const RAYDIUM_LIQUIDITY_POOL: &str = "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8";
const SOL_MINT: &str = "So11111111111111111111111111111111111111112";

#[substreams::handlers::map]
fn map_swap(block: Block) -> Result<Swaps, substreams::errors::Error> {
    let mut swaps: Vec<Swap> = Vec::new();

    for successfulTxn in block.transactions {
        let accounts = successfulTxn.resolved_accounts_as_strings();
        let signature = bs58::encode(successfulTxn.clone().transaction.unwrap().signatures[0].clone()).into_string();
        let meta = successfulTxn.clone().meta.unwrap();

        if meta.clone().err.is_some() {
            continue;
        }

        let mut mints: HashMap<String, String> = HashMap::new();
        for token_balance in meta.clone().pre_token_balances {
            mints.insert(accounts[token_balance.account_index as usize].clone(), token_balance.mint);
        }

        for (index, instruction) in successfulTxn.instructions().enumerate() {
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

            let user_source_token_account = accounts[instruction.instruction.accounts[15] as usize].clone();
            let user_dest_token_account = accounts[instruction.instruction.accounts[16] as usize].clone();

            let token_in = mints.get(&user_source_token_account).unwrap_or(&SOL_MINT.to_string()).clone();
            let token_out = mints.get(&user_dest_token_account).unwrap_or(&SOL_MINT.to_string()).clone();

            let amm = accounts[instruction.instruction.accounts[1] as usize].clone();

            swaps.push(Swap {
                signer: accounts[0].clone(),
                token_in,
                token_out,
                amount_in,
                amount_out,
                signature: signature.clone(),
                amm,
            });
        }

        for inner_instructions in meta.clone().inner_instructions {
            for (i, inner_instruction) in inner_instructions.instructions.clone().iter().enumerate() {
                let program_id = &accounts[inner_instruction.program_id_index as usize];
                if program_id != RAYDIUM_LIQUIDITY_POOL {
                    continue;
                }

                let user_source_token_account = accounts[inner_instruction.accounts[15] as usize].clone();
                let user_dest_token_account = accounts[inner_instruction.accounts[16] as usize].clone();

                let data = inner_instructions.instructions[i + 1].data.clone();
                let amount_in = u64::from_le_bytes(data[1..9].try_into().expect("Slice with incorrect length."));

                let data = inner_instructions.instructions[i + 2].data.clone();
                let amount_out = u64::from_le_bytes(data[1..9].try_into().expect("Slice with incorrect length."));

                let token_in = mints.get(&user_source_token_account).unwrap_or(&SOL_MINT.to_string()).clone();
                let token_out = mints.get(&user_dest_token_account).unwrap_or(&SOL_MINT.to_string()).clone();

                let amm = accounts[inner_instruction.accounts[1] as usize].clone();

                swaps.push(Swap {
                    signer: accounts[0].clone(),
                    token_in,
                    token_out,
                    amount_in,
                    amount_out,
                    signature: signature.clone(),
                    amm,
                })
            }
        }
    }
    // swaps = swaps.iter().filter(|x| x.amm == "EGG6GngmG1sHKSMQr2aKo9HuMuLGXoP3DJ6FHkBaJFo3").cloned().collect();
    return Ok(Swaps {swaps});
}

#[substreams::handlers::map]
fn db_out(bm: BlockMeta) -> Result<DatabaseChanges, substreams::errors::Error> {
    // Initialize changes container
    let mut tables = DatabaseChangeTables::new();

    tables
        .create_row("block", [("hash", bm.hash)])
        .set("parent_hash", bm.parent_hash)
        .set("block_height", bm.slot)
        .set("transaction_count", bm.transaction_count);
    Ok(tables.to_database_changes())
}
