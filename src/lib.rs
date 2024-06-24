use std::collections::HashMap;
use bs58;

use substreams::errors::Error;
use substreams_solana::pb::sf::solana::r#type::v1::ConfirmedTransaction;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

use substreams_solana_program_instructions::pubkey::Pubkey;
use structured_instructions::{
    get_structured_instructions,
    StructuredInstruction,
    StructuredInstructions,
};

use substreams_solana_spl_token::{
    TokenInstruction,
    TOKEN_PROGRAM,
};
use substreams_solana_spl_token as spl_token;

use utils::ConfirmedTransactionExt;

mod pb;
mod utils;

#[substreams::handlers::map]
fn events(block: Block) -> Result<pb::spl_token::Events, Error> {
    let events = parse_block(block);
    Ok(pb::spl_token::Events { events })
}

fn parse_block(block: Block) -> Vec<pb::spl_token::Event> {
    let mut events: Vec<pb::spl_token::Event> = Vec::new();
    for confirmed_transaction in block.transactions() {
        events.extend(parse_confirmed_transaction(confirmed_transaction, block.slot));
    }
    events
}

fn parse_confirmed_transaction(confirmed_transaction: &ConfirmedTransaction, slot: u64) -> Vec<pb::spl_token::Event> {
    let mut events: Vec<pb::spl_token::Event> = Vec::new();

    let instructions = get_structured_instructions(&confirmed_transaction);
    let accounts = confirmed_transaction.resolved_accounts();
    let mut token_accounts = utils::get_token_accounts(confirmed_transaction);

    if let Some(_) = confirmed_transaction.meta.as_ref().unwrap().err {
        return Vec::new();
    }

    let transaction = pb::spl_token::TransactionData {
        signature: bs58::encode(confirmed_transaction.signature()).into_string(),
        slot,
    };
    for instruction in instructions.flattened() {
        // if bs58::encode(accounts[instruction.program_id_index as usize]).into_string() == TOKEN_PROGRAM {
        //     substreams::log::println(format!("{}", bs58::encode(confirmed_transaction.signature()).into_string()));
        // }
        if bs58::encode(accounts[instruction.program_id_index as usize]).into_string() != TOKEN_PROGRAM {
            continue;
        }
        match parse_instruction(&instruction, &accounts, &mut token_accounts) {
            Ok(event) => {
                if event.is_none() {
                    continue;
                }
                events.push(pb::spl_token::Event {
                    transaction: Some(transaction.clone()),
                    event: Some(pb::spl_token::SplTokenEvent {
                        event
                    })
                })
            }
            Err(e) => panic!("Transaction {}: {}", transaction.signature, e),
        }
    }

    // let events: Vec<_> = events.iter().filter(|x| match x.event.as_ref().unwrap() {
    //     pb::spl_token::event::Event::SetAuthority(_) => true,
    //     _ => false,
    // }).cloned().collect();
    events
}

pub fn parse_instruction(
    instruction: &StructuredInstruction,
    accounts: &Vec<&Vec<u8>>,
    token_accounts: &mut HashMap<Vec<u8>, utils::TokenAccount>
) -> Result<Option<pb::spl_token::spl_token_event::Event>, &'static str> {
    if bs58::encode(accounts[instruction.program_id_index as usize]).into_string() != TOKEN_PROGRAM {
        return Err("Not a Token program instruction.");
    }

    let unpacked = TokenInstruction::unpack(&instruction.data);
    if unpacked.is_err() {
        return Err("Failed to parse Token program instruction.");
    }

    match unpacked.unwrap() {
        TokenInstruction::InitializeMint { decimals, mint_authority, freeze_authority } |
        TokenInstruction::InitializeMint2 { decimals, mint_authority, freeze_authority } => {
            let event = parse_initialize_mint_instruction(instruction, accounts, token_accounts, decimals as u32, mint_authority, freeze_authority)?;
            Ok(Some(pb::spl_token::spl_token_event::Event::InitializeMint(event)))
        },

        TokenInstruction::InitializeAccount => {
            let event = parse_initialize_account_instruction(instruction, accounts, token_accounts, None)?;
            Ok(Some(pb::spl_token::spl_token_event::Event::InitializeAccount(event)))
        },
        TokenInstruction::InitializeAccount2 { owner } |
        TokenInstruction::InitializeAccount3 { owner } => {
            let event = parse_initialize_account_instruction(instruction, accounts, token_accounts, Some(owner))?;
            Ok(Some(pb::spl_token::spl_token_event::Event::InitializeAccount(event)))
        },

        TokenInstruction::InitializeMultisig { m } |
        TokenInstruction::InitializeMultisig2 { m } => {
            let event = parse_initialize_multisig_instruction(instruction, accounts, token_accounts, m)?;
            Ok(Some(pb::spl_token::spl_token_event::Event::InitializeMultisig(event)))
        },

        TokenInstruction::Transfer { amount } => {
            let event = parse_transfer_instruction(instruction, accounts, token_accounts, amount, None)?;
            Ok(Some(pb::spl_token::spl_token_event::Event::Transfer(event)))
        },
        TokenInstruction::TransferChecked { amount, decimals } => {
            let event = parse_transfer_instruction(instruction, accounts, token_accounts, amount, Some(decimals))?;
            Ok(Some(pb::spl_token::spl_token_event::Event::Transfer(event)))
        },

        TokenInstruction::Approve { amount } => {
            let event = parse_approve_instruction(instruction, accounts, token_accounts, amount, None)?;
            Ok(Some(pb::spl_token::spl_token_event::Event::Approve(event)))
        },
        TokenInstruction::ApproveChecked { amount, decimals } => {
            let event = parse_approve_instruction(instruction, accounts, token_accounts, amount, Some(decimals))?;
            Ok(Some(pb::spl_token::spl_token_event::Event::Approve(event)))
        },

        TokenInstruction::Revoke => {
            let event = parse_revoke_instruction(instruction, accounts, token_accounts)?;
            Ok(Some(pb::spl_token::spl_token_event::Event::Revoke(event)))
        },

        TokenInstruction::SetAuthority { authority_type, new_authority } => {
            let event = parse_set_authority_instruction(instruction, accounts, token_accounts, authority_type, new_authority)?;
            Ok(Some(pb::spl_token::spl_token_event::Event::SetAuthority(event)))
        },

        TokenInstruction::MintTo { amount } => {
            let event = parse_mint_to_instruction(instruction, accounts, token_accounts, amount)?;
            Ok(Some(pb::spl_token::spl_token_event::Event::MintTo(event)))
        },
        TokenInstruction::MintToChecked { amount, decimals } => {
            let event = parse_mint_to_instruction(instruction, accounts, token_accounts, amount)?;
            Ok(Some(pb::spl_token::spl_token_event::Event::MintTo(event)))
        },

        TokenInstruction::Burn { amount } => {
            let event = parse_burn_instruction(instruction, accounts, token_accounts, amount)?;
            Ok(Some(pb::spl_token::spl_token_event::Event::Burn(event)))
        },
        TokenInstruction::BurnChecked { amount, decimals } => {
            let event = parse_burn_instruction(instruction, accounts, token_accounts, amount)?;
            Ok(Some(pb::spl_token::spl_token_event::Event::Burn(event)))
        },

        TokenInstruction::CloseAccount => {
            let event = parse_close_account_instruction(instruction, accounts, token_accounts)?;
            Ok(Some(pb::spl_token::spl_token_event::Event::CloseAccount(event)))
        },

        TokenInstruction::FreezeAccount => {
            let event = parse_freeze_account_instruction(instruction, accounts, token_accounts)?;
            Ok(Some(pb::spl_token::spl_token_event::Event::FreezeAccount(event)))
        },

        TokenInstruction::ThawAccount => {
            let event = parse_thaw_account_instruction(instruction, accounts, token_accounts)?;
            Ok(Some(pb::spl_token::spl_token_event::Event::ThawAccount(event)))
        },

        TokenInstruction::InitializeImmutableOwner => {
            let event = parse_initialize_immutable_owner_instruction(instruction, accounts, token_accounts)?;
            Ok(Some(pb::spl_token::spl_token_event::Event::InitializeImmutableOwner(event)))
        },

        TokenInstruction::SyncNative => Ok(None),
        TokenInstruction::AmountToUiAmount { amount } => Ok(None),
        TokenInstruction::GetAccountDataSize => Ok(None),
        TokenInstruction::UiAmountToAmount { ui_amount } => Ok(None),
    }
}

fn parse_initialize_mint_instruction(
    instruction: &StructuredInstruction,
    accounts: &Vec<&Vec<u8>>,
    token_accounts: &HashMap<Vec<u8>, utils::TokenAccount>,
    decimals: u32,
    mint_authority: Pubkey,
    freeze_authority: Option<Pubkey>,
) -> Result<pb::spl_token::InitializeMintEvent, &'static str> {
    let mint = bs58::encode(accounts[instruction.accounts[0] as usize]).into_string();
    let mint_authority = bs58::encode(mint_authority).into_string();
    let freeze_authority = freeze_authority.map(|x| bs58::encode(x).into_string());

    Ok(pb::spl_token::InitializeMintEvent {
        mint,
        decimals,
        mint_authority,
        freeze_authority,
    })
}

fn parse_initialize_account_instruction(
    instruction: &StructuredInstruction,
    accounts: &Vec<&Vec<u8>>,
    token_accounts: &mut HashMap<Vec<u8>, utils::TokenAccount>,
    owner: Option<Pubkey>,
) -> Result<pb::spl_token::InitializeAccountEvent, &'static str> {
    let address = accounts[instruction.accounts[0] as usize];
    let mint = accounts[instruction.accounts[1] as usize].clone();
    let owner = owner.map(|x| x.to_bytes().to_vec()).unwrap_or_else(|| accounts[instruction.accounts[2] as usize].clone());

    token_accounts.entry(address.clone()).or_insert(utils::TokenAccount{
        address: address.clone(),
        mint,
        owner,
    });

    Ok(pb::spl_token::InitializeAccountEvent {
        account: Some(token_accounts[address].clone().into())
    })
}

fn parse_initialize_multisig_instruction(
    instruction: &StructuredInstruction,
    accounts: &Vec<&Vec<u8>>,
    token_accounts: &HashMap<Vec<u8>, utils::TokenAccount>,
    m: u8,
) -> Result<pb::spl_token::InitializeMultisigEvent, &'static str> {
    let multisig = accounts[instruction.accounts[0] as usize];

    Ok(pb::spl_token::InitializeMultisigEvent::default())
}

fn parse_transfer_instruction(
    instruction: &StructuredInstruction,
    accounts: &Vec<&Vec<u8>>,
    token_accounts: &HashMap<Vec<u8>, utils::TokenAccount>,
    amount: u64,
    expected_decimals: Option<u8>,
) -> Result<pb::spl_token::TransferEvent, &'static str> {
    let delta: usize = if expected_decimals.is_none() { 0 } else { 1 };

    let source = &token_accounts[accounts[instruction.accounts[0] as usize]];
    let destination = &token_accounts[accounts[instruction.accounts[1 + delta] as usize]];

    Ok(pb::spl_token::TransferEvent {
        source: Some(source.clone().into()),
        destination: Some(destination.clone().into()),
        amount,
    })
}

fn parse_approve_instruction(
    instruction: &StructuredInstruction,
    accounts: &Vec<&Vec<u8>>,
    token_accounts: &HashMap<Vec<u8>, utils::TokenAccount>,
    amount: u64,
    expected_decimals: Option<u8>,
) -> Result<pb::spl_token::ApproveEvent, &'static str> {
    let delta: usize = if expected_decimals.is_none() { 0 } else { 1 };

    let source = &token_accounts[accounts[instruction.accounts[0] as usize]];
    let delegate = bs58::encode(accounts[instruction.accounts[1 + delta] as usize]).into_string();

    Ok(pb::spl_token::ApproveEvent {
        source: Some(source.clone().into()),
        delegate,
        amount,
    })
}

fn parse_revoke_instruction(
    instruction: &StructuredInstruction,
    accounts: &Vec<&Vec<u8>>,
    token_accounts: &HashMap<Vec<u8>, utils::TokenAccount>,
) -> Result<pb::spl_token::RevokeEvent, &'static str> {
    let source = &token_accounts[accounts[instruction.accounts[0] as usize]];

    Ok(pb::spl_token::RevokeEvent {
        source: Some(source.clone().into()),
    })
}

fn parse_set_authority_instruction(
    instruction: &StructuredInstruction,
    accounts: &Vec<&Vec<u8>>,
    token_accounts: &HashMap<Vec<u8>, utils::TokenAccount>,
    authority_type: spl_token::AuthorityType,
    new_authority: Option<Pubkey>,
) -> Result<pb::spl_token::SetAuthorityEvent, &'static str> {
    let account = accounts[instruction.accounts[0] as usize];
    let authority = accounts[instruction.accounts[1] as usize];
    substreams::log::println(format!("{:#?}", authority_type));
    let authority_type: i32 = match authority_type {
        spl_token::AuthorityType::MintTokens => pb::spl_token::AuthorityType::MintTokens.into(),
        spl_token::AuthorityType::FreezeAccount => pb::spl_token::AuthorityType::FreezeAccount.into(),
        spl_token::AuthorityType::AccountOwner => pb::spl_token::AuthorityType::AccountOwner.into(),
        spl_token::AuthorityType::CloseAccount => pb::spl_token::AuthorityType::CloseAccount.into(),
    };
    unimplemented!()
}

fn parse_mint_to_instruction(
    instruction: &StructuredInstruction,
    accounts: &Vec<&Vec<u8>>,
    token_accounts: &HashMap<Vec<u8>, utils::TokenAccount>,
    amount: u64,
) -> Result<pb::spl_token::MintToEvent, &'static str> {
    let mint = bs58::encode(accounts[instruction.accounts[0] as usize]).into_string();
    let destination = &token_accounts[accounts[instruction.accounts[1] as usize]];

    Ok(pb::spl_token::MintToEvent {
        destination: Some(destination.clone().into()),
        amount,
        mint,
    })
}

fn parse_burn_instruction(
    instruction: &StructuredInstruction,
    accounts: &Vec<&Vec<u8>>,
    token_accounts: &HashMap<Vec<u8>, utils::TokenAccount>,
    amount: u64,
    // expected_decimals: Option<u8>,
) -> Result<pb::spl_token::BurnEvent, &'static str> {
    let source = &token_accounts[accounts[instruction.accounts[0] as usize]];

    Ok(pb::spl_token::BurnEvent {
        source: Some(source.clone().into()),
        amount,
    })
}

fn parse_close_account_instruction(
    instruction: &StructuredInstruction,
    accounts: &Vec<&Vec<u8>>,
    token_accounts: &HashMap<Vec<u8>, utils::TokenAccount>,
) -> Result<pb::spl_token::CloseAccountEvent, &'static str> {
    let source = token_accounts.get(accounts[instruction.accounts[0] as usize])
        .ok_or("parse_close_account_instruction: source account not found.")?;
    let destination = bs58::encode(accounts[instruction.accounts[1] as usize]).into_string();

    Ok(pb::spl_token::CloseAccountEvent {
        source: Some(source.clone().into()),
        destination,
    })
}

fn parse_freeze_account_instruction(
    instruction: &StructuredInstruction,
    accounts: &Vec<&Vec<u8>>,
    token_accounts: &HashMap<Vec<u8>, utils::TokenAccount>,
) -> Result<pb::spl_token::FreezeAccountEvent, &'static str> {
    let source = &token_accounts[accounts[instruction.accounts[0] as usize]];

    Ok(pb::spl_token::FreezeAccountEvent {
        source: Some(source.clone().into()),
    })
}

fn parse_thaw_account_instruction(
    instruction: &StructuredInstruction,
    accounts: &Vec<&Vec<u8>>,
    token_accounts: &HashMap<Vec<u8>, utils::TokenAccount>,
) -> Result<pb::spl_token::ThawAccountEvent, &'static str> {
    let source = &token_accounts[accounts[instruction.accounts[0] as usize]];

    Ok(pb::spl_token::ThawAccountEvent {
        source: Some(source.clone().into()),
    })
}

fn parse_initialize_immutable_owner_instruction(
    instruction: &StructuredInstruction,
    accounts: &Vec<&Vec<u8>>,
    token_accounts: &HashMap<Vec<u8>, utils::TokenAccount>,
) -> Result<pb::spl_token::InitializeImmutableOwnerEvent, &'static str> {
    // let account = &token_accounts[accounts[instruction.accounts[0] as usize]];

    // pb::spl_token::InitializeImmutableOwnerEvent {
    //     account: Some(account.clone().into()),
    // }
    Ok(pb::spl_token::InitializeImmutableOwnerEvent::default())
}

impl From<utils::TokenAccount> for pb::spl_token::TokenAccount {
    fn from(value: utils::TokenAccount) -> Self {
        Self {
            address: bs58::encode(value.address).into_string(),
            owner: bs58::encode(value.owner).into_string(),
            mint: bs58::encode(value.mint).into_string(),
        }
    }
}
