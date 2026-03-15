#!/usr/bin/env node

const { platform, arch } = process;

// Map Node.js platform/arch to napi-rs target triple
function getTargetTriple() {
    const platformMap = {
        darwin: 'darwin',
        linux: 'linux',
        win32: 'win32',
    };

    const archMap = {
        x64: 'x64',
        arm64: 'arm64',
        ia32: 'ia32',
    };

    const platformName = platformMap[platform];
    const archName = archMap[arch];

    if (!platformName || !archName) {
        throw new Error(`Unsupported platform: ${platform} ${arch}`);
    }

    return `${platformName}-${archName}`;
}

// Load the native module
function loadNativeModule() {
    const target = getTargetTriple();
    const packageName = '@donvadimon/react-intl-auto';

    // Try to load from optional dependencies first
    try {
        const nativePackage = `${packageName}-${target}`;
        return require(nativePackage);
    } catch (e) {
        // Fallback: try loading from local build
        try {
            return require(`./react-intl-auto.${target}.node`);
        } catch (e2) {
            throw new Error(
                `Failed to load native module for ${platform}-${arch}. ` +
                    `Please ensure @donvadimon/react-intl-auto-${target} is installed.`,
            );
        }
    }
}

const native = loadNativeModule();

// Run CLI with arguments from command line
const exitCode = native.runCli(process.argv.slice(2));
process.exit(exitCode);
