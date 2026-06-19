# Getting Started with Kairos

Welcome to Kairos! This guide walks you through setting up the project locally.

---

## Prerequisites

| Tool | Version | Install |
|---|---|---|
| Node.js | ≥ 18 | https://nodejs.org |
| pnpm | ≥ 8 | `npm install -g pnpm` |
| Git | any | https://git-scm.com |
| Rust | stable | https://rustup.rs |
| Stellar CLI | latest | `cargo install --locked stellar-cli --features opt` |
| VS Code | any | https://code.visualstudio.com (recommended) |

After installing Rust, add the Soroban WASM target:
```bash
rustup target add wasm32-unknown-unknown
```

---

## Step 1: Fork & Clone

1. Click **Fork** at the top of the [Kairos repository](https://github.com/CasinoHubInc/kairos).

2. Clone your fork:
```bash
git clone https://github.com/YOUR_USERNAME/kairos.git
cd kairos
```

---

## Step 2: Install Dependencies

```bash
pnpm install
```

This installs dependencies for all workspace packages (`client`, `landing_page`, `server`, `indexer`).

---

## Step 3: Set Up Environment Variables

Copy the example env files:
```bash
cp server/.env.example server/.env
cp client/.env.example client/.env.local
```

Fill in the values — see the [README](../README.md#2-environment-variables) for the full list.

---

## Step 4: Start the Development Servers

```bash
# Landing page — http://localhost:3000
pnpm dev:landing

# Client app — http://localhost:3001
pnpm dev:client

# API server — http://localhost:3678
pnpm dev:server
```

---

## Step 5: Set Up Stellar Smart Contracts

Contracts live in `stellar_contracts/` and are written in Rust using the Soroban SDK.

### Build
```bash
cd stellar_contracts
cargo build --target wasm32-unknown-unknown --release
```

### Test
```bash
cargo test
```

### Deploy to testnet (optional)
```bash
stellar keys generate --global kairos-deployer --network testnet
stellar keys fund kairos-deployer --network testnet
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/kairos_prediction_hub.wasm \
  --source kairos-deployer \
  --network testnet
```

See the [README → Smart Contracts](../README.md#smart-contracts-soroban) for full deployment details.

---

## Step 6: Install Freighter Wallet

1. Install [Freighter](https://freighter.app) browser extension
2. Create or import a Stellar account
3. Switch to **Testnet** in Freighter settings
4. Fund via [friendbot.stellar.org](https://friendbot.stellar.org)

---

## Step 7: Making Changes

```bash
# Create a branch
git checkout -b feat/your-feature-name

# Commit
git add .
git commit -m "feat: describe your change"
```

Husky pre-commit hooks run linting automatically — do not skip with `--no-verify`.

---

## Step 8: Push & Open a Pull Request

```bash
git push origin feat/your-feature-name
```

Open a PR at [github.com/CasinoHubInc/kairos/pulls](https://github.com/CasinoHubInc/kairos/pulls).
A draft PR is expected within **48 hours** of being assigned an issue.

---

## Need Help?

- Telegram: https://t.me/+AJ39q-QOXERmMzU0
- GitHub Issues: https://github.com/CasinoHubInc/kairos/issues

Happy coding!

