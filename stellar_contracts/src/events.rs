use soroban_sdk::{Address, Env};

// ================ Events ================
// All events are published via env.events().publish((topic, ...), data)

// TODO: emit (symbol "mod_add", moderator) → added_by
pub fn emit_moderator_added(env: &Env, moderator: Address, added_by: Address) {
    todo!()
}

// TODO: emit (symbol "mod_rem", moderator) → removed_by
pub fn emit_moderator_removed(env: &Env, moderator: Address, removed_by: Address) {
    todo!()
}

// TODO: emit (symbol "em_pause") → paused_by
pub fn emit_emergency_paused(env: &Env, paused_by: Address) {
    todo!()
}

// TODO: emit (symbol "mkt_crt", market_id) → (creator, market_type)
pub fn emit_market_created(env: &Env, market_id: u64, creator: Address, market_type: u32) {
    todo!()
}

// TODO: emit (symbol "mkt_res", market_id) → (resolver, winning_choice)
pub fn emit_market_resolved(env: &Env, market_id: u64, resolver: Address, winning_choice: u32) {
    todo!()
}

// TODO: emit (symbol "wager", market_id, user) → (choice, amount, fee_amount, net_amount)
pub fn emit_wager_placed(
    env: &Env,
    market_id: u64,
    user: Address,
    choice: u32,
    amount: i128,
    fee_amount: i128,
    net_amount: i128,
) {
    todo!()
}

// TODO: emit (symbol "fee_col", market_id) → (fee_amount, fee_recipient)
pub fn emit_fees_collected(env: &Env, market_id: u64, fee_amount: i128, fee_recipient: Address) {
    todo!()
}

// TODO: emit (symbol "win_col", market_id, user) → amount
pub fn emit_winnings_collected(env: &Env, market_id: u64, user: Address, amount: i128) {
    todo!()
}
