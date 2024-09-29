use std::fmt;
use borsh::BorshDeserialize;
use substreams_solana_utils::pubkey::Pubkey;
use super::super::processor::AuthorizationData;


/// Delegate args can specify Metadata delegates and Token delegates.
#[derive(BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub enum DelegateArgs {
    CollectionV1 {
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
    SaleV1 {
        amount: u64,
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
    TransferV1 {
        amount: u64,
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
    DataV1 {
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
    UtilityV1 {
        amount: u64,
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
    StakingV1 {
        amount: u64,
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
    StandardV1 {
        amount: u64,
    },
    LockedTransferV1 {
        amount: u64,
        #[deprecated(
            since = "1.13.2",
            note = "The locked address is deprecated and will soon be removed."
        )]
        /// locked destination pubkey
        locked_address: Pubkey,
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
    ProgrammableConfigV1 {
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
    AuthorityItemV1 {
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
    DataItemV1 {
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
    CollectionItemV1 {
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
    ProgrammableConfigItemV1 {
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
    PrintDelegateV1 {
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
}

#[derive(BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub enum RevokeArgs {
    CollectionV1,
    SaleV1,
    TransferV1,
    DataV1,
    UtilityV1,
    StakingV1,
    StandardV1,
    LockedTransferV1,
    ProgrammableConfigV1,
    MigrationV1,
    AuthorityItemV1,
    DataItemV1,
    CollectionItemV1,
    ProgrammableConfigItemV1,
    PrintDelegateV1,
}

#[derive(BorshDeserialize, PartialEq, Eq, Debug, Clone, Copy)]
pub enum MetadataDelegateRole {
    AuthorityItem,
    Collection,
    Use,
    Data,
    ProgrammableConfig,
    DataItem,
    CollectionItem,
    ProgrammableConfigItem,
}

impl fmt::Display for MetadataDelegateRole {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            Self::AuthorityItem => "authority_item_delegate".to_string(),
            Self::Collection => "collection_delegate".to_string(),
            Self::Use => "use_delegate".to_string(),
            Self::Data => "data_delegate".to_string(),
            Self::ProgrammableConfig => "programmable_config_delegate".to_string(),
            Self::DataItem => "data_item_delegate".to_string(),
            Self::CollectionItem => "collection_item_delegate".to_string(),
            Self::ProgrammableConfigItem => "prog_config_item_delegate".to_string(),
        };

        write!(f, "{message}")
    }
}

#[derive(BorshDeserialize, PartialEq, Eq, Debug, Clone, Copy)]
pub enum HolderDelegateRole {
    PrintDelegate,
}

impl fmt::Display for HolderDelegateRole {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            Self::PrintDelegate => "print_delegate".to_string(),
        };

        write!(f, "{message}")
    }
}
