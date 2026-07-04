# @donvadimon/react-intl-auto

Hybrid Rust/JavaScript solution for React Intl - SWC plugin and CLI tool for automatic ID management.

Ported from the popular Babel plugin [babel-plugin-react-intl-auto](https://github.com/akameco/babel-plugin-react-intl-auto).

Based on [swc-plugin-react-intl-auto](https://github.com/lcl9288/swc-plugin-react-intl-auto) by lcl9288.

## Features

This package provides three ways to work with React Intl:

1. **SWC Plugin** - Transform your code at build time
2. **CLI Tool** - Extract messages from source files
3. **JavaScript API** - Programmatic access to extraction

Automatically adds `id` attributes to:

- **JSX Elements**: `FormattedMessage` and `FormattedHTMLMessage` components
- **defineMessages**: Object literal messages
- **formatMessage**: Function calls (including `intl.formatMessage` via `injectIntl`)

## Documentation

- [CLI Documentation](docs/CLI.md) - Detailed CLI reference
- [JS API Documentation](docs/JS_API.md) - JavaScript API reference
- [Migration Guide](docs/MIGRATION.md) - Migrating from babel-plugin-react-intl-auto

## Installation

```bash
npm install -D @donvadimon/react-intl-auto
```

## Usage

### 1. SWC Plugin

Add to your `.swcrc` or SWC configuration:

```json
{
    "jsc": {
        "experimental": {
            "plugins": [
                [
                    "@donvadimon/react-intl-auto/swc-plugin",
                    {
                        "removePrefix": "src/",
                        "separator": "."
                    }
                ]
            ]
        }
    }
}
```

Or use programmatically:

```javascript
const { transform } = require('@swc/core');
const pluginPath = require('@donvadimon/react-intl-auto/swc-plugin');

const result = await transform(code, {
    filename: 'example.js',
    jsc: {
        experimental: {
            plugins: [[pluginPath, {}]],
        },
    },
});
```

**Plugin Options:**

| Option             | Type                  | Default                           | Description                                                                 |
| ------------------ | --------------------- | --------------------------------- | --------------------------------------------------------------------------- |
| `removePrefix`     | `boolean \| string`   | `undefined`                       | Remove prefix from file path when generating IDs (see below for regex)     |
| `moduleSourceName` | `string`              | `'react-intl'`                    | Module name to detect imports from                                          |
| `separator`        | `string`              | `'.'`                             | Separator used in generated IDs                                             |
| `relativeTo`       | `string`              | auto-detected project root        | Base path for relative file paths                                           |
| `hashId`           | `boolean`             | `false`                           | Apply murmur3 hash to generated IDs                                        |
| `hashAlgorithm`    | `string`              | `'murmur3'`                       | Hash algorithm (only `'murmur3'` is supported)                              |

**`removePrefix` values:**

- `undefined` / not set - use full file path relative to project root
- `true` - strip the entire path prefix, return only the message key/descriptor
- `"src/"` - remove a specific prefix string
- `"^src/components/"` - string containing regex patterns (`.*`, `.+`, `[`, `(`) is treated as regex

### 2. CLI Tool

Extract messages from your source files:

```bash
# Extract to single file
npx @donvadimon/react-intl-auto 'src/**/*.{ts,tsx}' --output-mode=aggregated --output=./messages.json

# Extract to separate files
npx @donvadimon/react-intl-auto 'src/**/*.{ts,tsx}' --output-mode=perfile --output=./locales

# With options
npx @donvadimon/react-intl-auto 'src/**/*.ts' \
  --remove-prefix='src/' \
  --separator='.' \
  --extract-source-location
```

**CLI Options:**

| Option                      | Description                                                           |
| --------------------------- | --------------------------------------------------------------------- |
| `patterns`                  | Glob patterns for source files (e.g., `'src/**/*.{ts,tsx}'`)          |
| `--ignore`                  | Glob patterns to ignore (default: `**/node_modules/**`, `**/.git/**`) |
| `-o, --output`              | Output file or directory path                                         |
| `--output-mode`             | `aggregated` (single file) or `perfile` (separate files)              |
| `--extract-source-location` | Include source file path in output                                    |
| `--remove-prefix`           | Remove prefix from path (`true`, `false`, or string/regex pattern)    |
| `--module-source-name`      | Module name for react-intl imports (default: `react-intl`)            |
| `--separator`               | Separator for ID generation (default: `.`)                            |
| `--relative-to`             | Base path for relative path calculation                               |
| `--hash-id`                 | Hash message IDs using murmur3                                        |
| `--hash-algorithm`          | Hash algorithm (only `murmur3` is supported, default: `murmur3`)      |

See [CLI Documentation](docs/CLI.md) for detailed CLI documentation.

### 3. JavaScript API

```javascript
const {
    extractSync,
    extract,
    parseFile,
    runCli,
} = require('@donvadimon/react-intl-auto/extract');

// Extract from multiple files (sync)
const result = extractSync(['src/**/*.ts'], {
    removePrefix: 'src/',
    separator: '.',
    extractSourceLocation: true,
});
console.log(result.messages); // Array of messages
console.log(result.filesProcessed); // Number of files processed

// Extract from multiple files (async)
const result = await extract(['src/**/*.ts'], {
    removePrefix: 'src/',
    hashId: true,
});

// Parse single file
const messages = parseFile('src/components/App.tsx', {
    removePrefix: 'src/',
});

// Run CLI programmatically
const exitCode = runCli(['node', 'src/**/*.ts', '--output', 'messages.json']);
```

See [JS API Documentation](docs/JS_API.md) for detailed JS API documentation.

## Examples

### JSX Elements

**Input:**

```jsx
import { FormattedMessage } from 'react-intl';

<FormattedMessage defaultMessage="Hello World" />;
```

**Output:**

```jsx
<FormattedMessage
    id="components.App.aG1FCg=="
    defaultMessage="Hello World"
/>
```

### defineMessages

**Input:**

```javascript
import { defineMessages } from 'react-intl';

export const messages = defineMessages({
    hello: 'Hello World',
    goodbye: 'Goodbye World',
});
```

**Output:**

```javascript
export const messages = defineMessages({
    hello: { id: 'components.messages.hello', defaultMessage: 'Hello World' },
    goodbye: {
        id: 'components.messages.goodbye',
        defaultMessage: 'Goodbye World',
    },
});
```

### formatMessage (via injectIntl)

**Input:**

```javascript
import { injectIntl } from 'react-intl';

function App({ intl }) {
    return intl.formatMessage({ defaultMessage: 'Hello World' });
}

export default injectIntl(App);
```

**Output:**

```javascript
function App({ intl }) {
    return intl.formatMessage({
        id: 'components.App.aG1FCg==',
        defaultMessage: 'Hello World',
    });
}
```

### formatMessage (direct import)

**Input:**

```javascript
import { formatMessage } from 'react-intl';

formatMessage({ defaultMessage: 'Hello World' });
```

**Output:**

```javascript
formatMessage({
    id: 'components.App.aG1FCg==',
    defaultMessage: 'Hello World',
});
```

## ID Generation

IDs are generated based on the file path and message content:

- **defineMessages**: `<filepath>.<key>` (e.g., `components.App.hello`)
- **FormattedMessage / formatMessage**: `<filepath>.<murmur3(defaultMessage)>` (e.g., `components.App.aG1FCg==`)

When `hashId: true` is set, the entire ID (including path) is hashed with murmur3.

## Development

This project is written in Rust and uses the SWC plugin API.

### Project Structure

```
crates/
├── react-intl-core/    # Shared Rust library (ID generation, AST traversal)
├── swc-plugin/         # SWC Plugin (WASM target)
└── cli/                # CLI tool with napi-rs (native addon)
```

### Prerequisites

- Rust toolchain
- `wasm32-wasip1` target: `rustup target add wasm32-wasip1`
- Node.js 20+

### Building

```bash
# Install dependencies
npm install

# Build SWC plugin (WASM)
npm run build:plugin

# Build CLI/JS API (napi-rs native addon)
npm run build:napi

# Build CLI (alias for build:napi)
npm run build:cli
```

### Testing

```bash
# Full test cycle (recommended)
npm run test:all       # build + Rust tests + Jest tests

# Individual test commands
cargo test              # Rust unit tests
npm test                # Jest integration tests
npm run test:jest       # Jest in watch mode
```

## Supported Platforms

The native addon (CLI/JS API) is built for:

| Platform              | Architecture |
| --------------------- | ------------ |
| Windows (MSVC)        | x64          |
| Linux (GNU)           | x64, arm64   |
| Linux (musl)          | x64, arm64   |
| macOS (Intel)         | x64          |
| macOS (Apple Silicon) | arm64        |

Additional platforms supported via WASI fallback.

The SWC plugin (WASM) works on any platform supported by `@swc/core`.

## Releasing

Publishing is done manually through GitHub Actions:

1. **Bump version** using the version CLI:

```bash
npm run version:patch    # 0.0.1 -> 0.0.2
npm run version:minor    # 0.0.1 -> 0.1.0
npm run version:major    # 0.0.1 -> 1.0.0
npm run version:bump     # 0.0.1 -> 0.0.2-rc.0 (pre-release)
```

2. Push the version bump and tag:

```bash
git push origin master
```

3. Go to GitHub → Actions → CI → Run workflow

**Requirements:**

- `NPM_TOKEN` secret configured in GitHub repository settings
