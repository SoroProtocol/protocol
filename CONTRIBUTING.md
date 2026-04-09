# Contributing to SoroProtocol

## Setup

1. Install Rust via [rustup.rs](https://rustup.rs)
2. Add WASM target: `rustup target add wasm32-unknown-unknown`
3. Install Soroban CLI: `cargo install --locked soroban-cli`
4. Run `make test` to confirm your setup works

## Workflow

1. Find or open an issue
2. Branch from `main`: `git checkout -b feat/your-feature`
3. Write tests for any new contract logic
4. Ensure `make lint` and `make fmt` pass before opening a PR
5. Open a PR with a clear description linking the issue

## Code Standards

- All public contract functions need rustdoc comments
- Tests must cover happy path, edge cases, and error conditions
- Never use `unwrap()` in production contract code
