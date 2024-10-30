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
    #[prost(oneof="mpl_token_metadata_event::Event", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58")]
    pub event: ::core::option::Option<mpl_token_metadata_event::Event>,
}
/// Nested message and enum types in `MplTokenMetadataEvent`.
pub mod mpl_token_metadata_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        #[prost(message, tag="1")]
        CreateMetadataAccountV3(super::CreateMetadataAccountV3Event),
        #[prost(message, tag="2")]
        ApproveCollectionAuthority(super::ApproveCollectionAuthorityEvent),
        #[prost(message, tag="3")]
        ApproveUseAuthority(super::ApproveUseAuthorityEvent),
        #[prost(message, tag="4")]
        BubblegumSetCollectionSize(super::BubblegumSetCollectionSizeEvent),
        #[prost(message, tag="5")]
        Burn(super::BurnEvent),
        #[prost(message, tag="6")]
        BurnEditionNft(super::BurnEditionNftEvent),
        #[prost(message, tag="7")]
        BurnNft(super::BurnNftEvent),
        #[prost(message, tag="8")]
        CloseEscrowAccount(super::CloseEscrowAccountEvent),
        #[prost(message, tag="9")]
        ConvertMasterEditionV1ToV2(super::ConvertMasterEditionV1ToV2Event),
        #[prost(message, tag="10")]
        Create(super::CreateEvent),
        #[prost(message, tag="11")]
        CreateEscrowAccount(super::CreateEscrowAccountEvent),
        #[prost(message, tag="12")]
        CreateMasterEdition(super::CreateMasterEditionEvent),
        #[prost(message, tag="13")]
        CreateMasterEditionV3(super::CreateMasterEditionV3Event),
        #[prost(message, tag="14")]
        CreateMetadataAccount(super::CreateMetadataAccountEvent),
        #[prost(message, tag="15")]
        CreateMetadataAccountV2(super::CreateMetadataAccountV2Event),
        #[prost(message, tag="16")]
        Delegate(super::DelegateEvent),
        #[prost(message, tag="17")]
        DeprecatedCreateMasterEdition(super::DeprecatedCreateMasterEditionEvent),
        #[prost(message, tag="18")]
        DeprecatedCreateReservationList(super::DeprecatedCreateReservationListEvent),
        #[prost(message, tag="19")]
        DeprecatedMintNewEditionFromMasterEditionViaPrintingToken(super::DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenEvent),
        #[prost(message, tag="20")]
        DeprecatedMintPrintingTokens(super::DeprecatedMintPrintingTokensEvent),
        #[prost(message, tag="21")]
        DeprecatedMintPrintingTokensViaToken(super::DeprecatedMintPrintingTokensViaTokenEvent),
        #[prost(message, tag="22")]
        DeprecatedSetReservationList(super::DeprecatedSetReservationListEvent),
        #[prost(message, tag="23")]
        FreezeDelegatedAccount(super::FreezeDelegatedAccountEvent),
        #[prost(message, tag="24")]
        Lock(super::LockEvent),
        #[prost(message, tag="25")]
        Migrate(super::MigrateEvent),
        #[prost(message, tag="26")]
        MintNewEditionFromMasterEditionViaToken(super::MintNewEditionFromMasterEditionViaTokenEvent),
        #[prost(message, tag="27")]
        MintNewEditionFromMasterEditionViaVaultProxy(super::MintNewEditionFromMasterEditionViaVaultProxyEvent),
        #[prost(message, tag="28")]
        PuffMetadata(super::PuffMetadataEvent),
        #[prost(message, tag="29")]
        RemoveCreatorVerification(super::RemoveCreatorVerificationEvent),
        #[prost(message, tag="30")]
        Revoke(super::RevokeEvent),
        #[prost(message, tag="31")]
        RevokeCollectionAuthority(super::RevokeCollectionAuthorityEvent),
        #[prost(message, tag="32")]
        RevokeUseAuthority(super::RevokeUseAuthorityEvent),
        #[prost(message, tag="33")]
        SetAndVerifyCollection(super::SetAndVerifyCollectionEvent),
        #[prost(message, tag="34")]
        SetAndVerifySizedCollectionItem(super::SetAndVerifySizedCollectionItemEvent),
        #[prost(message, tag="35")]
        SetTokenStandard(super::SetTokenStandardEvent),
        #[prost(message, tag="36")]
        SignMetadata(super::SignMetadataEvent),
        #[prost(message, tag="37")]
        ThawDelegatedAccount(super::ThawDelegatedAccountEvent),
        #[prost(message, tag="38")]
        Transfer(super::TransferEvent),
        #[prost(message, tag="39")]
        TransferOutOfEscrow(super::TransferOutOfEscrowEvent),
        #[prost(message, tag="40")]
        Unlock(super::UnlockEvent),
        #[prost(message, tag="41")]
        Unverify(super::UnverifyEvent),
        #[prost(message, tag="42")]
        UnverifyCollection(super::UnverifyCollectionEvent),
        #[prost(message, tag="43")]
        UnverifySizedCollectionItem(super::UnverifySizedCollectionItemEvent),
        #[prost(message, tag="44")]
        Update(super::UpdateEvent),
        #[prost(message, tag="45")]
        UpdateMetadataAccount(super::UpdateMetadataAccountEvent),
        #[prost(message, tag="46")]
        UpdateMetadataAccountV2(super::UpdateMetadataAccountV2Event),
        #[prost(message, tag="47")]
        UpdatePrimarySaleHappenedViaToken(super::UpdatePrimarySaleHappenedViaTokenEvent),
        #[prost(message, tag="48")]
        Utilize(super::UtilizeEvent),
        #[prost(message, tag="49")]
        Print(super::PrintEvent),
        #[prost(message, tag="50")]
        Verify(super::VerifyEvent),
        #[prost(message, tag="51")]
        Mint(super::MintEvent),
        #[prost(message, tag="52")]
        SetCollectionSize(super::SetCollectionSizeEvent),
        #[prost(message, tag="53")]
        Collect(super::CollectEvent),
        #[prost(message, tag="54")]
        Use(super::UseEvent),
        #[prost(message, tag="55")]
        VerifySizedCollectionItem(super::VerifySizedCollectionItemEvent),
        #[prost(message, tag="56")]
        VerifyCollection(super::VerifyCollectionEvent),
        #[prost(message, tag="57")]
        Resize(super::ResizeEvent),
        #[prost(message, tag="58")]
        CloseAccounts(super::CloseAccountsEvent),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApproveCollectionAuthorityEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApproveUseAuthorityEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BubblegumSetCollectionSizeEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BurnEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BurnEditionNftEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BurnNftEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloseEscrowAccountEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConvertMasterEditionV1ToV2Event {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEscrowAccountEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateMasterEditionEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateMasterEditionV3Event {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateMetadataAccountEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateMetadataAccountV2Event {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelegateEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeprecatedCreateMasterEditionEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeprecatedCreateReservationListEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeprecatedMintPrintingTokensEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeprecatedMintPrintingTokensViaTokenEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeprecatedSetReservationListEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FreezeDelegatedAccountEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MigrateEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MintNewEditionFromMasterEditionViaTokenEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MintNewEditionFromMasterEditionViaVaultProxyEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PuffMetadataEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveCreatorVerificationEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RevokeEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RevokeCollectionAuthorityEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RevokeUseAuthorityEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAndVerifyCollectionEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAndVerifySizedCollectionItemEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetTokenStandardEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignMetadataEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThawDelegatedAccountEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferOutOfEscrowEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnlockEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnverifyEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnverifyCollectionEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnverifySizedCollectionItemEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateMetadataAccountEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePrimarySaleHappenedViaTokenEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UtilizeEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrintEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MintEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetCollectionSizeEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UseEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifySizedCollectionItemEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyCollectionEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResizeEvent {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloseAccountsEvent {
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
pub struct CreateEvent {
    #[prost(string, tag="1")]
    pub metadata: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub mint: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub mint_authority: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub update_authority: ::prost::alloc::string::String,
    #[prost(message, optional, tag="5")]
    pub asset_data: ::core::option::Option<AssetData>,
    #[prost(uint32, optional, tag="6")]
    pub decimals: ::core::option::Option<u32>,
    #[prost(message, optional, tag="7")]
    pub print_supply: ::core::option::Option<PrintSupply>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateEvent {
    #[prost(string, tag="1")]
    pub authority: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub mint: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub metadata: ::prost::alloc::string::String,
    #[prost(oneof="update_event::UpdateArgs", tags="4, 5, 6, 7, 8, 9, 10, 11, 12")]
    pub update_args: ::core::option::Option<update_event::UpdateArgs>,
}
/// Nested message and enum types in `UpdateEvent`.
pub mod update_event {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum UpdateArgs {
        #[prost(message, tag="4")]
        V1(super::UpdateArgsV1),
        #[prost(message, tag="5")]
        AsUpdateAuthorityV2(super::UpdateArgsAsUpdateAuthorityV2),
        #[prost(message, tag="6")]
        AsAuthorityItemDelegateV2(super::UpdateArgsAsAuthorityItemDelegateV2),
        #[prost(message, tag="7")]
        AsCollectionDelegateV2(super::UpdateArgsAsCollectionDelegateV2),
        #[prost(message, tag="8")]
        AsDataDelegateV2(super::UpdateArgsAsDataDelegateV2),
        #[prost(message, tag="9")]
        AsProgrammableConfigDelegateV2(super::UpdateArgsAsProgrammableConfigDelegateV2),
        #[prost(message, tag="10")]
        AsDataItemDelegateV2(super::UpdateArgsAsDataItemDelegateV2),
        #[prost(message, tag="11")]
        AsCollectionItemDelegateV2(super::UpdateArgsAsCollectionItemDelegateV2),
        #[prost(message, tag="12")]
        AsProgrammableConfigItemDelegateV2(super::UpdateArgsAsProgrammableConfigItemDelegateV2),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateMetadataAccountV2Event {
    #[prost(message, optional, tag="1")]
    pub data: ::core::option::Option<DataV2>,
    #[prost(string, optional, tag="2")]
    pub update_authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag="3")]
    pub primary_sale_happened: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="4")]
    pub is_mutable: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateArgsV1 {
    #[prost(string, optional, tag="1")]
    pub new_update_authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="2")]
    pub data: ::core::option::Option<Data>,
    #[prost(bool, optional, tag="3")]
    pub primary_sale_happened: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="4")]
    pub is_mutable: ::core::option::Option<bool>,
    #[prost(message, optional, tag="5")]
    pub collection: ::core::option::Option<CollectionToggle>,
    #[prost(message, optional, tag="6")]
    pub collection_details: ::core::option::Option<CollectionDetailsToggle>,
    #[prost(message, optional, tag="7")]
    pub uses: ::core::option::Option<UsesToggle>,
    #[prost(message, optional, tag="8")]
    pub rule_set: ::core::option::Option<RuleSetToggle>,
    #[prost(message, optional, tag="9")]
    pub authorization_data: ::core::option::Option<AuthorizationData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateArgsAsUpdateAuthorityV2 {
    #[prost(string, optional, tag="1")]
    pub new_update_authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="2")]
    pub data: ::core::option::Option<Data>,
    #[prost(bool, optional, tag="3")]
    pub primary_sale_happened: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="4")]
    pub is_mutable: ::core::option::Option<bool>,
    #[prost(message, optional, tag="5")]
    pub collection: ::core::option::Option<CollectionToggle>,
    #[prost(message, optional, tag="6")]
    pub collection_details: ::core::option::Option<CollectionDetailsToggle>,
    #[prost(message, optional, tag="7")]
    pub uses: ::core::option::Option<UsesToggle>,
    #[prost(message, optional, tag="8")]
    pub rule_set: ::core::option::Option<RuleSetToggle>,
    #[prost(message, optional, tag="9")]
    pub token_standard: ::core::option::Option<TokenStandard>,
    #[prost(message, optional, tag="10")]
    pub authorization_data: ::core::option::Option<AuthorizationData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateArgsAsAuthorityItemDelegateV2 {
    #[prost(string, optional, tag="1")]
    pub new_update_authority: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag="2")]
    pub primary_sale_happened: ::core::option::Option<bool>,
    #[prost(bool, optional, tag="3")]
    pub is_mutable: ::core::option::Option<bool>,
    #[prost(message, optional, tag="4")]
    pub token_standard: ::core::option::Option<TokenStandard>,
    #[prost(message, optional, tag="5")]
    pub authorization_data: ::core::option::Option<AuthorizationData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateArgsAsCollectionDelegateV2 {
    #[prost(message, optional, tag="1")]
    pub collection: ::core::option::Option<CollectionToggle>,
    #[prost(message, optional, tag="2")]
    pub authorization_data: ::core::option::Option<AuthorizationData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateArgsAsDataDelegateV2 {
    #[prost(message, optional, tag="1")]
    pub data: ::core::option::Option<Data>,
    #[prost(message, optional, tag="2")]
    pub authorization_data: ::core::option::Option<AuthorizationData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateArgsAsProgrammableConfigDelegateV2 {
    #[prost(message, optional, tag="1")]
    pub rule_set: ::core::option::Option<RuleSetToggle>,
    #[prost(message, optional, tag="2")]
    pub authorization_data: ::core::option::Option<AuthorizationData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateArgsAsDataItemDelegateV2 {
    #[prost(message, optional, tag="1")]
    pub data: ::core::option::Option<Data>,
    #[prost(message, optional, tag="2")]
    pub authorization_data: ::core::option::Option<AuthorizationData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateArgsAsCollectionItemDelegateV2 {
    #[prost(message, optional, tag="1")]
    pub collection: ::core::option::Option<CollectionToggle>,
    #[prost(message, optional, tag="2")]
    pub authorization_data: ::core::option::Option<AuthorizationData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateArgsAsProgrammableConfigItemDelegateV2 {
    #[prost(message, optional, tag="1")]
    pub rule_set: ::core::option::Option<RuleSetToggle>,
    #[prost(message, optional, tag="2")]
    pub authorization_data: ::core::option::Option<AuthorizationData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Data {
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
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrintSupply {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetData {
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
    #[prost(bool, tag="6")]
    pub primary_sale_happened: bool,
    #[prost(bool, tag="7")]
    pub is_mutable: bool,
    #[prost(message, optional, tag="8")]
    pub token_standard: ::core::option::Option<TokenStandard>,
    #[prost(message, optional, tag="9")]
    pub collection: ::core::option::Option<Collection>,
    #[prost(message, optional, tag="10")]
    pub uses: ::core::option::Option<Uses>,
    #[prost(message, optional, tag="11")]
    pub collection_details: ::core::option::Option<CollectionDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuleSetToggle {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectionToggle {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizationData {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectionDetailsToggle {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsesToggle {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TokenStandard {
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
            UseMethod::Null => "USE_METHOD_NULL",
            UseMethod::Burn => "BURN",
            UseMethod::Multiple => "MULTIPLE",
            UseMethod::Single => "SINGLE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "USE_METHOD_NULL" => Some(Self::Null),
            "BURN" => Some(Self::Burn),
            "MULTIPLE" => Some(Self::Multiple),
            "SINGLE" => Some(Self::Single),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Toggle {
    Null = 0,
    None = 1,
    Clear = 2,
    Set = 3,
}
impl Toggle {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Toggle::Null => "TOGGLE_NULL",
            Toggle::None => "NONE",
            Toggle::Clear => "CLEAR",
            Toggle::Set => "SET",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TOGGLE_NULL" => Some(Self::Null),
            "NONE" => Some(Self::None),
            "CLEAR" => Some(Self::Clear),
            "SET" => Some(Self::Set),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
