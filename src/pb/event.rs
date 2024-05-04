// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionsWithEvents {
    #[prost(message, repeated, tag="1")]
    pub transactions: ::prost::alloc::vec::Vec<TransactionsWithEvents>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionWithEvents {
    #[prost(string, repeated, tag="1")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint64, tag="2")]
    pub slot: u64,
    #[prost(string, tag="3")]
    pub signature: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="4")]
    pub events: ::prost::alloc::vec::Vec<Event>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    #[prost(enumeration="EventType", tag="1")]
    pub r#type: i32,
    #[prost(oneof="event::Data", tags="2")]
    pub data: ::core::option::Option<event::Data>,
}
/// Nested message and enum types in `Event`.
pub mod event {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        #[prost(message, tag="2")]
        Transfer(super::TransferData),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferData {
    #[prost(message, optional, tag="1")]
    pub source: ::core::option::Option<TokenAccount>,
    #[prost(message, optional, tag="2")]
    pub destination: ::core::option::Option<TokenAccount>,
    #[prost(uint64, tag="3")]
    pub amount: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenAccount {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub owner: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EventType {
    Transfer = 0,
}
impl EventType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EventType::Transfer => "TRANSFER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TRANSFER" => Some(Self::Transfer),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
