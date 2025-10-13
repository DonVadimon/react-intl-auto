const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

console.log('Building SWC plugin...');

try {
  // Build the Rust plugin to WASM
  console.log('Building Rust plugin to WASM...');
  execSync('cargo build --release --target wasm32-wasi', { 
    stdio: 'inherit',
    cwd: __dirname 
  });

  // Copy the WASM file to the package root
  const wasmPath = path.join(__dirname, 'target', 'wasm32-wasi', 'release', 'swc_plugin_react_intl_auto.wasm');
  const destPath = path.join(__dirname, 'swc_plugin_react_intl_auto.wasm');
  
  if (fs.existsSync(wasmPath)) {
    fs.copyFileSync(wasmPath, destPath);
    console.log('WASM file copied successfully');
  } else {
    console.error('WASM file not found at:', wasmPath);
    process.exit(1);
  }

  console.log('Build completed successfully!');
} catch (error) {
  console.error('Build failed:', error.message);
  process.exit(1);
}
