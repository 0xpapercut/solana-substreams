use super::*;

pub const MAX_NAME_LENGTH: usize = 32;

pub const MAX_SYMBOL_LENGTH: usize = 10;

pub const MAX_URI_LENGTH: usize = 200;

pub const MAX_METADATA_LEN: usize = 1 // key
+ 32             // update auth pubkey
+ 32             // mint pubkey
+ MAX_DATA_SIZE
+ 1              // primary sale
+ 1              // mutable
+ 9              // nonce (pretty sure this only needs to be 2)
+ 2              // token standard
+ 34             // collection
+ 18             // uses
+ 10             // collection details
+ 33             // programmable config
+ 75; // Padding

pub const MAX_DATA_SIZE: usize = 4
    + MAX_NAME_LENGTH
    + 4
    + MAX_SYMBOL_LENGTH
    + 4
    + MAX_URI_LENGTH
    + 2
    + 1
    + 4
    + MAX_CREATOR_LIMIT * MAX_CREATOR_LEN;

// The last byte of the account contains the fee flag, indicating
// if the account has fees available for retrieval.
pub const METADATA_FEE_FLAG_INDEX: usize = MAX_METADATA_LEN - 1;

#[macro_export]
macro_rules! metadata_seeds {
    ($mint:expr) => {{
        let path = vec!["metadata".as_bytes(), $crate::ID.as_ref(), $mint.as_ref()];
        let (_, bump) = Pubkey::find_program_address(&path, &$crate::ID);
        &[
            "metadata".as_bytes(),
            $crate::ID.as_ref(),
            $mint.as_ref(),
            &[bump],
        ]
    }};
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Metadata {
    /// Account discriminator.
    pub key: Key,
    /// Address of the update authority.
    #[cfg_attr(feature = "serde-feature", serde(with = "As::<DisplayFromStr>"))]
    pub update_authority: Pubkey,
    /// Address of the mint.
    #[cfg_attr(feature = "serde-feature", serde(with = "As::<DisplayFromStr>"))]
    pub mint: Pubkey,
    /// Asset data.
    pub data: Data,
    // Immutable, once flipped, all sales of this metadata are considered secondary.
    pub primary_sale_happened: bool,
    // Whether or not the data struct is mutable, default is not
    pub is_mutable: bool,
    /// nonce for easy calculation of editions, if present
    pub edition_nonce: Option<u8>,
    /// Since we cannot easily change Metadata, we add the new DataV2 fields here at the end.
    pub token_standard: Option<TokenStandard>,
    /// Collection
    pub collection: Option<Collection>,
    /// Uses
    pub uses: Option<Uses>,
    /// Collection Details
    pub collection_details: Option<CollectionDetails>,
    /// Programmable Config
    pub programmable_config: Option<ProgrammableConfig>,
}

impl Metadata {
    pub fn into_asset_data(self) -> AssetData {
        let mut asset_data = AssetData::new(
            self.token_standard.unwrap_or(TokenStandard::NonFungible),
            self.data.name,
            self.data.symbol,
            self.data.uri,
        );
        asset_data.seller_fee_basis_points = self.data.seller_fee_basis_points;
        asset_data.creators = self.data.creators;
        asset_data.primary_sale_happened = self.primary_sale_happened;
        asset_data.is_mutable = self.is_mutable;
        asset_data.collection = self.collection;
        asset_data.uses = self.uses;
        asset_data.collection_details = self.collection_details;
        asset_data.rule_set =
            if let Some(ProgrammableConfig::V1 { rule_set }) = self.programmable_config {
                rule_set
            } else {
                None
            };

        asset_data
    }
}

impl Default for Metadata {
    fn default() -> Self {
        Metadata {
            key: Key::MetadataV1,
            update_authority: Pubkey::default(),
            mint: Pubkey::default(),
            data: Data::default(),
            primary_sale_happened: false,
            is_mutable: false,
            edition_nonce: None,
            token_standard: None,
            collection: None,
            uses: None,
            collection_details: None,
            programmable_config: None,
        }
    }
}

// impl TokenMetadataAccount for Metadata {
//     fn key() -> Key {
//         Key::MetadataV1
//     }

//     fn size() -> usize {
//         MAX_METADATA_LEN
//     }
// }

// // We have a custom implementation of BorshDeserialize for Metadata because of corrupted metadata issues
// // caused by resizing of the Creators array. We use a custom `meta_deser_unchecked` function
// // that has fallback values for corrupted fields.
// impl borsh::de::BorshDeserialize for Metadata {
//     fn deserialize(buf: &mut &[u8]) -> ::core::result::Result<Self, BorshError> {
//         let md = meta_deser_unchecked(buf)?;
//         Ok(md)
//     }
//     fn deserialize_reader<R: std::io::prelude::Read>(reader: &mut R) -> std::io::Result<Self> {
//         let md = meta_deser_unchecked_reader(reader)?;
//         Ok(md)
//     }
// }

#[derive(BorshDeserialize, PartialEq, Eq, Debug, Clone)]
/// Represents the print supply of a non-fungible asset.
pub enum PrintSupply {
    /// The asset does not have any prints.
    Zero,
    /// The asset has a limited amount of prints.
    Limited(u64),
    /// The asset has an unlimited amount of prints.
    Unlimited,
}

impl PrintSupply {
    /// Converts the print supply to an option.
    pub fn to_option(&self) -> Option<u64> {
        match self {
            PrintSupply::Zero => Some(0),
            PrintSupply::Limited(supply) => Some(*supply),
            PrintSupply::Unlimited => None,
        }
    }
}

/// Configuration for programmable assets.
#[derive(BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub enum ProgrammableConfig {
    V1 {
        /// Programmable authorization rules.
        #[cfg_attr(
            feature = "serde-feature",
            serde(
                deserialize_with = "deser_option_pubkey",
                serialize_with = "ser_option_pubkey"
            )
        )]
        rule_set: Option<Pubkey>,
    },
}
