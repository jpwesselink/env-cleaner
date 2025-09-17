# env-cleaner npm Package Publishing Checklist

## Prerequisites
- [x] Create Rust CLI with walkdir
- [x] Set up npm package structure (package.json, install.js)
- [x] Configure GitHub Actions workflow for multi-platform builds
- [x] Fix GitHub Actions deprecation warnings
- [x] Remove unused dependencies (glob, rayon, thiserror)

## Publishing Steps

### 1. Test Locally
- [ ] Build the Rust binary: `cargo build --release`
- [ ] Copy binary to bin directory: `mkdir -p bin && cp target/release/env-cleaner bin/`
- [ ] Test with npm link: `npm link`
- [ ] Verify command works: `env-cleaner find .`
- [ ] Unlink after testing: `npm unlink`

### 2. Create GitHub Release
- [ ] Commit all changes: `git add . && git commit -m "Initial release"`
- [ ] Push to main: `git push origin main`
- [ ] Create and push tag: `git tag v0.1.0 && git push --tags`
- [ ] Wait for GitHub Actions to complete building all platform binaries
- [ ] Verify release is created at https://github.com/jpwesselink/env-cleaner/releases

### 3. Publish to npm
- [ ] Check npm package name availability: `npm view env-cleaner`
- [ ] Login to npm: `npm login`
- [ ] Publish package: `npm publish`
- [ ] Test installation: `npm install -g env-cleaner`

## Post-Publishing
- [ ] Update README with installation instructions
- [ ] Test on different platforms (Linux, macOS, Windows)
- [ ] Consider adding more features (delete .env files, backup, etc.)

## Commands Summary
```bash
# Build locally
cargo build --release
mkdir -p bin && cp target/release/env-cleaner bin/

# Test locally
npm link
env-cleaner find .
npm unlink

# Release
git add . && git commit -m "Initial release"
git push origin main
git tag v0.1.0 && git push --tags

# Publish
npm login
npm publish

# Test published package
npm install -g env-cleaner
env-cleaner find .
```