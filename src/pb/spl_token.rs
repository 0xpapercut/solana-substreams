// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Events {
    #[prost(message, repeated, tag="1")]
    pub events: ::prost::alloc::vec::Vec<Event>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    #[prost(string, tag="1")]
    pub signature: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint64, tag="3")]
    pub slot: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    #[prost(message, optional, tag="1")]
    pub transaction: ::core::option::Option<Transaction>,
    #[prost(oneof="event::Event", tags="2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15")]
    pub event: ::core::option::Option<event::Event>,
}
/// Nested message and enum types in `Event`.
pub mod event {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        #[prost(message, tag="2")]
        Transfer(super::TransferEvent),
        #[prost(message, tag="3")]
        InitializeMint(super::InitializeMintEvent),
        #[prost(message, tag="4")]
        InitializeAccount(super::InitializeAccountEvent),
        #[prost(message, tag="5")]
        InitializeMultiSig(super::InitializeMultiSigEvent),
        #[prost(message, tag="6")]
        InitializeImmutableOwner(super::InitializeImmutableOwnerEvent),
        #[prost(message, tag="7")]
        Approve(super::ApproveEvent),
        #[prost(message, tag="8")]
        Revoke(super::RevokeEvent),
        #[prost(message, tag="9")]
        SetAuthority(super::SetAuthorityEvent),
        #[prost(message, tag="10")]
        MintTo(super::MintToEvent),
        #[prost(message, tag="11")]
        Burn(super::BurnEvent),
        #[prost(message, tag="12")]
        CloseAccount(super::CloseAccountEvent),
        #[prost(message, tag="13")]
        FreezeAccount(super::FreezeAcccountEvent),
        #[prost(message, tag="14")]
        ThawAccount(super::ThawAccountEvent),
        #[prost(message, tag="15")]
        SyncNative(super::SyncNativeEvent),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferEvent {
    #[prost(message, optional, tag="1")]
    pub source: ::core::option::Option<TokenAccount>,
    #[prost(message, optional, tag="2")]
    pub destination: ::core::option::Option<TokenAccount>,
    #[prost(uint64, tag="3")]
    pub amount: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeMintEvent {
    #[prost(string, tag="1")]
    pub mint: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub mint_authority: ::prost::alloc::string::String,
    #[prost(string, optional, tag="3")]
    pub freeze_authority: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeMultiSigEvent {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub m: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeImmutableOwnerEvent {
    #[prost(message, optional, tag="1")]
    pub account: ::core::option::Option<TokenAccount>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApproveEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RevokeEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAuthorityEvent {
    #[prost(enumeration="AuthorityType", tag="1")]
    pub authority_type: i32,
    #[prost(string, optional, tag="2")]
    pub new_authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag="3")]
    pub authority: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub account: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MintToEvent {
    #[prost(message, optional, tag="1")]
    pub destination: ::core::option::Option<TokenAccount>,
    #[prost(uint64, tag="2")]
    pub amount: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BurnEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloseAccountEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FreezeAcccountEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThawAccountEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyncNativeEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeAccountEvent {
    #[prost(message, optional, tag="1")]
    pub account: ::core::option::Option<TokenAccount>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenAccount {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub mint: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AuthorityType {
    MintTokens = 0,
    FreezeAccount = 1,
    AccountOwner = 2,
    CloseAccount = 3,
}
impl AuthorityType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AuthorityType::MintTokens => "MintTokens",
            AuthorityType::FreezeAccount => "FreezeAccount",
            AuthorityType::AccountOwner => "AccountOwner",
            AuthorityType::CloseAccount => "CloseAccount",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MintTokens" => Some(Self::MintTokens),
            "FreezeAccount" => Some(Self::FreezeAccount),
            "AccountOwner" => Some(Self::AccountOwner),
            "CloseAccount" => Some(Self::CloseAccount),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
