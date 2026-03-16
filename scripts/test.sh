#!/usr/bin/env bash
set -euo pipefail
echo "Running all contract tests..."
cargo test --all -- --nocapture "$@"
