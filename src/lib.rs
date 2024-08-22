use substreams::errors::Error;
use substreams_solana::pb::sf::solana::r#type::v1::ConfirmedTransaction;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

use substreams_solana_utils as utils;
use utils::transaction::{get_context, TransactionContext};
use utils::instruction::{get_structured_instructions, StructuredInstructions, StructuredInstruction};
use utils::system_program::{self, SystemInstruction, SYSTEM_PROGRAM_ID};
use utils::pubkey::Pubkey;

pub mod pb;
use pb::system_program::*;
use pb::system_program::system_program_event::Event;

#[substreams::handlers::map]
fn system_program_events(block: Block) -> Result<SystemProgramBlockEvents, Error> {
    let transactions = parse_block(&block);
    Ok(SystemProgramBlockEvents { transactions })
}

pub fn parse_block(block: &Block) -> Vec<SystemProgramTransactionEvents> {
    let mut block_events: Vec<SystemProgramTransactionEvents> = Vec::new();
    for (i, transaction) in block.transactions.iter().enumerate() {
        if let Ok(events) = parse_transaction(transaction) {
            if !events.is_empty() {
                block_events.push(SystemProgramTransactionEvents {
                    signature: utils::transaction::get_signature(transaction),
                    transaction_index: i as u32,
                    events,
                });
            }
        }
    }
    block_events
}

pub fn parse_transaction(transaction: &ConfirmedTransaction) -> Result<Vec<SystemProgramEvent>, String> {
    if let Some(_) = transaction.meta.as_ref().unwrap().err {
        return Err("Cannot parse failed transaction.".to_string());
    }

    let mut events: Vec<SystemProgramEvent> = Vec::new();

    let context = get_context(transaction);
    let instructions = get_structured_instructions(transaction)?;

    for (i, instruction) in instructions.flattened().iter().enumerate() {
        if *instruction.program_id() == *SYSTEM_PROGRAM_ID {
            match parse_instruction(instruction, &context) {
                Ok(event) => {
                    events.push(SystemProgramEvent { instruction_index: i as u32, event });
                },
                Err(e) => substreams::log::println(e),
            }
        }
    }

    Ok(events)
}

pub fn parse_instruction<'a>(
    instruction: &'a StructuredInstruction<'a>,
    context: &TransactionContext
) -> Result<Option<Event>, String> {
    if *instruction.program_id() != *SYSTEM_PROGRAM_ID {
        return Err("Not a System Program instruction.".to_string());
    }
    let unpacked = SystemInstruction::unpack(&instruction.data())?;
    match unpacked {
        SystemInstruction::CreateAccount(create_account) => {
            _parse_create_account_instruction(instruction, context, &create_account).map(|x| Some(Event::CreateAccount(x)))
        },
        SystemInstruction::Assign(assign) => {
            _parse_assign_instruction(instruction, context, &assign).map(|x| Some(Event::Assign(x)))
        },
        SystemInstruction::Transfer(transfer) => {
            _parse_transfer_instruction(instruction, context, &transfer).map(|x| Some(Event::Transfer(x)))
        },
        SystemInstruction::CreateAccountWithSeed(create_account_with_seed) => {
            _parse_create_account_with_seed_instruction(instruction, context, &create_account_with_seed).map(|x| Some(Event::CreateAccountWithSeed(x)))
        },
        SystemInstruction::AdvanceNonceAccount => {
            _parse_advance_nonce_account_instruction(instruction, context).map(|x| Some(Event::AdvanceNonceAccount(x)))
        },
        SystemInstruction::WithdrawNonceAccount(lamports) => {
            _parse_withdraw_nonce_account_instruction(instruction, context, lamports).map(|x| Some(Event::WithdrawNonceAccount(x)))
        },
        SystemInstruction::InitializeNonceAccount(pubkey) => {
            _parse_initialize_nonce_account_instruction(instruction, context, pubkey).map(|x| Some(Event::InitializeNonceAccount(x)))
        },
        SystemInstruction::AuthorizeNonceAccount(pubkey) => {
            _parse_authorize_nonce_account_instruction(instruction, context, pubkey).map(|x| Some(Event::AuthorizeNonceAccount(x)))
        },
        SystemInstruction::Allocate(allocate) => {
            _parse_allocate_instruction(instruction, context, &allocate).map(|x| Some(Event::Allocate(x)))
        },
        SystemInstruction::AllocateWithSeed(allocate_with_seed) => {
            _parse_allocate_with_seed_instruction(instruction, context, &allocate_with_seed).map(|x| Some(Event::AllocateWithSeed(x)))
        },
        SystemInstruction::AssignWithSeed(assign_with_seed) => {
            _parse_assign_with_seed_instruction(instruction, context, &assign_with_seed).map(|x| Some(Event::AssignWithSeed(x)))
        },
        SystemInstruction::TransferWithSeed(transfer_with_seed) => {
            _parse_transfer_with_seed_instruction(instruction, context, transfer_with_seed).map(|x| Some(Event::TransferWithSeed(x)))
        },
        SystemInstruction::UpgradeNonceAccount => {
            _parse_upgrade_nonce_account_instruction(instruction, context).map(|x| Some(Event::UpgradeNonceAccount(x)))
        }
    }
}

fn _parse_create_account_instruction(
    instruction: &StructuredInstruction,
    _context: &TransactionContext,
    create_account: &system_program::CreateAccount,
) -> Result<CreateAccountEvent, String> {
    let funding_account = instruction.accounts()[0].to_string();
    let new_account = instruction.accounts()[1].to_string();
    let lamports = create_account.lamports;
    let owner = create_account.owner.to_string();
    let space = create_account.space;

    Ok(CreateAccountEvent {
        funding_account,
        new_account,
        lamports,
        owner,
        space,
    })
}

fn _parse_assign_instruction(
    instruction: &StructuredInstruction,
    _context: &TransactionContext,
    assign: &system_program::Assign,
) -> Result<AssignEvent, String> {
    let assigned_account = instruction.accounts()[0].to_string();
    let owner = assign.owner.to_string();

    Ok(AssignEvent {
        assigned_account,
        owner,
    })
}

fn _parse_transfer_instruction(
    instruction: &StructuredInstruction,
    _context: &TransactionContext,
    transfer: &system_program::Transfer,
) -> Result<TransferEvent, String> {
    let funding_account = instruction.accounts()[0].to_string();
    let recipient_account = instruction.accounts()[1].to_string();
    let lamports = transfer.lamports;

    Ok(TransferEvent {
        funding_account,
        recipient_account,
        lamports,
    })
}

fn _parse_create_account_with_seed_instruction(
    instruction: &StructuredInstruction,
    _context: &TransactionContext,
    create_account_with_seed: &system_program::CreateAccountWithSeed,
) -> Result<CreateAccountWithSeedEvent, String> {
    let funding_account = instruction.accounts()[0].to_string();
    let created_account = instruction.accounts()[1].to_string();
    let base_account = create_account_with_seed.base.to_string();
    let lamports = create_account_with_seed.lamports;
    let owner = create_account_with_seed.owner.to_string();
    let seed = create_account_with_seed.seed.0.clone();
    let space = create_account_with_seed.space;

    Ok(CreateAccountWithSeedEvent {
        funding_account,
        created_account,
        base_account,
        seed,
        lamports,
        space,
        owner,
    })
}

fn _parse_advance_nonce_account_instruction(
    instruction: &StructuredInstruction,
    _context: &TransactionContext,
) -> Result<AdvanceNonceAccountEvent, String> {
    let nonce_account = instruction.accounts()[0].to_string();
    let nonce_authority = instruction.accounts()[2].to_string();

    Ok(AdvanceNonceAccountEvent {
        nonce_account,
        nonce_authority,
    })
}

fn _parse_withdraw_nonce_account_instruction(
    instruction: &StructuredInstruction,
    _context: &TransactionContext,
    lamports: u64,
) -> Result<WithdrawNonceAccountEvent, String> {
    let nonce_account = instruction.accounts()[0].to_string();
    let recipient_account = instruction.accounts()[1].to_string();
    let nonce_authority = instruction.accounts()[4].to_string();

    Ok(WithdrawNonceAccountEvent {
        nonce_account,
        recipient_account,
        nonce_authority,
        lamports,
    })
}

fn _parse_initialize_nonce_account_instruction(
    instruction: &StructuredInstruction,
    _context: &TransactionContext,
    authority: Pubkey,
) -> Result<InitializeNonceAccountEvent, String> {
    let nonce_account = instruction.accounts()[0].to_string();
    let nonce_authority = authority.to_string();

    Ok(InitializeNonceAccountEvent {
        nonce_account,
        nonce_authority,
    })
}

fn _parse_authorize_nonce_account_instruction(
    instruction: &StructuredInstruction,
    _context: &TransactionContext,
    pubkey: Pubkey,
) -> Result<AuthorizeNonceAccountEvent, String> {
    let nonce_account = instruction.accounts()[0].to_string();
    let nonce_authority = instruction.accounts()[1].to_string();
    let new_nonce_authority = pubkey.to_string();

    Ok(AuthorizeNonceAccountEvent {
        nonce_account,
        nonce_authority,
        new_nonce_authority,
    })
}

fn _parse_allocate_instruction(
    instruction: &StructuredInstruction,
    _context: &TransactionContext,
    allocate: &system_program::Allocate,
) -> Result<AllocateEvent, String> {
    let account = instruction.accounts()[0].to_string();
    let space = allocate.space;

    Ok(AllocateEvent {
        account,
        space,
    })
}

fn _parse_allocate_with_seed_instruction(
    instruction: &StructuredInstruction,
    _context: &TransactionContext,
    allocate_with_seed: &system_program::AllocateWithSeed,
) -> Result<AllocateWithSeedEvent, String> {
    let allocated_account = instruction.accounts()[0].to_string();
    let space = allocate_with_seed.space;
    let base_account = allocate_with_seed.base.to_string();
    let owner = allocate_with_seed.owner.to_string();
    let seed = allocate_with_seed.seed.0.clone();

    Ok(AllocateWithSeedEvent {
        allocated_account,
        base_account,
        seed,
        owner,
        space,
    })
}

fn _parse_assign_with_seed_instruction(
    instruction: &StructuredInstruction,
    _context: &TransactionContext,
    assign_with_seed: &system_program::AssignWithSeed,
) -> Result<AssignWithSeedEvent, String> {
    let assigned_account = instruction.accounts()[0].to_string();
    let base_account = assign_with_seed.base.to_string();
    let owner = assign_with_seed.owner.to_string();
    let seed = assign_with_seed.seed.0.clone();

    Ok(AssignWithSeedEvent {
        assigned_account,
        base_account,
        owner,
        seed,
    })
}

fn _parse_transfer_with_seed_instruction(
    instruction: &StructuredInstruction,
    _context: &TransactionContext,
    transfer_with_seed: system_program::TransferWithSeed
) -> Result<TransferWithSeedEvent, String> {
    let funding_account = instruction.accounts()[0].to_string();
    let base_account = instruction.accounts()[1].to_string();
    let recipient_account = instruction.accounts()[2].to_string();
    let from_owner = transfer_with_seed.from_owner.to_string();
    let from_seed = transfer_with_seed.from_seed.0.clone();
    let lamports = transfer_with_seed.lamports;

    Ok(TransferWithSeedEvent {
        funding_account,
        base_account,
        recipient_account,
        from_owner,
        from_seed,
        lamports,
    })
}

fn _parse_upgrade_nonce_account_instruction(
    instruction: &StructuredInstruction,
    _context: &TransactionContext,
) -> Result<UpgradeNonceAccountEvent, String> {
    let nonce_account = instruction.accounts()[0].to_string();

    Ok(UpgradeNonceAccountEvent {
        nonce_account,
    })
}

pub fn parse_create_account_instruction<'a>(
    instruction: &'a StructuredInstruction<'a>,
    context: &TransactionContext,
) -> Result<CreateAccountEvent, String> {
    match parse_instruction(instruction, context) {
        Ok(Some(Event::CreateAccount(event))) => Ok(event),
        _ => Err("Not a CreateAccountInstruction.".to_string())
    }
}

pub fn parse_assign_instruction<'a>(
    instruction: &'a StructuredInstruction<'a>,
    context: &TransactionContext,
) -> Result<AssignEvent, String> {
    match parse_instruction(instruction, context) {
        Ok(Some(Event::Assign(event))) => Ok(event),
        _ => Err("Not an AssignInstruction.".to_string())
    }
}

pub fn parse_transfer_instruction<'a>(
    instruction: &'a StructuredInstruction<'a>,
    context: &TransactionContext,
) -> Result<TransferEvent, String> {
    match parse_instruction(instruction, context) {
        Ok(Some(Event::Transfer(event))) => Ok(event),
        _ => Err("Not a TransferInstruction.".to_string())
    }
}

pub fn parse_create_account_with_seed_instruction<'a>(
    instruction: &'a StructuredInstruction<'a>,
    context: &TransactionContext,
) -> Result<CreateAccountWithSeedEvent, String> {
    match parse_instruction(instruction, context) {
        Ok(Some(Event::CreateAccountWithSeed(event))) => Ok(event),
        _ => Err("Not a CreateAccountWithSeedInstruction.".to_string())
    }
}

pub fn parse_advance_nonce_account_instruction<'a>(
    instruction: &'a StructuredInstruction<'a>,
    context: &TransactionContext,
) -> Result<AdvanceNonceAccountEvent, String> {
    match parse_instruction(instruction, context) {
        Ok(Some(Event::AdvanceNonceAccount(event))) => Ok(event),
        _ => Err("Not an AdvanceNonceAccountInstruction.".to_string())
    }
}

pub fn parse_withdraw_nonce_account_instruction<'a>(
    instruction: &'a StructuredInstruction<'a>,
    context: &TransactionContext,
) -> Result<WithdrawNonceAccountEvent, String> {
    match parse_instruction(instruction, context) {
        Ok(Some(Event::WithdrawNonceAccount(event))) => Ok(event),
        _ => Err("Not a WithdrawNonceAccountInstruction.".to_string())
    }
}

pub fn parse_initialize_nonce_account_instruction<'a>(
    instruction: &'a StructuredInstruction<'a>,
    context: &TransactionContext,
) -> Result<InitializeNonceAccountEvent, String> {
    match parse_instruction(instruction, context) {
        Ok(Some(Event::InitializeNonceAccount(event))) => Ok(event),
        _ => Err("Not an InitializeNonceAccountInstruction.".to_string())
    }
}

pub fn parse_authorize_nonce_account_instruction<'a>(
    instruction: &'a StructuredInstruction<'a>,
    context: &TransactionContext,
) -> Result<AuthorizeNonceAccountEvent, String> {
    match parse_instruction(instruction, context) {
        Ok(Some(Event::AuthorizeNonceAccount(event))) => Ok(event),
        _ => Err("Not an AuthorizeNonceAccountInstruction.".to_string())
    }
}

pub fn parse_allocate_instruction<'a>(
    instruction: &'a StructuredInstruction<'a>,
    context: &TransactionContext,
) -> Result<AllocateEvent, String> {
    match parse_instruction(instruction, context) {
        Ok(Some(Event::Allocate(event))) => Ok(event),
        _ => Err("Not an AllocateInstruction.".to_string())
    }
}

pub fn parse_allocate_with_seed_instruction<'a>(
    instruction: &'a StructuredInstruction<'a>,
    context: &TransactionContext,
) -> Result<AllocateWithSeedEvent, String> {
    match parse_instruction(instruction, context) {
        Ok(Some(Event::AllocateWithSeed(event))) => Ok(event),
        _ => Err("Not an AllocateWithSeedInstruction.".to_string())
    }
}

pub fn parse_assign_with_seed_instruction<'a>(
    instruction: &'a StructuredInstruction<'a>,
    context: &TransactionContext,
) -> Result<AssignWithSeedEvent, String> {
    match parse_instruction(instruction, context) {
        Ok(Some(Event::AssignWithSeed(event))) => Ok(event),
        _ => Err("Not an AssignWithSeedInstruction.".to_string())
    }
}

pub fn parse_transfer_with_seed_instruction<'a>(
    instruction: &'a StructuredInstruction<'a>,
    context: &TransactionContext,
) -> Result<TransferWithSeedEvent, String> {
    match parse_instruction(instruction, context) {
        Ok(Some(Event::TransferWithSeed(event))) => Ok(event),
        _ => Err("Not a TransferWithSeedInstruction.".to_string())
    }
}

pub fn parse_upgrade_nonce_account_instruction<'a>(
    instruction: &'a StructuredInstruction<'a>,
    context: &TransactionContext,
) -> Result<UpgradeNonceAccountEvent, String> {
    match parse_instruction(instruction, context) {
        Ok(Some(Event::UpgradeNonceAccount(event))) => Ok(event),
        _ => Err("Not an UpgradeNonceAccountInstruction.".to_string())
    }
}
