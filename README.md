# Kairos — Decentralized Prediction Markets on Stellar

> **Predict. Stake. Earn.** Kairos is a fully decentralized prediction market platform built on the Stellar blockchain, powered by Soroban smart contracts.

[![License: ISC](https://img.shields.io/badge/License-ISC-blue.svg)](https://opensource.org/licenses/ISC)
[![Stellar](https://img.shields.io/badge/Blockchain-Stellar-blue)](https://stellar.org)
[![Soroban](https://img.shields.io/badge/Contracts-Soroban-purple)](https://soroban.stellar.org)
[![Next.js](https://img.shields.io/badge/Frontend-Next.js%2015-black)](https://nextjs.org)

---

## Table of Contents

- [What is Kairos?](#what-is-kairos)
- [Architecture](#architecture)
- [Project Structure](#project-structure)
- [Tech Stack](#tech-stack)
- [Prerequisites](#prerequisites)
- [Local Development](#local-development)
  - [1. Clone & Install](#1-clone--install)
  - [2. Environment Variables](#2-environment-variables)
  - [3. Run the App](#3-run-the-app)
- [Smart Contracts (Soroban)](#smart-contracts-soroban)
  - [Contract Architecture](#contract-architecture)
  - [How the AMM Works](#how-the-amm-works)
  - [Building Contracts](#building-contracts)
  - [Deploying to Testnet](#deploying-to-testnet)
- [Indexer](#indexer)
- [API Server](#api-server)
- [Wallet Integration](#wallet-integration)
- [Market Categories](#market-categories)
- [Contributing](#contributing)
- [Roadmap](#roadmap)
- [Community](#community)

---

## What is Kairos?

**Kairos** (Greek: *the right, critical, or opportune moment*) is a decentralized prediction market where users:

- **Create markets** on any real-world outcome — sports, crypto prices, politics, news events
- **Buy shares** on their predicted outcome using XLM
- **Earn rewards** proportional to their stake when predictions are correct
- **Participate trustlessly** — all funds are locked in Soroban smart contracts, no custodian

Markets resolve automatically based on outcomes verified by moderators or oracles. The platform uses an **Automated Market Maker (AMM)** to price shares dynamically based on how much is staked on each outcome.

---

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        Kairos Platform                          │
│                                                                 │
│  ┌──────────────┐   ┌──────────────┐   ┌────────────────────┐  │
│  │  Landing Page │   │  Client App  │   │   Market Agent     │  │
│  │  (Next.js)   │   │  (Next.js)   │   │   (AI assistant)   │  │
│  └──────┬───────┘   └──────┬───────┘   └────────────────────┘  │
│         │                  │                                     │
│         └──────────────────┼──────────────────────────────────  │
│                            │                                     │
│                    ┌───────▼────────┐                           │
│                    │  API Server    │                           │
│                    │  (Node/Express)│                           │
│                    │  Port: 3678    │                           │
│                    └───────┬────────┘                           │
│                            │                                     │
│              ┌─────────────┼─────────────┐                      │
│              │             │             │                       │
│        ┌─────▼──┐   ┌──────▼──┐   ┌─────▼──────────────────┐  │
│        │ Redis  │   │Postgres │   │   Stellar Network       │  │
│        │(cache) │   │  (DB)   │   │                         │  │
│        └────────┘   └─────────┘   │  ┌─────────────────┐   │  │
│                                   │  │ PredictionHub   │   │  │
│                                   │  │ Soroban Contract│   │  │
│                                   │  └────────┬────────┘   │  │
│                                   │           │            │  │
│                                   │  ┌────────▼────────┐  │  │
│                                   │  │    Indexer      │  │  │
│                                   │  │ (Substreams +   │  │  │
│                                   │  │  Subgraph)      │  │  │
│                                   │  └─────────────────┘  │  │
│                                   └────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

---

## Project Structure

```
kairos/
├── client/                  # Main Next.js frontend application
│   ├── app/
│   │   ├── abis/            # Contract ABI definitions
│   │   ├── components/      # UI components
│   │   │   ├── layout/      # Header, Footer
│   │   │   ├── sections/    # Page sections (marquee, purchase, etc.)
│   │   │   ├── ui/          # Reusable UI primitives
│   │   │   └── utils/       # Wallet connectors, constants, providers
│   │   ├── context/         # React context (app state)
│   │   ├── dashboard/       # Dashboard pages (markets, predictions)
│   │   ├── hooks/           # Custom hooks (useMarket, useBet, usePurchase)
│   │   └── types/           # TypeScript type definitions
│   └── public/              # Static assets
│
├── landing_page/            # Marketing landing page (Next.js)
│   └── app/
│       └── components/      # Hero, Features, HowItWorks, CTA, Footer
│
├── server/                  # Backend API (Node.js / Express)
│   └── src/
│       ├── api/v1/          # REST API routes
│       ├── config/          # Environment & app config
│       ├── middleware/       # Auth, rate limiting
│       └── utils/           # Helpers
│
├── stellar_contracts/       # Soroban smart contracts (Rust) ← active
│   └── src/
│       ├── lib.rs           # Contract entry point & public API
│       ├── market.rs        # Market creation, queries, resolution
│       ├── betting.rs       # AMM share pricing, buy_shares, claim
│       ├── admin.rs         # Pause controls, fee/oracle management
│       ├── events.rs        # Contract event emitters
│       └── types.rs         # All contract types & storage keys
│
├── starknet_contracts/      # Legacy Cairo contracts (reference only)
│
├── indexer/
│   └── kairos/
│       ├── src/             # Substreams Rust indexer
│       └── subgraph/        # The Graph subgraph mappings
│
├── agents/
│   └── market_agent/        # AI agent for market management
│
└── docs/
    ├── GettingStarted.md
    └── CONTRIBUTING.md
```

---

## Tech Stack

| Layer | Technology |
|---|---|
| Blockchain | [Stellar](https://stellar.org) |
| Smart Contracts | [Soroban](https://soroban.stellar.org) (Rust) |
| Contract Language | Rust (`soroban-sdk v21`) |
| Frontend | [Next.js 15](https://nextjs.org) + React 19 |
| Styling | Tailwind CSS |
| Wallet | [Freighter](https://freighter.app) (`@stellar/freighter-api`) |
| Stellar SDK | `@stellar/stellar-sdk` |
| Backend | Node.js + Express |
| Database | PostgreSQL + Redis |
| Indexer | [Substreams](https://substreams.streamingfast.io) + [The Graph](https://thegraph.com) |
| Package Manager | [pnpm](https://pnpm.io) workspaces |
| Monorepo | pnpm workspaces |

---

## Prerequisites

Make sure you have the following installed:

| Tool | Version | Purpose |
|---|---|---|
| [Node.js](https://nodejs.org) | ≥ 18 | Frontend & server |
| [pnpm](https://pnpm.io) | ≥ 8 | Package management |
| [Rust](https://rustup.rs) | stable | Soroban contract compilation |
| [Stellar CLI](https://developers.stellar.org/docs/tools/developer-tools/cli/stellar-cli) | latest | Contract deploy & interact |
| [Git](https://git-scm.com) | any | Version control |
| [PostgreSQL](https://postgresql.org) | ≥ 14 | Backend database |
| [Redis](https://redis.io) | ≥ 7 | Caching layer |
| [Freighter](https://freighter.app) | latest | Browser wallet extension |

Install Rust + the Soroban target:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-unknown-unknown
```

Install Stellar CLI:
```bash
cargo install --locked stellar-cli --features opt
```

---

## Local Development

### 1. Clone & Install

```bash
git clone https://github.com/CasinoHubInc/kairos.git
cd kairos
pnpm install
```

### 2. Environment Variables

Create `.env` files in the relevant packages. Copy the examples:

```bash
cp server/.env.example server/.env
cp client/.env.example client/.env.local
```

**`server/.env`**
```env
PORT=3678
NODE_ENV=development

# PostgreSQL
POSTGRES_HOST=localhost
POSTGRES_PORT=5432
POSTGRES_USERNAME=root
POSTGRES_PASSWORD=your_password
POSTGRES_DB=kairos

# Redis
REDIS_HOST=localhost
REDIS_PORT=6379
REDIS_PASSWORD=

# JWT
ACCESS_TOKEN_SECRET=your_access_token_secret
ACCESS_TOKEN_EXP=30m
REFRESH_TOKEN_SECRET=your_refresh_token_secret
REFRESH_TOKEN_EXP=1w
```

**`client/.env.local`**
```env
# Set to "mainnet" for production
NEXT_PUBLIC_NETWORK=testnet

# Deployed Soroban contract ID (C... format)
NEXT_PUBLIC_KAIROS_CONTRACT_ID=CXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
```

### 3. Run the App

Run each service in a separate terminal:

```bash
# Landing page — http://localhost:3000
pnpm dev:landing

# Client app — http://localhost:3001
pnpm dev:client

# API server — http://localhost:3678
pnpm dev:server
```

Or run client + server together:
```bash
pnpm all
```

---

## Smart Contracts (Soroban)

All active contracts live in `stellar_contracts/`. They are written in Rust using the `soroban-sdk`.

### Contract Architecture

The single `PredictionHub` contract manages the entire platform and is split across modules:

```
stellar_contracts/src/
│
├── lib.rs          — Public contract API. Thin wrappers that delegate to modules.
│
├── market.rs       — Market lifecycle
│   ├── create_predictions()          Create a new market
│   ├── resolve_prediction()          Set the winning outcome
│   ├── toggle_market_status()        Lock / unlock a market
│   ├── get_prediction()              Fetch a single market by ID
│   ├── get_all_predictions()         Fetch all markets
│   ├── get_all_open/locked/resolved_markets()
│   ├── get_active_*_markets()        Filter by category
│   └── get_all_*_bets_for_user()     User portfolio queries
│
├── betting.rs      — Share trading
│   ├── calculate_share_prices()      AMM pricing (50/50 → dynamic)
│   ├── buy_shares()                  Place a bet, mint shares
│   ├── claim()                       Claim winnings after resolution
│   ├── get_user_stake_details()      User's share breakdown
│   ├── get_market_activity()         Bet history for a market
│   ├── get_market_liquidity()        Locked XLM per market
│   └── get_total_value_locked()      Total XLM in protocol
│
├── admin.rs        — Governance & security
│   ├── emergency_pause/unpause()
│   ├── pause/unpause_market_creation/betting/resolution()
│   ├── set_platform_fee()            Fee in basis points (200 = 2%)
│   ├── set_time_restrictions()       Min/max market duration
│   ├── set_oracle_address()          Price feed contract
│   ├── emergency_close/resolve_market()
│   ├── emergency_withdraw_tokens()
│   └── set_protocol_restrictions()   Min/max bet amounts
│
├── events.rs       — Event emission helpers
│   ├── emit_market_created()
│   ├── emit_market_resolved()
│   ├── emit_wager_placed()
│   ├── emit_winnings_collected()
│   └── emit_fees_collected()
│
└── types.rs        — Shared types & storage keys
    ├── PredictionMarket              Core market struct
    ├── UserStake                     User's share holdings
    ├── BetActivity                   Per-bet record
    ├── MarketCategory                Normal/Politics/Sports/Crypto/...
    ├── MarketStatus                  Active/Locked/Resolved/Closed
    └── DataKey                       All persistent storage keys
```

### How the AMM Works

Kairos uses a constant-product style share pricing model:

```
Initial state (no bets): price_a = price_b = 50%

When users buy shares on outcome A:
  price_a = total_shares_b / (total_shares_a + total_shares_b)
  price_b = total_shares_a / (total_shares_a + total_shares_b)

Payout on correct prediction:
  payout = (user_winning_shares / total_winning_shares) × total_pool
```

**Fee flow:**
1. User sends `amount` XLM to contract
2. `fee = amount × platform_fee_bps / 10_000` is sent to `fee_recipient`
3. `net_amount = amount - fee` mints shares and enters the pool
4. On claim: winner receives proportional share of the entire pool

### Building Contracts

```bash
cd stellar_contracts

# Build WASM
cargo build --target wasm32-unknown-unknown --release

# Run tests
cargo test

# Optimize WASM (for deployment)
stellar contract optimize \
  --wasm target/wasm32-unknown-unknown/release/kairos_prediction_hub.wasm
```

### Deploying to Testnet

```bash
# 1. Create a testnet identity
stellar keys generate --global kairos-deployer --network testnet

# 2. Fund it from friendbot
stellar keys fund kairos-deployer --network testnet

# 3. Deploy the contract
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/kairos_prediction_hub.optimized.wasm \
  --source kairos-deployer \
  --network testnet

# 4. Initialize the contract (replace C... with your deployed contract ID)
stellar contract invoke \
  --id CXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX \
  --source kairos-deployer \
  --network testnet \
  -- initialize \
  --admin $(stellar keys address kairos-deployer) \
  --fee_recipient $(stellar keys address kairos-deployer) \
  --protocol_token CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCN3
```

> The `protocol_token` above is the XLM Stellar Asset Contract (SAC) on testnet.

---

## Indexer

The indexer tracks on-chain contract events and exposes them via a GraphQL API.

```
indexer/kairos/
├── src/         Substreams module (Rust) — streams raw Stellar events
└── subgraph/    The Graph subgraph — transforms events into queryable data
```

**Run the subgraph locally:**
```bash
cd indexer/kairos/subgraph
pnpm install
pnpm codegen
pnpm build
```

---

## API Server

The backend runs on **port 3678** and provides:

- Market data caching (reduces on-chain RPC calls)
- User session management (JWT)
- Notification endpoints
- Off-chain metadata storage

**Base URL:** `http://localhost:3678/api/v1`

```bash
cd server
pnpm dev
```

---

## Wallet Integration

Kairos uses **Freighter** — the official Stellar browser wallet.

Install the extension: [freighter.app](https://freighter.app)

**Connecting in the app:**
1. Install Freighter and create/import a Stellar account
2. Fund your account with testnet XLM via [Stellar Friendbot](https://friendbot.stellar.org)
3. Click **Connect Wallet** in the Kairos app
4. Approve the connection in Freighter

**SDK usage (for contributors):**
```ts
import { requestAccess, signTransaction } from "@stellar/freighter-api";

// Request wallet access
const { publicKey } = await requestAccess();

// Sign a Soroban transaction XDR
const { signedTxXdr } = await signTransaction(xdr, {
  networkPassphrase: "Test SDF Network ; September 2015",
});
```

---

## Market Categories

| ID | Category | Examples |
|---|---|---|
| 0 | Normal | General events, miscellaneous |
| 1 | Politics | Elections, policy outcomes |
| 2 | Sports | Match results, tournament winners |
| 3 | Crypto | BTC price, token launches |
| 4 | Business | Earnings, acquisitions |
| 5 | Entertainment | Awards, box office |
| 6 | Science | Research outcomes, space events |
| 7 | Other | Everything else |

---

## Contributing

We welcome contributions of all kinds. Please read the full guide before starting:

- **[Getting Started](docs/GettingStarted.md)** — setup instructions
- **[Contributing Guidelines](docs/CONTRIBUTING.md)** — branching, PRs, code style

**Quick steps:**
```bash
# 1. Fork the repo and clone your fork
git clone https://github.com/YOUR_USERNAME/kairos.git

# 2. Create a feature branch
git checkout -b feat/your-feature-name

# 3. Make changes and commit
git add .
git commit -m "feat: describe your change"

# 4. Push and open a PR
git push origin feat/your-feature-name
```

**Branch naming:**
- `feat/` — new features
- `fix/` — bug fixes
- `docs/` — documentation only
- `refactor/` — code restructure
- `test/` — tests

**PR rules:**
- Draft PR expected within **48 hours** of being assigned an issue
- All PRs require at least one reviewer approval
- Husky pre-commit hooks enforce linting — don't skip with `--no-verify`

---

## Roadmap

- [x] Prediction market smart contracts (Cairo/Starknet — legacy)
- [x] Migrate contracts to Stellar/Soroban
- [ ] Implement Soroban contract functions (all TODOs)
- [ ] Soroban testnet deployment
- [ ] Freighter wallet integration (replace Starknet wallet)
- [ ] Stellar Horizon + Soroban RPC integration in frontend
- [ ] Indexer migration to Stellar events
- [ ] Oracle integration for crypto price markets
- [ ] Mobile-responsive UI polish
- [ ] Mainnet launch

---

## Community

- **Telegram:** https://t.me/+AJ39q-QOXERmMzU0
- **GitHub Issues:** https://github.com/CasinoHubInc/kairos/issues
- **Pull Requests:** https://github.com/CasinoHubInc/kairos/pulls

---

> Built with ❤️ by the Kairos community. Contributions are what make open source great.
