#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Bytes, Env, Symbol, Vec};

pub mod admin;
pub mod betting;
pub mod events;
pub mod market;
pub mod types;

use types::*;

#[contract]
pub struct PredictionHub;

#[contractimpl]
impl PredictionHub {
    // TODO: set admin, fee_recipient, protocol_token in instance storage
    // TODO: set defaults — platform_fee=200bps, min_bet=10_000_000, max_bet=1_000_000_000 (stroops)
    // TODO: set time defaults — min_duration=3600s, max_duration=2592000s, resolution_window=86400s
    pub fn initialize(env: Env, admin: Address, fee_recipient: Address, protocol_token: Address) {
        todo!()
    }

    // ── Market ──────────────────────────────────────────────────────────────
    pub fn create_predictions(env: Env, title: Bytes, description: Bytes, choice_a: Symbol, choice_b: Symbol, category: u32, end_time: u64, prediction_market_type: u32, crypto_asset: Option<Symbol>, crypto_target_value: Option<i128>) {
        market::create_predictions(&env, title, description, choice_a, choice_b, category, end_time, prediction_market_type, crypto_asset, crypto_target_value)
    }
    pub fn get_prediction_count(env: Env) -> u64 { market::get_prediction_count(&env) }
    pub fn get_prediction(env: Env, market_id: u64) -> PredictionMarket { market::get_prediction(&env, market_id) }
    pub fn get_all_predictions(env: Env) -> Vec<PredictionMarket> { market::get_all_predictions(&env) }
    pub fn get_all_general_predictions(env: Env) -> Vec<PredictionMarket> { market::get_all_general_predictions(&env) }
    pub fn get_all_predictions_by_market_category(env: Env, category: u32) -> Vec<PredictionMarket> { market::get_all_predictions_by_market_category(&env, category) }
    pub fn get_market_status(env: Env, market_id: u64) -> (bool, bool) { market::get_market_status(&env, market_id) }
    pub fn get_all_open_markets(env: Env) -> Vec<PredictionMarket> { market::get_all_open_markets(&env) }
    pub fn get_all_locked_markets(env: Env) -> Vec<PredictionMarket> { market::get_all_locked_markets(&env) }
    pub fn get_all_resolved_markets(env: Env) -> Vec<PredictionMarket> { market::get_all_resolved_markets(&env) }
    pub fn get_active_prediction_markets(env: Env) -> Vec<PredictionMarket> { market::get_active_prediction_markets(&env) }
    pub fn get_active_general_prediction_markets(env: Env) -> Vec<PredictionMarket> { market::get_active_general_prediction_markets(&env) }
    pub fn get_active_sport_markets(env: Env) -> Vec<PredictionMarket> { market::get_active_sport_markets(&env) }
    pub fn get_active_crypto_markets(env: Env) -> Vec<PredictionMarket> { market::get_active_crypto_markets(&env) }
    pub fn get_all_resolved_prediction_markets(env: Env) -> Vec<PredictionMarket> { market::get_all_resolved_prediction_markets(&env) }
    pub fn get_all_closed_bets_for_user(env: Env, user: Address) -> Vec<PredictionMarket> { market::get_all_closed_bets_for_user(&env, user) }
    pub fn get_all_open_bets_for_user(env: Env, user: Address) -> Vec<PredictionMarket> { market::get_all_open_bets_for_user(&env, user) }
    pub fn get_all_locked_bets_for_user(env: Env, user: Address) -> Vec<PredictionMarket> { market::get_all_locked_bets_for_user(&env, user) }
    pub fn get_all_bets_for_user(env: Env, user: Address) -> Vec<PredictionMarket> { market::get_all_bets_for_user(&env, user) }
    pub fn is_prediction_market_open_for_betting(env: Env, market_id: u64) -> bool { market::is_prediction_market_open_for_betting(&env, market_id) }
    pub fn resolve_prediction(env: Env, market_id: u64, winning_choice: u32) { market::resolve_prediction(&env, market_id, winning_choice) }
    pub fn toggle_market_status(env: Env, market_id: u64) { market::toggle_market_status(&env, market_id) }
    pub fn remove_all_predictions(env: Env) { market::remove_all_predictions(&env) }
    pub fn add_moderator(env: Env, moderator: Address) { market::add_moderator(&env, moderator) }
    pub fn get_admin(env: Env) -> Address { market::get_admin(&env) }
    pub fn get_fee_recipient(env: Env) -> Address { market::get_fee_recipient(&env) }
    pub fn set_fee_recipient(env: Env, recipient: Address) { market::set_fee_recipient(&env, recipient) }

    // ── Betting ─────────────────────────────────────────────────────────────
    pub fn calculate_share_prices(env: Env, market_id: u64) -> (i128, i128) { betting::calculate_share_prices(&env, market_id) }
    pub fn buy_shares(env: Env, market_id: u64, choice: u32, amount: i128, token: Address) { betting::buy_shares(&env, market_id, choice, amount, token) }
    pub fn claim(env: Env, market_id: u64) { betting::claim(&env, market_id) }
    pub fn get_user_stake_details(env: Env, market_id: u64, user: Address) -> UserStake { betting::get_user_stake_details(&env, market_id, user) }
    pub fn get_market_activity(env: Env, market_id: u64) -> Vec<BetActivity> { betting::get_market_activity(&env, market_id) }
    pub fn get_protocol_token(env: Env) -> Address { betting::get_protocol_token(&env) }
    pub fn get_betting_restrictions(env: Env) -> (i128, i128) { betting::get_betting_restrictions(&env) }
    pub fn get_market_liquidity(env: Env, market_id: u64) -> i128 { betting::get_market_liquidity(&env, market_id) }
    pub fn get_total_value_locked(env: Env) -> i128 { betting::get_total_value_locked(&env) }

    // ── Admin ────────────────────────────────────────────────────────────────
    pub fn remove_moderator(env: Env, moderator: Address) { admin::remove_moderator(&env, moderator) }
    pub fn is_moderator(env: Env, address: Address) -> bool { admin::is_moderator(&env, address) }
    pub fn get_moderator_count(env: Env) -> u32 { admin::get_moderator_count(&env) }
    pub fn emergency_pause(env: Env) { admin::emergency_pause(&env) }
    pub fn emergency_unpause(env: Env) { admin::emergency_unpause(&env) }
    pub fn pause_market_creation(env: Env) { admin::pause_market_creation(&env) }
    pub fn unpause_market_creation(env: Env) { admin::unpause_market_creation(&env) }
    pub fn pause_betting(env: Env) { admin::pause_betting(&env) }
    pub fn unpause_betting(env: Env) { admin::unpause_betting(&env) }
    pub fn pause_resolution(env: Env) { admin::pause_resolution(&env) }
    pub fn unpause_resolution(env: Env) { admin::unpause_resolution(&env) }
    pub fn set_time_restrictions(env: Env, min_duration: u64, max_duration: u64, resolution_window: u64) { admin::set_time_restrictions(&env, min_duration, max_duration, resolution_window) }
    pub fn set_platform_fee(env: Env, fee_bps: i128) { admin::set_platform_fee(&env, fee_bps) }
    pub fn get_platform_fee(env: Env) -> i128 { admin::get_platform_fee(&env) }
    pub fn is_paused(env: Env) -> bool { admin::is_paused(&env) }
    pub fn get_time_restrictions(env: Env) -> (u64, u64, u64) { admin::get_time_restrictions(&env) }
    pub fn is_market_creation_paused(env: Env) -> bool { admin::is_market_creation_paused(&env) }
    pub fn is_betting_paused(env: Env) -> bool { admin::is_betting_paused(&env) }
    pub fn is_resolution_paused(env: Env) -> bool { admin::is_resolution_paused(&env) }
    pub fn set_oracle_address(env: Env, oracle: Address) { admin::set_oracle_address(&env, oracle) }
    pub fn get_oracle_address(env: Env) -> Address { admin::get_oracle_address(&env) }
    pub fn get_market_stats(env: Env) -> (u64, u64, u64) { admin::get_market_stats(&env) }
    pub fn emergency_close_market(env: Env, market_id: u64) { admin::emergency_close_market(&env, market_id) }
    pub fn emergency_close_multiple_markets(env: Env, market_ids: Vec<u64>) { admin::emergency_close_multiple_markets(&env, market_ids) }
    pub fn emergency_resolve_multiple_markets(env: Env, market_ids: Vec<u64>, winning_choices: Vec<u32>) { admin::emergency_resolve_multiple_markets(&env, market_ids, winning_choices) }
    pub fn set_protocol_token(env: Env, token_address: Address) { admin::set_protocol_token(&env, token_address) }
    pub fn set_protocol_restrictions(env: Env, min_amount: i128, max_amount: i128) { admin::set_protocol_restrictions(&env, min_amount, max_amount) }
    pub fn emergency_withdraw_tokens(env: Env, amount: i128, recipient: Address) { admin::emergency_withdraw_tokens(&env, amount, recipient) }
}
