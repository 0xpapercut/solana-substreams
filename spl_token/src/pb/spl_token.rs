// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SplTokenBlockEvents {
    #[prost(message, repeated, tag="1")]
    pub transactions: ::prost::alloc::vec::Vec<SplTokenTransactionEvents>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SplTokenTransactionEvents {
    #[prost(string, tag="1")]
    pub signature: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub events: ::prost::alloc::vec::Vec<SplTokenEvent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SplTokenEvent {
    #[prost(oneof="spl_token_event::Event", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14")]
    pub event: ::core::option::Option<spl_token_event::Event>,
}
/// Nested message and enum types in `SplTokenEvent`.
pub mod spl_token_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        #[prost(message, tag="1")]
        Transfer(super::TransferEvent),
        #[prost(message, tag="2")]
        InitializeMint(super::InitializeMintEvent),
        #[prost(message, tag="3")]
        InitializeImmutableOwner(super::InitializeImmutableOwnerEvent),
        #[prost(message, tag="4")]
        InitializeAccount(super::InitializeAccountEvent),
        #[prost(message, tag="5")]
        InitializeMultisig(super::InitializeMultisigEvent),
        #[prost(message, tag="6")]
        Approve(super::ApproveEvent),
        #[prost(message, tag="7")]
        MintTo(super::MintToEvent),
        #[prost(message, tag="8")]
        Revoke(super::RevokeEvent),
        #[prost(message, tag="9")]
        SetAuthority(super::SetAuthorityEvent),
        #[prost(message, tag="10")]
        Burn(super::BurnEvent),
        #[prost(message, tag="11")]
        CloseAccount(super::CloseAccountEvent),
        #[prost(message, tag="12")]
        FreezeAccount(super::FreezeAccountEvent),
        #[prost(message, tag="13")]
        ThawAccount(super::ThawAccountEvent),
        #[prost(message, tag="14")]
        SyncNative(super::SyncNativeEvent),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeMintEvent {
    #[prost(string, tag="1")]
    pub mint: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub decimals: u32,
    #[prost(string, tag="3")]
    pub mint_authority: ::prost::alloc::string::String,
    #[prost(string, optional, tag="4")]
    pub freeze_authority: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeAccountEvent {
    #[prost(message, optional, tag="1")]
    pub account: ::core::option::Option<TokenAccount>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeMultisigEvent {
    #[prost(string, tag="1")]
    pub multisig: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint32, tag="3")]
    pub m: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferEvent {
    #[prost(message, optional, tag="1")]
    pub source: ::core::option::Option<TokenAccount>,
    #[prost(message, optional, tag="2")]
    pub destination: ::core::option::Option<TokenAccount>,
    #[prost(string, tag="3")]
    pub authority: ::prost::alloc::string::String,
    /// optional uint64 source_pre_balance = 5;
    /// optional uint64 destination_pre_balance = 6;
    #[prost(uint64, tag="4")]
    pub amount: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApproveEvent {
    #[prost(message, optional, tag="1")]
    pub source: ::core::option::Option<TokenAccount>,
    #[prost(string, tag="2")]
    pub delegate: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub amount: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RevokeEvent {
    #[prost(message, optional, tag="1")]
    pub source: ::core::option::Option<TokenAccount>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAuthorityEvent {
    #[prost(string, tag="1")]
    pub mint: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub authority: ::prost::alloc::string::String,
    #[prost(enumeration="AuthorityType", tag="3")]
    pub authority_type: i32,
    #[prost(string, optional, tag="4")]
    pub new_authority: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MintToEvent {
    #[prost(string, tag="1")]
    pub mint: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub mint_authority: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub destination: ::core::option::Option<TokenAccount>,
    #[prost(uint64, tag="4")]
    pub amount: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BurnEvent {
    #[prost(message, optional, tag="1")]
    pub source: ::core::option::Option<TokenAccount>,
    #[prost(string, tag="3")]
    pub authority: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub amount: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloseAccountEvent {
    #[prost(message, optional, tag="1")]
    pub source: ::core::option::Option<TokenAccount>,
    /// TODO: amount
    #[prost(string, tag="2")]
    pub destination: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FreezeAccountEvent {
    #[prost(message, optional, tag="1")]
    pub source: ::core::option::Option<TokenAccount>,
    #[prost(string, tag="2")]
    pub freeze_authority: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThawAccountEvent {
    #[prost(message, optional, tag="1")]
    pub source: ::core::option::Option<TokenAccount>,
    #[prost(string, tag="2")]
    pub freeze_authority: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeImmutableOwnerEvent {
    #[prost(message, optional, tag="1")]
    pub account: ::core::option::Option<TokenAccount>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyncNativeEvent {
    /// TODO: amount
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
    #[prost(uint64, optional, tag="4")]
    pub pre_balance: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="5")]
    pub post_balance: ::core::option::Option<u64>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AuthorityType {
    Null = 0,
    MintTokens = 1,
    FreezeAccount = 2,
    AccountOwner = 3,
    CloseAccount = 4,
}
impl AuthorityType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AuthorityType::Null => "Null",
            AuthorityType::MintTokens => "MintTokens",
            AuthorityType::FreezeAccount => "FreezeAccount",
            AuthorityType::AccountOwner => "AccountOwner",
            AuthorityType::CloseAccount => "CloseAccount",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Null" => Some(Self::Null),
            "MintTokens" => Some(Self::MintTokens),
            "FreezeAccount" => Some(Self::FreezeAccount),
            "AccountOwner" => Some(Self::AccountOwner),
            "CloseAccount" => Some(Self::CloseAccount),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
