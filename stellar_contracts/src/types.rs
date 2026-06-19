use soroban_sdk::{contracttype, Address, Bytes, Symbol};

// ================ Core Market Types ================

#[contracttype]
#[derive(Clone)]
pub struct PredictionMarket {
    pub title: Bytes,
    pub market_id: u64,
    pub description: Bytes,
    /// Label for choice A (e.g., "Yes")
    pub choice_a: Symbol,
    /// Label for choice B (e.g., "No")
    pub choice_b: Symbol,
    pub category: u32,
    pub is_resolved: bool,
    pub is_open: bool,
    /// Unix timestamp (seconds) when market closes for betting
    pub end_time: u64,
    pub status: MarketStatus,
    /// 0 = unresolved, 1 = choice_a won, 2 = choice_b won
    pub winning_choice: u32,
    pub total_shares_option_one: i128,
    pub total_shares_option_two: i128,
    pub total_pool: i128,
    /// For crypto prediction markets — the tracked asset symbol (e.g., "BTC")
    pub crypto_asset: Symbol,
    /// Target price value for crypto prediction (scaled by PRECISION)
    pub crypto_target_value: i128,
}

#[contracttype]
#[derive(Clone, PartialEq)]
pub enum MarketCategory {
    Normal = 0,
    Politics = 1,
    Sports = 2,
    Crypto = 3,
    Business = 4,
    Entertainment = 5,
    Science = 6,
    Other = 7,
}

#[contracttype]
#[derive(Clone, PartialEq)]
pub enum MarketStatus {
    Active,
    Locked,
    Resolved,
    Closed,
}

#[contracttype]
#[derive(Clone)]
pub struct UserStake {
    /// Shares held on choice A (scaled by PRECISION = 10^7 in Stellar)
    pub shares_a: i128,
    /// Shares held on choice B (scaled by PRECISION)
    pub shares_b: i128,
    /// Total XLM (in stroops) invested across both choices
    pub total_invested: i128,
}

#[contracttype]
#[derive(Clone)]
pub struct BetActivity {
    /// 1 = choice_a, 2 = choice_b
    pub choice: u32,
    pub amount: i128,
}

#[contracttype]
#[derive(Clone)]
pub struct MarketStats {
    pub total_traders: u64,
    pub traders_option_a: u64,
    pub traders_option_b: u64,
    pub amount_staked_option_a: i128,
    pub amount_staked_option_b: i128,
    pub total_trades: u64,
}

// ================ Storage Keys ================

#[contracttype]
pub enum DataKey {
    Admin,
    FeeRecipient,
    ProtocolToken,
    PlatformFee,
    PredictionCount,
    IsPaused,
    MarketCreationPaused,
    BettingPaused,
    ResolutionPaused,
    MinMarketDuration,
    MaxMarketDuration,
    ResolutionWindow,
    MinBetAmount,
    MaxBetAmount,
    TotalValueLocked,
    OracleAddress,
    ReentrancyGuard,
    /// (market_id) → PredictionMarket
    Market(u64),
    /// (market_id) → MarketStats
    MarketStat(u64),
    /// (market_id) → i128 (liquidity)
    MarketLiquidity(u64),
    /// (market_id, user) → UserStake
    UserStake(u64, Address),
    /// (market_id, user) → bool (claimed)
    Claimed(u64, Address),
    /// (market_id, user) → bool (has traded)
    UserTraded(u64, Address),
    /// user → Vec<u64> (list of market ids)
    UserPredictions(Address),
    /// (market_id) → Vec<BetActivity>
    MarketActivity(u64),
    /// address → bool
    Moderator(Address),
    ModeratorCount,
}
