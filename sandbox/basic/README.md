# React Intl Auto - Basic Example

This example demonstrates how to use the SWC plugin for automatic react-intl ID management.

## Setup

```bash
cd sandbox/basic
npm install
```

## Build

```bash
npm run build
```

This will transform the TypeScript files in `src/` and output the transformed JavaScript to `.build/`.

## What gets transformed

### Before transformation:

```tsx
import { defineMessages, FormattedMessage } from 'react-intl';

export const messages = defineMessages({
    hello: 'Hello {name}!',
    welcome: 'Welcome to our app',
});

<FormattedMessage defaultMessage="Welcome to React Intl Auto" />;
```

### After transformation:

```tsx
export const messages = defineMessages({
    hello: {
        id: 'sandbox.basic.src.components.App.hello',
        defaultMessage: 'Hello {name}!',
    },
    welcome: {
        id: 'sandbox.basic.src.components.App.welcome',
        defaultMessage: 'Welcome to our app',
    },
});

<FormattedMessage
    id="sandbox.basic.src.components.App.189751785"
    defaultMessage="Welcome to React Intl Auto"
/>;
```

## Configuration

The plugin is configured in `.swcrc` with the following options:

- `removePrefix: 'sandbox/basic/src/'` - Removes prefix from the file path when generating IDs
- `separator: '.'` - Uses '.' as the separator in generated IDs

## CLI Usage

Extract messages from this example:

```bash
# From repo root
npx react-intl-auto extract 'sandbox/basic/src/**/*.{ts,tsx}' \
  --remove-prefix='sandbox/basic/src/' \
  --output-mode=perfile \
  --output='./sandbox/basic/.react-intl' \
  --extract-source-location
```

## Development

To watch for changes and rebuild automatically:

```bash
npm run dev
```
