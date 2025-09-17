#!/usr/bin/env node

const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');
const https = require('https');

const BINARY_NAME = 'env-cleaner';
const VERSION = require('./package.json').version;

function getPlatform() {
  const type = process.platform;
  const arch = process.arch;

  if (type === 'darwin') {
    return arch === 'arm64' ? 'darwin-arm64' : 'darwin-x64';
  }
  if (type === 'win32') {
    return 'windows-x64';
  }
  if (type === 'linux') {
    return arch === 'arm64' ? 'linux-arm64' : 'linux-x64';
  }
  
  throw new Error(`Unsupported platform: ${type} ${arch}`);
}

function downloadBinary() {
  const platform = getPlatform();
  const binaryUrl = `https://github.com/jpwesselink/env-cleaner/releases/download/v${VERSION}/${BINARY_NAME}-${platform}`;
  const binPath = path.join(__dirname, 'bin');
  const binaryPath = path.join(binPath, BINARY_NAME);

  if (!fs.existsSync(binPath)) {
    fs.mkdirSync(binPath, { recursive: true });
  }

  console.log(`Downloading ${BINARY_NAME} for ${platform}...`);

  const file = fs.createWriteStream(binaryPath);
  
  https.get(binaryUrl, (response) => {
    response.pipe(file);
    file.on('finish', () => {
      file.close();
      fs.chmodSync(binaryPath, '755');
      console.log(`${BINARY_NAME} installed successfully!`);
    });
  }).on('error', (err) => {
    fs.unlink(binaryPath, () => {});
    console.error('Download failed:', err.message);
    console.log('Attempting to build from source...');
    buildFromSource();
  });
}

function buildFromSource() {
  try {
    console.log('Building from source with Cargo...');
    execSync('cargo --version', { stdio: 'ignore' });
    execSync('cargo build --release', { stdio: 'inherit' });
    
    const binPath = path.join(__dirname, 'bin');
    if (!fs.existsSync(binPath)) {
      fs.mkdirSync(binPath, { recursive: true });
    }
    
    fs.copyFileSync(
      path.join(__dirname, 'target', 'release', BINARY_NAME),
      path.join(binPath, BINARY_NAME)
    );
    fs.chmodSync(path.join(binPath, BINARY_NAME), '755');
    console.log('Built from source successfully!');
  } catch (e) {
    console.error('Failed to build from source. Please install Rust: https://rustup.rs/');
    process.exit(1);
  }
}

// Try downloading pre-built binary first, fall back to building from source
downloadBinary();