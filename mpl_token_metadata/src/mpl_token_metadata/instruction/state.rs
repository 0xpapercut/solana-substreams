use borsh::BorshDeserialize;
use super::super::processor::AuthorizationData;

#[derive(BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub enum LockArgs {
    V1 {
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
}

#[derive(BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub enum UnlockArgs {
    V1 {
        /// Required authorization data to validate the request.
        authorization_data: Option<AuthorizationData>,
    },
}
