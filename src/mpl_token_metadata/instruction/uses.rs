use borsh::BorshDeserialize;
use super::super::processor::AuthorizationData;

#[derive(BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct ApproveUseAuthorityArgs {
    pub number_of_uses: u64,
}

#[derive(BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct UtilizeArgs {
    pub number_of_uses: u64,
}

#[derive(BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub enum UseArgs {
    V1 {
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
}
