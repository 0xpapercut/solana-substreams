// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PumpfunBlockEvents {
    #[prost(message, repeated, tag="1")]
    pub transactions: ::prost::alloc::vec::Vec<PumpfunTransactionEvents>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PumpfunTransactionEvents {
    #[prost(string, tag="1")]
    pub signature: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub events: ::prost::alloc::vec::Vec<PumpfunEvent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PumpfunEvent {
    #[prost(oneof="pumpfun_event::Event", tags="1, 2, 3, 4, 5")]
    pub event: ::core::option::Option<pumpfun_event::Event>,
}
/// Nested message and enum types in `PumpfunEvent`.
pub mod pumpfun_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        #[prost(message, tag="1")]
        Initialize(super::InitializeEvent),
        #[prost(message, tag="2")]
        SetParams(super::SetParamsEvent),
        #[prost(message, tag="3")]
        Swap(super::SwapEvent),
        #[prost(message, tag="4")]
        Withdraw(super::WithdrawEvent),
        #[prost(message, tag="5")]
        Create(super::CreateEvent),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEvent {
    #[prost(string, tag="1")]
    pub user: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub uri: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub mint: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub bonding_curve: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub associated_bonding_curve: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub metadata: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeEvent {
    #[prost(string, tag="1")]
    pub user: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetParamsEvent {
    #[prost(string, tag="1")]
    pub user: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub fee_recipient: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub initial_virtual_token_reserves: u64,
    #[prost(uint64, tag="4")]
    pub initial_virtual_sol_reserves: u64,
    #[prost(uint64, tag="5")]
    pub initial_real_token_reserves: u64,
    #[prost(uint64, tag="6")]
    pub token_total_supply: u64,
    #[prost(uint64, tag="7")]
    pub fee_basis_points: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapEvent {
    #[prost(string, tag="1")]
    pub user: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub mint: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub bonding_curve: ::prost::alloc::string::String,
    #[prost(uint64, optional, tag="4")]
    pub sol_amount: ::core::option::Option<u64>,
    #[prost(uint64, tag="5")]
    pub token_amount: u64,
    #[prost(string, tag="6")]
    pub direction: ::prost::alloc::string::String,
    #[prost(uint64, optional, tag="7")]
    pub virtual_sol_reserves: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="8")]
    pub virtual_token_reserves: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="9")]
    pub real_sol_reserves: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="10")]
    pub real_token_reserves: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawEvent {
    #[prost(string, tag="1")]
    pub mint: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
