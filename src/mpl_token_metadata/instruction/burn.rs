use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub enum BurnArgs {
    V1 {
        /// The amount of the token to burn
        amount: u64,
    },
}
