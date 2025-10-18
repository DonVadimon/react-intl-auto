const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

console.log('Building SWC React Intl Auto Plugin...');

try {
  // Build the plugin
  execSync('cargo build --release --target wasm32-wasip1', { stdio: 'inherit' });
  
  // Copy the built plugin to the root directory
  const sourcePath = path.join(__dirname, 'target', 'wasm32-wasip1', 'release', 'swc_plugin_react_intl_auto.wasm');
  const destPath = path.join(__dirname, 'swc-plugin-react-intl-auto.wasm');
  
  if (fs.existsSync(sourcePath)) {
    fs.copyFileSync(sourcePath, destPath);
    console.log('✅ Plugin built successfully and copied to swc-plugin.wasm');
  } else {
    console.error('❌ Plugin file not found at expected location');
    process.exit(1);
  }
} catch (error) {
  console.error('❌ Build failed:', error.message);
  process.exit(1);
}
