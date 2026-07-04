# Migration Guide

## Overview

This guide helps you migrate from `babel-plugin-react-intl-auto` or upgrade from earlier versions of `@donvadimon/react-intl-auto`.

## Breaking Changes in v0.0.2+

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

**Migration:** Remove `hashAlgorithm: 'base64'`. The `hashAlgorithm` option still exists but only accepts `'murmur3'`. The murmur3 algorithm now produces base64-encoded output (e.g., `aG1FCg==`), which is a shorter string compared to the previous decimal format.

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

**Migration:** Use `removePrefix` instead. Set to `true` to strip the entire path prefix (returning only the message key), or provide a specific string prefix to remove.

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
npx @donvadimon/react-intl-auto 'src/**/*.{ts,tsx}' --output messages.json
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

**Before (babel-plugin-react-intl-auto):**

```
1311768467284833366  // Decimal string
```

**After (@donvadimon/react-intl-auto):**

```
aG1FCg==  // Base64 encoded
```

This change ensures compatibility with systems expecting shorter, URL-safe IDs while maintaining the statistical properties of murmur3.

### ID Generation Logic

The ID generation differs between `defineMessages` and `FormattedMessage`/`formatMessage`:

- **defineMessages**: ID = `<filepath>.<key>` (uses the object key name)
- **FormattedMessage / formatMessage**: ID = `<filepath>.<murmur3(defaultMessage)>` (hash of the default message)

This matches the behavior of babel-plugin-react-intl-auto.

## Path Resolution Changes

### Auto-detected Project Root

If `relativeTo` is not specified, the plugin auto-detects the project root by searching upward for:
1. `package-lock.json`
2. `package.json`
3. `yarn.lock`
4. `.git` directory

This is different from babel-plugin-react-intl-auto which defaulted to `process.cwd()`.

### `removePrefix` Behavior

- `removePrefix: true` - Strips the entire path prefix, returning only the message key/descriptor
- `removePrefix: "src/"` - Removes the specific string prefix from the path
- `removePrefix: "^src/components/"` - Strings containing regex metacharacters (`.*`, `.+`, `[`, `(`) are treated as regex patterns

## CLI Deduplication

The CLI automatically deduplicates messages by ID. If you have duplicate message definitions across multiple files, only the first occurrence will be included in the output.

**Example:**

```javascript
// File A
const msg1 = defineMessages({ hello: 'World' });

// File B
const msg2 = defineMessages({ hello: 'World' });
```

**Output:** Only one message with ID containing `hello` will be present.

## Option Mapping Reference

| babel-plugin-react-intl-auto | @donvadimon/react-intl-auto  | Notes                                            |
| ---------------------------- | ---------------------------- | ------------------------------------------------ |
| `removePrefix`               | `removePrefix`               | Same behavior, also supports regex strings       |
| `moduleSourceName`           | `moduleSourceName`           | Same behavior                                    |
| `separator`                  | `separator`                  | Same behavior                                    |
| `relativeTo`                 | `relativeTo`                 | Default changed: auto-detected project root      |
| `hashId`                     | `hashId`                     | Same behavior                                    |
| `hashAlgorithm`              | `hashAlgorithm`              | Only `'murmur3'` supported (was also `'base64'`) |
| `filebase`                   | -                            | Removed, use `removePrefix: true`                |
| `useKey`                     | -                            | Removed, automatic in defineMessages              |
| -                            | `extractSourceLocation`      | New CLI/JS API option                            |
| -                            | `outputMode`                 | New CLI option (`aggregated`/`perfile`)          |

## Troubleshooting

### Different IDs After Migration

If you notice different IDs being generated after migration:

1. **Check path handling:** The new plugin auto-detects project root instead of defaulting to `process.cwd()`. Set `relativeTo` explicitly if needed.
2. **Check separator:** Verify `separator` option matches your previous configuration
3. **Hash format:** Remember that murmur3 now produces base64 output instead of decimal
4. **Check removePrefix:** Ensure `removePrefix` configuration matches. The `true` value now strips the entire path prefix.

### Missing Messages

If some messages are missing after extraction:

1. **Check module source name:** Ensure `moduleSourceName` matches your import statements
2. **Check file patterns:** Verify glob patterns include all relevant files
3. **Deduplication:** Remember that duplicate IDs are automatically removed
4. **Import detection:** The plugin only processes components/functions imported from `moduleSourceName`

### TypeScript Errors

If you encounter TypeScript errors:

1. Ensure `@swc/core` is installed as a peer dependency (`^1.15.0`)
2. Check that your `tsconfig.json` is compatible with SWC
3. Verify the plugin path is correct: `@donvadimon/react-intl-auto/swc-plugin`

### CLI Not Found

If `npx @donvadimon/react-intl-auto` doesn't work:

1. Ensure `@donvadimon/react-intl-auto` is installed
2. Try `node node_modules/@donvadimon/react-intl-auto/cli.js 'src/**/*.ts'`
3. Or add an npm script: `"intl:extract": "@donvadimon/react-intl-auto 'src/**/*.{ts,tsx}'"`

## Getting Help

If you encounter issues during migration:

1. Check the [CLI documentation](CLI.md) for detailed CLI options
2. Review the [JS API documentation](JS_API.md) for programmatic usage
3. Open an issue on GitHub with your configuration and error messages
