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
    SOL_MINT,
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

    let transaction = pb::spl_token::Transaction {
        signature: bs58::encode(confirmed_transaction.signature()).into_string(),
        slot,
        signers: Vec::new(),
    };
    for instruction in instructions.flattened() {
        // if bs58::encode(accounts[instruction.program_id_index as usize]).into_string() == TOKEN_PROGRAM {
        //     substreams::log::println(format!("{}", bs58::encode(confirmed_transaction.signature()).into_string()));
        // }
        match parse_instruction(&instruction, &accounts, &mut token_accounts) {
            Ok(event) => {
                events.push(pb::spl_token::Event {
                    transaction: Some(transaction.clone()),
                    event: Some(event),
                })
            }
            Err(_) => (),
        }
    }
    // events

    let events: Vec<_> = events.iter().filter(|x| match x.event.as_ref().unwrap() {
        pb::spl_token::event::Event::SetAuthority(_) => true,
        _ => false,
    }).cloned().collect();
    events
}

fn parse_instruction(
    instruction: &StructuredInstruction,
    accounts: &Vec<&Vec<u8>>,
    token_accounts: &mut HashMap<Vec<u8>, utils::TokenAccount>
) -> Result<pb::spl_token::event::Event, &'static str> {
    if bs58::encode(accounts[instruction.program_id_index as usize]).into_string() != TOKEN_PROGRAM {
        return Err("Not a Token program instruction.");
    }

    let unpacked = TokenInstruction::unpack(&instruction.data);
    if unpacked.is_err() {
        return Err("Failed to parse Token program instruction.");
    }

    match unpacked.unwrap() {
        TokenInstruction::Transfer { amount } => {
            Ok(pb::spl_token::event::Event::Transfer(parse_transfer_instruction(instruction, accounts, token_accounts, amount, None)))
        },
        TokenInstruction::TransferChecked { amount, decimals } => {
            Ok(pb::spl_token::event::Event::Transfer(parse_transfer_instruction(instruction, accounts, token_accounts, amount, Some(decimals))))
        },
        TokenInstruction::InitializeMint { decimals, mint_authority, freeze_authority } |
        TokenInstruction::InitializeMint2 { decimals, mint_authority, freeze_authority } => {
            Ok(pb::spl_token::event::Event::InitializeMint(parse_initialize_mint_instruction(instruction, accounts, token_accounts, mint_authority, freeze_authority)))
        },
        TokenInstruction::InitializeAccount => {
            Ok(pb::spl_token::event::Event::InitializeAccount(parse_initialize_account_instruction(instruction, accounts, token_accounts, None)))
        },
        TokenInstruction::InitializeAccount2 { owner } |
        TokenInstruction::InitializeAccount3 { owner } => {
            Ok(pb::spl_token::event::Event::InitializeAccount(parse_initialize_account_instruction(instruction, accounts, token_accounts, Some(owner))))
        },
        TokenInstruction::InitializeMultisig { m } |
        TokenInstruction::InitializeMultisig2 { m } => {
            Ok(pb::spl_token::event::Event::InitializeMultiSig(parse_initialize_multi_sig_instruction(instruction, accounts, token_accounts, m)))
        },
        TokenInstruction::InitializeImmutableOwner => {
            Ok(pb::spl_token::event::Event::InitializeImmutableOwner(parse_initialize_immutable_owner_instruction(instruction, accounts, token_accounts)))
        },
        TokenInstruction::Approve { amount } => {
            Ok(pb::spl_token::event::Event::Approve(parse_approve_instruction(instruction, accounts, token_accounts, amount)))
        },
        TokenInstruction::ApproveChecked { amount, decimals } => {
            Ok(pb::spl_token::event::Event::Approve(parse_approve_instruction(instruction, accounts, token_accounts, amount)))
        },
        TokenInstruction::Revoke => {
            Ok(pb::spl_token::event::Event::Revoke(parse_revoke_instruction(instruction, accounts, token_accounts)))
        },
        TokenInstruction::SetAuthority { authority_type, new_authority } => {
            Ok(pb::spl_token::event::Event::SetAuthority(parse_set_authority_instruction(instruction, accounts, token_accounts, authority_type, new_authority)))
        },
        TokenInstruction::MintTo { amount } => {
            Ok(pb::spl_token::event::Event::MintTo(parse_mint_to_instruction(instruction, accounts, token_accounts, amount)))
        },
        TokenInstruction::MintToChecked { amount, decimals } => {
            Ok(pb::spl_token::event::Event::MintTo(parse_mint_to_instruction(instruction, accounts, token_accounts, amount)))
        },
        TokenInstruction::Burn { amount } => {
            Ok(pb::spl_token::event::Event::Burn(parse_burn_instruction(instruction, accounts, token_accounts, amount)))
        },
        TokenInstruction::BurnChecked { amount, decimals } => {
            Ok(pb::spl_token::event::Event::Burn(parse_burn_instruction(instruction, accounts, token_accounts, amount)))
        },
        TokenInstruction::CloseAccount => {
            Ok(pb::spl_token::event::Event::CloseAccount(parse_close_account_instruction(instruction, accounts, token_accounts)))
        },
        TokenInstruction::FreezeAccount => {
            Ok(pb::spl_token::event::Event::FreezeAccount(parse_freeze_account_instruction(instruction, accounts, token_accounts)))
        },
        TokenInstruction::ThawAccount => {
            Ok(pb::spl_token::event::Event::ThawAccount(parse_thaw_account_instruction(instruction, accounts, token_accounts)))
        },
        TokenInstruction::SyncNative => {
            Ok(pb::spl_token::event::Event::SyncNative(parse_sync_native_instruction(instruction, accounts, token_accounts)))
        },
        TokenInstruction::AmountToUiAmount { amount } => Err("AmountToUiAmount instruction is not an event."),
        TokenInstruction::GetAccountDataSize => Err("GetAccountDataSize instruction is not an event."),
        TokenInstruction::UiAmountToAmount { ui_amount } => Err("UiAmountToAmount instruction is not an event."),
    }
}

fn parse_initialize_multi_sig_instruction(
    instruction: &StructuredInstruction,
    accounts: &Vec<&Vec<u8>>,
    token_accounts: &HashMap<Vec<u8>, utils::TokenAccount>,
    m: u8,
) -> pb::spl_token::InitializeMultiSigEvent {
    let multi_sig = accounts[instruction.accounts[0] as usize];

    pb::spl_token::InitializeMultiSigEvent {
        address: bs58::encode(multi_sig).into_string(),
        m: m as u32
    }
}

fn parse_approve_instruction(
    instruction: &StructuredInstruction,
    accounts: &Vec<&Vec<u8>>,
    token_accounts: &HashMap<Vec<u8>, utils::TokenAccount>,
    amount: u64,
) -> pb::spl_token::ApproveEvent {
    pb::spl_token::ApproveEvent { }
}

fn parse_revoke_instruction(
    instruction: &StructuredInstruction,
    accounts: &Vec<&Vec<u8>>,
    token_accounts: &HashMap<Vec<u8>, utils::TokenAccount>,
) -> pb::spl_token::RevokeEvent {
    pb::spl_token::RevokeEvent { }
}

fn parse_set_authority_instruction(
    instruction: &StructuredInstruction,
    accounts: &Vec<&Vec<u8>>,
    token_accounts: &HashMap<Vec<u8>, utils::TokenAccount>,
    authority_type: spl_token::AuthorityType,
    new_authority: Option<Pubkey>,
) -> pb::spl_token::SetAuthorityEvent {
    let account = accounts[instruction.accounts[0] as usize];
    let authority = accounts[instruction.accounts[1] as usize];
    substreams::log::println(format!("{:#?}", authority_type));
    let authority_type: i32 = match authority_type {
        spl_token::AuthorityType::MintTokens => pb::spl_token::AuthorityType::MintTokens.into(),
        spl_token::AuthorityType::FreezeAccount => pb::spl_token::AuthorityType::FreezeAccount.into(),
        spl_token::AuthorityType::AccountOwner => pb::spl_token::AuthorityType::AccountOwner.into(),
        spl_token::AuthorityType::CloseAccount => pb::spl_token::AuthorityType::CloseAccount.into(),
    };
    // let authority_type =
    substreams::log::println(format!("{:#?}", authority_type));


    // &token_accounts[account];

    pb::spl_token::SetAuthorityEvent {
        account: bs58::encode(account).into_string(),
        authority: bs58::encode(authority).into_string(),
        new_authority: Some(bs58::encode(account).into_string()),
        authority_type,
    }
}

fn parse_burn_instruction(
    instruction: &StructuredInstruction,
    accounts: &Vec<&Vec<u8>>,
    token_accounts: &HashMap<Vec<u8>, utils::TokenAccount>,
    amount: u64,
) -> pb::spl_token::BurnEvent {
    pb::spl_token::BurnEvent { }
}

fn parse_close_account_instruction(
    instruction: &StructuredInstruction,
    accounts: &Vec<&Vec<u8>>,
    token_accounts: &HashMap<Vec<u8>, utils::TokenAccount>,
) -> pb::spl_token::CloseAccountEvent {
    pb::spl_token::CloseAccountEvent { }
}

fn parse_freeze_account_instruction(
    instruction: &StructuredInstruction,
    accounts: &Vec<&Vec<u8>>,
    token_accounts: &HashMap<Vec<u8>, utils::TokenAccount>,
) -> pb::spl_token::FreezeAcccountEvent {
    pb::spl_token::FreezeAcccountEvent { }
}

fn parse_thaw_account_instruction(
    instruction: &StructuredInstruction,
    accounts: &Vec<&Vec<u8>>,
    token_accounts: &HashMap<Vec<u8>, utils::TokenAccount>,
) -> pb::spl_token::ThawAccountEvent {
    pb::spl_token::ThawAccountEvent { }
}

fn parse_sync_native_instruction(
    instruction: &StructuredInstruction,
    accounts: &Vec<&Vec<u8>>,
    token_accounts: &HashMap<Vec<u8>, utils::TokenAccount>,
) -> pb::spl_token::SyncNativeEvent {
    pb::spl_token::SyncNativeEvent { }
}

fn parse_transfer_instruction(
    instruction: &StructuredInstruction,
    accounts: &Vec<&Vec<u8>>,
    token_accounts: &HashMap<Vec<u8>, utils::TokenAccount>,
    amount: u64,
    expected_decimals: Option<u8>,
) -> pb::spl_token::TransferEvent {
    let source = &token_accounts[accounts[instruction.accounts[0] as usize]];
    let delta: usize = if expected_decimals.is_none() { 0 } else { 1 };
    let destination = &token_accounts[accounts[instruction.accounts[1 + delta] as usize]];

    pb::spl_token::TransferEvent {
        source: Some(pb::spl_token::TokenAccount {
            address: bs58::encode(&source.address).into_string(),
            owner: bs58::encode(&source.owner).into_string(),
            mint: bs58::encode(&source.mint).into_string(),
        }),
        destination: Some(pb::spl_token::TokenAccount {
            address: bs58::encode(&destination.address).into_string(),
            owner: bs58::encode(&destination.owner).into_string(),
            mint: bs58::encode(&destination.mint).into_string(),
        }),
        amount,
    }
}

fn parse_mint_to_instruction(
    instruction: &StructuredInstruction,
    accounts: &Vec<&Vec<u8>>,
    token_accounts: &HashMap<Vec<u8>, utils::TokenAccount>,
    amount: u64,
) -> pb::spl_token::MintToEvent {
    let address = accounts[instruction.accounts[1] as usize];
    let destination = &token_accounts[address];

    pb::spl_token::MintToEvent {
        destination: Some(pb::spl_token::TokenAccount {
            address: bs58::encode(&destination.address).into_string(),
            mint: bs58::encode(&destination.mint).into_string(),
            owner: bs58::encode(&destination.owner).into_string()
        }),
        amount,
    }
}

fn parse_initialize_mint_instruction(
    instruction: &StructuredInstruction,
    accounts: &Vec<&Vec<u8>>,
    token_accounts: &HashMap<Vec<u8>, utils::TokenAccount>,
    mint_authority: Pubkey,
    freeze_authority: Option<Pubkey>,
) -> pb::spl_token::InitializeMintEvent {
    let mint = accounts[instruction.accounts[0] as usize];
    pb::spl_token::InitializeMintEvent {
        mint: bs58::encode(mint).into_string(),
        mint_authority: bs58::encode(mint_authority).into_string(),
        freeze_authority: freeze_authority.map(|x| bs58::encode(x).into_string()),
    }
}

fn parse_initialize_immutable_owner_instruction(
    instruction: &StructuredInstruction,
    accounts: &Vec<&Vec<u8>>,
    token_accounts: &HashMap<Vec<u8>, utils::TokenAccount>,
) -> pb::spl_token::InitializeImmutableOwnerEvent {
    // let address = accounts[instruction.accounts[0] as usize];
    // substreams::log::println(format!("{}", bs58::encode(address).into_string()));
    // let token_account = &token_accounts[address];

    pb::spl_token::InitializeImmutableOwnerEvent {
        account: None
    }
}

fn parse_initialize_account_instruction(
    instruction: &StructuredInstruction,
    accounts: &Vec<&Vec<u8>>,
    token_accounts: &mut HashMap<Vec<u8>, utils::TokenAccount>,
    owner: Option<Pubkey>,
) -> pb::spl_token::InitializeAccountEvent {
    let address = accounts[instruction.accounts[0] as usize];
    let mint = accounts[instruction.accounts[1] as usize];
    let owner = owner.map(|x| x.to_bytes().to_vec()).unwrap_or_else(|| accounts[instruction.accounts[2] as usize].clone());

    token_accounts.entry(address.clone()).or_insert(utils::TokenAccount{
        address: address.clone(),
        mint: mint.clone(),
        owner: owner.clone(),
    });

    pb::spl_token::InitializeAccountEvent {
        account: Some(token_accounts[address].clone().into())
    }
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
