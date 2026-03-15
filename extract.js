const { platform, arch } = process;

// Map Node.js platform/arch to napi-rs target triple
function getTargetTriple() {
    const platformMap = {
        'darwin': 'darwin',
        'linux': 'linux',
        'win32': 'win32'
    };

    const archMap = {
        'x64': 'x64',
        'arm64': 'arm64',
        'ia32': 'ia32'
    };

    const platformName = platformMap[platform];
    const archName = archMap[arch];

    if (!platformName || !archName) {
        throw new Error(`Unsupported platform: ${platform} ${arch}`);
    }

    // napi-rs naming convention: {packageName}.{platform}-{arch}.node
    // For scoped packages: @scope/package-name.darwin-arm64.node
    return `${platformName}-${archName}`;
}

// Load the native module
function loadNativeModule() {
    const target = getTargetTriple();
    const packageName = '@donvadimon/react-intl-auto';
    
    // Try to load from optional dependencies first
    try {
        // Standard napi-rs pattern: @scope/package-name.platform-arch.node
        const nativePackage = `${packageName}-${target}`;
        return require(nativePackage);
    } catch (e) {
        // Fallback: try loading from local build
        try {
            return require(`./react-intl-auto.${target}.node`);
        } catch (e2) {
            throw new Error(
                `Failed to load native module for ${platform}-${arch}. ` +
                `Please ensure @donvadimon/react-intl-auto-${target} is installed.`
            );
        }
    }
}

const native = loadNativeModule();

/**
 * Extract messages from files matching glob patterns (async)
 * @param {string[]} patterns - Glob patterns for source files (e.g., `['src/*.{ts,tsx}']`)
 * @param {ExtractOptions} [options] - Extraction options
 * @returns {Promise<ExtractResult>} - Extraction result
 */
function extract(patterns, options) {
    return native.extract(patterns, options);
}

/**
 * Extract messages from files matching glob patterns (sync)
 * @param {string[]} patterns - Glob patterns for source files (e.g., `['src/*.{ts,tsx}']`)
 * @param {ExtractOptions} [options] - Extraction options
 * @returns {ExtractResult} - Extraction result
 */
function extractSync(patterns, options) {
    return native.extractSync(patterns, options);
}

/**
 * Parse a single file and extract messages
 * @param {string} filePath - Path to the file to parse
 * @param {ExtractOptions} [options] - Extraction options
 * @returns {ExtractedMessage[]} - Array of extracted messages
 */
function parseFile(filePath, options) {
    return native.parseFile(filePath, options);
}

module.exports = {
    extract,
    extractSync,
    parseFile
};
