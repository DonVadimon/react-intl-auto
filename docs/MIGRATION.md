# Migration Guide

## Overview

This guide helps you migrate from `babel-plugin-react-intl-auto` or upgrade from earlier versions of `@donvadimon/react-intl-auto`.

## Breaking Changes in v1.0.0

### Removed Options

#### 1. `hashAlgorithm: 'base64'`

**Before:**

```javascript
// babel-plugin-react-intl-auto
{
  "plugins": [
    ["react-intl-auto", {
      "hashId": true,
      "hashAlgorithm": "base64"
    }]
  ]
}
```

**After:**

```javascript
// @donvadimon/react-intl-auto
{
  "jsc": {
    "experimental": {
      "plugins": [
        ["@donvadimon/react-intl-auto/swc-plugin", {
          "hashId": true
          // hashAlgorithm defaults to 'murmur3'
        }]
      ]
    }
  }
}
```

**Migration:** Remove `hashAlgorithm: 'base64'` and use the default `murmur3` algorithm. The murmur3 algorithm now produces base64-encoded output by default, which is compatible with the previous base64 behavior but uses a proper hash function.

#### 2. `filebase` Option

**Before:**

```javascript
// babel-plugin-react-intl-auto
{
  "plugins": [
    ["react-intl-auto", {
      "filebase": true
    }]
  ]
}
```

**After:**

```javascript
// @donvadimon/react-intl-auto
{
  "jsc": {
    "experimental": {
      "plugins": [
        ["@donvadimon/react-intl-auto/swc-plugin", {
          "removePrefix": true  // or specific prefix
        }]
      ]
    }
  }
}
```

**Migration:** Use `removePrefix` instead. Set to `true` for automatic prefix removal, or provide a specific string prefix to remove.

#### 3. `useKey` Option

**Before:**

```javascript
// babel-plugin-react-intl-auto
{
  "plugins": [
    ["react-intl-auto", {
      "useKey": true
    }]
  ]
}
```

**After:**

```javascript
// @donvadimon/react-intl-auto
{
  "jsc": {
    "experimental": {
      "plugins": [
        ["@donvadimon/react-intl-auto/swc-plugin", {}]
        // Keys are automatically used in defineMessages
      ]
    }
  }
}
```

**Migration:** Simply remove the `useKey` option. Keys are now automatically used in `defineMessages` without any configuration.

## Migrating from babel-plugin-react-intl-auto

### Configuration Migration

**Before (Babel):**

```javascript
// babel.config.js
module.exports = {
    plugins: [
        [
            'react-intl-auto',
            {
                removePrefix: 'src/',
                moduleSourceName: 'react-intl',
                separator: '.',
            },
        ],
    ],
};
```

**After (SWC):**

```json
// .swcrc
{
    "jsc": {
        "experimental": {
            "plugins": [
                [
                    "@donvadimon/react-intl-auto/swc-plugin",
                    {
                        "removePrefix": "src/",
                        "moduleSourceName": "react-intl",
                        "separator": "."
                    }
                ]
            ]
        }
    }
}
```

### CLI Migration

**Before:**

```bash
# babel-plugin-react-intl-auto didn't have a CLI
# You would use babel to extract
```

**After:**

```bash
# Extract messages with the new CLI
npx react-intl-auto extract 'src/**/*.{ts,tsx}' --output messages.json
```

### Programmatic API Migration

**Before:**

```javascript
// babel-plugin-react-intl-auto
// No programmatic API available
```

**After:**

```javascript
// @donvadimon/react-intl-auto
const { extractSync } = require('@donvadimon/react-intl-auto/extract');

const result = extractSync(['src/**/*.ts'], {
    removePrefix: 'src/',
});

console.log(result.messages);
```

## ID Generation Changes

### Murmur3 Hash Format

The murmur3 hash algorithm now produces base64-encoded output instead of decimal strings:

**Before:**

```
1311768467284833366  // Decimal string
```

**After:**

```
aG1FCg==  // Base64 encoded
```

This change ensures compatibility with systems expecting shorter, URL-safe IDs while maintaining the statistical properties of murmur3.

## CLI Deduplication

The CLI now automatically deduplicates messages by ID. If you have duplicate message definitions across multiple files, only the first occurrence will be included in the output.

**Example:**

```javascript
// File A
const msg1 = defineMessages({ hello: 'World' });

// File B
const msg2 = defineMessages({ hello: 'World' });
```

**Output:** Only one message with ID containing `hello` will be present.

## Option Mapping Reference

| babel-plugin-react-intl-auto | @donvadimon/react-intl-auto | Notes                                |
| ---------------------------- | --------------------------- | ------------------------------------ |
| `removePrefix`               | `removePrefix`              | Same behavior                        |
| `moduleSourceName`           | `moduleSourceName`          | Same behavior                        |
| `separator`                  | `separator`                 | Same behavior                        |
| `relativeTo`                 | `relativeTo`                | Same behavior                        |
| `hashId`                     | `hashId`                    | Same behavior                        |
| `hashAlgorithm`              | `hashAlgorithm`             | Only 'murmur3' supported             |
| `filebase`                   | -                           | Removed, use `removePrefix`          |
| `useKey`                     | -                           | Removed, automatic in defineMessages |
| -                            | `extractSourceLocation`     | New CLI option                       |
| -                            | `outputMode`                | New CLI option                       |

## Troubleshooting

### Different IDs After Migration

If you notice different IDs being generated after migration:

1. **Check path handling:** Ensure `removePrefix` and `relativeTo` options match your previous configuration
2. **Check separator:** Verify `separator` option matches
3. **Hash format:** Remember that murmur3 now produces base64 output

### Missing Messages

If some messages are missing after extraction:

1. **Check module source name:** Ensure `moduleSourceName` matches your import statements
2. **Check file patterns:** Verify glob patterns include all relevant files
3. **Deduplication:** Remember that duplicate IDs are automatically removed

### TypeScript Errors

If you encounter TypeScript errors:

1. Ensure `@swc/core` is installed as a peer dependency
2. Check that your `tsconfig.json` is compatible with SWC
3. Verify the plugin path is correct: `@donvadimon/react-intl-auto/swc-plugin`

## Getting Help

If you encounter issues during migration:

1. Check the [CLI documentation](CLI.md) for detailed CLI options
2. Review the [JS API documentation](JS_API.md) for programmatic usage
3. Open an issue on GitHub with your configuration and error messages
