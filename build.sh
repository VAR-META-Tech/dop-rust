#!/bin/bash

set -e  # Exit immediately on error

echo "📦 Installing Node dependencies..."
cd ts-lib

if [ ! -d "node_modules" ]; then
  npm install
fi

echo "🔨 Building TypeScript..."
npm run build

if [ ! -f "dist/index.js" ]; then
  echo "❌ Build failed: dist/index.js not found."
  exit 1
fi

echo "✅ Build complete: ts-lib/dist/index.js is ready."
