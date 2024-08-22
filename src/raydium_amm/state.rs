use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use bytemuck::{Pod, Zeroable};
use safe_transmute::{self, trivial::TriviallyTransmutable};

pub const TEN_THOUSAND: u64 = 10000;
pub const MAX_ORDER_LIMIT: usize = 10;

#[cfg_attr(feature = "client", derive(Debug))]
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TargetOrder {
    pub price: u64,
    pub vol: u64,
}
#[cfg(target_endian = "little")]
unsafe impl Zeroable for TargetOrder {}
#[cfg(target_endian = "little")]
unsafe impl Pod for TargetOrder {}
#[cfg(target_endian = "little")]
unsafe impl TriviallyTransmutable for TargetOrder {}

#[cfg_attr(feature = "client", derive(Debug))]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TargetOrders {
    pub owner: [u64; 4],
    pub buy_orders: [TargetOrder; 50],
    pub padding1: [u64; 8],
    pub target_x: u128,
    pub target_y: u128,
    pub plan_x_buy: u128,
    pub plan_y_buy: u128,
    pub plan_x_sell: u128,
    pub plan_y_sell: u128,
    pub placed_x: u128,
    pub placed_y: u128,
    pub calc_pnl_x: u128,
    pub calc_pnl_y: u128,
    pub sell_orders: [TargetOrder; 50],
    pub padding2: [u64; 6],
    pub replace_buy_client_id: [u64; MAX_ORDER_LIMIT],
    pub replace_sell_client_id: [u64; MAX_ORDER_LIMIT],
    pub last_order_numerator: u64,
    pub last_order_denominator: u64,

    pub plan_orders_cur: u64,
    pub place_orders_cur: u64,

    pub valid_buy_order_num: u64,
    pub valid_sell_order_num: u64,

    pub padding3: [u64; 10],

    pub free_slot_bits: u128,
}

#[cfg(test)]
impl Default for TargetOrders {
    #[inline]
    fn default() -> TargetOrders {
        TargetOrders {
            owner: [0; 4],
            buy_orders: [TargetOrder::default(); 50],
            padding1: [0; 8],
            target_x: 0,
            target_y: 0,
            plan_x_buy: 0,
            plan_y_buy: 0,
            plan_x_sell: 0,
            plan_y_sell: 0,
            placed_x: 0,
            placed_y: 0,
            calc_pnl_x: 0,
            calc_pnl_y: 0,
            sell_orders: [TargetOrder::default(); 50],
            padding2: [0; 6],
            replace_buy_client_id: [0; MAX_ORDER_LIMIT],
            replace_sell_client_id: [0; MAX_ORDER_LIMIT],
            last_order_denominator: 0,
            last_order_numerator: 0,
            plan_orders_cur: 0,
            place_orders_cur: 0,
            valid_buy_order_num: 0,
            valid_sell_order_num: 0,
            padding3: [0; 10],
            free_slot_bits: std::u128::MAX,
        }
    }
}

#[repr(u64)]
pub enum AmmStatus {
    Uninitialized = 0u64,
    Initialized = 1u64,
    Disabled = 2u64,
    WithdrawOnly = 3u64,
    // pool only can add or remove liquidity, can't swap and plan orders
    LiquidityOnly = 4u64,
    // pool only can add or remove liquidity and plan orders, can't swap
    OrderBookOnly = 5u64,
    // pool only can add or remove liquidity and swap, can't plan orders
    SwapOnly = 6u64,
    // pool status after created and will auto update to SwapOnly during swap after open_time
    WaitingTrade = 7u64,
}
impl AmmStatus {
    pub fn from_u64(status: u64) -> Self {
        match status {
            0u64 => AmmStatus::Uninitialized,
            1u64 => AmmStatus::Initialized,
            2u64 => AmmStatus::Disabled,
            3u64 => AmmStatus::WithdrawOnly,
            4u64 => AmmStatus::LiquidityOnly,
            5u64 => AmmStatus::OrderBookOnly,
            6u64 => AmmStatus::SwapOnly,
            7u64 => AmmStatus::WaitingTrade,
            _ => unreachable!(),
        }
    }

    pub fn into_u64(&self) -> u64 {
        match self {
            AmmStatus::Uninitialized => 0u64,
            AmmStatus::Initialized => 1u64,
            AmmStatus::Disabled => 2u64,
            AmmStatus::WithdrawOnly => 3u64,
            AmmStatus::LiquidityOnly => 4u64,
            AmmStatus::OrderBookOnly => 5u64,
            AmmStatus::SwapOnly => 6u64,
            AmmStatus::WaitingTrade => 7u64,
        }
    }
    pub fn valid_status(status: u64) -> bool {
        match status {
            1u64 | 2u64 | 3u64 | 4u64 | 5u64 | 6u64 | 7u64 => return true,
            _ => return false,
        }
    }

    pub fn deposit_permission(&self) -> bool {
        match self {
            AmmStatus::Uninitialized => false,
            AmmStatus::Initialized => true,
            AmmStatus::Disabled => false,
            AmmStatus::WithdrawOnly => false,
            AmmStatus::LiquidityOnly => true,
            AmmStatus::OrderBookOnly => true,
            AmmStatus::SwapOnly => true,
            AmmStatus::WaitingTrade => true,
        }
    }

    pub fn withdraw_permission(&self) -> bool {
        match self {
            AmmStatus::Uninitialized => false,
            AmmStatus::Initialized => true,
            AmmStatus::Disabled => false,
            AmmStatus::WithdrawOnly => true,
            AmmStatus::LiquidityOnly => true,
            AmmStatus::OrderBookOnly => true,
            AmmStatus::SwapOnly => true,
            AmmStatus::WaitingTrade => true,
        }
    }

    pub fn swap_permission(&self) -> bool {
        match self {
            AmmStatus::Uninitialized => false,
            AmmStatus::Initialized => true,
            AmmStatus::Disabled => false,
            AmmStatus::WithdrawOnly => false,
            AmmStatus::LiquidityOnly => false,
            AmmStatus::OrderBookOnly => false,
            AmmStatus::SwapOnly => true,
            AmmStatus::WaitingTrade => true,
        }
    }

    pub fn orderbook_permission(&self) -> bool {
        match self {
            AmmStatus::Uninitialized => false,
            AmmStatus::Initialized => true,
            AmmStatus::Disabled => false,
            AmmStatus::WithdrawOnly => false,
            AmmStatus::LiquidityOnly => false,
            AmmStatus::OrderBookOnly => true,
            AmmStatus::SwapOnly => false,
            AmmStatus::WaitingTrade => false,
        }
    }
}

#[repr(u64)]
pub enum AmmState {
    InvlidState = 0u64,
    IdleState = 1u64,
    CancelAllOrdersState = 2u64,
    PlanOrdersState = 3u64,
    CancelOrderState = 4u64,
    PlaceOrdersState = 5u64,
    PurgeOrderState = 6u64,
}
impl AmmState {
    pub fn from_u64(state: u64) -> Self {
        match state {
            0u64 => AmmState::InvlidState,
            1u64 => AmmState::IdleState,
            2u64 => AmmState::CancelAllOrdersState,
            3u64 => AmmState::PlanOrdersState,
            4u64 => AmmState::CancelOrderState,
            5u64 => AmmState::PlaceOrdersState,
            6u64 => AmmState::PurgeOrderState,
            _ => unreachable!(),
        }
    }

    pub fn into_u64(&self) -> u64 {
        match self {
            AmmState::InvlidState => 0u64,
            AmmState::IdleState => 1u64,
            AmmState::CancelAllOrdersState => 2u64,
            AmmState::PlanOrdersState => 3u64,
            AmmState::CancelOrderState => 4u64,
            AmmState::PlaceOrdersState => 5u64,
            AmmState::PurgeOrderState => 6u64,
        }
    }
    pub fn valid_state(state: u64) -> bool {
        match state {
            0u64 | 1u64 | 2u64 | 3u64 | 4u64 | 5u64 | 6u64 => return true,
            _ => return false,
        }
    }
}

#[cfg_attr(feature = "client", derive(Debug))]
#[derive(Copy, Clone)]
#[repr(u64)]
pub enum AmmParams {
    Status = 0u64,
    State = 1u64,
    OrderNum = 2u64,
    Depth = 3u64,
    AmountWave = 4u64,
    MinPriceMultiplier = 5u64,
    MaxPriceMultiplier = 6u64,
    MinSize = 7u64,
    VolMaxCutRatio = 8u64,
    Fees = 9u64,
    AmmOwner = 10u64,
    SetOpenTime = 11u64,
    LastOrderDistance = 12u64,
    InitOrderDepth = 13u64,
    SetSwitchTime = 14u64,
    ClearOpenTime = 15u64,
    Seperate = 16u64,
    UpdateOpenOrder = 17u64,
}
impl AmmParams {
    pub fn from_u64(state: u64) -> Self {
        match state {
            0u64 => AmmParams::Status,
            1u64 => AmmParams::State,
            2u64 => AmmParams::OrderNum,
            3u64 => AmmParams::Depth,
            4u64 => AmmParams::AmountWave,
            5u64 => AmmParams::MinPriceMultiplier,
            6u64 => AmmParams::MaxPriceMultiplier,
            7u64 => AmmParams::MinSize,
            8u64 => AmmParams::VolMaxCutRatio,
            9u64 => AmmParams::Fees,
            10u64 => AmmParams::AmmOwner,
            11u64 => AmmParams::SetOpenTime,
            12u64 => AmmParams::LastOrderDistance,
            13u64 => AmmParams::InitOrderDepth,
            14u64 => AmmParams::SetSwitchTime,
            15u64 => AmmParams::ClearOpenTime,
            16u64 => AmmParams::Seperate,
            17u64 => AmmParams::UpdateOpenOrder,
            _ => unreachable!(),
        }
    }

    pub fn into_u64(&self) -> u64 {
        match self {
            AmmParams::Status => 0u64,
            AmmParams::State => 1u64,
            AmmParams::OrderNum => 2u64,
            AmmParams::Depth => 3u64,
            AmmParams::AmountWave => 4u64,
            AmmParams::MinPriceMultiplier => 5u64,
            AmmParams::MaxPriceMultiplier => 6u64,
            AmmParams::MinSize => 7u64,
            AmmParams::VolMaxCutRatio => 8u64,
            AmmParams::Fees => 9u64,
            AmmParams::AmmOwner => 10u64,
            AmmParams::SetOpenTime => 11u64,
            AmmParams::LastOrderDistance => 12u64,
            AmmParams::InitOrderDepth => 13u64,
            AmmParams::SetSwitchTime => 14u64,
            AmmParams::ClearOpenTime => 15u64,
            AmmParams::Seperate => 16u64,
            AmmParams::UpdateOpenOrder => 17u64,
        }
    }
}

#[cfg_attr(feature = "client", derive(Debug))]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u64)]
pub enum AmmResetFlag {
    ResetYes = 0u64,
    ResetNo = 1u64,
}
impl AmmResetFlag {
    pub fn from_u64(flag: u64) -> Self {
        match flag {
            0u64 => AmmResetFlag::ResetYes,
            1u64 => AmmResetFlag::ResetNo,
            _ => unreachable!(),
        }
    }

    pub fn into_u64(&self) -> u64 {
        match self {
            AmmResetFlag::ResetYes => 0u64,
            AmmResetFlag::ResetNo => 1u64,
        }
    }
}

fn validate_fraction(numerator: u64, denominator: u64) -> Result<(), &'static str> {
    if numerator >= denominator || denominator == 0 {
        Err("Invalid fee")
    } else {
        Ok(())
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Fees {
    /// numerator of the min_separate
    pub min_separate_numerator: u64,
    /// denominator of the min_separate
    pub min_separate_denominator: u64,

    /// numerator of the fee
    pub trade_fee_numerator: u64,
    /// denominator of the fee
    /// and 'trade_fee_denominator' must be equal to 'min_separate_denominator'
    pub trade_fee_denominator: u64,

    /// numerator of the pnl
    pub pnl_numerator: u64,
    /// denominator of the pnl
    pub pnl_denominator: u64,

    /// numerator of the swap_fee
    pub swap_fee_numerator: u64,
    /// denominator of the swap_fee
    pub swap_fee_denominator: u64,
}

impl Fees {
    /// Validate that the fees are reasonable
    pub fn validate(&self) -> Result<(), &'static str> {
        validate_fraction(self.min_separate_numerator, self.min_separate_denominator)?;
        validate_fraction(self.trade_fee_numerator, self.trade_fee_denominator)?;
        validate_fraction(self.pnl_numerator, self.pnl_denominator)?;
        validate_fraction(self.swap_fee_numerator, self.swap_fee_denominator)?;
        Ok(())
    }

    pub fn initialize(&mut self) -> Result<(), &'static str> {
        // min_separate = 5/10000
        self.min_separate_numerator = 5;
        self.min_separate_denominator = TEN_THOUSAND;
        // trade_fee = 25/10000
        self.trade_fee_numerator = 25;
        self.trade_fee_denominator = TEN_THOUSAND;
        // pnl = 12/100
        self.pnl_numerator = 12;
        self.pnl_denominator = 100;
        // swap_fee = 25 / 10000
        self.swap_fee_numerator = 25;
        self.swap_fee_denominator = TEN_THOUSAND;
        Ok(())
    }
}

impl Fees {
    pub const LEN: usize = 64;
    pub fn pack_into_slice(&self, output: &mut [u8]) {
        let output = array_mut_ref![output, 0, 64];
        let (
            min_separate_numerator,
            min_separate_denominator,
            trade_fee_numerator,
            trade_fee_denominator,
            pnl_numerator,
            pnl_denominator,
            swap_fee_numerator,
            swap_fee_denominator,
        ) = mut_array_refs![output, 8, 8, 8, 8, 8, 8, 8, 8];
        *min_separate_numerator = self.min_separate_numerator.to_le_bytes();
        *min_separate_denominator = self.min_separate_denominator.to_le_bytes();
        *trade_fee_numerator = self.trade_fee_numerator.to_le_bytes();
        *trade_fee_denominator = self.trade_fee_denominator.to_le_bytes();
        *pnl_numerator = self.pnl_numerator.to_le_bytes();
        *pnl_denominator = self.pnl_denominator.to_le_bytes();
        *swap_fee_numerator = self.swap_fee_numerator.to_le_bytes();
        *swap_fee_denominator = self.swap_fee_denominator.to_le_bytes();
    }

    pub fn unpack_from_slice(input: &[u8]) -> Result<Fees, &'static str> {
        let input = array_ref![input, 0, 64];
        #[allow(clippy::ptr_offset_with_cast)]
        let (
            min_separate_numerator,
            min_separate_denominator,
            trade_fee_numerator,
            trade_fee_denominator,
            pnl_numerator,
            pnl_denominator,
            swap_fee_numerator,
            swap_fee_denominator,
        ) = array_refs![input, 8, 8, 8, 8, 8, 8, 8, 8];
        Ok(Self {
            min_separate_numerator: u64::from_le_bytes(*min_separate_numerator),
            min_separate_denominator: u64::from_le_bytes(*min_separate_denominator),
            trade_fee_numerator: u64::from_le_bytes(*trade_fee_numerator),
            trade_fee_denominator: u64::from_le_bytes(*trade_fee_denominator),
            pnl_numerator: u64::from_le_bytes(*pnl_numerator),
            pnl_denominator: u64::from_le_bytes(*pnl_denominator),
            swap_fee_numerator: u64::from_le_bytes(*swap_fee_numerator),
            swap_fee_denominator: u64::from_le_bytes(*swap_fee_denominator),
        })
    }
}


#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LastOrderDistance {
    pub last_order_numerator: u64,
    pub last_order_denominator: u64,
}

/// For simulateTransaction to get instruction data
#[cfg_attr(feature = "client", derive(Debug))]
#[derive(Copy, Clone)]
#[repr(u64)]
pub enum SimulateParams {
    PoolInfo = 0u64,
    SwapBaseInInfo = 1u64,
    SwapBaseOutInfo = 2u64,
    RunCrankInfo = 3u64,
}
impl SimulateParams {
    pub fn from_u64(flag: u64) -> Self {
        match flag {
            0u64 => SimulateParams::PoolInfo,
            1u64 => SimulateParams::SwapBaseInInfo,
            2u64 => SimulateParams::SwapBaseOutInfo,
            3u64 => SimulateParams::RunCrankInfo,
            _ => unreachable!(),
        }
    }

    pub fn into_u64(&self) -> u64 {
        match self {
            SimulateParams::PoolInfo => 0u64,
            SimulateParams::SwapBaseInInfo => 1u64,
            SimulateParams::SwapBaseOutInfo => 2u64,
            SimulateParams::RunCrankInfo => 3u64,
        }
    }
}
