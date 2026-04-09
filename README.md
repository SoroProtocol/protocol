# SoroProtocol — Smart Contracts

Payment streaming protocol built on Stellar's Soroban platform.

## Overview

SoroProtocol enables continuous, real-time token streaming on Stellar.
Create payment streams that release tokens per second — for salaries,
subscriptions, vesting, and grants.

## Contracts

| Contract      | Description                                  |
|---------------|----------------------------------------------|
| `stream`      | Core streaming: create, withdraw, cancel     |
| `vesting`     | Linear + cliff-based token vesting           |
| `distributor` | Batch stream creation (payroll, grants)      |

## Quickstart

```bash
# Install Rust + WASM target
rustup target add wasm32-unknown-unknown

# Build all contracts
make build

# Run tests
make test

# Deploy to testnet
make deploy-testnet
```

## Architecture

See [docs/ARCHITECTURE.md](./docs/ARCHITECTURE.md).

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md). All contributions welcome.

## License

MIT — see [LICENSE](./LICENSE).
