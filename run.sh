#!/bin/bash
set -e

echo "Build TypeScript"
cd ts-lib
npm run build
cd ..

echo "Run Rust"
cargo run
