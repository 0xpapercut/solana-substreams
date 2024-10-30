#![allow(deprecated)]

use borsh::BorshDeserialize;
use substreams::errors::Error;
use substreams_solana::pb::sf::solana::r#type::v1::ConfirmedTransaction;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

use substreams_solana_utils as utils;
use utils::instruction::{get_structured_instructions, StructuredInstruction, StructuredInstructions};
use utils::transaction::{get_context, TransactionContext};

pub mod mpl_token_metadata;
use mpl_token_metadata::constants::MPL_TOKEN_METADATA_PROGRAM_ID;
use mpl_token_metadata::instruction::MetadataInstruction;

pub mod pb;
use pb::mpl_token_metadata::*;
use pb::mpl_token_metadata::mpl_token_metadata_event::Event;

#[substreams::handlers::map]
fn mpl_token_metadata_events(block: Block) -> Result<MplTokenMetadataBlockEvents, Error> {
    let transactions = parse_block(&block);
    Ok(MplTokenMetadataBlockEvents { transactions })
}

pub fn parse_block(block: &Block) -> Vec<MplTokenMetadataTransactionEvents> {
    let mut block_events: Vec<MplTokenMetadataTransactionEvents> = Vec::new();

    for transaction in block.transactions() {
        if let Ok(events) = parse_transaction(transaction) {
            if !events.is_empty() {
                block_events.push(MplTokenMetadataTransactionEvents {
                    signature: utils::transaction::get_signature(&transaction),
                    events,
                });
            }
        }
    }
    block_events
}

pub fn parse_transaction(transaction: &ConfirmedTransaction) -> Result<Vec<MplTokenMetadataEvent>, String> {
    let mut events: Vec<MplTokenMetadataEvent> = Vec::new();

    let context = get_context(transaction).unwrap();
    let instructions = get_structured_instructions(transaction).unwrap();

    for instruction in instructions.flattened().iter() {
        if instruction.program_id() != MPL_TOKEN_METADATA_PROGRAM_ID {
            continue;
        }
        match parse_instruction(instruction, &context) {
            Ok(event) => events.push(MplTokenMetadataEvent { event } ),
            _ => (),
        }
    }
    Ok(events)
}

pub fn parse_instruction(
    instruction: &StructuredInstruction,
    context: &TransactionContext
) -> Result<Option<Event>, String> {
    if instruction.program_id() != MPL_TOKEN_METADATA_PROGRAM_ID {
        return Err("Not a Metaplex Token Metadata instruction.".into());
    }
    let unpacked = MetadataInstruction::try_from_slice(instruction.data()).map_err(|_| "Failed to parse MetadataInstruction.")?;
    match unpacked {
        MetadataInstruction::ApproveCollectionAuthority => {
            Ok(Some(Event::ApproveCollectionAuthority(ApproveCollectionAuthorityEvent {})))
        },
        MetadataInstruction::ApproveUseAuthority(_) => {
            Ok(Some(Event::ApproveUseAuthority(ApproveUseAuthorityEvent {})))
        },
        MetadataInstruction::BubblegumSetCollectionSize(_) => {
            Ok(Some(Event::BubblegumSetCollectionSize(BubblegumSetCollectionSizeEvent {})))
        },
        MetadataInstruction::Burn(_) => {
            Ok(Some(Event::Burn(BurnEvent {})))
        },
        MetadataInstruction::BurnEditionNft => {
            Ok(Some(Event::BurnEditionNft(BurnEditionNftEvent {})))
        },
        MetadataInstruction::BurnNft => {
            Ok(Some(Event::BurnNft(BurnNftEvent {})))
        },
        MetadataInstruction::CloseEscrowAccount => {
            Ok(Some(Event::CloseEscrowAccount(CloseEscrowAccountEvent {})))
        },
        MetadataInstruction::ConvertMasterEditionV1ToV2 => {
            Ok(Some(Event::ConvertMasterEditionV1ToV2(ConvertMasterEditionV1ToV2Event {})))
        },
        MetadataInstruction::Create(_) => {
            // _parse_create_instruction(instruction, context, create).map(|x| Some(Event::Create(x)))
            Ok(Some(Event::Create(CreateEvent {})))
        },
        MetadataInstruction::CreateEscrowAccount => {
            Ok(Some(Event::CreateEscrowAccount(CreateEscrowAccountEvent {})))
        },
        MetadataInstruction::CreateMasterEdition => {
            Ok(Some(Event::CreateMasterEdition(CreateMasterEditionEvent {})))
        },
        MetadataInstruction::CreateMasterEditionV3(_) => {
            Ok(Some(Event::CreateMasterEditionV3(CreateMasterEditionV3Event {})))
        },
        MetadataInstruction::CreateMetadataAccount => {
            Ok(Some(Event::CreateMetadataAccount(CreateMetadataAccountEvent {})))
        },
        MetadataInstruction::CreateMetadataAccountV2 => {
            Ok(Some(Event::CreateMetadataAccountV2(CreateMetadataAccountV2Event {})))
        },
        MetadataInstruction::CreateMetadataAccountV3(create_metadata_account_v3) => {
            _parse_create_metadata_account_v3_instruction(instruction, context, create_metadata_account_v3).map(|x| Some(Event::CreateMetadataAccountV3(x)))
        },
        MetadataInstruction::Delegate(_) => {
            Ok(Some(Event::Delegate(DelegateEvent {})))
        },
        MetadataInstruction::DeprecatedCreateMasterEdition => {
            Ok(Some(Event::DeprecatedCreateMasterEdition(DeprecatedCreateMasterEditionEvent {})))
        },
        MetadataInstruction::DeprecatedCreateReservationList => {
            Ok(Some(Event::DeprecatedCreateReservationList(DeprecatedCreateReservationListEvent {})))
        },
        MetadataInstruction::DeprecatedMintNewEditionFromMasterEditionViaPrintingToken => {
            Ok(Some(Event::DeprecatedMintNewEditionFromMasterEditionViaPrintingToken(DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenEvent {})))
        },
        MetadataInstruction::DeprecatedMintPrintingTokens => {
            Ok(Some(Event::DeprecatedMintPrintingTokens(DeprecatedMintPrintingTokensEvent {})))
        },
        MetadataInstruction::DeprecatedMintPrintingTokensViaToken => {
            Ok(Some(Event::DeprecatedMintPrintingTokensViaToken(DeprecatedMintPrintingTokensViaTokenEvent {})))
        },
        MetadataInstruction::DeprecatedSetReservationList => {
            Ok(Some(Event::DeprecatedSetReservationList(DeprecatedSetReservationListEvent {})))
        },
        MetadataInstruction::FreezeDelegatedAccount => {
            Ok(Some(Event::FreezeDelegatedAccount(FreezeDelegatedAccountEvent {})))
        },
        MetadataInstruction::Lock(_) => {
            Ok(Some(Event::Lock(LockEvent {})))
        },
        MetadataInstruction::Migrate => {
            Ok(Some(Event::Migrate(MigrateEvent {})))
        },
        MetadataInstruction::MintNewEditionFromMasterEditionViaToken(_) => {
            Ok(Some(Event::MintNewEditionFromMasterEditionViaToken(MintNewEditionFromMasterEditionViaTokenEvent {})))
        },
        MetadataInstruction::MintNewEditionFromMasterEditionViaVaultProxy(_) => {
            Ok(Some(Event::MintNewEditionFromMasterEditionViaVaultProxy(MintNewEditionFromMasterEditionViaVaultProxyEvent {})))
        },
        MetadataInstruction::PuffMetadata => {
            Ok(Some(Event::PuffMetadata(PuffMetadataEvent {})))
        },
        MetadataInstruction::RemoveCreatorVerification => {
            Ok(Some(Event::RemoveCreatorVerification(RemoveCreatorVerificationEvent {})))
        },
        MetadataInstruction::Revoke(_) => {
            Ok(Some(Event::Revoke(RevokeEvent {})))
        },
        MetadataInstruction::RevokeCollectionAuthority => {
            Ok(Some(Event::RevokeCollectionAuthority(RevokeCollectionAuthorityEvent {})))
        },
        MetadataInstruction::RevokeUseAuthority => {
            Ok(Some(Event::RevokeUseAuthority(RevokeUseAuthorityEvent {})))
        },
        MetadataInstruction::SetAndVerifyCollection => {
            Ok(Some(Event::SetAndVerifyCollection(SetAndVerifyCollectionEvent {})))
        },
        MetadataInstruction::SetAndVerifySizedCollectionItem => {
            Ok(Some(Event::SetAndVerifySizedCollectionItem(SetAndVerifySizedCollectionItemEvent {})))
        },
        MetadataInstruction::SetTokenStandard => {
            Ok(Some(Event::SetTokenStandard(SetTokenStandardEvent {})))
        },
        MetadataInstruction::SignMetadata => {
            Ok(Some(Event::SignMetadata(SignMetadataEvent {})))
        },
        MetadataInstruction::ThawDelegatedAccount => {
            Ok(Some(Event::ThawDelegatedAccount(ThawDelegatedAccountEvent {})))
        },
        MetadataInstruction::Transfer(_) => {
            Ok(Some(Event::Transfer(TransferEvent {})))
        },
        MetadataInstruction::TransferOutOfEscrow(_) => {
            Ok(Some(Event::TransferOutOfEscrow(TransferOutOfEscrowEvent {})))
        },
        MetadataInstruction::Unlock(_) => {
            Ok(Some(Event::Unlock(UnlockEvent {})))
        },
        MetadataInstruction::Unverify(_) => {
            Ok(Some(Event::Unverify(UnverifyEvent {})))
        },
        MetadataInstruction::UnverifyCollection => {
            Ok(Some(Event::UnverifyCollection(UnverifyCollectionEvent {})))
        },
        MetadataInstruction::UnverifySizedCollectionItem => {
            Ok(Some(Event::UnverifySizedCollectionItem(UnverifySizedCollectionItemEvent {})))
        },
        MetadataInstruction::Update(_) => {
            // _parse_update_instruction(instruction, context, update).map(|x| Some(Event::Update(x)))
            Ok(Some(Event::Update(UpdateEvent {})))
        },
        MetadataInstruction::UpdateMetadataAccount => {
            Ok(Some(Event::UpdateMetadataAccount(UpdateMetadataAccountEvent {})))
        },
        MetadataInstruction::UpdateMetadataAccountV2(_) => {
            // _parse_update_metadata_account_v2_instruction(instruction, context, update_metadata_account_v2).map(|x| Some(Event::UpdateMetadataAccountV2(x)))
            Ok(Some(Event::UpdateMetadataAccountV2(UpdateMetadataAccountV2Event {})))
        },
        MetadataInstruction::UpdatePrimarySaleHappenedViaToken => {
            Ok(Some(Event::UpdatePrimarySaleHappenedViaToken(UpdatePrimarySaleHappenedViaTokenEvent {})))
        },
        MetadataInstruction::Utilize(_) => {
            Ok(Some(Event::Utilize(UtilizeEvent {})))
        },
        MetadataInstruction::Print(_) => {
            Ok(Some(Event::Print(PrintEvent {})))
        },
        MetadataInstruction::Verify(_) => {
            Ok(Some(Event::Verify(VerifyEvent {})))
        },
        MetadataInstruction::Mint(_) => {
            Ok(Some(Event::Mint(MintEvent {})))
        },
        MetadataInstruction::SetCollectionSize(_) => {
            Ok(Some(Event::SetCollectionSize(SetCollectionSizeEvent {})))
        },
        MetadataInstruction::Collect => {
            Ok(Some(Event::Collect(CollectEvent {})))
        },
        MetadataInstruction::Use(_) => {
            Ok(Some(Event::Use(UseEvent {})))
        },
        MetadataInstruction::VerifySizedCollectionItem => {
            Ok(Some(Event::VerifySizedCollectionItem(VerifySizedCollectionItemEvent {})))
        },
        MetadataInstruction::VerifyCollection => {
            Ok(Some(Event::VerifyCollection(VerifyCollectionEvent {})))
        },
        MetadataInstruction::Resize => {
            Ok(Some(Event::Resize(ResizeEvent {})))
        },
        MetadataInstruction::CloseAccounts => {
            Ok(Some(Event::CloseAccounts(CloseAccountsEvent {})))
        }
    }
}

fn _parse_create_metadata_account_v3_instruction<'a>(
    instruction: &StructuredInstruction<'a>,
    _context: &TransactionContext,
    create_metadata_account_v3: mpl_token_metadata::instruction::CreateMetadataAccountArgsV3,
) -> Result<CreateMetadataAccountV3Event, String> {
    let metadata = instruction.accounts()[0].to_string();
    let mint = instruction.accounts()[1].to_string();
    let update_authority = instruction.accounts()[4].to_string();
    let data = Some(create_metadata_account_v3.data.into());
    let is_mutable = create_metadata_account_v3.is_mutable;
    let collection_details = create_metadata_account_v3.collection_details.map(|x| x.into());

    Ok(CreateMetadataAccountV3Event {
        metadata,
        mint,
        update_authority,
        data,
        is_mutable,
        collection_details,
    })
}

// fn _parse_create_instruction<'a>(
//     instruction: &StructuredInstruction<'a>,
//     _context: &TransactionContext,
//     create: mpl_token_metadata::instruction::CreateArgs,
// ) -> Result<CreateEvent, String> {
//     unimplemented!()
// }

// fn _parse_update_instruction<'a>(
//     instruction: &StructuredInstruction,
//     context: &TransactionContext,
//     update: mpl_token_metadata::instruction::UpdateArgs,
// ) -> Result<UpdateEvent, String> {
//     let metadata = instruction.accounts()[0].to_string();
//     let delta = (instruction.accounts()[6] == SYSTEM_PROGRAM_ID) as usize;
//     let mint = instruction.accounts()[1 + delta].to_string();
//     let authority = instruction.accounts()[2 + delta].to_string();

//     let update_args = match update {
//         mpl_token_metadata::instruction::UpdateArgs::V1 { new_update_authority, data, primary_sale_happened, is_mutable, collection, collection_details, uses, rule_set, authorization_data } => {
//             Some(update_event::UpdateArgs::V1(UpdateArgsV1 {
//                 new_update_authority: new_update_authority.map(|x| x.to_string()),
//                 data: data.map(|x| x.into()),
//                 primary_sale_happened,
//                 is_mutable,
//                 collection: Some(CollectionToggle {}),
//                 collection_details: Some(CollectionDetailsToggle {}),
//                 uses: Some(UsesToggle {}),
//                 rule_set: Some(RuleSetToggle {}),
//                 authorization_data: Some(AuthorizationData {}),
//             }))
//         },
//         mpl_token_metadata::instruction::UpdateArgs::AsUpdateAuthorityV2 { new_update_authority, data, primary_sale_happened, is_mutable, collection, collection_details, uses, rule_set, token_standard, authorization_data } => {
//             Some(update_event::UpdateArgs::AsUpdateAuthorityV2(UpdateArgsAsUpdateAuthorityV2 {
//                 new_update_authority: None,
//                 data: None,
//                 primary_sale_happened: None,
//                 is_mutable: None,
//                 collection: None,
//                 collection_details: None,
//                 uses: None,
//                 rule_set: None,
//                 token_standard: None,
//                 authorization_data: None,
//             }))
//         },
//         mpl_token_metadata::instruction::UpdateArgs::AsAuthorityItemDelegateV2 { new_update_authority, primary_sale_happened, is_mutable, token_standard, authorization_data } => {
//             Some(update_event::UpdateArgs::AsAuthorityItemDelegateV2(UpdateArgsAsAuthorityItemDelegateV2 {
//                 new_update_authority: None,
//                 primary_sale_happened: None,
//                 is_mutable: None,
//                 token_standard: None,
//                 authorization_data: None,
//             }))
//         },
//         mpl_token_metadata::instruction::UpdateArgs::AsCollectionDelegateV2 { collection, authorization_data } => {
//             None
//         },
//         mpl_token_metadata::instruction::UpdateArgs::AsDataDelegateV2 { data, authorization_data } => {
//             None
//         },
//         mpl_token_metadata::instruction::UpdateArgs::AsProgrammableConfigDelegateV2 { rule_set, authorization_data } => {
//             None
//         },
//         mpl_token_metadata::instruction::UpdateArgs::AsDataItemDelegateV2 { data, authorization_data } => {
//             Some(update_event::UpdateArgs::AsDataItemDelegateV2(UpdateArgsAsDataItemDelegateV2  {
//                 data: data.map(|x| x.into()),
//                 authorization_data: None,
//             }))
//         },
//         mpl_token_metadata::instruction::UpdateArgs::AsCollectionItemDelegateV2 { collection, authorization_data } => {
//             None
//         },
//         mpl_token_metadata::instruction::UpdateArgs::AsProgrammableConfigItemDelegateV2 { rule_set, authorization_data } => {
//             None
//         },
//     };

//     Ok(UpdateEvent {
//         authority,
//         metadata,
//         mint,
//         update_args,
//     })
// }

// fn _parse_update_metadata_account_v2_instruction<'a>(
//     instruction: &StructuredInstruction,
//     context: &TransactionContext,
//     update_metadata_account_v2: mpl_token_metadata::instruction::UpdateMetadataAccountArgsV2,
// ) -> Result<UpdateMetadataAccountV2Event, String> {
//     unimplemented!()
// }

impl From<mpl_token_metadata::state::Data> for Data {
    fn from(value: mpl_token_metadata::state::Data) -> Self {
        Data {
            name: value.name,
            symbol: value.symbol,
            uri: value.uri,
            seller_fee_basis_points: value.seller_fee_basis_points.into(),
            creators: value.creators.unwrap_or_else(Vec::new).iter().map(|x| x.into()).collect(),
        }
    }
}

impl From<mpl_token_metadata::state::DataV2> for DataV2 {
    fn from(value: mpl_token_metadata::state::DataV2) -> Self {
        DataV2 {
            collection: value.collection.map(|x| x.into()),
            creators: value.creators.unwrap_or_else(Vec::new).iter().map(|x| x.into()).collect(),
            name: value.name,
            seller_fee_basis_points: value.seller_fee_basis_points.into(),
            symbol: value.symbol,
            uri: value.uri,
            uses: value.uses.map(|x| x.into())
        }
    }
}

impl From<mpl_token_metadata::state::Collection> for Collection {
    fn from(value: mpl_token_metadata::state::Collection) -> Self {
        Collection {
            key: value.key.to_string(),
            verified: value.verified,
        }
    }
}

impl From<&mpl_token_metadata::state::Creator> for Creator {
    fn from(value: &mpl_token_metadata::state::Creator) -> Self {
        Creator {
            address: value.address.to_string(),
            verified: value.verified,
            share: value.share.into(),
        }
    }
}

impl From<mpl_token_metadata::state::Uses> for Uses {
    fn from(value: mpl_token_metadata::state::Uses) -> Self {
        Uses {
            remaining: value.remaining,
            use_method: UseMethod::from(value.use_method).into(),
            total: value.total,
        }
    }
}

impl From<mpl_token_metadata::state::UseMethod> for UseMethod {
    fn from(value: mpl_token_metadata::state::UseMethod) -> Self {
        match value {
            mpl_token_metadata::state::UseMethod::Burn => Self::Burn,
            mpl_token_metadata::state::UseMethod::Multiple => Self::Multiple,
            mpl_token_metadata::state::UseMethod::Single => Self::Single,
        }
    }
}

impl From<mpl_token_metadata::state::CollectionDetails> for CollectionDetails {
    fn from(value: mpl_token_metadata::state::CollectionDetails) -> Self {
        match value {
            mpl_token_metadata::state::CollectionDetails::V1 { size } => {
                let v1 = CollectionDetailsV1 { size };
                Self { version: Some(collection_details::Version::V1(v1)) }
            },
            mpl_token_metadata::state::CollectionDetails::V2 { padding } => {
                let v2 = CollectionDetailsV2 { padding: padding.iter().map(|x| *x as u64).collect() };
                Self { version: Some(collection_details::Version::V2(v2)) }
            },
        }
    }
}
