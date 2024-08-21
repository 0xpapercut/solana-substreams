use std::fmt;
use borsh::BorshDeserialize;

pub const MAX_SEED_LEN: u32 = 32;

#[derive(BorshDeserialize)]
pub struct Pubkey(pub [u8; 32]);

impl Pubkey {
    pub fn try_from(pubkey: &[u8]) -> Result<Self, &'static str> {
        Ok(Pubkey(pubkey.try_into().map_err(|_| "Failed to convert to Pubkey.")?))
    }
    pub fn unpack(data: &[u8]) -> Result<Self, &'static str> where Self: Sized {
        Pubkey::try_from(data).map_err(|_| "Failed to unpack Pubkey.")
    }
}

impl fmt::Debug for Pubkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // f.debug_struct("Pubkey").field(name, value)
        f.debug_tuple("Pubkey")
            .field(&bs58::encode(self.0).into_string())
            .finish()
    }
}
