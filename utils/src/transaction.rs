use std::collections::HashMap;

use substreams_solana::pb::sf::solana::r#type::v1::ConfirmedTransaction;

use crate::pubkey::{Pubkey, PubkeyRef};
use crate::instruction::{WrappedInstruction, get_flattened_instructions};
use crate::spl_token::{TokenInstruction, TokenAccount, TOKEN_PROGRAM_ID};

use anyhow::{anyhow, Error};

/// Context that can provide enough information to process an instruction
pub struct TransactionContext<'a> {
    pub accounts: Vec<PubkeyRef<'a>>,
    pub token_accounts: HashMap<PubkeyRef<'a>, TokenAccount<'a>>,
    pub signature: String,
}

impl<'a> TransactionContext<'a> {
    fn new(transaction: &'a ConfirmedTransaction) -> Self {
        let accounts = transaction.resolved_accounts().iter().map(|x| PubkeyRef { 0: x }).collect();
        let signature = bs58::encode(transaction.transaction.as_ref().unwrap().signatures.get(0).unwrap()).into_string();
        Self {
            accounts,
            token_accounts: HashMap::new(),
            signature,
        }
    }

    pub fn build_partial(transaction: &'a ConfirmedTransaction) -> Result<Self, &'static str> {
        let mut context = Self::new(transaction);

        for token_balance in &transaction.meta.as_ref().unwrap().pre_token_balances {
            let address = context.accounts[token_balance.account_index as usize].clone();
            let token_account = TokenAccount {
                address: address.clone(),
                mint: Pubkey::try_from_string(&token_balance.mint).unwrap(),
                owner: Pubkey::try_from_string(&token_balance.owner).unwrap(),
                amount: Some(token_balance.ui_token_amount.as_ref().unwrap().amount.parse::<u64>().expect("Failed to parse u64"))
            };
            context.token_accounts.insert(address, token_account);
        }

        Ok(context)
    }

    pub fn build(transaction: &'a ConfirmedTransaction) -> Result<Self, &'static str> {
        let mut context = Self::new(transaction);

        for token_balance in &transaction.meta.as_ref().unwrap().pre_token_balances {
            let address = context.accounts[token_balance.account_index as usize].clone();
            let token_account = TokenAccount {
                address: address.clone(),
                mint: Pubkey::try_from_string(&token_balance.mint).unwrap(),
                owner: Pubkey::try_from_string(&token_balance.owner).unwrap(),
                amount: Some(token_balance.ui_token_amount.as_ref().unwrap().amount.parse::<u64>().expect("Failed to parse u64"))
            };
            context.token_accounts.insert(address, token_account);
        }

        let instructions = get_flattened_instructions(transaction);
        for instruction in instructions {
            context.update(&instruction);
        }

        Ok(context)
    }

    fn update(&mut self, instruction: &WrappedInstruction) {
        if self.accounts[instruction.program_id_index() as usize] != TOKEN_PROGRAM_ID {
            return;
        }
        match TokenInstruction::unpack(&instruction.data()) {
            Ok(TokenInstruction::InitializeAccount) => {
                let token_account = parse_token_account_from_initialize_account_instruction(instruction, self, None);
                self.token_accounts.insert(token_account.address.clone(), token_account);
            }
            Ok(TokenInstruction::InitializeAccount2 { owner }) |
            Ok(TokenInstruction::InitializeAccount3 { owner }) => {
                let token_account = parse_token_account_from_initialize_account_instruction(instruction, self, Some(owner));
                self.token_accounts.insert(token_account.address.clone(), token_account);
            }
            Ok(TokenInstruction::Transfer { amount }) => {
                let source_address = self.accounts[instruction.accounts()[0] as usize];
                let destination_address = self.accounts[instruction.accounts()[1] as usize];

                let source_account = self.token_accounts.get_mut(&source_address).unwrap();
                source_account.amount = source_account.amount.map(|x| x - amount);

                let destination_account = self.token_accounts.get_mut(&destination_address).unwrap();
                destination_account.amount = destination_account.amount.map(|x| x + amount);
            },
            Ok(TokenInstruction::TransferChecked { amount, decimals: _ }) => {
                let source_address = self.accounts[instruction.accounts()[0] as usize];
                let destination_address = self.accounts[instruction.accounts()[2] as usize];

                let source_account = self.token_accounts.get_mut(&source_address).unwrap();
                source_account.amount = source_account.amount.map(|x| x - amount);

                let destination_account = self.token_accounts.get_mut(&destination_address).unwrap();
                destination_account.amount = destination_account.amount.map(|x| x + amount);
            },
            Ok(TokenInstruction::MintTo { amount }) => {
                let address = self.accounts[instruction.accounts()[1] as usize];
                let account = self.token_accounts.get_mut(&address).unwrap();
                account.amount = account.amount.map(|x| x + amount);
            },
            Ok(TokenInstruction::MintToChecked { amount, decimals: _ }) => {
                let address = self.accounts[instruction.accounts()[1] as usize];
                let account = self.token_accounts.get_mut(&address).unwrap();
                account.amount = account.amount.map(|x| x + amount);
            },
            Ok(TokenInstruction::Burn { amount }) => {
                let address = self.accounts[instruction.accounts()[0] as usize];
                let account = self.token_accounts.get_mut(&address).unwrap();
                account.amount = account.amount.map(|x| x - amount);
            },
            Ok(TokenInstruction::BurnChecked { amount, decimals: _ }) => {
                let address = self.accounts[instruction.accounts()[0] as usize];
                let account = self.token_accounts.get_mut(&address).unwrap();
                account.amount = account.amount.map(|x| x - amount);
            },
            Ok(TokenInstruction::SyncNative) => {
                let address = self.accounts[instruction.accounts()[0] as usize];
                let account = self.token_accounts.get_mut(&address).unwrap();
                account.amount = None;
            },
            Ok(TokenInstruction::CloseAccount) => {
                let address = self.accounts[instruction.accounts()[0] as usize];
                let account = self.token_accounts.get_mut(&address).unwrap();
                account.amount = Some(0);
            },
            _ => ()
        }
    }

    pub fn get_token_account(&self, address: &PubkeyRef<'a>) -> Option<&TokenAccount> {
        self.token_accounts.get(address)
    }
}

/// Parses the Initialize SPL Token Instruction and returns a TokenAccount
fn parse_token_account_from_initialize_account_instruction<'a>(instruction: &WrappedInstruction, context: &TransactionContext<'a>, owner: Option<Pubkey>) -> TokenAccount<'a> {
    let address = context.accounts[instruction.accounts()[0] as usize].clone();
    let mint = context.accounts[instruction.accounts()[1] as usize].to_pubkey().unwrap();
    let owner = match owner {
        Some(pubkey) => pubkey,
        None => context.accounts[instruction.accounts()[2] as usize].to_pubkey().unwrap(),
    };
    TokenAccount {
        address,
        mint,
        owner,
        amount: Some(0),
    }
}

pub fn get_context<'a>(transaction: &'a ConfirmedTransaction) -> Result<TransactionContext<'a>, Error> {
    if let Some(_) = transaction.meta.as_ref().unwrap().err {
        return Err(anyhow!("Cannot get context of failed instruction."));
    }
    TransactionContext::build(transaction).map_err(|x| anyhow!(x))
}

pub fn get_signature(transaction: &ConfirmedTransaction) -> String {
    bs58::encode(transaction.transaction.as_ref().unwrap().signatures.get(0).unwrap()).into_string()
}
