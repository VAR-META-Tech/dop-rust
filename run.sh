#!/bin/bash
set -e  # Stop on error

echo "================================="
echo "Building TypeScript SDK (ts-lib)"
echo "================================="

cd ts-lib || exit 1
npm run build || exit 1
cd ..

echo "================="
echo "Running Rust App"
echo "================="

RUSTFLAGS="-A dead_code" cargo build
RUSTFLAGS="-A dead_code" cargo run
