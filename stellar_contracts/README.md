# Kairos — Stellar Smart Contracts

Soroban (Rust) port of the Kairos prediction market contracts, migrated from Starknet/Cairo.

## Stack
- **Blockchain:** Stellar
- **Smart Contract Runtime:** Soroban
- **Language:** Rust (no_std)
- **SDK:** `soroban-sdk v21`
- **Token Standard:** Stellar Asset Contract (SAC) — XLM or any Stellar asset

## Structure

```
src/
├── lib.rs       — Contract entry point; thin wrappers delegating to modules
├── types.rs     — All contracttypes (PredictionMarket, UserStake, BetActivity, DataKey, ...)
├── events.rs    — Event emit helpers
├── market.rs    — Market creation, queries, resolution, moderator management
├── betting.rs   — Share price AMM, buy_shares, claim, stake queries
└── admin.rs     — Emergency controls, pause flags, fee/token/oracle management
```

## Status
All functions are stubbed with `todo!()`. See each file's TODO comments for implementation requirements.

## Key Differences from Starknet Version

| Starknet / Cairo | Stellar / Soroban |
|---|---|
| `ContractAddress` | `Address` |
| `u256` amounts | `i128` amounts (stroops for XLM) |
| `felt252` labels | `Symbol` (≤ 9 chars) or `Bytes` |
| `ByteArray` | `Bytes` |
| `starknet::storage::Map` | `env.storage().persistent()` / `instance()` |
| Pragma oracle | Band Protocol or custom Soroban oracle |
| ArgentX / Braavos wallets | Freighter wallet |
| `get_caller_address()` | `caller.require_auth()` + pass address explicitly |
| `get_block_timestamp()` | `env.ledger().timestamp()` |
| Class hash upgrades | Soroban contract upgrade via `env.deployer()` |

## Building
```bash
cargo build --target wasm32-unknown-unknown --release
```

## Testing
```bash
cargo test
```
