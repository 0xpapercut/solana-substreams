use super::*;
use substreams_solana_utils::pubkey::Pubkey;

pub const MAX_EDITION_LEN: usize = 1 + 32 + 8 + 200;

// The last byte of the account contains the token standard value for
// pNFT assets. This is used to restrict legacy operations on the master
// edition account.
pub const TOKEN_STANDARD_INDEX_EDITION: usize = MAX_EDITION_LEN - 1;

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, Eq, BorshDeserialize)]
/// All Editions should never have a supply greater than 1.
/// To enforce this, a transfer mint authority instruction will happen when
/// a normal token is turned into an Edition, and in order for a Metadata update authority
/// to do this transaction they will also need to sign the transaction as the Mint authority.
pub struct Edition {
    pub key: Key,

    /// Points at MasterEdition struct
    #[cfg_attr(feature = "serde-feature", serde(with = "As::<DisplayFromStr>"))]
    pub parent: Pubkey,

    /// Starting at 0 for master record, this is incremented for each edition minted.
    pub edition: u64,
}

impl Default for Edition {
    fn default() -> Self {
        Edition {
            key: Key::EditionV1,
            parent: Pubkey::default(),
            edition: 0,
        }
    }
}

impl TokenMetadataAccount for Edition {
    fn key() -> Key {
        Key::EditionV1
    }

    fn size() -> usize {
        MAX_EDITION_LEN
    }
}
