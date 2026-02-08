# Agent Guidelines for swc-plugin-react-intl-auto

This is a hybrid Rust/JavaScript SWC plugin that automatically adds IDs to React Intl components.

## Build Commands

### Complete Development Workflow

```bash
# 1. Install dependencies (both JS and Rust)
npm install

# 2. Build the plugin (compiles Rust to WASM)
npm run build

# 3. Run all tests
npm run test:full       # Full cycle: build + Rust tests + Jest tests
cargo test              # Run Rust unit tests
npm test                # Run Jest integration tests

# Alternative test commands
npm run test:watch      # Run tests in watch mode
jest tests/components.test.ts              # Run single test file
jest tests/definition.test.ts -t "default"  # Run specific test
```

### Dependency Management

**JavaScript dependencies:**
```bash
# Install JS dependencies
npm install

# Install specific package
npm install <package-name>

# Install dev dependency
npm install -D <package-name>
```

**Rust dependencies:**
```bash
# Add Rust dependency
cargo add <crate-name>

# Add with features
cargo add <crate-name> --features feature1,feature2

# Example: Add swc_core with specific version
cargo add 'swc_core@47.0.*' --features ecma_plugin_transform,ecma_ast,ecma_visit,ecma_utils
```

### Build Targets

```bash
# wasm32-wasip1 target (default for SWC plugins)
cargo build-wasip1

# wasm32-unknown-unknown target
cargo build-wasm32

# Release build with optimizations
cargo build --release --target wasm32-wasip1
```

### Release Commands

```bash
npm run release         # Build and publish to npm
npm run test-release    # Test release process locally
```

## Code Style Guidelines

### Rust Code Style

- **Naming**: `snake_case` for functions/variables, `PascalCase` for types/structs/enums
- **Indentation**: 4 spaces
- **Imports**: Group `use` statements at top; std lib first, then crates, then local modules
- **Visibility**: Explicit `pub` for all public items; no implicit re-exports
- **Module organization**: Use `mod types;` declarations in lib.rs, keep modules in separate files
- **Error handling**: Use `unwrap_or()` / `unwrap_or_else()` with defaults; avoid panicking in production code
- **Types**: Use explicit type annotations in function signatures; leverage type inference in bodies
- **Documentation**: Add doc comments (`///`) for public APIs and complex logic

Example:
```rust
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::types::PluginOptions;

#[derive(Debug, Clone)]
pub struct PluginState {
    pub filename: PathBuf,
    pub opts: PluginOptions,
}

impl PluginState {
    pub fn new(filename: PathBuf, opts: PluginOptions) -> Self {
        Self { filename, opts }
    }
}
```

### JavaScript Test Style

- **Imports**: Use ES modules (`import`) for test files
- **Indentation**: 2 spaces
- **Test structure**: Use `describe` blocks for grouping, `it` for individual tests
- **Naming**: Descriptive test names: "should [expected behavior] when [condition]"
- **Snapshots**: Use `toMatchSnapshot()` for output comparison
- **Async**: Use `async/await` for asynchronous tests
- **Configuration Testing**: Use `createConfigurationSuites` from `testUtils.ts` to test multiple option combinations

Example:
```typescript
import { cases, createConfigurationSuites } from './testUtils';

const defaultTest = {
    title: 'default',
    code: `
import { defineMessages } from 'react-intl'

export default defineMessages({
  hello: 'hello',
})
`,
};

// snapshot tests for multiple configuration options
createConfigurationSuites('title', [defaultTest]);

// specific non snapshot tests
describe('title', () => {
    it('should add id to FormattedMessage without id', async () => {
      const result = await transformWithPlugin(code);
      expect(result).toContain('"id": "123123"');
    });
})
```

### Project Conventions

- **Plugin options**: Use serde with `#[serde(default, alias = "camelCase")]` for JS-compatible options
- **WASM output**: Plugin builds to `swc-plugin-react-intl-auto.wasm`
- **Entry point**: `index.js` loads and exports the WASM binary
- **Testing**: Unit tests inline in Rust files (`#[cfg(test)]`); integration tests in `__tests__/*.test.js`
- **Project root detection**: Scans for `yarn.lock`, `package.json`, `.git` to find project boundaries
- **Path handling**: Always use `PathBuf` and handle both absolute and relative paths

### File Organization

```
crates/
├── react-intl-core/    # Shared Rust library
│   └── src/
│       ├── lib.rs           # Library exports
│       ├── types.rs         # CoreOptions, CoreState structs
│       ├── id_generator.rs  # Hash functions (murmur3, base64)
│       ├── path_utils.rs    # Path processing utilities
│       └── message_extractor.rs  # Message extraction logic
├── swc-plugin/         # SWC Plugin (WASM)
│   └── src/
│       ├── lib.rs           # Plugin entry point
│       ├── types.rs         # Re-exports from react-intl-core
│       ├── utils.rs         # SWC-specific utilities
│       └── visitors.rs      # AST visitors
├── cli/                # CLI tool (future)
│   └── src/
│       └── main.rs          # CLI entry point
tests/                 # Jest integration tests
├── testUtils.ts       # Test utilities (createConfigurationSuites)
├── *.test.ts          # Test suites by feature
└── __snapshots__/     # Jest snapshots
```

## Important Notes

- This is an SWC plugin - it transforms JavaScript/TypeScript AST at compile time
- The plugin targets `@swc/core` v1.x as a peer dependency
- Uses murmur3 hashing (seed 0) for ID generation to match babel-plugin-react-intl behavior
- Supports both `wasm32-wasip1` and `wasm32-unknown-unknown` targets
- Keep backwards compatibility with existing option formats from babel-plugin-react-intl
