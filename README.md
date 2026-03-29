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
- **formatMessage**: Function calls

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

| Option             | Type                          | Default         | Description                                      |
| ------------------ | ----------------------------- | --------------- | ------------------------------------------------ |
| `removePrefix`     | `boolean \| string \| RegExp` | `false`         | Remove prefix from file path when generating IDs |
| `moduleSourceName` | `string`                      | `'react-intl'`  | Module name to detect imports from               |
| `separator`        | `string`                      | `'.'`           | Separator used in generated IDs                  |
| `relativeTo`       | `string`                      | `process.cwd()` | Base path for relative file paths                |
| `hashId`           | `boolean`                     | `false`         | Apply murmur3 hash to generated IDs              |

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

**CLI Options:**

| Option                      | Description                                                           |
| --------------------------- | --------------------------------------------------------------------- |
| `patterns`                  | Glob patterns for source files (e.g., `'src/**/*.{ts,tsx}'`)          |
| `--ignore`                  | Glob patterns to ignore (default: `**/node_modules/**`, `**/.git/**`) |
| `--output`                  | Output file or directory path                                         |
| `--output-mode`             | `aggregated` (single file) or `perfile` (separate files)              |
| `--extract-source-location` | Include source file path in output                                    |
| `--remove-prefix`           | Remove prefix from path (boolean, string, or regex)                   |
| `--module-source-name`      | Module name for react-intl imports (default: `react-intl`)            |
| `--separator`               | Separator for ID generation (default: `.`)                            |
| `--relative-to`             | Base path for relative path calculation                               |
| `--hash-id`                 | Hash message IDs using murmur3                                        |

See [CLI Documentation](docs/CLI.md) for detailed CLI documentation.

### 3. JavaScript API

```javascript
const {
    extractSync,
    extract,
    parseFile,
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
- Node.js 24+

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
4. **Publish** - Manual npm publish via workflow_dispatch

### Supported Platforms

- Linux x64 (gnu)
- macOS x64 (Intel)
- macOS arm64 (Apple Silicon)
- Windows x64 (MSVC)

### Releasing

Publishing is done manually through GitHub Actions:

1. **Bump version** using the version CLI:

```bash
# On master branch - specify version type
npm run version:bump patch    # 0.0.1 -> 0.0.2
npm run version:bump minor    # 0.0.1 -> 0.1.0
npm run version:bump major    # 0.0.1 -> 1.0.0

# On other branches - creates pre-release automatically
npm run version:bump          # 0.0.1 -> 0.0.2-rc.0
```

2. Push the version bump and tag:

```bash
git push origin master
git push origin v0.0.2  # Push the tag created by npm version
```

3. Go to GitHub → Actions → CI → Run workflow
4. Select:
    - Branch: `master`
    - version_type: `patch` / `minor` / `major` / `prerelease`
5. Click "Run workflow"

GitHub Actions will:

- Build for all platforms
- Run all tests
- Publish to npm

**Requirements:**

- `NPM_TOKEN` secret configured in GitHub repository settings

**Note:** All jobs except publish run automatically on push to master. Publish job is manual only.
