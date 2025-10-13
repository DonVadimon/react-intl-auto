# SWC Plugin React Intl Auto - Example

This example demonstrates how to use the SWC plugin for automatic react-intl ID management.

## Setup

```bash
cd examples/basic
npm install
```

## Build

```bash
npm run build
```

This will transform the TypeScript files in `src/` and output the transformed JavaScript to `dist/`.

## What gets transformed

### Before transformation:

```tsx
export const messages = defineMessages({
  hello: 'Hello {name}!',
  welcome: 'Welcome to our app',
});

<FormattedMessage defaultMessage="Welcome to React Intl Auto" />
```

### After transformation:

```tsx
export const messages = defineMessages({
  hello: {
    id: 'components.App.hello',
    defaultMessage: 'Hello {name}!',
  },
  welcome: {
    id: 'components.App.welcome',
    defaultMessage: 'Welcome to our app',
  },
});

<FormattedMessage 
  id="components.App.189751785"
  defaultMessage="Welcome to React Intl Auto" 
/>
```

## Configuration

The plugin is configured in `swc.config.js` with the following options:

- `removePrefix: 'src/'` - Removes 'src/' from the file path when generating IDs
- `separator: '.'` - Uses '.' as the separator in generated IDs
- `filebase: false` - Doesn't include the filename in the ID
- `extractComments: true` - Extracts comments as descriptions

## Development

To watch for changes and rebuild automatically:

```bash
npm run dev
```
