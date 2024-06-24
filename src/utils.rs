use std::collections::HashMap;

use substreams_solana::pb::sf::solana::r#type::v1::ConfirmedTransaction;
use substreams_solana_program_instructions::pubkey::Pubkey;
use structured_instructions::StructuredInstruction;

#[derive(Debug, Clone)]
pub struct TokenAccount {
    pub owner: Vec<u8>,
    pub mint: Vec<u8>,
    pub address: Vec<u8>,
}

pub fn get_token_accounts(transaction: &ConfirmedTransaction) -> HashMap<Vec<u8>, TokenAccount> {
    let meta = transaction.meta.as_ref().unwrap();
    let accounts = transaction.resolved_accounts();

    let mut map: HashMap<Vec<u8>, TokenAccount> = HashMap::new();
    for token_balance in &meta.pre_token_balances {
        let address: Vec<u8> = accounts[token_balance.account_index as usize].clone();
        let mint = bs58::decode(token_balance.mint.clone()).into_vec().unwrap();
        let owner = bs58::decode(token_balance.owner.clone()).into_vec().unwrap();

        map.insert(address.clone(), TokenAccount {
            mint,
            owner,
            address,
        });
    }
    map
}

pub fn pubkey_from_string(address: &String) -> Result<Pubkey, &'static str> {
    match bs58::decode(address).into_vec() {
        Ok(decoded) => pubkey_from_vec(decoded),
        Err(e) => Err("String is not a Pubkey."),
    }
}

pub fn pubkey_from_vec(address: Vec<u8>) -> Result<Pubkey, &'static str> {
    let bytes: Result<[u8; 32], _> = address.try_into();
    match bytes {
        Ok(arr) => Ok(Pubkey::from(arr)),
        Err(e) => Err("Vec is not a Pubkey."),
    }
}

pub trait ConfirmedTransactionExt {
    fn signature(&self) -> &Vec<u8>;
}

impl ConfirmedTransactionExt for ConfirmedTransaction {
    fn signature(&self) -> &Vec<u8> {
        &self.transaction.as_ref().unwrap().signatures[0]
    }
}

struct TransactionParser {
    signatures: Vec<String>,
    signers: Vec<Pubkey>,
    token_accounts: HashMap<Pubkey, TokenAccount>,
    instructions: Vec<StructuredInstruction>,
}

impl TransactionParser {
    pub fn new(transaction: &ConfirmedTransaction) {

    }
}
