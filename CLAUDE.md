# Claude Code Instructions

## Project Overview
This is a Rust CLI tool that finds `.env` files in directories while excluding `node_modules`. It's packaged as an npm module with pre-built binaries for multiple platforms.

## Current Status
The project is ready for initial npm publishing. See [docs/todos.md](docs/todos.md) for the complete publishing checklist and current progress.

## Key Files
- `src/main.rs` - Main Rust CLI application using walkdir
- `src/lib.rs` - WASM library version with camelCase JS bindings
- `package.json` - npm package configuration
- `install.js` - Post-install script that downloads pre-built binaries or builds from source
- `.github/workflows/release.yml` - GitHub Actions workflow for multi-platform builds

## Important Commands
```bash
# Build locally
cargo build --release

# Test with cargo
cargo run find .

# Build WASM version (requires wasm-pack)
./build-wasm.sh

# Test npm package locally
npm link
env-cleaner find .
```

## Architecture Decisions
1. **walkdir over glob**: We use walkdir for directory traversal as it provides better control over which directories to skip
2. **Cross-compilation**: Linux ARM64 builds use `cross` tool in GitHub Actions for reliable cross-compilation
3. **Binary distribution**: Pre-built binaries are downloaded from GitHub releases, with fallback to source compilation

## Publishing Process
Refer to [docs/todos.md](docs/todos.md) for the step-by-step publishing checklist.

## Features
- Shows all paths being searched with visual indicators
- Explicitly shows when `node_modules` is skipped
- Provides summary of total paths searched and matches found
- Supports both CLI and WASM usage

## Platform Support
- Linux x64
- Linux ARM64 (for Raspberry Pi, AWS Graviton)
- macOS x64 (Intel)
- macOS ARM64 (Apple Silicon)
- Windows x64