// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MplTokenMetadataBlockEvents {
    #[prost(message, repeated, tag="1")]
    pub transactions: ::prost::alloc::vec::Vec<MplTokenMetadataTransactionEvents>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MplTokenMetadataTransactionEvents {
    #[prost(string, tag="1")]
    pub signature: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub events: ::prost::alloc::vec::Vec<MplTokenMetadataEvent>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MplTokenMetadataEvent {
    #[prost(oneof="mpl_token_metadata_event::Event", tags="1")]
    pub event: ::core::option::Option<mpl_token_metadata_event::Event>,
}
/// Nested message and enum types in `MplTokenMetadataEvent`.
pub mod mpl_token_metadata_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        #[prost(message, tag="1")]
        CreateMetadataAccountV3(super::CreateMetadataAccountV3Event),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateMetadataAccountV3Event {
    #[prost(string, tag="1")]
    pub metadata: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub mint: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub update_authority: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub data: ::core::option::Option<DataV2>,
    #[prost(bool, tag="5")]
    pub is_mutable: bool,
    #[prost(message, optional, tag="6")]
    pub collection_details: ::core::option::Option<CollectionDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataV2 {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub uri: ::prost::alloc::string::String,
    #[prost(uint32, tag="4")]
    pub seller_fee_basis_points: u32,
    #[prost(message, repeated, tag="5")]
    pub creators: ::prost::alloc::vec::Vec<Creator>,
    #[prost(message, optional, tag="6")]
    pub collection: ::core::option::Option<Collection>,
    #[prost(message, optional, tag="7")]
    pub uses: ::core::option::Option<Uses>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Uses {
    #[prost(enumeration="UseMethod", tag="1")]
    pub use_method: i32,
    #[prost(uint64, tag="2")]
    pub remaining: u64,
    #[prost(uint64, tag="3")]
    pub total: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Collection {
    #[prost(bool, tag="1")]
    pub verified: bool,
    #[prost(string, tag="2")]
    pub key: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectionDetails {
    #[prost(oneof="collection_details::Version", tags="1, 2")]
    pub version: ::core::option::Option<collection_details::Version>,
}
/// Nested message and enum types in `CollectionDetails`.
pub mod collection_details {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Version {
        #[prost(message, tag="1")]
        V1(super::CollectionDetailsV1),
        #[prost(message, tag="2")]
        V2(super::CollectionDetailsV2),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectionDetailsV1 {
    #[prost(uint64, tag="1")]
    pub size: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectionDetailsV2 {
    #[prost(uint64, repeated, tag="1")]
    pub padding: ::prost::alloc::vec::Vec<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Creator {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub verified: bool,
    #[prost(uint32, tag="3")]
    pub share: u32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UseMethod {
    Null = 0,
    Burn = 1,
    Multiple = 2,
    Single = 3,
}
impl UseMethod {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            UseMethod::Null => "NULL",
            UseMethod::Burn => "BURN",
            UseMethod::Multiple => "MULTIPLE",
            UseMethod::Single => "SINGLE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NULL" => Some(Self::Null),
            "BURN" => Some(Self::Burn),
            "MULTIPLE" => Some(Self::Multiple),
            "SINGLE" => Some(Self::Single),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
