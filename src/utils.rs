use std::collections::HashMap;

use substreams_solana::pb::sf::solana::r#type::v1::ConfirmedTransaction;

pub struct TokenAccount {
    pub owner: String,
    pub mint: String,
}

pub fn get_token_accounts(transaction: &ConfirmedTransaction) -> HashMap<String, TokenAccount> {
    let meta = transaction.meta.as_ref().unwrap();
    let accounts = transaction.resolved_accounts_as_strings();

    let mut map: HashMap<String, TokenAccount> = HashMap::new();
    for token_balance in &meta.pre_token_balances {
        let address = &accounts[token_balance.account_index as usize];
        map.insert(address.clone(), TokenAccount {
            mint: token_balance.mint.clone(),
            owner: token_balance.owner.clone(),
        });
    }
    map
}
