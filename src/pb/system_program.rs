// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemProgramBlockEvents {
    #[prost(uint64, tag="1")]
    pub slot: u64,
    #[prost(message, repeated, tag="2")]
    pub transactions: ::prost::alloc::vec::Vec<SystemProgramTransactionEvents>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemProgramTransactionEvents {
    #[prost(string, tag="1")]
    pub signature: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub transaction_index: u32,
    #[prost(message, repeated, tag="3")]
    pub events: ::prost::alloc::vec::Vec<SystemProgramEvent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemProgramEvent {
    #[prost(uint32, tag="1")]
    pub instruction_index: u32,
    #[prost(oneof="system_program_event::Event", tags="2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14")]
    pub event: ::core::option::Option<system_program_event::Event>,
}
/// Nested message and enum types in `SystemProgramEvent`.
pub mod system_program_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        #[prost(message, tag="2")]
        CreateAccount(super::CreateAccountEvent),
        #[prost(message, tag="3")]
        Assign(super::AssignEvent),
        #[prost(message, tag="4")]
        Transfer(super::TransferEvent),
        #[prost(message, tag="5")]
        CreateAccountWithSeed(super::CreateAccountWithSeedEvent),
        #[prost(message, tag="6")]
        AdvanceNonceAccount(super::AdvanceNonceAccountEvent),
        #[prost(message, tag="7")]
        WithdrawNonceAccount(super::WithdrawNonceAccountEvent),
        #[prost(message, tag="8")]
        InitializeNonceAccount(super::InitializeNonceAccountEvent),
        #[prost(message, tag="9")]
        AuthorizeNonceAccount(super::AuthorizeNonceAccountEvent),
        #[prost(message, tag="10")]
        Allocate(super::AllocateEvent),
        #[prost(message, tag="11")]
        AllocateWithSeed(super::AllocateWithSeedEvent),
        #[prost(message, tag="12")]
        AssignWithSeed(super::AssignWithSeedEvent),
        #[prost(message, tag="13")]
        TransferWithSeed(super::TransferWithSeedEvent),
        #[prost(message, tag="14")]
        UpgradeNonceAccount(super::UpgradeNonceAccountEvent),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAccountEvent {
    #[prost(string, tag="1")]
    pub funding_account: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub new_account: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub lamports: u64,
    #[prost(uint64, tag="4")]
    pub space: u64,
    #[prost(string, tag="5")]
    pub owner: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssignEvent {
    #[prost(string, tag="1")]
    pub assigned_account: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferEvent {
    #[prost(string, tag="1")]
    pub funding_account: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub recipient_account: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub lamports: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAccountWithSeedEvent {
    #[prost(string, tag="1")]
    pub funding_account: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub created_account: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub base_account: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub seed: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub lamports: u64,
    #[prost(uint64, tag="6")]
    pub space: u64,
    #[prost(string, tag="7")]
    pub owner: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdvanceNonceAccountEvent {
    #[prost(string, tag="1")]
    pub nonce_account: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub nonce_authority: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawNonceAccountEvent {
    #[prost(string, tag="1")]
    pub nonce_account: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub recipient_account: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub nonce_authority: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub lamports: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeNonceAccountEvent {
    #[prost(string, tag="1")]
    pub nonce_account: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub nonce_authority: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizeNonceAccountEvent {
    #[prost(string, tag="1")]
    pub nonce_account: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub nonce_authority: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub new_nonce_authority: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllocateEvent {
    #[prost(string, tag="1")]
    pub account: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub space: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllocateWithSeedEvent {
    #[prost(string, tag="1")]
    pub allocated_account: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub base_account: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub seed: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub space: u64,
    #[prost(string, tag="5")]
    pub owner: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssignWithSeedEvent {
    #[prost(string, tag="1")]
    pub assigned_account: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub base_account: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub seed: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub owner: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferWithSeedEvent {
    #[prost(string, tag="1")]
    pub funding_account: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub base_account: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub recipient_account: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub lamports: u64,
    #[prost(string, tag="5")]
    pub from_seed: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub from_owner: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpgradeNonceAccountEvent {
    #[prost(string, tag="1")]
    pub nonce_account: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
