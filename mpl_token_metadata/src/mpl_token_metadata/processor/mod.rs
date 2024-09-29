// mod burn;
// mod collection;
// mod delegate;
// mod edition;
// pub(crate) mod escrow;
// mod fee;
// mod freeze;
// mod metadata;
// mod state;
// mod uses;
// mod verification;

use borsh::BorshDeserialize;
// pub use burn::*;
// pub use collection::*;
// pub use delegate::*;
// pub use edition::*;
// pub use escrow::*;
// pub use freeze::*;
// pub use metadata::*;
// use mpl_token_auth_rules::payload::Payload;
// pub use state::*;
// pub use uses::*;
// pub use verification::*;

use substreams_solana_utils::pubkey::Pubkey;
use std::collections::HashMap;

#[repr(C)]
#[derive(BorshDeserialize, PartialEq, Eq, Debug, Clone)]
/// A seed path type used by the `DerivedKeyMatch` rule.
pub struct SeedsVec {
    /// The vector of derivation seeds.
    pub seeds: Vec<Vec<u8>>,
}

impl SeedsVec {
    /// Create a new `SeedsVec`.
    pub fn new(seeds: Vec<Vec<u8>>) -> Self {
        Self { seeds }
    }
}

#[repr(C)]
#[derive(BorshDeserialize, PartialEq, Eq, Debug, Clone)]
/// A proof type used by the `PubkeyTreeMatch` rule.
pub struct ProofInfo {
    /// The merkle proof.
    pub proof: Vec<[u8; 32]>,
}

impl ProofInfo {
    /// Create a new `ProofInfo`.
    pub fn new(proof: Vec<[u8; 32]>) -> Self {
        Self { proof }
    }
}

#[repr(C)]
#[derive(BorshDeserialize, PartialEq, Eq, Debug, Clone)]
/// Variants representing the different types represented in a payload.
pub enum PayloadType {
    /// A plain `Pubkey`.
    Pubkey(Pubkey),
    /// PDA derivation seeds.
    Seeds(SeedsVec),
    /// A merkle proof.
    MerkleProof(ProofInfo),
    /// A plain `u64` used for `Amount`.
    Number(u64),
}

#[derive(
    BorshDeserialize, PartialEq, Eq, Debug, Clone, Default,
)]
/// A wrapper type for the payload hashmap.
pub struct Payload {
    map: HashMap<String, PayloadType>,
}



#[derive(BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct AuthorizationData {
    pub payload: Payload,
}

impl AuthorizationData {
    pub fn new(payload: Payload) -> Self {
        Self { payload }
    }
    pub fn new_empty() -> Self {
        Self {
            payload: Payload::new(),
        }
    }
}

impl Payload {
    /// Create a new empty `Payload`.
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}
