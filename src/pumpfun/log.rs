use std::fmt::{self, Display};
use borsh::BorshDeserialize;

#[derive(BorshDeserialize)]
pub struct Pubkey(pub [u8; 32]);

impl Display for Pubkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", bs58::encode(self.0).into_string())
    }
}

impl fmt::Debug for Pubkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Pubkey")
            .field(&bs58::encode(self.0).into_string())
            .finish()
    }
}

#[derive(Debug)]
pub enum PumpfunLog {
    Create(CreateLog),
    Trade(TradeLog),
    Complete(CompleteLog),
    SetParams(SetParamsLog),
}

impl PumpfunLog {
    pub fn unpack(data: &[u8]) -> Result<Self, &'static str> {
        let (discriminator, data) = data.split_at(8);
        match discriminator {
            [27, 114, 169, 77, 222, 235, 99, 118] => CreateLog::try_from_slice(data).map(Self::Create).map_err(|_| "Failed to unpack CreateEvent."),
            [189, 219, 127, 211, 78, 230, 97, 238] => TradeLog::try_from_slice(data).map(Self::Trade).map_err(|_| "Failed to unpack TradeEvent."),
            [95, 114, 97, 156, 212, 46, 152, 8] => CompleteLog::try_from_slice(data).map(Self::Complete).map_err(|_| "Failed to unpack CompleteEvent."),
            [223, 195, 159, 246, 62, 48, 143, 131] => SetParamsLog::try_from_slice(data).map(Self::SetParams).map_err(|_| "Failed to unpack SetParamsEvent."),
            _ => Err("Unknown Pumpfun event."),
        }
    }
}

#[derive(Debug, BorshDeserialize)]
pub struct CreateLog {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub user: Pubkey,
}

#[derive(Debug, BorshDeserialize)]
pub struct TradeLog {
    pub mint: Pubkey,
    pub sol_amount: u64,
    pub token_amount: u64,
    pub is_buy: bool,
    pub user: Pubkey,
    pub timestamp: i64,
    pub virtual_sol_reserves: u64,
    pub virtual_token_reserves: u64,
    pub real_sol_reserves: u64,
    pub real_token_reserves: u64,
}

#[derive(Debug, BorshDeserialize)]
pub struct CompleteLog {
    pub user: Pubkey,
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub timestamp: i64,
}

#[derive(Debug, BorshDeserialize)]
pub struct SetParamsLog {
    pub fee_recipient: Pubkey,
    pub initial_virtual_token_reserves: u64,
    pub initial_virtual_sol_reserves: u64,
    pub initial_real_token_reserves: u64,
    pub token_total_supply: u64,
    pub fee_basis_points: u64,
}
