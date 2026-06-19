# Contributing to Kairos

Thank you for your interest in contributing to Kairos!
We welcome contributions of all kinds — bug fixes, new features, documentation, and tests.

---

## Table of Contents
1. [Before You Start](#before-you-start)
2. [Setting Up the Dev Environment](#setting-up-the-dev-environment)
3. [Working on Soroban Smart Contracts](#working-on-soroban-smart-contracts)
4. [Making Changes](#making-changes)
5. [Submitting a Pull Request](#submitting-a-pull-request)
6. [Reporting Issues](#reporting-issues)

---

## Before You Start

Read [GettingStarted.md](./GettingStarted.md) and make sure your local environment is running before writing any code.

We use **pnpm workspaces** for package management and **Husky** for git hook enforcement.

---

## Setting Up the Dev Environment

See [GettingStarted.md](./GettingStarted.md) for the full walkthrough. The short version:

```bash
git clone https://github.com/YOUR_USERNAME/kairos.git
cd kairos
pnpm install
pnpm dev:client   # or dev:landing / dev:server
```

---

## Working on Soroban Smart Contracts

Kairos contracts are written in **Rust** using the [Soroban SDK](https://soroban.stellar.org) and live in `stellar_contracts/`.

### Setup
```bash
rustup target add wasm32-unknown-unknown
cargo install --locked stellar-cli --features opt
```

### Build
```bash
cd stellar_contracts
cargo build --target wasm32-unknown-unknown --release
```

### Test
```bash
cargo test
```

### Module overview

| File | Responsibility |
|---|---|
| `lib.rs` | Public contract API, delegates to modules |
| `market.rs` | Market creation, resolution, queries |
| `betting.rs` | AMM pricing, share purchases, claim payouts |
| `admin.rs` | Pause controls, fees, oracle, emergency ops |
| `events.rs` | Contract event emission helpers |
| `types.rs` | All `#[contracttype]` structs, enums, storage keys |

Functions marked `todo!()` are open for contribution — pick one, implement it, write a test, open a PR.

---

## Making Changes

1. **Create a branch** — always branch off `main`
```bash
git checkout -b feat/your-feature-name
```

Branch prefixes: `feat/`, `fix/`, `docs/`, `refactor/`, `test/`

2. **Write your code** — match the style of surrounding code, no unnecessary abstractions.

3. **Run tests**
```bash
pnpm test           # frontend/server tests
cargo test          # contract tests (from stellar_contracts/)
```

4. **Commit**
```bash
git add .
git commit -m "feat: describe your change"
```

Husky runs linting on commit — do not skip hooks with `--no-verify`.

---

## Submitting a Pull Request

```bash
git push origin feat/your-feature-name
```

Open a PR at [github.com/CasinoHubInc/kairos/pulls](https://github.com/CasinoHubInc/kairos/pulls):

- Write a clear title and description
- Reference the issue you're closing (`Closes #42`)
- A **draft PR** is expected within **48 hours** of being assigned — even if incomplete
- Wait for at least one maintainer approval before merging

---

## Reporting Issues

Open an issue at [github.com/CasinoHubInc/kairos/issues](https://github.com/CasinoHubInc/kairos/issues).

Include:
- A clear description of the bug or feature request
- Why the change is needed
- Steps to reproduce (for bugs)
- Your estimated ETA if you plan to fix it yourself

Keep descriptions concise and specific.

---

Thank you for contributing to Kairos!
