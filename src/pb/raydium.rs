// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RaydiumBlockEvents {
    #[prost(message, repeated, tag="1")]
    pub transactions: ::prost::alloc::vec::Vec<RaydiumTransactionEvents>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RaydiumTransactionEvents {
    #[prost(string, tag="1")]
    pub signature: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub events: ::prost::alloc::vec::Vec<RaydiumEvent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RaydiumEvent {
    #[prost(string, tag="1")]
    pub amm: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub user: ::prost::alloc::string::String,
    #[prost(oneof="raydium_event::Data", tags="3, 4, 5, 6")]
    pub data: ::core::option::Option<raydium_event::Data>,
}
/// Nested message and enum types in `RaydiumEvent`.
pub mod raydium_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        #[prost(message, tag="3")]
        Initialize(super::InitializeData),
        #[prost(message, tag="4")]
        Deposit(super::DepositData),
        #[prost(message, tag="5")]
        Withdraw(super::WithdrawData),
        #[prost(message, tag="6")]
        Swap(super::SwapData),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitializeData {
    #[prost(uint64, tag="1")]
    pub pc_init_amount: u64,
    #[prost(uint64, tag="2")]
    pub coin_init_amount: u64,
    #[prost(uint64, tag="3")]
    pub lp_init_amount: u64,
    #[prost(string, tag="4")]
    pub pc_mint: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub coin_mint: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub lp_mint: ::prost::alloc::string::String,
    #[prost(uint32, tag="7")]
    pub nonce: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DepositData {
    #[prost(uint64, tag="1")]
    pub pc_amount: u64,
    #[prost(uint64, tag="2")]
    pub coin_amount: u64,
    #[prost(uint64, tag="3")]
    pub lp_amount: u64,
    #[prost(string, tag="4")]
    pub pc_mint: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub coin_mint: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub lp_mint: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawData {
    #[prost(uint64, tag="1")]
    pub pc_amount: u64,
    #[prost(uint64, tag="2")]
    pub coin_amount: u64,
    #[prost(uint64, tag="3")]
    pub lp_amount: u64,
    #[prost(string, tag="4")]
    pub pc_mint: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub coin_mint: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub lp_mint: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapData {
    #[prost(string, tag="1")]
    pub mint_in: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub mint_out: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub amount_in: u64,
    #[prost(uint64, tag="4")]
    pub amount_out: u64,
}
// @@protoc_insertion_point(module)
