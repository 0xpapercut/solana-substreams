use bs58;

use substreams::errors::Error;
use substreams_solana::pb::sf::solana::r#type::v1::ConfirmedTransaction;
use substreams_solana::pb::sf::solana::r#type::v1::Block;
use substreams_solana_program_instructions::pubkey::Pubkey;

use substreams_solana_spl_token as spl_token;
use spl_token::{TokenInstruction, TOKEN_PROGRAM};
use structured_instructions::{
    get_structured_instructions,
    StructuredInstruction,
    StructuredInstructions,
};

use substreams_solana_utils::{
    InstructionContext,
    ConfirmedTransactionExt,
};

mod pb;

#[substreams::handlers::map]
fn spl_token_block_events(block: Block) -> Result<pb::spl_token::SplTokenBlockEvents, Error> {
    Ok(pb::spl_token::SplTokenBlockEvents {
        transactions: parse_block(block)
    })
}

fn parse_block(block: Block) -> Vec<pb::spl_token::SplTokenTransactionEvents> {
    let mut transactions_events: Vec<pb::spl_token::SplTokenTransactionEvents> = Vec::new();
    for transaction in block.transactions() {
        let events = parse_transaction(transaction);
        if events.is_empty() {
            continue;
        }

        transactions_events.push(pb::spl_token::SplTokenTransactionEvents {
            signature: bs58::encode(transaction.signature()).into_string(),
            slot: block.slot,
            events
        })
    }
    transactions_events
}

fn parse_transaction(transaction: &ConfirmedTransaction) -> Vec<pb::spl_token::SplTokenEvent> {
    let context = InstructionContext::construct(transaction);
    let mut events: Vec<pb::spl_token::SplTokenEvent> = Vec::new();
    let instructions = get_structured_instructions(&transaction);
    let signature = bs58::encode(transaction.signature()).into_string();

    if let Some(_) = transaction.meta.as_ref().unwrap().err {
        return Vec::new();
    }

    for instruction in instructions.flattened() {
        if bs58::encode(context.get_account_from_index(instruction.program_id_index as usize)).into_string() != TOKEN_PROGRAM {
            continue;
        }
        match parse_instruction(&instruction, &context) {
            Ok(event) => {
                events.push(pb::spl_token::SplTokenEvent { event });
            }
            Err(e) => panic!("Transaction {}: {}", signature, e),
        }
    }
    events
}

pub fn parse_instruction(
    instruction: &StructuredInstruction,
    context: &InstructionContext,
) -> Result<Option<pb::spl_token::spl_token_event::Event>, &'static str> {
    if bs58::encode(context.get_account_from_index(instruction.program_id_index as usize)).into_string() != TOKEN_PROGRAM {
        return Err("Not a Token program instruction.");
    }

    let unpacked = TokenInstruction::unpack(&instruction.data);
    if unpacked.is_err() {
        return Err("Failed to parse Token program instruction.");
    }

    match unpacked.unwrap() {
        TokenInstruction::InitializeMint { decimals, mint_authority, freeze_authority } |
        TokenInstruction::InitializeMint2 { decimals, mint_authority, freeze_authority } => {
            let event = parse_initialize_mint_instruction(instruction, context, decimals as u32, mint_authority, freeze_authority)?;
            Ok(Some(pb::spl_token::spl_token_event::Event::InitializeMint(event)))
        },

        TokenInstruction::InitializeAccount => {
            let event = parse_initialize_account_instruction(instruction, context, None)?;
            Ok(Some(pb::spl_token::spl_token_event::Event::InitializeAccount(event)))
        },
        TokenInstruction::InitializeAccount2 { owner } |
        TokenInstruction::InitializeAccount3 { owner } => {
            let event = parse_initialize_account_instruction(instruction, context, Some(owner))?;
            Ok(Some(pb::spl_token::spl_token_event::Event::InitializeAccount(event)))
        },

        TokenInstruction::InitializeMultisig { m } |
        TokenInstruction::InitializeMultisig2 { m } => {
            let event = parse_initialize_multisig_instruction(instruction, context, m)?;
            Ok(Some(pb::spl_token::spl_token_event::Event::InitializeMultisig(event)))
        },

        TokenInstruction::Transfer { amount } => {
            let event = parse_transfer_instruction(instruction, context, amount, None)?;
            Ok(Some(pb::spl_token::spl_token_event::Event::Transfer(event)))
        },
        TokenInstruction::TransferChecked { amount, decimals } => {
            let event = parse_transfer_instruction(instruction, context, amount, Some(decimals))?;
            Ok(Some(pb::spl_token::spl_token_event::Event::Transfer(event)))
        },

        TokenInstruction::Approve { amount } => {
            let event = parse_approve_instruction(instruction, context, amount, None)?;
            Ok(Some(pb::spl_token::spl_token_event::Event::Approve(event)))
        },
        TokenInstruction::ApproveChecked { amount, decimals } => {
            let event = parse_approve_instruction(instruction, context, amount, Some(decimals))?;
            Ok(Some(pb::spl_token::spl_token_event::Event::Approve(event)))
        },

        TokenInstruction::Revoke => {
            let event = parse_revoke_instruction(instruction, context)?;
            Ok(Some(pb::spl_token::spl_token_event::Event::Revoke(event)))
        },

        TokenInstruction::SetAuthority { authority_type, new_authority } => {
            let event = parse_set_authority_instruction(instruction, context, authority_type, new_authority)?;
            Ok(Some(pb::spl_token::spl_token_event::Event::SetAuthority(event)))
        },

        TokenInstruction::MintTo { amount } => {
            let event = parse_mint_to_instruction(instruction, context, amount)?;
            Ok(Some(pb::spl_token::spl_token_event::Event::MintTo(event)))
        },
        TokenInstruction::MintToChecked { amount, decimals: _ } => {
            let event = parse_mint_to_instruction(instruction, context, amount)?;
            Ok(Some(pb::spl_token::spl_token_event::Event::MintTo(event)))
        },

        TokenInstruction::Burn { amount } => {
            let event = parse_burn_instruction(instruction, context, amount)?;
            Ok(Some(pb::spl_token::spl_token_event::Event::Burn(event)))
        },
        TokenInstruction::BurnChecked { amount, decimals: _ } => {
            let event = parse_burn_instruction(instruction, context, amount)?;
            Ok(Some(pb::spl_token::spl_token_event::Event::Burn(event)))
        },

        TokenInstruction::CloseAccount => {
            let event = parse_close_account_instruction(instruction, context)?;
            Ok(Some(pb::spl_token::spl_token_event::Event::CloseAccount(event)))
        },

        TokenInstruction::FreezeAccount => {
            let event = parse_freeze_account_instruction(instruction, context)?;
            Ok(Some(pb::spl_token::spl_token_event::Event::FreezeAccount(event)))
        },

        TokenInstruction::ThawAccount => {
            let event = parse_thaw_account_instruction(instruction, context)?;
            Ok(Some(pb::spl_token::spl_token_event::Event::ThawAccount(event)))
        },

        TokenInstruction::InitializeImmutableOwner => {
            let event = parse_initialize_immutable_owner_instruction(instruction, context)?;
            Ok(Some(pb::spl_token::spl_token_event::Event::InitializeImmutableOwner(event)))
        },

        TokenInstruction::SyncNative => Ok(None),
        TokenInstruction::AmountToUiAmount { amount: _ } => Ok(None),
        TokenInstruction::GetAccountDataSize => Ok(None),
        TokenInstruction::UiAmountToAmount { ui_amount: _ } => Ok(None),
    }
}

fn parse_initialize_mint_instruction(
    instruction: &StructuredInstruction,
    context: &InstructionContext,
    decimals: u32,
    mint_authority: Pubkey,
    freeze_authority: Option<Pubkey>,
) -> Result<pb::spl_token::InitializeMintEvent, &'static str> {
    let mint = bs58::encode(context.get_account_from_index(instruction.accounts[0] as usize)).into_string();
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
    context: &InstructionContext,
    _owner: Option<Pubkey>,
) -> Result<pb::spl_token::InitializeAccountEvent, &'static str> {
    let address = context.get_account_from_index(instruction.accounts[0] as usize);

    Ok(pb::spl_token::InitializeAccountEvent {
        account: Some((&context.token_accounts[address]).into())
    })
}

fn parse_initialize_multisig_instruction(
    _instruction: &StructuredInstruction,
    _context: &InstructionContext,
    _m: u8,
) -> Result<pb::spl_token::InitializeMultisigEvent, &'static str> {
    Ok(pb::spl_token::InitializeMultisigEvent::default())
}

fn parse_transfer_instruction(
    instruction: &StructuredInstruction,
    context: &InstructionContext,
    amount: u64,
    expected_decimals: Option<u8>,
) -> Result<pb::spl_token::TransferEvent, &'static str> {
    let delta: usize = if expected_decimals.is_none() { 0 } else { 1 };
    let source = context.get_token_account_from_index(instruction.accounts[0] as usize);
    let destination = context.get_token_account_from_index(instruction.accounts[1 + delta] as usize);
    Ok(pb::spl_token::TransferEvent {
        source: Some(source.into()),
        destination: Some(destination.into()),
        amount,
    })
}

fn parse_approve_instruction(
    instruction: &StructuredInstruction,
    context: &InstructionContext,
    amount: u64,
    expected_decimals: Option<u8>,
) -> Result<pb::spl_token::ApproveEvent, &'static str> {
    let delta: usize = if expected_decimals.is_none() { 0 } else { 1 };
    let source = context.get_token_account_from_index(instruction.accounts[0] as usize);
    let delegate = bs58::encode(context.get_account_from_index(instruction.accounts[1 + delta] as usize)).into_string();
    Ok(pb::spl_token::ApproveEvent {
        source: Some(source.into()),
        delegate,
        amount,
    })
}

fn parse_revoke_instruction(
    instruction: &StructuredInstruction,
    context: &InstructionContext,
) -> Result<pb::spl_token::RevokeEvent, &'static str> {
    let source = context.get_token_account_from_index(instruction.accounts[0] as usize);
    Ok(pb::spl_token::RevokeEvent {
        source: Some(source.into()),
    })
}

fn parse_set_authority_instruction(
    instruction: &StructuredInstruction,
    context: &InstructionContext,
    authority_type: spl_token::AuthorityType,
    new_authority: Option<Pubkey>,
) -> Result<pb::spl_token::SetAuthorityEvent, &'static str> {
    let mint = bs58::encode(context.get_account_from_index(instruction.accounts[0] as usize)).into_string();
    let authority_type: i32 = match authority_type {
        spl_token::AuthorityType::MintTokens => pb::spl_token::AuthorityType::MintTokens.into(),
        spl_token::AuthorityType::FreezeAccount => pb::spl_token::AuthorityType::FreezeAccount.into(),
        spl_token::AuthorityType::AccountOwner => pb::spl_token::AuthorityType::AccountOwner.into(),
        spl_token::AuthorityType::CloseAccount => pb::spl_token::AuthorityType::CloseAccount.into(),
    };
    let new_authority = new_authority.map(|x| bs58::encode(x).into_string());
    Ok(pb::spl_token::SetAuthorityEvent {
        mint,
        authority_type,
        new_authority,
    })
}

fn parse_mint_to_instruction(
    instruction: &StructuredInstruction,
    context: &InstructionContext,
    amount: u64,
) -> Result<pb::spl_token::MintToEvent, &'static str> {
    let mint = bs58::encode(context.get_account_from_index(instruction.accounts[0] as usize)).into_string();
    let destination = context.get_token_account_from_index(instruction.accounts[1] as usize);
    Ok(pb::spl_token::MintToEvent {
        destination: Some(destination.into()),
        amount,
        mint,
    })
}

fn parse_burn_instruction(
    instruction: &StructuredInstruction,
    context: &InstructionContext,
    amount: u64,
) -> Result<pb::spl_token::BurnEvent, &'static str> {
    let source = context.get_token_account_from_index(instruction.accounts[0] as usize);
    Ok(pb::spl_token::BurnEvent {
        source: Some(source.into()),
        amount,
    })
}

fn parse_close_account_instruction(
    instruction: &StructuredInstruction,
    context: &InstructionContext,
) -> Result<pb::spl_token::CloseAccountEvent, &'static str> {
    let source = context.get_token_account_from_index(instruction.accounts[0] as usize);
    let destination = bs58::encode(context.get_account_from_index(instruction.accounts[1] as usize)).into_string();
    Ok(pb::spl_token::CloseAccountEvent {
        source: Some(source.into()),
        destination,
    })
}

fn parse_freeze_account_instruction(
    instruction: &StructuredInstruction,
    context: &InstructionContext,
) -> Result<pb::spl_token::FreezeAccountEvent, &'static str> {
    let source = context.get_token_account_from_index(instruction.accounts[0] as usize);
    Ok(pb::spl_token::FreezeAccountEvent {
        source: Some(source.into()),
    })
}

fn parse_thaw_account_instruction(
    instruction: &StructuredInstruction,
    context: &InstructionContext,
) -> Result<pb::spl_token::ThawAccountEvent, &'static str> {
    let source = context.get_token_account_from_index(instruction.accounts[0] as usize);
    Ok(pb::spl_token::ThawAccountEvent {
        source: Some(source.into()),
    })
}

fn parse_initialize_immutable_owner_instruction(
    _instruction: &StructuredInstruction,
    _context: &InstructionContext,
) -> Result<pb::spl_token::InitializeImmutableOwnerEvent, &'static str> {
    Ok(pb::spl_token::InitializeImmutableOwnerEvent::default())
}

impl From<&substreams_solana_utils::TokenAccount> for pb::spl_token::TokenAccount {
    fn from(value: &substreams_solana_utils::TokenAccount) -> Self {
        Self {
            address: bs58::encode(value.address.clone()).into_string(),
            owner: bs58::encode(value.owner.clone()).into_string(),
            mint: bs58::encode(value.mint.clone()).into_string(),
        }
    }
}
