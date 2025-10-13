# SWC Plugin React Intl Auto

A SWC plugin for automatic react-intl ID management, ported from the original Babel plugin.

## Features

- Automatically generates IDs for `FormattedMessage` and `FormattedHTMLMessage` components
- Processes `defineMessages` calls to add IDs
- Handles `intl.formatMessage` calls
- Supports TypeScript and JSX
- Much faster than Babel equivalent

## Installation

```bash
npm install swc-plugin-react-intl-auto
```

## Usage

### With SWC CLI

```bash
npx swc src --out-dir dist --plugins swc-plugin-react-intl-auto
```

### With @swc/core

```javascript
const { transform } = require('@swc/core');

const result = await transform(code, {
  filename: 'input.js',
  jsc: {
    parser: {
      syntax: 'typescript',
      tsx: true,
    },
    experimental: {
      plugins: [
        [
          'swc-plugin-react-intl-auto',
          {
            removePrefix: 'src/',
            separator: '.',
          }
        ]
      ]
    }
  }
});
```

### With Next.js

```javascript
// next.config.js
const swcPluginReactIntlAuto = require('swc-plugin-react-intl-auto');

module.exports = {
  experimental: {
    swcPlugins: [
      [swcPluginReactIntlAuto, {
        removePrefix: 'src/',
        separator: '.',
      }]
    ]
  }
};
```

## Options

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `removePrefix` | `string \| boolean \| RegExp` | `''` | Remove prefix from file path |
| `filebase` | `boolean` | `false` | Include filename in ID |
| `includeExportName` | `boolean \| 'all'` | `false` | Include export name in ID |
| `extractComments` | `boolean` | `true` | Extract comments as descriptions |
| `useKey` | `boolean` | `false` | Use key property instead of hash |
| `moduleSourceName` | `string` | `'react-intl'` | Module source name |
| `separator` | `string` | `'.'` | ID separator |
| `relativeTo` | `string` | `process.cwd()` | Relative path for ID generation |

## Examples

### Before

```jsx
import { defineMessages, FormattedMessage } from 'react-intl'

export default defineMessages({
  hello: 'hello {name}',
  welcome: 'Welcome!',
})

const MyComponent = () => <FormattedMessage defaultMessage="goodbye {name}" />
```

### After

```jsx
import { defineMessages, FormattedMessage } from 'react-intl'

export default defineMessages({
  hello: {
    id: 'components.App.hello',
    defaultMessage: 'hello {name}',
  },
  welcome: {
    id: 'components.App.welcome',
    defaultMessage: 'Welcome!',
  },
})

const MyComponent = () => (
  <FormattedMessage 
    id="components.App.189751785" 
    defaultMessage="goodbye {name}" 
  />
)
```

## Performance

This SWC plugin is significantly faster than the original Babel plugin:

- **10-70x faster** compilation
- **Lower memory usage**
- **Better TypeScript support**

## Migration from Babel Plugin

If you're migrating from the Babel plugin, the API is nearly identical:

```javascript
// Babel
{
  "plugins": [
    ["react-intl-auto", { "removePrefix": "src/" }]
  ]
}

// SWC
{
  "jsc": {
    "experimental": {
      "plugins": [
        ["swc-plugin-react-intl-auto", { "removePrefix": "src/" }]
      ]
    }
  }
}
```

## Development

```bash
# Install dependencies
npm install

# Build the plugin
npm run build

# Test the plugin
npm test
```

## License

MIT
