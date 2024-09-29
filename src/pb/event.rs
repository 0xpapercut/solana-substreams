// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Events {
    #[prost(message, repeated, tag="1")]
    pub events: ::prost::alloc::vec::Vec<Event>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    #[prost(string, tag="1")]
    pub signer: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub signature: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub slot: u64,
    #[prost(message, optional, tag="4")]
    pub event: ::core::option::Option<RaydiumEvent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RaydiumEvent {
    #[prost(string, tag="1")]
    pub amm: ::prost::alloc::string::String,
    #[prost(enumeration="EventType", tag="2")]
    pub r#type: i32,
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EventType {
    Initialize = 0,
    Deposit = 1,
    Withdraw = 2,
    Swap = 3,
}
impl EventType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EventType::Initialize => "INITIALIZE",
            EventType::Deposit => "DEPOSIT",
            EventType::Withdraw => "WITHDRAW",
            EventType::Swap => "SWAP",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INITIALIZE" => Some(Self::Initialize),
            "DEPOSIT" => Some(Self::Deposit),
            "WITHDRAW" => Some(Self::Withdraw),
            "SWAP" => Some(Self::Swap),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
