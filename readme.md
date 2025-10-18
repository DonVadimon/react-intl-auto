# SWC React Intl Auto Plugin

A SWC plugin that automatically adds IDs to React Intl components and function calls, ported from the popular Babel plugin [babel-plugin-react-intl-auto](https://github.com/akameco/babel-plugin-react-intl-auto).

## Features

This plugin automatically adds `id` attributes/properties to:

1. **JSX Elements**: `FormattedMessage` and `FormattedHTMLMessage` components
2. **defineMessages**: `defineMessages` function calls
3. **formatMessage**: `intl.formatMessage` function calls

## Installation

```bash
npm install swc-plugin-react-intl-auto
```

## Usage

### Basic Usage

```javascript
const { transform } = require('@swc/core');
const plugin = require('swc-plugin-react-intl-auto');

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
const plugin = require('swc-plugin-react-intl-auto');

const result = await transform(code, {
  filename: 'example.js',
  plugins: [
    [plugin.getPluginPath(), {
      removePrefix: false,
      filebase: false,
      includeExportName: false,
      extractComments: true,
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
| `includeExportName` | `boolean \| 'all'` | `false` | Include export name in ID |
| `extractComments` | `boolean` | `true` | Extract comments as descriptions |
| `useKey` | `boolean` | `false` | Use key attribute instead of message hash |
| `moduleSourceName` | `string` | `'react-intl'` | Module name to detect imports |
| `separator` | `string` | `'.'` | Separator for ID parts |
| `relativeTo` | `string` | `undefined` | Base path for relative file paths |

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

# Or build for specific targets
npm run build:wasm32
npm run build:wasip1
```

## Development

This plugin is written in Rust and uses the SWC plugin API. The source code is in the `src/` directory.

### Prerequisites

- Rust toolchain
- `wasm32-wasip1` target: `rustup target add wasm32-wasip1`

### Building

```bash
cargo build --release --target wasm32-wasip1
```

## License

MIT
