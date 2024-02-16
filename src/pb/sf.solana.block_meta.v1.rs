// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockMeta {
    #[prost(string, tag="1")]
    pub hash: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub parent_hash: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub slot: u64,
    #[prost(uint64, tag="4")]
    pub parent_slot: u64,
    #[prost(uint64, tag="5")]
    pub transaction_count: u64,
    #[prost(uint64, optional, tag="6")]
    pub block_height: ::core::option::Option<u64>,
}
// @@protoc_insertion_point(module)
