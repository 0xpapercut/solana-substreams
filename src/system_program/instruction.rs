// use substreams_solana_program_instructions::pubkey::Pubkey;
use crate::pubkey::Pubkey;
use borsh::BorshDeserialize;

#[derive(Debug, BorshDeserialize)]
pub struct CreateAccount {
    /// Number of lamports to transfer to the new account
    pub lamports: u64,
    /// Number of bytes of memory to allocate
    pub space: u64,
    /// Address of program that will own the new account
    pub owner: Pubkey,
}

#[derive(Debug, BorshDeserialize)]
pub struct Assign {
    /// Owner program account
    pub owner: Pubkey,
}

#[derive(Debug, BorshDeserialize)]
pub struct Transfer {
    pub lamports: u64
}

#[derive(Debug, BorshDeserialize)]
pub struct TransferWithSeed {
    /// Amount to transfer
    pub lamports: u64,
    /// Seed to use to derive the funding account address
    pub from_seed: Seed,
    /// Owner to use to derive the funding account address
    pub from_owner: Pubkey,
}

#[derive(Debug, BorshDeserialize)]
pub struct CreateAccountWithSeed {
    /// Base public key
    pub base: Pubkey,
    /// String of ASCII chars, no longer than `Pubkey::MAX_SEED_LEN`
    pub seed: Seed,
    /// Number of lamports to transfer to the new account
    pub lamports: u64,
    /// Number of bytes of memory to allocate
    pub space: u64,
    /// Owner program account address
    pub owner: Pubkey,
}

#[derive(Debug, BorshDeserialize)]
pub struct Allocate {
    /// Number of bytes of memory to allocate
    pub space: u64,
}

#[derive(Debug, BorshDeserialize)]
pub struct AllocateWithSeed {
    /// Base public key
    pub base: Pubkey,
    /// String of ASCII chars, no longer than `pubkey::MAX_SEED_LEN`
    pub seed: Seed,
    /// Number of bytes of memory to allocate
    pub space: u64,
    /// Owner program account
    pub owner: Pubkey,
}

#[derive(Debug, BorshDeserialize)]
pub struct AssignWithSeed {
    /// Base public key
    pub base: Pubkey,
    /// String of ASCII chars, no longer than `pubkey::MAX_SEED_LEN`
    pub seed: Seed,
    /// Owner program account
    pub owner: Pubkey,
}

#[derive(Debug, BorshDeserialize)]
pub enum SystemInstruction {
    /// Create a new account
    ///
    /// # Account references
    ///   0. `[WRITE, SIGNER]` Funding account
    ///   1. `[WRITE, SIGNER]` New account
    CreateAccount(CreateAccount),

    /// Assign account to a program
    ///
    /// # Account references
    ///   0. `[WRITE, SIGNER]` Assigned account public key
    Assign(Assign),

    /// Transfer lamports
    ///
    /// # Account references
    ///   0. `[WRITE, SIGNER]` Funding account
    ///   1. `[WRITE]` Recipient account
    Transfer(Transfer),

    /// Create a new account at an address derived from a base pubkey and a seed
    ///
    /// # Account references
    ///   0. `[WRITE, SIGNER]` Funding account
    ///   1. `[WRITE]` Created account
    ///   2. `[SIGNER]` (optional) Base account; the account matching the base Pubkey below must be
    ///                          provided as a signer, but may be the same as the funding account
    ///                          and provided as account 0
    CreateAccountWithSeed(CreateAccountWithSeed),

    /// Consumes a stored nonce, replacing it with a successor
    ///
    /// # Account references
    ///   0. `[WRITE]` Nonce account
    ///   1. `[]` RecentBlockhashes sysvar
    ///   2. `[SIGNER]` Nonce authority
    AdvanceNonceAccount,

    /// Withdraw funds from a nonce account
    ///
    /// # Account references
    ///   0. `[WRITE]` Nonce account
    ///   1. `[WRITE]` Recipient account
    ///   2. `[]` RecentBlockhashes sysvar
    ///   3. `[]` Rent sysvar
    ///   4. `[SIGNER]` Nonce authority
    ///
    /// The `u64` parameter is the lamports to withdraw, which must leave the
    /// account balance above the rent exempt reserve or at zero.
    WithdrawNonceAccount(u64),

    /// Drive state of Uninitialized nonce account to Initialized, setting the nonce value
    ///
    /// # Account references
    ///   0. `[WRITE]` Nonce account
    ///   1. `[]` RecentBlockhashes sysvar
    ///   2. `[]` Rent sysvar
    ///
    /// The `Pubkey` parameter specifies the entity authorized to execute nonce
    /// instruction on the account
    ///
    /// No signatures are required to execute this instruction, enabling derived
    /// nonce account addresses
    InitializeNonceAccount(Pubkey),

    /// Change the entity authorized to execute nonce instructions on the account
    ///
    /// # Account references
    ///   0. `[WRITE]` Nonce account
    ///   1. `[SIGNER]` Nonce authority
    ///
    /// The `Pubkey` parameter identifies the entity to authorize
    AuthorizeNonceAccount(Pubkey),

    /// Allocate space in a (possibly new) account without funding
    ///
    /// # Account references
    ///   0. `[WRITE, SIGNER]` New account
    Allocate(Allocate),

    /// Allocate space for and assign an account at an address
    ///    derived from a base public key and a seed
    ///
    /// # Account references
    ///   0. `[WRITE]` Allocated account
    ///   1. `[SIGNER]` Base account
    AllocateWithSeed(AllocateWithSeed),

    /// Assign account to a program based on a seed
    ///
    /// # Account references
    ///   0. `[WRITE]` Assigned account
    ///   1. `[SIGNER]` Base account
    AssignWithSeed(AssignWithSeed),

    /// Transfer lamports from a derived address
    ///
    /// # Account references
    ///   0. `[WRITE]` Funding account
    ///   1. `[SIGNER]` Base for funding account
    ///   2. `[WRITE]` Recipient account
    TransferWithSeed(TransferWithSeed),

    /// One-time idempotent upgrade of legacy nonce versions in order to bump
    /// them out of chain blockhash domain.
    ///
    /// # Account references
    ///   0. `[WRITE]` Nonce account
    UpgradeNonceAccount,
}

impl SystemInstruction {
    pub fn unpack(data: &[u8]) -> Result<Self, &'static str> {
        let (tag, data) = data.split_at(4);
        match tag[0] {
            0 => CreateAccount::unpack(data).map(Self::CreateAccount),
            1 => Assign::unpack(data).map(Self::Assign),
            2 => Transfer::unpack(data).map(Self::Transfer),
            3 => CreateAccountWithSeed::unpack(data).map(Self::CreateAccountWithSeed),
            4 => Ok(Self::AdvanceNonceAccount),
            5 => u64::unpack(data).map(Self::WithdrawNonceAccount),
            6 => Pubkey::unpack(data).map(Self::InitializeNonceAccount),
            7 => Pubkey::unpack(data).map(Self::AuthorizeNonceAccount),
            8 => Allocate::unpack(data).map(Self::Allocate),
            9 => AllocateWithSeed::unpack(data).map(Self::AllocateWithSeed),
            10 => AssignWithSeed::unpack(data).map(Self::AssignWithSeed),
            11 => TransferWithSeed::unpack(data).map(Self::TransferWithSeed),
            12 => Ok(Self::UpgradeNonceAccount),
            _ => Err("Failed to unpack System Program instruction.")
        }
    }
}

impl CreateAccount {
    pub fn unpack(data: &[u8]) -> Result<Self, &'static str> {
        Ok(Self::try_from_slice(data).unwrap())
    }
}

impl Assign {
    pub fn unpack(data: &[u8]) -> Result<Self, &'static str> {
        Ok(Self::try_from_slice(data).unwrap())
    }
}

impl Transfer {
    pub fn unpack(data: &[u8]) -> Result<Self, &'static str> {
        Ok(Self::try_from_slice(data).unwrap())
    }
}

impl TransferWithSeed {
    pub fn unpack(data: &[u8]) -> Result<Self, &'static str> {
        Ok(Self::try_from_slice(data).unwrap())
    }
}

impl CreateAccountWithSeed {
    pub fn unpack(data: &[u8]) -> Result<Self, &'static str> {
        Ok(Self::try_from_slice(data).unwrap())
    }
}

impl Allocate {
    pub fn unpack(data: &[u8]) -> Result<Self, &'static str> {
        Ok(Self::try_from_slice(data).unwrap())
        // let space = u64::from_le_bytes(data.try_into().unwrap());
        // Ok(Allocate { space })
    }
}

impl AllocateWithSeed {
    pub fn unpack(data: &[u8]) -> Result<Self, &'static str> {
        Ok(Self::try_from_slice(data).unwrap())
    }
}

impl AssignWithSeed {
    pub fn unpack(data: &[u8]) -> Result<Self, &'static str> {
        Ok(Self::try_from_slice(data).unwrap())
    }
}

trait Unpack {
    fn unpack(data: &[u8]) -> Result<Self, &'static str> where Self: Sized;
}

impl Unpack for u64 {
    fn unpack(data: &[u8]) -> Result<Self, &'static str> where Self: Sized {
        Ok(u64::from_le_bytes(data.try_into().unwrap()))
    }
}

#[derive(Debug, Clone)]
pub struct Seed(pub String);

impl BorshDeserialize for Seed {
    fn deserialize_reader<R: std::io::prelude::Read>(reader: &mut R) -> std::io::Result<Self> {
        let mut length_bytes = [0u8; 8];
        reader.read_exact(&mut length_bytes)?;
        let length = u64::from_le_bytes(length_bytes) as usize;

        let mut string_bytes = vec![0u8; length];
        reader.read_exact(&mut string_bytes)?;

        let seed_string = String::from_utf8(string_bytes)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid UTF-8 sequence"))?;

        Ok(Seed(seed_string))
    }
}
