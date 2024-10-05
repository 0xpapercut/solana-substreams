use std::fmt;
use borsh::BorshDeserialize;
use anyhow::{anyhow, Context, Error};

#[derive(Clone, Copy, BorshDeserialize, Hash, Eq, PartialEq)]
pub struct Pubkey(pub [u8; 32]);

impl Pubkey {
    pub fn try_from(pubkey: &[u8]) -> Result<Self, Error> {
        Ok(Pubkey(pubkey.try_into().context("Failed to convert &[u8] to Pubkey")?))
    }
    pub fn unpack(data: &[u8]) -> Result<Self, Error> where Self: Sized {
        Pubkey::try_from(data)
    }
    pub fn try_from_string(pubkey: &str) -> Result<Self, Error> {
        let decoded = bs58::decode(pubkey).into_vec()?;
        let slice: [u8; 32] = decoded.try_into().map_err(|_| anyhow!("Failed to convert to pubkey to slice."))?;
        Ok(Self(slice))
    }
    pub fn from_string(pubkey: &str) -> Self {
        Pubkey::try_from_string(pubkey).unwrap()
    }
    pub fn to_string(&self) -> String {
        bs58::encode(self.0).into_string()
    }
    pub fn default() -> Self {
        Pubkey([0u8; 32])
    }
}

impl fmt::Debug for Pubkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Pubkey")
            .field(&bs58::encode(self.0).into_string())
            .finish()
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct PubkeyRef<'a>(pub &'a Vec<u8>);

impl<'a> PubkeyRef<'a> {
    pub fn to_pubkey(&self) -> Result<Pubkey, Error> {
        Pubkey::try_from(self.0)
    }
    pub fn to_string(&self) -> String {
        bs58::encode(self.0).into_string()
    }
}

impl fmt::Debug for PubkeyRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Pubkey")
            .field(&bs58::encode(self.0).into_string())
            .finish()
    }
}

impl PartialEq<PubkeyRef<'_>> for Pubkey {
    fn eq(&self, other: &PubkeyRef) -> bool {
        self.0 == other.0.as_slice()
    }
}

impl PartialEq<Pubkey> for PubkeyRef<'_> {
    fn eq(&self, other: &Pubkey) -> bool {
        self.0.as_slice() == other.0
    }
}

impl PartialEq<&PubkeyRef<'_>> for Pubkey {
    fn eq(&self, other: &&PubkeyRef) -> bool {
        self.0 == other.0.as_slice()
    }
}

impl PartialEq<&Pubkey> for PubkeyRef<'_> {
    fn eq(&self, other: &&Pubkey) -> bool {
        self.0.as_slice() == other.0
    }
}
