# Release Process

This document explains how to release new versions of @donvadimon/react-intl-auto.

## Overview

The project uses **GitHub Actions** with **napi-rs** for cross-platform builds and **manual publishing**.

### What Gets Published

1. **Main package** (`@donvadimon/react-intl-auto`) - Contains:
    - SWC plugin WASM file
    - CLI entry point
    - JavaScript API

2. **Platform-specific packages** (optional dependencies):
    - `@donvadimon/react-intl-auto-darwin-arm64`
    - `@donvadimon/react-intl-auto-darwin-x64`
    - `@donvadimon/react-intl-auto-linux-x64-gnu`

## Prerequisites

### 1. NPM Token

Create an npm access token and add it as a GitHub secret:

1. Go to [npmjs.com](https://www.npmjs.com) → Access Tokens
2. Generate an **Automation** token
3. In GitHub repository: Settings → Secrets and variables → Actions
4. Add secret named `NPM_TOKEN`

### 2. Git Repository Setup

Ensure you have:

- Push access to the repository
- Permission to push to master branch

## Release Steps

### Step 1: Prepare Release

```bash
# Ensure you're on master branch
git checkout master
git pull origin master

# Check working directory is clean
git status
```

### Step 2: Update Version

Update version in both files:

**package.json:**

```json
{
    "version": "1.1.0"
}
```

**Cargo.toml** (root workspace):

```toml
[workspace.package]
version = "1.1.0"
```

### Step 3: Commit Version Bump

```bash
git add package.json Cargo.toml
git commit -m "chore: bump version to 1.1.0"
git push origin master
```

### Step 4: Trigger Manual Publish

Go to GitHub Actions and run the workflow manually:

1. Navigate to: `https://github.com/DonVadimon/react-intl-auto/actions`
2. Click on **"CI"** workflow
3. Click **"Run workflow"** button
4. Select:
    - **Branch**: `master`
    - **version_type**: Choose one:
        - `patch` - for bug fixes (1.0.0 → 1.0.1)
        - `minor` - for new features (1.0.0 → 1.1.0)
        - `major` - for breaking changes (1.0.0 → 2.0.0)
        - `prerelease` - for beta/alpha versions
5. Click **"Run workflow"**

### Step 5: Monitor CI/CD

GitHub Actions will automatically:

1. **Build stage:**
    - Build WASM plugin (one time)
    - Build napi-rs addons for all platforms (Linux, macOS x64, macOS arm64)

2. **Test stage:**
    - Run Rust unit tests
    - Run Jest integration tests
    - Test native bindings on all platforms

3. **Publish stage:** (manual trigger only)
    - Create platform-specific npm packages
    - Publish all packages to npm

**Note:** The publish job only runs when manually triggered via "Run workflow". All other jobs (lint, build, test) run automatically on every push to master.

Monitor progress at: `https://github.com/DonVadimon/react-intl-auto/actions`

### Step 6: Verify Release

Check that packages are published:

```bash
# Main package
npm view @donvadimon/react-intl-auto

# Platform packages
npm view @donvadimon/react-intl-auto-darwin-arm64
npm view @donvadimon/react-intl-auto-linux-x64-gnu
```

## Troubleshooting

### Build Failures

**Rust compilation errors:**

```bash
# Test locally before pushing
cargo build --release --target wasm32-wasip1
cargo test --workspace
```

**napi-rs build fails:**

```bash
# Test napi build locally
npm run build:napi
```

### NPM Publishing Failures

**"You do not have permission to publish"**

- Check `NPM_TOKEN` is valid and not expired
- Verify you have publish rights on npm

**"Version already exists"**

- Cannot publish same version twice
- Must bump version before releasing

### Platform-specific Failures

**macOS ARM64 build fails:**

- Check `.cargo/config.toml` has correct rustflags
- May need to update MACOSX_DEPLOYMENT_TARGET

**Linux build fails:**

- Ensure `napi-cross` is available in CI
- Check glibc compatibility

## Rollback

If a release has critical issues:

### 1. Deprecate on NPM (recommended)

```bash
npm deprecate @donvadimon/react-intl-auto@1.1.0 "Critical bug in v1.1.0, use v1.1.1 instead"
```

### 2. Delete Git Tag (optional)

```bash
git tag -d v1.1.0
git push origin :refs/tags/v1.1.0
```

### 3. Prepare Hotfix

```bash
# Create hotfix branch
git checkout -b hotfix/v1.1.1

# Fix the issue, bump version to 1.1.1
git add .
git commit -m "fix: resolve critical issue"
git push origin hotfix/v1.1.1

# Merge to master and tag
git checkout master
git merge hotfix/v1.1.1
git tag -a v1.1.1 -m "Release v1.1.1"
git push origin master
git push origin v1.1.1
```

## CI/CD Workflow Details

The workflow (`.github/workflows/napi-rs.yml`) has these jobs:

### Job: lint

- Runs on: Ubuntu
- Checks: `cargo fmt`, `cargo clippy`

### Job: build-wasm

- Runs on: Ubuntu
- Builds: `swc-plugin.wasm`
- Uploads artifact: `wasm-plugin`

### Job: build-napi

- Matrix: 3 platforms
- Builds: `.node` files for each platform
- Uploads artifacts: `bindings-{target}`

### Job: test-rust

- Depends on: `build-wasm`
- Runs: `cargo test --workspace`

### Job: test-node

- Depends on: `build-wasm`, `build-napi`
- Downloads: WASM + Linux addon
- Runs: `npm test` (Jest)

### Job: test-native-bindings

- Depends on: `build-napi`
- Matrix: 3 platforms × 3 Node versions
- Tests: Native addon loading

### Job: publish

- **Trigger**: Manual only (`workflow_dispatch`)
- Depends on: All test jobs (lint, build-wasm, build-napi, test-rust, test-node)
- Runs on: Ubuntu
- Steps:
    1. Create npm directories (`npx napi create-npm-dirs`)
    2. Download all artifacts
    3. Move artifacts to npm dirs (`npx napi artifacts`)
    4. Copy WASM to packages
    5. Publish to npm with selected version type

## Best Practices

1. **Always test locally before releasing:**

    ```bash
    npm run test:full
    ```

2. **Use semantic versioning:**
    - Patch (1.0.1): Bug fixes
    - Minor (1.1.0): New features, backward compatible
    - Major (2.0.0): Breaking changes

3. **Write good commit messages:**
    - They appear in GitHub releases
    - Help users understand changes

4. **Test the published package:**

    ```bash
    npm install @donvadimon/react-intl-auto@latest
    # Test in a fresh project
    ```

5. **Update documentation:**
    - README.md with new features
    - CHANGELOG.md if maintained

## Monitoring

- **GitHub Actions:** https://github.com/DonVadimon/react-intl-auto/actions
- **NPM Package:** https://www.npmjs.com/package/@donvadimon/react-intl-auto
- **Releases:** https://github.com/DonVadimon/react-intl-auto/releases
