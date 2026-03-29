# JavaScript API Documentation

## Overview

The JavaScript API provides programmatic access to the message extraction functionality. It's available through the `@donvadimon/react-intl-auto/extract` module.

## Installation

```bash
npm install -D @donvadimon/react-intl-auto
```

## Import

```javascript
const {
    extractSync,
    extract,
    parseFile,
} = require('@donvadimon/react-intl-auto/extract');
```

## Functions

### `extractSync(patterns, options?)`

Synchronously extracts messages from files matching the given glob patterns.

**Parameters:**

| Parameter  | Type             | Required | Description                             |
| ---------- | ---------------- | -------- | --------------------------------------- |
| `patterns` | `string[]`       | Yes      | Array of glob patterns for source files |
| `options`  | `ExtractOptions` | No       | Extraction options                      |

**Returns:** `ExtractResult`

**Example:**

```javascript
const { extractSync } = require('@donvadimon/react-intl-auto/extract');

const result = extractSync(['src/**/*.ts', 'lib/**/*.tsx'], {
    removePrefix: 'src/',
    separator: '.',
    hashId: false,
    extractSourceLocation: true,
});

console.log(`Processed ${result.filesProcessed} files`);
console.log(`Found ${result.messages.length} messages`);

result.messages.forEach((msg) => {
    console.log(`${msg.id}: ${msg.defaultMessage}`);
});
```

### `extract(patterns, options?)`

Asynchronously extracts messages from files matching the given glob patterns.

**Parameters:**

| Parameter  | Type             | Required | Description                             |
| ---------- | ---------------- | -------- | --------------------------------------- |
| `patterns` | `string[]`       | Yes      | Array of glob patterns for source files |
| `options`  | `ExtractOptions` | No       | Extraction options                      |

**Returns:** `Promise<ExtractResult>`

**Example:**

```javascript
const { extract } = require('@donvadimon/react-intl-auto/extract');

async function extractMessages() {
    const result = await extract(['src/**/*.ts'], {
        removePrefix: 'src/',
        hashId: true,
    });

    return result.messages;
}

extractMessages().then((messages) => {
    console.log('Extracted messages:', messages);
});
```

**Note:** Currently, this function calls the synchronous implementation internally. In future versions, it may be parallelized.

### `parseFile(filePath, options?)`

Parses a single file and extracts messages from it.

**Parameters:**

| Parameter  | Type             | Required | Description             |
| ---------- | ---------------- | -------- | ----------------------- |
| `filePath` | `string`         | Yes      | Path to the source file |
| `options`  | `ExtractOptions` | No       | Extraction options      |

**Returns:** `ExtractedMessage[]`

**Example:**

```javascript
const { parseFile } = require('@donvadimon/react-intl-auto/extract');

const messages = parseFile('src/components/App.tsx', {
    removePrefix: 'src/',
    separator: '.',
});

messages.forEach((msg) => {
    console.log(`ID: ${msg.id}`);
    console.log(`Message: ${msg.defaultMessage}`);
    if (msg.description) {
        console.log(`Description: ${msg.description}`);
    }
});
```

## Types

### `ExtractOptions`

Options for message extraction.

```typescript
interface ExtractOptions {
    /** Remove prefix from file path when generating IDs */
    removePrefix?: boolean | string | RegExp;

    /** Module name to detect imports from (default: 'react-intl') */
    moduleSourceName?: string;

    /** Separator used in generated IDs (default: '.') */
    separator?: string;

    /** Base path for relative file paths */
    relativeTo?: string;

    /** Apply murmur3 hash to generated IDs (default: false) */
    hashId?: boolean;

    /** Hash algorithm (only 'murmur3' is supported, default: 'murmur3') */
    hashAlgorithm?: 'murmur3';

    /** Include source file path in output (default: false) */
    extractSourceLocation?: boolean;

    /** Output mode: 'aggregated' or 'perfile' (default: 'aggregated') */
    outputMode?: 'aggregated' | 'perfile';
}
```

### `ExtractResult`

Result of extraction operation.

```typescript
interface ExtractResult {
    /** Array of extracted messages */
    messages: ExtractedMessage[];

    /** Number of files processed */
    filesProcessed: number;
}
```

### `ExtractedMessage`

Extracted message structure.

```typescript
interface ExtractedMessage {
    /** Message ID */
    id: string;

    /** Default message text */
    defaultMessage: string;

    /** Optional description */
    description?: string;

    /** Source file path (if extractSourceLocation is enabled) */
    file?: string;
}
```

## Options Details

### `removePrefix`

Controls prefix removal from file paths when generating IDs.

**Values:**

- `true` - Automatically detect and remove common prefix
- `false` or `undefined` - Don't remove prefix
- `string` - Remove specific prefix string

**Examples:**

```javascript
// Remove specific prefix
extractSync(['src/**/*.ts'], {
    removePrefix: 'src/',
});
// File: src/components/App.tsx
// ID without prefix: components.App.hello

// Auto-detect prefix
extractSync(['src/**/*.ts'], {
    removePrefix: true,
});

// No prefix removal
extractSync(['src/**/*.ts'], {
    removePrefix: false,
});
// ID: src.components.App.hello
```

### `moduleSourceName`

Specifies the module name for detecting react-intl imports.

**Default:** `'react-intl'`

**Examples:**

```javascript
// For gatsby-plugin-intl
extractSync(['src/**/*.ts'], {
    moduleSourceName: 'gatsby-plugin-intl',
});

// For custom wrapper
extractSync(['src/**/*.ts'], {
    moduleSourceName: '@company/react-intl',
});
```

### `separator`

Character used to separate parts of the generated ID.

**Default:** `'.'`

**Examples:**

```javascript
// Default dot separator
extractSync(['src/**/*.ts'], {
    separator: '.',
});
// ID: components.App.hello

// Underscore separator
extractSync(['src/**/*.ts'], {
    separator: '_',
});
// ID: components_App_hello

// Slash separator
extractSync(['src/**/*.ts'], {
    separator: '/',
});
// ID: components/App/hello
```

### `hashId`

When enabled, applies murmur3 hash to the generated ID.

**Default:** `false`

**Example:**

```javascript
const result = extractSync(['src/**/*.ts'], {
    hashId: true,
});

// Original ID: components.App.hello
// Hashed ID: aG1FCg==
```

### `extractSourceLocation`

Includes the source file path in the extracted message.

**Default:** `false`

**Example:**

```javascript
const result = extractSync(['src/**/*.ts'], {
    extractSourceLocation: true,
});

console.log(result.messages[0]);
// {
//   id: 'components.App.hello',
//   defaultMessage: 'Hello World',
//   file: 'src/components/App.tsx'
// }
```

### `outputMode`

Note: This option is primarily for CLI. In JS API, it doesn't affect the output format (always returns array), but affects how messages are processed internally.

**Default:** `'aggregated'`

## Complete Examples

### Basic extraction

```javascript
const { extractSync } = require('@donvadimon/react-intl-auto/extract');

const result = extractSync(['src/**/*.tsx']);

console.log(`Found ${result.messages.length} messages`);
```

### With options

```javascript
const { extractSync } = require('@donvadimon/react-intl-auto/extract');

const result = extractSync(['src/**/*.{ts,tsx}', 'lib/**/*.ts'], {
    removePrefix: 'src/',
    separator: '.',
    moduleSourceName: 'react-intl',
    hashId: false,
    extractSourceLocation: true,
});

// Save to file
const fs = require('fs');
fs.writeFileSync('messages.json', JSON.stringify(result.messages, null, 2));
```

### Processing single file

```javascript
const { parseFile } = require('@donvadimon/react-intl-auto/extract');

const messages = parseFile('src/components/App.tsx', {
    removePrefix: 'src/',
});

// Filter specific messages
const greetingMessages = messages.filter((msg) => msg.id.includes('greeting'));
```

### Async extraction

```javascript
const { extract } = require('@donvadimon/react-intl-auto/extract');

async function buildTranslations() {
    const locales = ['en', 'de', 'fr'];

    for (const locale of locales) {
        const result = await extract(['src/**/*.ts'], {
            removePrefix: 'src/',
        });

        // Transform messages for translation service
        const translations = result.messages.map((msg) => ({
            key: msg.id,
            source: msg.defaultMessage,
            target: '', // To be translated
            description: msg.description,
        }));

        console.log(`Prepared ${translations.length} strings for ${locale}`);
    }
}

buildTranslations();
```

### Integration with build tools

```javascript
// webpack.config.js
const { extractSync } = require('@donvadimon/react-intl-auto/extract');

module.exports = {
    // ... webpack config
    plugins: [
        {
            apply: (compiler) => {
                compiler.hooks.beforeRun.tap('ExtractMessages', () => {
                    const result = extractSync(['src/**/*.tsx'], {
                        removePrefix: 'src/',
                    });

                    require('fs').writeFileSync(
                        './dist/messages.json',
                        JSON.stringify(result.messages),
                    );
                });
            },
        },
    ],
};
```

## Error Handling

The API may throw errors in the following cases:

- Invalid glob patterns
- File not found
- Parse errors in source files
- Permission errors

**Example with error handling:**

```javascript
const { extractSync } = require('@donvadimon/react-intl-auto/extract');

try {
    const result = extractSync(['src/**/*.ts']);
    console.log('Success:', result);
} catch (error) {
    console.error('Extraction failed:', error.message);
}
```

## Notes

- The API uses the same extraction logic as the CLI and SWC plugin
- Duplicate message IDs are automatically deduplicated
- Supported file types: `.ts`, `.tsx`, `.mts`, `.js`, `.jsx`, `.mjs`, `.cjs`
- Messages are extracted from:
    - `FormattedMessage` and `FormattedHTMLMessage` components
    - `defineMessages()` calls
    - `intl.formatMessage()` calls
