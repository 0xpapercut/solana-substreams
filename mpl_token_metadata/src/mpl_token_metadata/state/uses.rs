use super::*;
use super::super::utils::try_from_slice_checked;

pub const USE_AUTHORITY_RECORD_SIZE: usize = 18; //8 byte padding

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone, FromPrimitive)]
pub enum UseMethod {
    Burn,
    Multiple,
    Single,
}

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct Uses {
    // 17 bytes + Option byte
    pub use_method: UseMethod, //1
    pub remaining: u64,        //8
    pub total: u64,            //8
}

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct UseAuthorityRecord {
    pub key: Key,          //1
    pub allowed_uses: u64, //8
    pub bump: u8,
}

impl Default for UseAuthorityRecord {
    fn default() -> Self {
        UseAuthorityRecord {
            key: Key::UseAuthorityRecord,
            allowed_uses: 0,
            bump: 255,
        }
    }
}

impl TokenMetadataAccount for UseAuthorityRecord {
    fn key() -> Key {
        Key::UseAuthorityRecord
    }

    fn size() -> usize {
        USE_AUTHORITY_RECORD_SIZE
    }
}

impl UseAuthorityRecord {
    pub fn from_bytes(b: &[u8]) -> Result<UseAuthorityRecord, ProgramError> {
        let ua: UseAuthorityRecord =
            try_from_slice_checked(b, Key::UseAuthorityRecord, USE_AUTHORITY_RECORD_SIZE)?;
        Ok(ua)
    }

    pub fn bump_empty(&self) -> bool {
        self.bump == 0 && self.key == Key::UseAuthorityRecord
    }
}
