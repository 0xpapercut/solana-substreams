use std::any::Any;
use bs58;

use substreams::errors::Error;
use substreams_solana::pb::sf::solana::r#type::v1::ConfirmedTransaction;
use substreams_solana::pb::sf::solana::r#type::v1::Block;
use substreams_solana_program_instructions::pubkey::Pubkey;

use substreams_solana_spl_token as spl_token;
use spl_token::{TokenInstruction, TOKEN_PROGRAM};
use substreams_solana_structured_instructions::{
    get_structured_instructions,
    StructuredInstruction,
    StructuredInstructions,
};

use substreams_solana_utils::{
    TransactionContext,
    ConfirmedTransactionExt,
};

pub mod pb;
use pb::spl_token::{
    SplTokenBlockEvents,
    SplTokenTransactionEvents,
    SplTokenEvent,
    InitializeMintEvent,
    InitializeAccountEvent,
    InitializeMultisigEvent,
    TransferEvent,
    ApproveEvent,
    RevokeEvent,
    SetAuthorityEvent,
    MintToEvent,
    BurnEvent,
    CloseAccountEvent,
    FreezeAccountEvent,
    ThawAccountEvent,
    InitializeImmutableOwnerEvent,
    TokenAccount,
    AuthorityType,
};
use pb::spl_token::spl_token_event::Event;

#[substreams::handlers::map]
fn spl_token_block_events(block: Block) -> Result<SplTokenBlockEvents, Error> {
    Ok(SplTokenBlockEvents { transactions: parse_block(&block) })
}

pub fn parse_block(block: &Block) -> Vec<SplTokenTransactionEvents> {
    let mut transactions_events: Vec<SplTokenTransactionEvents> = Vec::new();
    for transaction in block.transactions() {
        let events = parse_transaction(transaction);
        if !events.is_empty() {
            transactions_events.push(SplTokenTransactionEvents {
                signature: bs58::encode(transaction.signature()).into_string(),
                events
            })
        }
    }
    transactions_events
}

pub fn parse_transaction(transaction: &ConfirmedTransaction) -> Vec<SplTokenEvent> {
    let context = TransactionContext::construct(transaction);
    let mut events: Vec<SplTokenEvent> = Vec::new();
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
                events.push(SplTokenEvent { event });
            }
            Err(e) => panic!("Transaction {}: {}", signature, e),
        }
    }
    events
}

pub fn parse_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext,
) -> Result<Option<Event>, &'static str> {
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
            let event = _parse_initialize_mint_instruction(instruction, context, decimals as u32, mint_authority, freeze_authority)?;
            Ok(Some(Event::InitializeMint(event)))
        },

        TokenInstruction::InitializeAccount => {
            let event = _parse_initialize_account_instruction(instruction, context, None)?;
            Ok(Some(Event::InitializeAccount(event)))
        },
        TokenInstruction::InitializeAccount2 { owner } |
        TokenInstruction::InitializeAccount3 { owner } => {
            let event = _parse_initialize_account_instruction(instruction, context, Some(owner))?;
            Ok(Some(Event::InitializeAccount(event)))
        },

        TokenInstruction::InitializeMultisig { m } |
        TokenInstruction::InitializeMultisig2 { m } => {
            let event = _parse_initialize_multisig_instruction(instruction, context, m)?;
            Ok(Some(Event::InitializeMultisig(event)))
        },

        TokenInstruction::Transfer { amount } => {
            let event = _parse_transfer_instruction(instruction, context, amount, None)?;
            Ok(Some(Event::Transfer(event)))
        },
        TokenInstruction::TransferChecked { amount, decimals } => {
            let event = _parse_transfer_instruction(instruction, context, amount, Some(decimals))?;
            Ok(Some(Event::Transfer(event)))
        },

        TokenInstruction::Approve { amount } => {
            let event = _parse_approve_instruction(instruction, context, amount, None)?;
            Ok(Some(Event::Approve(event)))
        },
        TokenInstruction::ApproveChecked { amount, decimals } => {
            let event = _parse_approve_instruction(instruction, context, amount, Some(decimals))?;
            Ok(Some(Event::Approve(event)))
        },

        TokenInstruction::Revoke => {
            let event = _parse_revoke_instruction(instruction, context)?;
            Ok(Some(Event::Revoke(event)))
        },

        TokenInstruction::SetAuthority { authority_type, new_authority } => {
            let event = _parse_set_authority_instruction(instruction, context, authority_type, new_authority)?;
            Ok(Some(Event::SetAuthority(event)))
        },

        TokenInstruction::MintTo { amount } => {
            let event = _parse_mint_to_instruction(instruction, context, amount)?;
            Ok(Some(Event::MintTo(event)))
        },
        TokenInstruction::MintToChecked { amount, decimals: _ } => {
            let event = _parse_mint_to_instruction(instruction, context, amount)?;
            Ok(Some(Event::MintTo(event)))
        },

        TokenInstruction::Burn { amount } => {
            let event = _parse_burn_instruction(instruction, context, amount)?;
            Ok(Some(Event::Burn(event)))
        },
        TokenInstruction::BurnChecked { amount, decimals: _ } => {
            let event = _parse_burn_instruction(instruction, context, amount)?;
            Ok(Some(Event::Burn(event)))
        },

        TokenInstruction::CloseAccount => {
            let event = _parse_close_account_instruction(instruction, context)?;
            Ok(Some(Event::CloseAccount(event)))
        },

        TokenInstruction::FreezeAccount => {
            let event = _parse_freeze_account_instruction(instruction, context)?;
            Ok(Some(Event::FreezeAccount(event)))
        },

        TokenInstruction::ThawAccount => {
            let event = _parse_thaw_account_instruction(instruction, context)?;
            Ok(Some(Event::ThawAccount(event)))
        },

        TokenInstruction::InitializeImmutableOwner => {
            let event = _parse_initialize_immutable_owner_instruction(instruction, context)?;
            Ok(Some(Event::InitializeImmutableOwner(event)))
        },

        TokenInstruction::SyncNative => Ok(None),
        TokenInstruction::AmountToUiAmount { amount: _ } => Ok(None),
        TokenInstruction::GetAccountDataSize => Ok(None),
        TokenInstruction::UiAmountToAmount { ui_amount: _ } => Ok(None),
    }
}

fn _parse_initialize_mint_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext,
    decimals: u32,
    mint_authority: Pubkey,
    freeze_authority: Option<Pubkey>,
) -> Result<InitializeMintEvent, &'static str> {
    let mint = bs58::encode(context.get_account_from_index(instruction.accounts[0] as usize)).into_string();
    let mint_authority = bs58::encode(mint_authority).into_string();
    let freeze_authority = freeze_authority.map(|x| bs58::encode(x).into_string());

    Ok(InitializeMintEvent {
        mint,
        decimals,
        mint_authority,
        freeze_authority,
    })
}

fn _parse_initialize_account_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext,
    _owner: Option<Pubkey>,
) -> Result<InitializeAccountEvent, &'static str> {
    let account = context.get_token_account_from_index(instruction.accounts[0] as usize);

    Ok(InitializeAccountEvent {
        account: Some(account.into())
    })
}

fn _parse_initialize_multisig_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext,
    m: u8,
) -> Result<InitializeMultisigEvent, &'static str> {
    let multisig = bs58::encode(context.get_account_from_index(instruction.accounts[0] as usize)).into_string();
    let mut signers: Vec<String> = Vec::new();
    for i in 0..m {
        signers.push(bs58::encode(context.get_account_from_index(instruction.accounts[(i + 2) as usize] as usize)).into_string());
    }

    Ok(InitializeMultisigEvent {
        multisig,
        signers
    })
}

fn _parse_transfer_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext,
    amount: u64,
    expected_decimals: Option<u8>,
) -> Result<TransferEvent, &'static str> {
    let delta: usize = if expected_decimals.is_none() { 0 } else { 1 };
    let source = context.get_token_account_from_index(instruction.accounts[0] as usize);
    let destination = context.get_token_account_from_index(instruction.accounts[1 + delta] as usize);
    let authority = bs58::encode(context.get_account_from_index(instruction.accounts[2 + delta] as usize)).into_string();

    Ok(TransferEvent {
        source: Some(source.into()),
        destination: Some(destination.into()),
        amount,
        authority,
    })
}

fn _parse_approve_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext,
    amount: u64,
    expected_decimals: Option<u8>,
) -> Result<ApproveEvent, &'static str> {
    let delta: usize = if expected_decimals.is_none() { 0 } else { 1 };
    let source = context.get_token_account_from_index(instruction.accounts[0] as usize);
    let delegate = bs58::encode(context.get_account_from_index(instruction.accounts[1 + delta] as usize)).into_string();

    Ok(ApproveEvent {
        source: Some(source.into()),
        delegate,
        amount,
    })
}

fn _parse_revoke_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext,
) -> Result<RevokeEvent, &'static str> {
    let source = context.get_token_account_from_index(instruction.accounts[0] as usize);

    Ok(RevokeEvent {
        source: Some(source.into()),
    })
}

fn _parse_set_authority_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext,
    authority_type: spl_token::AuthorityType,
    new_authority: Option<Pubkey>,
) -> Result<SetAuthorityEvent, &'static str> {
    let mint = bs58::encode(context.get_account_from_index(instruction.accounts[0] as usize)).into_string();
    let authority = bs58::encode(context.get_account_from_index(instruction.accounts[1] as usize)).into_string();
    let authority_type: i32 = match authority_type {
        spl_token::AuthorityType::MintTokens => AuthorityType::MintTokens.into(),
        spl_token::AuthorityType::FreezeAccount => AuthorityType::FreezeAccount.into(),
        spl_token::AuthorityType::AccountOwner => AuthorityType::AccountOwner.into(),
        spl_token::AuthorityType::CloseAccount => AuthorityType::CloseAccount.into(),
    };
    let new_authority = new_authority.map(|x| bs58::encode(x).into_string());

    Ok(SetAuthorityEvent {
        mint,
        authority,
        authority_type,
        new_authority,
    })
}

fn _parse_mint_to_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext,
    amount: u64,
) -> Result<MintToEvent, &'static str> {
    let mint = bs58::encode(context.get_account_from_index(instruction.accounts[0] as usize)).into_string();
    let destination = context.get_token_account_from_index(instruction.accounts[1] as usize);
    let authority = bs58::encode(context.get_account_from_index(instruction.accounts[2] as usize)).into_string();

    Ok(MintToEvent {
        mint,
        destination: Some(destination.into()),
        authority,
        amount,
    })
}

fn _parse_burn_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext,
    amount: u64,
) -> Result<BurnEvent, &'static str> {
    let source = context.get_token_account_from_index(instruction.accounts[0] as usize);
    let _mint = bs58::encode(context.get_account_from_index(instruction.accounts[1] as usize)).into_string();
    let authority = bs58::encode(context.get_account_from_index(instruction.accounts[2] as usize)).into_string();

    Ok(BurnEvent {
        source: Some(source.into()),
        authority,
        amount,
    })
}

fn _parse_close_account_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext,
) -> Result<CloseAccountEvent, &'static str> {
    let source = context.get_token_account_from_index(instruction.accounts[0] as usize);
    let destination = bs58::encode(context.get_account_from_index(instruction.accounts[1] as usize)).into_string();

    Ok(CloseAccountEvent {
        source: Some(source.into()),
        destination,
    })
}

fn _parse_freeze_account_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext,
) -> Result<FreezeAccountEvent, &'static str> {
    let source = context.get_token_account_from_index(instruction.accounts[0] as usize);
    let freeze_authority = bs58::encode(context.get_account_from_index(instruction.accounts[1] as usize)).into_string();

    Ok(FreezeAccountEvent {
        source: Some(source.into()),
        freeze_authority,
    })
}

fn _parse_thaw_account_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext,
) -> Result<ThawAccountEvent, &'static str> {
    let source = context.get_token_account_from_index(instruction.accounts[0] as usize);
    let freeze_authority = bs58::encode(context.get_account_from_index(instruction.accounts[1] as usize)).into_string();

    Ok(ThawAccountEvent {
        source: Some(source.into()),
        freeze_authority,
    })
}

fn _parse_initialize_immutable_owner_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext,
) -> Result<InitializeImmutableOwnerEvent, &'static str> {
    let account = context.get_token_account_from_index(instruction.accounts[0] as usize);

    Ok(InitializeImmutableOwnerEvent {
        account: Some(account.into()),
    })
}

pub fn parse_initialize_mint_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext,
) -> Result<InitializeMintEvent, &'static str> {
    match parse_instruction(instruction, context) {
        Ok(Some(Event::InitializeMint(initialize_mint))) => Ok(initialize_mint),
        _ => Err("Failed to parse initialize mint instruction."),
    }
}

pub fn parse_initialize_account_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext,
) -> Result<InitializeAccountEvent, &'static str> {
    match parse_instruction(instruction, context) {
        Ok(Some(Event::InitializeAccount(initialize_account))) => Ok(initialize_account),
        _ => Err("Failed to parse initialize account instruction."),
    }
}

pub fn parse_initialize_multisig_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext,
) -> Result<InitializeMultisigEvent, &'static str> {
    match parse_instruction(instruction, context) {
        Ok(Some(Event::InitializeMultisig(initialize_multisig))) => Ok(initialize_multisig),
        _ => Err("Failed to parse initialize multisig instruction."),
    }
}


pub fn parse_transfer_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext,
) -> Result<TransferEvent, &'static str> {
    match parse_instruction(instruction, context) {
        Ok(Some(Event::Transfer(transfer))) => Ok(transfer),
        _ => Err("Failed to parse transfer instruction."),
    }
}

pub fn parse_approve_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext,
) -> Result<ApproveEvent, &'static str> {
    match parse_instruction(instruction, context) {
        Ok(Some(Event::Approve(approve))) => Ok(approve),
        _ => Err("Failed to parse approve instruction."),
    }
}

pub fn parse_revoke_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext,
) -> Result<RevokeEvent, &'static str> {
    match parse_instruction(instruction, context) {
        Ok(Some(Event::Revoke(revoke))) => Ok(revoke),
        _ => Err("Failed to parse revoke instruction."),
    }
}

pub fn parse_set_authority_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext,
) -> Result<SetAuthorityEvent, &'static str> {
    match parse_instruction(instruction, context) {
        Ok(Some(Event::SetAuthority(set_authority))) => Ok(set_authority),
        _ => Err("Failed to parse set authority instruction."),
    }
}

pub fn parse_mint_to_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext,
) -> Result<MintToEvent, &'static str> {
    match parse_instruction(instruction, context) {
        Ok(Some(Event::MintTo(mint_to))) => Ok(mint_to),
        _ => Err("Failed to parse mint to instruction."),
    }
}

pub fn parse_burn_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext,
) -> Result<BurnEvent, &'static str> {
    match parse_instruction(instruction, context) {
        Ok(Some(Event::Burn(burn))) => Ok(burn),
        _ => Err("Failed to parse burn instruction."),
    }
}


pub fn parse_close_account_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext,
) -> Result<CloseAccountEvent, &'static str> {
    match parse_instruction(instruction, context) {
        Ok(Some(Event::CloseAccount(close_account))) => Ok(close_account),
        _ => Err("Failed to parse close account instruction."),
    }
}

pub fn parse_freeze_account_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext,
) -> Result<FreezeAccountEvent, &'static str> {
    match parse_instruction(instruction, context) {
        Ok(Some(Event::FreezeAccount(freeze_account))) => Ok(freeze_account),
        _ => Err("Failed to parse freeze account instruction."),
    }
}

pub fn parse_thaw_account_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext,
) -> Result<ThawAccountEvent, &'static str> {
    match parse_instruction(instruction, context) {
        Ok(Some(Event::ThawAccount(thaw_account))) => Ok(thaw_account),
        _ => Err("Failed to parse thaw account instruction."),
    }
}

pub fn parse_initialize_immutable_owner_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext,
) -> Result<InitializeImmutableOwnerEvent, &'static str> {
    match parse_instruction(instruction, context) {
        Ok(Some(Event::InitializeImmutableOwner(initialize_immutable_owner))) => Ok(initialize_immutable_owner),
        _ => Err("Failed to parse initialize immutable owner instruction."),
    }
}

impl From<&substreams_solana_utils::TokenAccount> for TokenAccount {
    fn from(value: &substreams_solana_utils::TokenAccount) -> Self {
        Self {
            address: bs58::encode(value.address.clone()).into_string(),
            owner: bs58::encode(value.owner.clone()).into_string(),
            mint: bs58::encode(value.mint.clone()).into_string(),
        }
    }
}

impl Event {
    pub fn cast<T: 'static>(&self) -> Option<&T> {
        match self {
            Event::InitializeMint(event) => (event as &dyn Any).downcast_ref::<T>(),
            Event::InitializeAccount(event) => (event as &dyn Any).downcast_ref::<T>(),
            Event::InitializeMultisig(event) => (event as &dyn Any).downcast_ref::<T>(),
            Event::Transfer(event) => (event as &dyn Any).downcast_ref::<T>(),
            Event::Approve(event) => (event as &dyn Any).downcast_ref::<T>(),
            Event::Revoke(event) => (event as &dyn Any).downcast_ref::<T>(),
            Event::SetAuthority(event) => (event as &dyn Any).downcast_ref::<T>(),
            Event::MintTo(event) => (event as &dyn Any).downcast_ref::<T>(),
            Event::Burn(event) => (event as &dyn Any).downcast_ref::<T>(),
            Event::CloseAccount(event) => (event as &dyn Any).downcast_ref::<T>(),
            Event::FreezeAccount(event) => (event as &dyn Any).downcast_ref::<T>(),
            Event::ThawAccount(event) => (event as &dyn Any).downcast_ref::<T>(),
            Event::InitializeImmutableOwner(event) => (event as &dyn Any).downcast_ref::<T>(),
        }
    }
}
