use super::*;
use substreams_solana_utils::pubkey::Pubkey;

// Large buffer because the older master editions have two pubkeys in them,
// need to keep two versions same size because the conversion process actually
// changes the same account by rewriting it.
pub const MAX_MASTER_EDITION_LEN: usize = 1 + 9 + 8 + 264;

// The last byte of the account containts the token standard value for
// pNFT assets. This is used to restrict legacy operations on the master
// edition account.
pub const TOKEN_STANDARD_INDEX: usize = MAX_MASTER_EDITION_LEN - 1;

// The second to last byte of the account contains the fee flag, indicating
// if the account has fees available for retrieval.
pub const MASTER_EDITION_FEE_FLAG_INDEX: usize = MAX_MASTER_EDITION_LEN - 2;

pub trait MasterEdition {
    fn key(&self) -> Key;
    fn supply(&self) -> u64;
    fn set_supply(&mut self, supply: u64);
    fn max_supply(&self) -> Option<u64>;
}

#[derive(Clone, Debug, PartialEq, Eq, BorshDeserialize)]
pub struct MasterEditionV2 {
    pub key: Key,

    pub supply: u64,

    pub max_supply: Option<u64>,
}

impl Default for MasterEditionV2 {
    fn default() -> Self {
        MasterEditionV2 {
            key: Key::MasterEditionV2,
            supply: 0,
            max_supply: Some(0),
        }
    }
}

impl TokenMetadataAccount for MasterEditionV2 {
    fn key() -> Key {
        Key::MasterEditionV2
    }

    fn size() -> usize {
        MAX_MASTER_EDITION_LEN
    }
}

impl MasterEdition for MasterEditionV2 {
    fn key(&self) -> Key {
        self.key
    }

    fn supply(&self) -> u64 {
        self.supply
    }

    fn set_supply(&mut self, supply: u64) {
        self.supply = supply;
    }

    fn max_supply(&self) -> Option<u64> {
        self.max_supply
    }
}

#[derive(Clone, Debug, PartialEq, Eq, BorshDeserialize)]
pub struct MasterEditionV1 {
    pub key: Key,

    pub supply: u64,

    pub max_supply: Option<u64>,

    /// Can be used to mint tokens that give one-time permission to mint a single limited edition.
    pub printing_mint: Pubkey,

    /// If you don't know how many printing tokens you are going to need, but you do know
    /// you are going to need some amount in the future, you can use a token from this mint.
    /// Coming back to token metadata with one of these tokens allows you to mint (one time)
    /// any number of printing tokens you want. This is used for instance by Auction Manager
    /// with participation NFTs, where we dont know how many people will bid and need participation
    /// printing tokens to redeem, so we give it ONE of these tokens to use after the auction is over,
    /// because when the auction begins we just dont know how many printing tokens we will need,
    /// but at the end we will. At the end it then burns this token with token-metadata to
    /// get the printing tokens it needs to give to bidders. Each bidder then redeems a printing token
    /// to get their limited editions.
    pub one_time_printing_authorization_mint: Pubkey,
}

impl TokenMetadataAccount for MasterEditionV1 {
    fn key() -> Key {
        Key::MasterEditionV1
    }

    fn size() -> usize {
        MAX_MASTER_EDITION_LEN
    }
}

impl MasterEdition for MasterEditionV1 {
    fn key(&self) -> Key {
        self.key
    }

    fn supply(&self) -> u64 {
        self.supply
    }

    fn max_supply(&self) -> Option<u64> {
        self.max_supply
    }

    fn set_supply(&mut self, supply: u64) {
        self.supply = supply;
    }
}
