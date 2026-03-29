# CLI Documentation

## Overview

The CLI tool extracts React Intl messages from your source files and outputs them as JSON. It supports both aggregated (single file) and per-file output modes.

## Installation

The CLI is included with the package:

```bash
npm install -D @donvadimon/react-intl-auto
```

## Usage

```bash
npx react-intl-auto extract [OPTIONS] <PATTERNS...>
```

## Commands

### `extract`

Extract messages from source files matching the given glob patterns.

```bash
npx react-intl-auto extract 'src/**/*.{ts,tsx}'
```

## Options

### Positional Arguments

#### `<PATTERNS...>`

Glob patterns for source files to extract from.

**Examples:**

```bash
# Single pattern
npx react-intl-auto extract 'src/**/*.ts'

# Multiple patterns
npx react-intl-auto extract 'src/**/*.ts' 'lib/**/*.tsx'

# Specific directories
npx react-intl-auto extract 'src/components/**/*.{ts,tsx}'
```

### Optional Arguments

#### `--ignore <PATTERNS>`

Glob patterns to ignore. Can be specified multiple times.

**Default:** `**/node_modules/**`, `**/.git/**`

**Examples:**

```bash
# Ignore test files
npx react-intl-auto extract 'src/**/*.ts' --ignore '**/*.test.ts' --ignore '**/*.spec.ts'

# Ignore specific directories
npx react-intl-auto extract 'src/**/*.ts' --ignore '**/generated/**' --ignore '**/legacy/**'
```

#### `--output <PATH>`

Output file or directory path.

- In `aggregated` mode: path to output JSON file
- In `perfile` mode: path to output directory

**Default:** `messages.json` (aggregated) or `messages/` (perfile)

**Examples:**

```bash
# Aggregated mode with custom file
npx react-intl-auto extract 'src/**/*.ts' --output ./locales/en.json

# Perfile mode with custom directory
npx react-intl-auto extract 'src/**/*.ts' --output-mode=perfile --output ./translations/
```

#### `--output-mode <MODE>`

Output mode for extracted messages.

**Values:**

- `aggregated` - Single JSON file with all messages
- `perfile` - Separate JSON file per source file

**Default:** `aggregated`

**Examples:**

```bash
# Aggregated mode (single file)
npx react-intl-auto extract 'src/**/*.ts' --output-mode=aggregated --output messages.json

# Perfile mode (directory with multiple files)
npx react-intl-auto extract 'src/**/*.ts' --output-mode=perfile --output locales/
```

**Output structure:**

Aggregated mode:

```json
[
    {
        "id": "components.App.hello",
        "defaultMessage": "Hello World"
    },
    {
        "id": "components.App.goodbye",
        "defaultMessage": "Goodbye World"
    }
]
```

Perfile mode:

```
locales/
├── components/
│   ├── App.json
│   └── Button.json
└── utils/
    └── helpers.json
```

#### `--extract-source-location`

Include source file path in the extracted message data.

**Default:** `false`

**Example:**

```bash
npx react-intl-auto extract 'src/**/*.ts' --extract-source-location
```

**Output:**

```json
[
    {
        "id": "components.App.hello",
        "defaultMessage": "Hello World",
        "file": "src/components/App.tsx"
    }
]
```

#### `--remove-prefix <VALUE>`

Remove prefix from file path when generating IDs.

**Values:**

- `true` - Remove common prefix automatically
- `false` - Don't remove prefix (default behavior)
- `<string>` - Remove specific prefix string

**Examples:**

```bash
# Remove specific prefix
npx react-intl-auto extract 'src/**/*.ts' --remove-prefix='src/'

# Remove prefix automatically
npx react-intl-auto extract 'src/**/*.ts' --remove-prefix=true

# Use regex pattern
npx react-intl-auto extract 'src/**/*.ts' --remove-prefix='^src/components/'
```

#### `--module-source-name <NAME>`

Module source name for detecting react-intl imports.

**Default:** `react-intl`

**Examples:**

```bash
# For gatsby-plugin-intl
npx react-intl-auto extract 'src/**/*.ts' --module-source-name='gatsby-plugin-intl'

# For custom wrapper
npx react-intl-auto extract 'src/**/*.ts' --module-source-name='@company/react-intl'
```

#### `--separator <CHAR>`

Separator used in generated message IDs.

**Default:** `.`

**Examples:**

```bash
# Use underscore as separator
npx react-intl-auto extract 'src/**/*.ts' --separator='_'

# Use slash
npx react-intl-auto extract 'src/**/*.ts' --separator='/'
```

**ID Examples:**

- With `.` separator: `components.App.hello`
- With `_` separator: `components_App_hello`
- With `/` separator: `components/App/hello`

#### `--relative-to <PATH>`

Base path for relative path calculation in ID generation.

**Default:** `process.cwd()`

**Example:**

```bash
# Calculate paths relative to project root
npx react-intl-auto extract 'src/**/*.ts' --relative-to='./'

# Use absolute path
npx react-intl-auto extract 'src/**/*.ts' --relative-to='/home/user/project'
```

#### `--hash-id`

Apply murmur3 hash to generated message IDs.

**Default:** `false`

**Example:**

```bash
npx react-intl-auto extract 'src/**/*.ts' --hash-id
```

**Output:**

```json
[
    {
        "id": "aG1FCg==",
        "defaultMessage": "Hello World"
    }
]
```

## Complete Examples

### Basic extraction

```bash
# Extract all TypeScript files to messages.json
npx react-intl-auto extract 'src/**/*.{ts,tsx}'
```

### With custom output

```bash
# Extract to specific file
npx react-intl-auto extract 'src/**/*.ts' --output ./locales/en.json

# Extract to directory (per file mode)
npx react-intl-auto extract 'src/**/*.ts' --output-mode=perfile --output ./translations/
```

### With source locations

```bash
# Include file paths in output
npx react-intl-auto extract 'src/**/*.ts' --extract-source-location --output messages.json
```

### With path prefix removal

```bash
# Remove 'src/' prefix from IDs
npx react-intl-auto extract 'src/**/*.ts' --remove-prefix='src/'

# This changes ID from:
#   src.components.App.hello
# To:
#   components.App.hello
```

### With custom separator

```bash
# Use underscore separator
npx react-intl-auto extract 'src/**/*.ts' --separator='_'

# This changes ID from:
#   components.App.hello
# To:
#   components_App_hello
```

### With hashing

```bash
# Hash all IDs for shorter names
npx react-intl-auto extract 'src/**/*.ts' --hash-id --output hashed-messages.json
```

### Complex example

```bash
npx react-intl-auto extract \
  'src/**/*.{ts,tsx}' \
  'lib/**/*.ts' \
  --ignore '**/*.test.ts' \
  --ignore '**/*.spec.ts' \
  --ignore '**/stories/**' \
  --output-mode=perfile \
  --output ./locales/ \
  --remove-prefix='src/' \
  --separator='.' \
  --extract-source-location
```

## Supported File Types

The CLI automatically detects and processes the following file types:

- `.ts` - TypeScript
- `.tsx` - TypeScript with JSX
- `.mts` - TypeScript module
- `.js` - JavaScript
- `.jsx` - JavaScript with JSX
- `.mjs` - ES module
- `.cjs` - CommonJS module

## Exit Codes

- `0` - Success
- `1` - Error (invalid arguments, file not found, parse error, etc.)

## Notes

- Duplicate message IDs within the same file are automatically deduplicated
- The CLI uses the same ID generation logic as the SWC plugin
- Messages are extracted from:
    - `FormattedMessage` and `FormattedHTMLMessage` components
    - `defineMessages()` calls
    - `intl.formatMessage()` calls
