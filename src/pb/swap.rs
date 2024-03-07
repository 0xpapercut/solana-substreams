// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Swaps {
    #[prost(message, repeated, tag="1")]
    pub swaps: ::prost::alloc::vec::Vec<Swap>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Swap {
    #[prost(string, tag="1")]
    pub signer: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub token_in: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub token_out: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub amount_in: u64,
    #[prost(uint64, tag="5")]
    pub amount_out: u64,
    #[prost(string, tag="6")]
    pub signature: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub amm: ::prost::alloc::string::String,
    #[prost(uint64, tag="8")]
    pub slot: u64,
}
// @@protoc_insertion_point(module)
