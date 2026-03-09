const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

console.log('Building SWC React Intl Auto Plugin...');

try {
    // Build the plugin
    execSync('cargo build --release --target wasm32-wasip1', {
        stdio: 'inherit',
    });

    const ROOT = path.resolve(__dirname, '..');

    // Copy the built plugin to the root directory
    const sourcePath = path.join(
        ROOT,
        'target',
        'wasm32-wasip1',
        'release',
        'swc_plugin.wasm',
    );
    const destPath = path.join(ROOT, 'swc-plugin-react-intl-auto-fs.wasm');

    if (fs.existsSync(sourcePath)) {
        fs.copyFileSync(sourcePath, destPath);
        console.log(
            '✅ Plugin built successfully and copied to swc-plugin-react-intl-auto-fs.wasm',
        );
    } else {
        console.error('❌ Plugin file not found at expected location');
        process.exit(1);
    }
} catch (error) {
    console.error('❌ Build failed:', error.message);
    process.exit(1);
}
