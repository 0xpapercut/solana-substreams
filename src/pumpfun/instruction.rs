use borsh::BorshDeserialize;

#[derive(BorshDeserialize)]
pub struct Pubkey(pub [u8; 32]);

use std::fmt::{self, Display};

impl Display for Pubkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", bs58::encode(self.0).into_string())
    }
}

#[derive(Debug, BorshDeserialize)]
pub enum PumpfunInstruction {
    Initialize,
    SetParams(SetParamsInstruction),
    Create(CreateInstruction),
    Buy(BuyInstruction),
    Sell(SellInstruction),
    Withdraw,
    Unknown,
}

impl PumpfunInstruction {
    pub fn unpack(data: &[u8]) -> Result<Self, &'static str> {
        let (tag, data) = data.split_at(8);
        match tag {
            [175, 175, 109, 31, 13, 152, 155, 237] => Ok(Self::Initialize),
            [165, 31, 134, 53, 189, 180, 130, 255] => Ok(Self::SetParams(SetParamsInstruction::unpack(data)?)),
            [24, 30, 200, 40, 5, 28, 7, 119] => Ok(Self::Create(CreateInstruction::unpack(data)?)),
            [102, 6, 61, 18, 1, 218, 235, 234] => Ok(Self::Buy(BuyInstruction::unpack(data)?)),
            [51, 230, 133, 164, 1, 127, 131, 173] => Ok(Self::Sell(SellInstruction::unpack(data)?)),
            [183, 18, 70, 156, 148, 109, 161, 34] => Ok(Self::Withdraw),
            _ => Ok(Self::Unknown),
        }
    }
}

#[derive(Debug, BorshDeserialize)]
pub struct SetParamsInstruction {
    pub fee_recipient: Pubkey,
    pub initial_virtual_token_reserves: u64,
    pub initial_virtual_sol_reserves: u64,
    pub initial_real_token_reserves: u64,
    pub token_total_supply: u64,
    pub fee_basis_points: u64,
}

impl SetParamsInstruction {
    fn unpack(data: &[u8]) -> Result<Self, &'static str> {
        Self::try_from_slice(data).map_err(|_| "Failed to deserialize SetParamsInstruction.")
    }
}

#[derive(Debug, BorshDeserialize)]
pub struct CreateInstruction {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

impl CreateInstruction {
    fn unpack(data: &[u8]) -> Result<Self, &'static str> {
        Self::try_from_slice(data).map_err(|_| "Failed to deserialize CreateInstruction.")
    }
}

#[derive(Debug, BorshDeserialize)]
pub struct BuyInstruction {
    pub amount: u64,
    pub max_sol_cost: u64,
}

impl BuyInstruction {
    fn unpack(data: &[u8]) -> Result<Self, &'static str> {
        Self::try_from_slice(data).map_err(|_| "Failed to deserialize BuyInstruction.")
    }
}

#[derive(Debug, BorshDeserialize)]
pub struct SellInstruction {
    pub amount: u64,
    pub min_sol_output: u64,
}

impl SellInstruction {
    fn unpack(data: &[u8]) -> Result<Self, &'static str> {
        Self::try_from_slice(data).map_err(|_| "Failed to deserialize SellInstruction.")
    }
}

impl fmt::Debug for Pubkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Pubkey")
            .field(&bs58::encode(self.0).into_string())
            .finish()
    }
}
