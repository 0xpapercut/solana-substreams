use borsh::BorshDeserialize;
use substreams_solana_utils::pubkey::Pubkey;
use super::super::state::{
    AssetData, Collection, CollectionDetails, Data, DataV2, PrintSupply,
    TokenStandard, Uses,
};
use super::super::processor::AuthorizationData;

#[derive(BorshDeserialize, PartialEq, Eq, Debug, Clone)]
/// Args for create call
pub struct CreateMetadataAccountArgsV3 {
    /// Note that unique metadatas are disabled for now.
    pub data: DataV2,
    /// Whether you want your metadata to be updateable in the future.
    pub is_mutable: bool,
    /// If this is a collection parent NFT.
    pub collection_details: Option<CollectionDetails>,
}

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub enum CreateArgs {
    V1 {
        asset_data: AssetData,
        decimals: Option<u8>,
        print_supply: Option<PrintSupply>,
    },
}

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub enum MintArgs {
    V1 {
        amount: u64,
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
}

#[repr(C)]
#[cfg_attr(feature = "serde-feature", derive(Serialize, Deserialize))]
#[derive(BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub enum TransferArgs {
    V1 {
        amount: u64,
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
}

/// Struct representing the values to be updated for an `update` instructions.
///
/// Values that are set to `None` are not changed.  Any value set to `Some(...)` will
/// have its value updated. There are properties that have three valid states, and
/// use a "toggle" type that allows the value to be set, cleared, or remain the same.
#[derive(BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub enum UpdateArgs {
    V1 {
        /// The new update authority.
        new_update_authority: Option<Pubkey>,
        /// The metadata details.
        data: Option<Data>,
        /// Indicates whether the primary sale has happened or not (once set to `true`, it cannot be
        /// changed back).
        primary_sale_happened: Option<bool>,
        // Indicates Whether the data struct is mutable or not (once set to `true`, it cannot be
        /// changed back).
        is_mutable: Option<bool>,
        /// Collection information.
        collection: CollectionToggle,
        /// Additional details of the collection.
        collection_details: CollectionDetailsToggle,
        /// Uses information.
        uses: UsesToggle,
        // Programmable rule set configuration (only applicable to `Programmable` asset types).
        rule_set: RuleSetToggle,
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
    AsUpdateAuthorityV2 {
        /// The new update authority.
        new_update_authority: Option<Pubkey>,
        /// The metadata details.
        data: Option<Data>,
        /// Indicates whether the primary sale has happened or not (once set to `true`, it cannot be
        /// changed back).
        primary_sale_happened: Option<bool>,
        // Indicates Whether the data struct is mutable or not (once set to `true`, it cannot be
        /// changed back).
        is_mutable: Option<bool>,
        /// Collection information.
        collection: CollectionToggle,
        /// Additional details of the collection.
        collection_details: CollectionDetailsToggle,
        /// Uses information.
        uses: UsesToggle,
        // Programmable rule set configuration (only applicable to `Programmable` asset types).
        rule_set: RuleSetToggle,
        /// Token standard.
        token_standard: Option<TokenStandard>,
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
    AsAuthorityItemDelegateV2 {
        /// The new update authority.
        #[deprecated(
            since = "1.13.3",
            note = "A delegate cannot change the update authority. This field will be removed in a future release."
        )]
        new_update_authority: Option<Pubkey>,
        /// Indicates whether the primary sale has happened or not (once set to `true`, it cannot be
        /// changed back).
        primary_sale_happened: Option<bool>,
        // Indicates Whether the data struct is mutable or not (once set to `true`, it cannot be
        /// changed back).
        is_mutable: Option<bool>,
        /// Token standard.
        token_standard: Option<TokenStandard>,
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
    AsCollectionDelegateV2 {
        /// Collection information.
        collection: CollectionToggle,
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
    AsDataDelegateV2 {
        /// The metadata details.
        data: Option<Data>,
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
    AsProgrammableConfigDelegateV2 {
        // Programmable rule set configuration (only applicable to `Programmable` asset types).
        rule_set: RuleSetToggle,
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
    AsDataItemDelegateV2 {
        /// The metadata details.
        data: Option<Data>,
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
    AsCollectionItemDelegateV2 {
        /// Collection information.
        collection: CollectionToggle,
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
    AsProgrammableConfigItemDelegateV2 {
        // Programmable rule set configuration (only applicable to `Programmable` asset types).
        rule_set: RuleSetToggle,
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
}

impl UpdateArgs {
    pub fn default_v1() -> Self {
        Self::V1 {
            new_update_authority: None,
            data: None,
            primary_sale_happened: None,
            is_mutable: None,
            collection: CollectionToggle::default(),
            collection_details: CollectionDetailsToggle::default(),
            uses: UsesToggle::default(),
            rule_set: RuleSetToggle::default(),
            authorization_data: None,
        }
    }

    pub fn default_as_update_authority() -> Self {
        Self::AsUpdateAuthorityV2 {
            new_update_authority: None,
            data: None,
            primary_sale_happened: None,
            is_mutable: None,
            collection: CollectionToggle::default(),
            collection_details: CollectionDetailsToggle::default(),
            uses: UsesToggle::default(),
            rule_set: RuleSetToggle::default(),
            token_standard: None,
            authorization_data: None,
        }
    }

    pub fn default_as_authority_item_delegate() -> Self {
        Self::AsAuthorityItemDelegateV2 {
            new_update_authority: None,
            primary_sale_happened: None,
            is_mutable: None,
            token_standard: None,
            authorization_data: None,
        }
    }

    pub fn default_as_collection_delegate() -> Self {
        Self::AsCollectionDelegateV2 {
            collection: CollectionToggle::default(),
            authorization_data: None,
        }
    }

    pub fn default_as_data_delegate() -> Self {
        Self::AsDataDelegateV2 {
            data: None,
            authorization_data: None,
        }
    }

    pub fn default_as_programmable_config_delegate() -> Self {
        Self::AsProgrammableConfigDelegateV2 {
            rule_set: RuleSetToggle::default(),
            authorization_data: None,
        }
    }

    pub fn default_as_data_item_delegate() -> Self {
        Self::AsDataItemDelegateV2 {
            data: None,
            authorization_data: None,
        }
    }

    pub fn default_as_collection_item_delegate() -> Self {
        Self::AsCollectionItemDelegateV2 {
            collection: CollectionToggle::default(),
            authorization_data: None,
        }
    }

    pub fn default_as_programmable_config_item_delegate() -> Self {
        Self::AsProgrammableConfigItemDelegateV2 {
            rule_set: RuleSetToggle::default(),
            authorization_data: None,
        }
    }
}

#[derive(BorshDeserialize, PartialEq, Eq, Debug, Clone, Default)]
pub enum CollectionToggle {
    #[default]
    None,
    Clear,
    Set(Collection),
}

impl CollectionToggle {
    pub fn is_some(&self) -> bool {
        matches!(self, CollectionToggle::Clear | CollectionToggle::Set(_))
    }

    pub fn is_none(&self) -> bool {
        matches!(self, CollectionToggle::None)
    }

    pub fn is_clear(&self) -> bool {
        matches!(self, CollectionToggle::Clear)
    }

    pub fn is_set(&self) -> bool {
        matches!(self, CollectionToggle::Set(_))
    }

    pub fn to_option(self) -> Option<Collection> {
        match self {
            CollectionToggle::Set(value) => Some(value),
            CollectionToggle::Clear => None,
            CollectionToggle::None => panic!("Tried to convert 'None' value"),
        }
    }
}

#[derive(BorshDeserialize, PartialEq, Eq, Debug, Clone, Default)]
pub enum UsesToggle {
    #[default]
    None,
    Clear,
    Set(Uses),
}

impl UsesToggle {
    pub fn is_some(&self) -> bool {
        matches!(self, UsesToggle::Clear | UsesToggle::Set(_))
    }

    pub fn is_none(&self) -> bool {
        matches!(self, UsesToggle::None)
    }

    pub fn is_clear(&self) -> bool {
        matches!(self, UsesToggle::Clear)
    }

    pub fn is_set(&self) -> bool {
        matches!(self, UsesToggle::Set(_))
    }

    pub fn to_option(self) -> Option<Uses> {
        match self {
            UsesToggle::Set(value) => Some(value),
            UsesToggle::Clear => None,
            UsesToggle::None => panic!("Tried to convert 'None' value"),
        }
    }
}

#[derive(BorshDeserialize, PartialEq, Eq, Debug, Clone, Default)]
pub enum CollectionDetailsToggle {
    #[default]
    None,
    Clear,
    Set(CollectionDetails),
}

impl CollectionDetailsToggle {
    pub fn is_some(&self) -> bool {
        matches!(
            self,
            CollectionDetailsToggle::Clear | CollectionDetailsToggle::Set(_)
        )
    }

    pub fn is_none(&self) -> bool {
        matches!(self, CollectionDetailsToggle::None)
    }

    pub fn is_clear(&self) -> bool {
        matches!(self, CollectionDetailsToggle::Clear)
    }

    pub fn is_set(&self) -> bool {
        matches!(self, CollectionDetailsToggle::Set(_))
    }

    pub fn to_option(self) -> Option<CollectionDetails> {
        match self {
            CollectionDetailsToggle::Set(value) => Some(value),
            CollectionDetailsToggle::Clear => None,
            CollectionDetailsToggle::None => panic!("Tried to convert 'None' value"),
        }
    }
}

#[derive(BorshDeserialize, PartialEq, Eq, Debug, Clone, Default)]
pub enum RuleSetToggle {
    #[default]
    None,
    Clear,
    Set(Pubkey),
}

impl RuleSetToggle {
    pub fn is_some(&self) -> bool {
        matches!(self, RuleSetToggle::Clear | RuleSetToggle::Set(_))
    }

    pub fn is_none(&self) -> bool {
        matches!(self, RuleSetToggle::None)
    }

    pub fn is_clear(&self) -> bool {
        matches!(self, RuleSetToggle::Clear)
    }

    pub fn is_set(&self) -> bool {
        matches!(self, RuleSetToggle::Set(_))
    }

    pub fn to_option(self) -> Option<Pubkey> {
        match self {
            RuleSetToggle::Set(t) => Some(t),
            RuleSetToggle::Clear => None,
            RuleSetToggle::None => panic!("Tried to convert 'None' value"),
        }
    }
}

#[derive(BorshDeserialize, PartialEq, Eq, Debug, Clone)]
/// Args for update call
pub struct UpdateMetadataAccountArgsV2 {
    pub data: Option<DataV2>,
    pub update_authority: Option<Pubkey>,
    pub primary_sale_happened: Option<bool>,
    pub is_mutable: Option<bool>,
}

#[derive(BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub enum PrintArgs {
    V1 { edition: u64 },
    V2 { edition: u64 },
}
