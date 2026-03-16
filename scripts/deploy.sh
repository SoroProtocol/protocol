#!/usr/bin/env bash
set -euo pipefail

NETWORK=${1:-testnet}
echo "==> Building contracts for $NETWORK..."
cargo build --target wasm32-unknown-unknown --release

echo "==> Deploying stream contract..."
STREAM_ID=$(soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/soroprotocol_stream.wasm \
  --network "$NETWORK" --source default)

echo "==> Deploying vesting contract..."
VESTING_ID=$(soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/soroprotocol_vesting.wasm \
  --network "$NETWORK" --source default)

echo "==> Deploying distributor contract..."
DIST_ID=$(soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/soroprotocol_distributor.wasm \
  --network "$NETWORK" --source default)

echo ""
echo "Stream contract:      $STREAM_ID"
echo "Vesting contract:     $VESTING_ID"
echo "Distributor contract: $DIST_ID"
echo "Done."
