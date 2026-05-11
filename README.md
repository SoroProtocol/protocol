# SoroProtocol — Smart Contracts

> Payment streaming contracts built on Stellar's Soroban platform.

SoroProtocol enables continuous, per-second token streaming on Stellar. Tokens are deposited into a smart contract escrow at stream creation and released linearly over time — making it suitable for salaries, subscriptions, grants, and vesting schedules.

---

## Table of Contents

- [Overview](#overview)
- [Contracts](#contracts)
- [Prerequisites](#prerequisites)
- [Quickstart](#quickstart)
- [Make Targets](#make-targets)
- [Architecture](#architecture)
- [Security Model](#security-model)
- [Contributing](#contributing)
- [License](#license)

---

## Overview

All contracts are written in Rust and compile to WASM for deployment on Soroban. They share no external dependencies beyond `soroban-sdk` and the Stellar token interface.

---

## Contracts

| Contract | Description |
|----------|-------------|
| `stream` | Core streaming contract — create, withdraw, and cancel payment streams |
| `vesting` | Linear vesting with optional cliff — tokens unlock from `cliff_time` to `end_time` |
| `distributor` | Batch stream creation in a single transaction — designed for payroll and grant distribution |

---

## Prerequisites

- [Rust](https://rustup.rs) (stable toolchain — see `rust-toolchain.toml`)
- `wasm32-unknown-unknown` target

```bash
rustup target add wasm32-unknown-unknown
```

---

## Quickstart

```bash
# Build all contracts
make build

# Run the full test suite
make test

# Format source files
make fmt

# Lint with Clippy (warnings treated as errors)
make lint

# Deploy to Stellar Testnet
make deploy-testnet
```

---

## Make Targets

| Target | Description |
|--------|-------------|
| `build` | Compile all contracts to WASM (release) |
| `test` | Run all unit and integration tests |
| `fmt` | Format all Rust source files with `rustfmt` |
| `lint` | Run `clippy` with `-D warnings` |
| `clean` | Remove build artifacts |
| `deploy-testnet` | Deploy contracts to Stellar Testnet via `scripts/deploy.sh` |
| `deploy-mainnet` | Deploy contracts to Stellar Mainnet (prompts for confirmation) |

---

## Architecture

See [docs/ARCHITECTURE.md](./docs/ARCHITECTURE.md) for a full breakdown of contract design, storage layout, and event schema.

**High-level summary:**

- **Stream contract** — escrows the full deposit upfront (`rate × duration`). Withdrawal is pull-based; recipients call `withdraw()` at any time to collect accrued tokens. Cancellation is atomic: the recipient receives the accrued portion and the sender receives the remainder.
- **Vesting contract** — no tokens flow before `cliff_time`. After the cliff, tokens vest linearly through `end_time`. Supports revocation by the funder.
- **Distributor contract** — wraps the stream contract to create multiple streams in one transaction, reducing ledger overhead for bulk operations.

---

## Security Model

- All state-mutating functions require `require_auth()` from the relevant party (sender, recipient, or funder).
- Token transfers use the Stellar token interface — no custom token logic.
- Integer overflow is avoided by using `i128` for all token amounts.
- Contracts are stateless beyond their own storage; no cross-contract calls between protocol contracts.

---

## Contributing

Contributions are welcome. Please read [CONTRIBUTING.md](./CONTRIBUTING.md) for branch conventions, commit message guidelines, and the pull request process.

---

## License

MIT — see [LICENSE](./LICENSE).
