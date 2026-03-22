# @donvadimon/react-intl-auto

Hybrid Rust/JavaScript solution for React Intl - SWC plugin and CLI tool for automatic ID management. Ported from the popular Babel plugin [babel-plugin-react-intl-auto](https://github.com/akameco/babel-plugin-react-intl-auto).

## Features

This package provides three ways to work with React Intl:

1. **SWC Plugin** - Transform your code at build time
2. **CLI Tool** - Extract messages from source files
3. **JavaScript API** - Programmatic access to extraction

Automatically adds `id` attributes to:

- **JSX Elements**: `FormattedMessage` and `FormattedHTMLMessage` components
- **defineMessages**: Object literal messages
- **formatMessage**: Function calls

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

### 2. CLI Tool

Extract messages from your source files:

```bash
# Extract to single file
npx react-intl-auto extract 'src/**/*.{ts,tsx}' --output-mode=aggregated --output=./messages.json

# Extract to separate files
npx react-intl-auto extract 'src/**/*.{ts,tsx}' --output-mode=perfile --output=./locales

# With options
npx react-intl-auto extract 'src/**/*.ts' \
  --remove-prefix='src/' \
  --separator='.' \
  --extract-source-location
```

### 3. JavaScript API

```javascript
const {
    extractSync,
    parseFile,
} = require('@donvadimon/react-intl-auto/extract');

// Extract from multiple files
const result = extractSync(['src/**/*.ts'], {
    removePrefix: 'src/',
    separator: '.',
    extractSourceLocation: true,
});
console.log(result.messages);

// Parse single file
const messages = parseFile('src/components/App.tsx', {
    removePrefix: 'src/',
});
```

## Options

### Plugin Options

| Option             | Type                          | Default         | Description                           |
| ------------------ | ----------------------------- | --------------- | ------------------------------------- |
| `removePrefix`     | `boolean \| string \| RegExp` | `false`         | Remove prefix from generated IDs      |
| `moduleSourceName` | `string`                      | `'react-intl'`  | Module name to detect imports         |
| `separator`        | `string`                      | `'.'`           | Separator for ID parts                |
| `relativeTo`       | `string`                      | `process.cwd()` | Base path for relative file paths     |
| `hashId`           | `boolean`                     | `false`         | Apply hash function to id             |
| `hashAlgorithm`    | `string`                      | `'murmur3'`     | Hash algorithm: `murmur3` or `base64` |

### CLI Options

| Option                      | Description                                              |
| --------------------------- | -------------------------------------------------------- |
| `--output-mode`             | `aggregated` (single file) or `perfile` (separate files) |
| `--output`                  | Output directory or file path                            |
| `--extract-source-location` | Include file path and line number in output              |
| `--remove-prefix`           | Remove prefix from IDs (same as plugin option)           |
| `--separator`               | Separator for ID parts                                   |

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
    id="components.App.Hello World"
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

### formatMessage

**Input:**

```javascript
intl.formatMessage({
    defaultMessage: 'Hello World',
});
```

**Output:**

```javascript
intl.formatMessage({
    id: 'components.App.Hello World',
    defaultMessage: 'Hello World',
});
```

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
- Node.js 18+

### Building

```bash
# Install dependencies
npm install

# Build SWC plugin (WASM)
npm run build:plugin

# Build CLI (napi-rs native addon)
npm run build:napi

# Build both
npm run build:cli
```

### Testing

```bash
# Full test cycle (recommended)
npm run test:full       # build + Rust tests + Jest tests

# Individual test commands
cargo test              # Rust unit tests
npm test                # Jest integration tests
npm run test:watch      # Jest in watch mode
```

## CI/CD

GitHub Actions workflow (`.github/workflows/napi-rs.yml`) handles:

1. **Lint** - Rust formatting and clippy
2. **Build** - WASM plugin and napi-rs addons for multiple platforms
3. **Test** - Rust tests, Jest tests, native binding tests
4. **Publish** - Automatic npm publish on version tags

### Supported Platforms

- Linux x64 (gnu)
- macOS x64 (Intel)
- macOS arm64 (Apple Silicon)

### Releasing

1. Update version in `package.json` and `Cargo.toml`
2. Create and push a tag:

```bash
git tag -a v1.0.0 -m "Release v1.0.0"
git push origin v1.0.0
```

GitHub Actions will automatically:

- Build for all platforms
- Run all tests
- Publish to npm

**Requirements:**

- `NPM_TOKEN` secret configured in GitHub repository settings

## Breaking Changes

### v1.0.0

Removed options (no longer needed):

- `filebase` - Use `removePrefix` instead
- `useKey` - Keys are now used automatically in defineMessages

## License

MIT
