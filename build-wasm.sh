#!/bin/bash

# Install wasm-pack if not installed
if ! command -v wasm-pack &> /dev/null; then
    echo "Installing wasm-pack..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

# Build the WASM module
wasm-pack build --target nodejs --out-dir pkg

# Create a wrapper package.json for the WASM module
cat > pkg/package.json << EOF
{
  "name": "env-cleaner-wasm",
  "version": "0.1.0",
  "description": "Find and clean .env files (WebAssembly version)",
  "main": "env_cleaner.js",
  "types": "env_cleaner.d.ts",
  "files": [
    "env_cleaner_bg.wasm",
    "env_cleaner.js",
    "env_cleaner.d.ts"
  ]
}
EOF