use super::*;
use super::super::utils::try_from_slice_checked;
use substreams_solana_utils::pubkey::Pubkey;

pub const COLLECTION_AUTHORITY_RECORD_SIZE: usize = 35;

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct Collection {
    pub verified: bool,
    #[cfg_attr(feature = "serde-feature", serde(with = "As::<DisplayFromStr>"))]
    pub key: Pubkey,
}

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct CollectionAuthorityRecord {
    pub key: Key,                         //1
    pub bump: u8,                         //1
    pub update_authority: Option<Pubkey>, //33 (1 + 32)
}

impl Default for CollectionAuthorityRecord {
    fn default() -> Self {
        CollectionAuthorityRecord {
            key: Key::CollectionAuthorityRecord,
            bump: 255,
            update_authority: None,
        }
    }
}

impl TokenMetadataAccount for CollectionAuthorityRecord {
    fn key() -> Key {
        Key::CollectionAuthorityRecord
    }

    fn size() -> usize {
        COLLECTION_AUTHORITY_RECORD_SIZE
    }
}

impl CollectionAuthorityRecord {
    pub fn from_bytes(b: &[u8]) -> Result<CollectionAuthorityRecord, ProgramError> {
        let ca: CollectionAuthorityRecord = try_from_slice_checked(
            b,
            Key::CollectionAuthorityRecord,
            COLLECTION_AUTHORITY_RECORD_SIZE,
        )?;
        Ok(ca)
    }
}

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub enum CollectionDetails {
    #[deprecated(
        since = "1.13.1",
        note = "The collection size tracking feature is deprecated and will soon be removed."
    )]
    V1 {
        size: u64,
    },
    V2 {
        padding: [u8; 8],
    },
}
