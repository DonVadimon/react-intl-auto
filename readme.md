# SWC React Intl Auto Plugin

A SWC plugin that automatically adds IDs to React Intl components and function calls, ported from the popular Babel plugin [babel-plugin-react-intl-auto](https://github.com/akameco/babel-plugin-react-intl-auto).

## Features

This plugin automatically adds `id` attributes/properties to:

1. **JSX Elements**: `FormattedMessage` and `FormattedHTMLMessage` components
2. **defineMessages**: `defineMessages` function calls
3. **formatMessage**: `intl.formatMessage` function calls

## Installation

```bash
npm install swc-plugin-react-intl-auto-fs
```

## Usage

### Basic Usage

```javascript
const { transform } = require('@swc/core');
const plugin = require('swc-plugin-react-intl-auto-fs');

const result = await transform(code, {
  filename: 'example.js',
  plugins: [
    [plugin.getPluginPath(), plugin.getDefaultOptions()]
  ]
});
```

### With Custom Options

```javascript
const { transform } = require('@swc/core');
const plugin = require('swc-plugin-react-intl-auto-fs');

const result = await transform(code, {
  filename: 'example.js',
  plugins: [
    [plugin.getPluginPath(), {
      removePrefix: false,
      filebase: false,
      includeExportName: false,
      useKey: false,
      moduleSourceName: 'react-intl',
      separator: '.',
      relativeTo: process.cwd()
    }]
  ]
});
```

## Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `removePrefix` | `boolean \| string \| RegExp` | `false` | Remove prefix from generated IDs |
| `filebase` | `boolean` | `false` | Use file basename instead of directory path |
| `includeExportName` | `boolean \| 'all'` | `true` | Include export name in ID |
| `useKey` | `boolean` | `false` | Use key attribute instead of message hash |
| `moduleSourceName` | `string` | `'react-intl'` | Module name to detect imports |
| `separator` | `string` | `'.'` | Separator for ID parts |
| `relativeTo` | `string` | `undefined` | Base path for relative file paths |
| `hashId` | `boolean` | `undefined` | Apply hash fn to id |
| `hashAlgorithm` | `murmur3 / base64` | `murmur3` | Hash fn for id |

## Examples

### JSX Elements

**Input:**
```jsx
<FormattedMessage defaultMessage="Hello World" />
```

**Output:**
```jsx
<FormattedMessage id="components.Hello World" defaultMessage="Hello World" />
```

### defineMessages

**Input:**
```javascript
defineMessages({
  hello: 'Hello World',
  goodbye: 'Goodbye World'
})
```

**Output:**
```javascript
defineMessages({
  hello: { id: 'components.hello', defaultMessage: 'Hello World' },
  goodbye: { id: 'components.goodbye', defaultMessage: 'Goodbye World' }
})
```

### formatMessage

**Input:**
```javascript
intl.formatMessage({
  defaultMessage: 'Hello World'
})
```

**Output:**
```javascript
intl.formatMessage({
  id: 'components.Hello World',
  defaultMessage: 'Hello World'
})
```

## Building from Source

```bash
# Install dependencies
npm install

# Build the plugin
npm run build
```

## Development

This plugin is written in Rust and uses the SWC plugin API. The project uses a Cargo workspace structure with multiple crates.

### Project Structure

```
crates/
├── react-intl-core/    # Shared Rust library (ID generation, path utils)
├── swc-plugin/         # SWC Plugin (WASM target)
└── cli/                # CLI tool (coming soon)
tests/                  # Jest integration tests
```

### Prerequisites

- Rust toolchain
- `wasm32-wasip1` target: `rustup target add wasm32-wasip1`
- Node.js 16+

### Building

```bash
# Build the plugin (compiles Rust to WASM)
npm run build

# Or build with cargo directly
cargo build --release --target wasm32-wasip1
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

### Development Workflow

The typical development workflow:

1. **Install all dependencies:**
   ```bash
   npm install
   ```

2. **Build the plugin:**
   ```bash
   npm run build
   ```

3. **Run all tests:**
   ```bash
   npm run test:full
   ```

4. **For active development with auto-rebuild:**
   ```bash
   npm run test:watch
   ```

### Version Compatibility

This plugin is tested and compatible with:
- **@swc/core:** ^1.15.0
- **swc_core (Rust):** 47.0.* - 51.0.*
- **Node.js:** 16, 18, 20

## CI/CD and Release Process

This project uses GitHub Actions for continuous integration and automated publishing to npm.

### Workflows

- **CI** (`.github/workflows/ci.yml`): Runs tests on every push and pull request
- **Test Matrix** (`.github/workflows/test-matrix.yml`): Tests compatibility across Node.js versions 16, 18, and 20
- **Publish** (`.github/workflows/publish.yml`): Automatically publishes to npm when a version tag is pushed
- **Dependabot** (`.github/workflows/dependabot.yml`): Automatically merges dependency updates

### Releasing a New Version

1. **Using the release script** (recommended):
   ```bash
   npm run release 1.0.1
   git push origin main
   git push origin v1.0.1
   ```

2. **Manual process**:
   ```bash
   npm version 1.0.1
   git tag -a v1.0.1 -m "Release 1.0.1"
   git push origin main
   git push origin v1.0.1
   ```

The GitHub Actions workflow will automatically:
- Build the plugin for wasm32-wasip1 target
- Run all tests
- Publish to npm
- Create a GitHub release

For detailed release instructions, see [`.github/RELEASE.md`](.github/RELEASE.md).

### Prerequisites for Publishing

- NPM_TOKEN secret must be configured in GitHub repository settings
- Repository must have proper permissions for GitHub Actions
- **Important**: NPM tokens expire after 90 days - see [Token Management](#token-management) below

### Token Management

NPM tokens expire after 90 days. Use these tools to manage token renewal:

```bash
# Check if your current token is valid and when it expires
npm run check-token

# Or check with a specific token
NPM_TOKEN=your_token npm run check-token
```

**Token Renewal Process**:
1. Create new token at [npmjs.com/settings/tokens](https://www.npmjs.com/settings/tokens)
2. Update `NPM_TOKEN` secret in GitHub repository settings
3. Test with: `npm run check-token`
4. Set calendar reminder for next renewal (80 days)

**Alternative**: Use GitHub Packages instead of npm (no token expiration):
- Enable the `publish-github-packages.yml` workflow
- Disable the regular `publish.yml` workflow
- Packages will be published to `@lcl9288/swc-plugin-react-intl-auto-fs`

### Repository Setup

If you encounter label errors with Dependabot, create the required labels:

```bash
# Create GitHub repository labels
GITHUB_TOKEN=your_token npm run create-labels
```

Or manually create these labels in your GitHub repository:
- `dependencies` (blue)
- `enhancement` (light blue)
- `javascript` (yellow) - optional
- `rust` (orange) - optional

## License

MIT
