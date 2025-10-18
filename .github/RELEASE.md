# Release Process

This document explains how to release new versions of the SWC React Intl Auto plugin.

## GitHub Actions Workflows

### CI Workflow (`.github/workflows/ci.yml`)
- **Triggers**: Push to `main`/`develop` branches, Pull Requests
- **Purpose**: Run tests and build verification
- **Actions**:
  - Sets up Node.js 18 and Rust toolchain
  - Installs dependencies and caches Rust dependencies
  - Builds the plugin for wasm32-wasip1 target
  - Runs both JavaScript and Rust tests

### Publish Workflow (`.github/workflows/publish.yml`)
- **Triggers**: Git tags starting with `v*`, Manual dispatch
- **Purpose**: Publish to npm and create GitHub releases
- **Actions**:
  - Builds the plugin
  - Extracts version from git tag
  - Updates both `package.json` and `Cargo.toml` versions
  - Publishes to npm using `NPM_TOKEN` secret
  - Creates GitHub release

## Prerequisites

### 1. NPM Token
You need to create an npm access token and add it as a GitHub secret:

1. Go to [npmjs.com](https://www.npmjs.com) and log in
2. Go to Access Tokens in your account settings
3. Generate a new "Automation" token
4. In your GitHub repository, go to Settings → Secrets and variables → Actions
5. Add a new secret named `NPM_TOKEN` with your npm token value

**⚠️ Important**: NPM tokens expire after 90 days. See the [Token Management](#token-management) section below for solutions.

### 2. Repository Permissions
Ensure the GitHub Actions have permission to:
- Read repository contents
- Write packages (if using GitHub Packages)
- Create releases

## Release Process

### Method 1: Using the Release Script (Recommended)

1. **Prepare for release**:
   ```bash
   # Ensure you're on the main branch and up to date
   git checkout main
   git pull origin main
   
   # Make sure working directory is clean
   git status
   ```

2. **Run the release script**:
   ```bash
   # For patch version (1.0.0 → 1.0.1)
   npm run release 1.0.1
   
   # For minor version (1.0.0 → 1.1.0)
   npm run release 1.1.0
   
   # For major version (1.0.0 → 2.0.0)
   npm run release 2.0.0
   ```

3. **Push the changes**:
   ```bash
   git push origin main
   git push origin v1.0.1  # Replace with your version
   ```

### Method 2: Manual Process

1. **Update versions**:
   ```bash
   # Update package.json
   npm version 1.0.1
   
   # Update Cargo.toml manually
   # Change: version = "0.1.0" to version = "1.0.1"
   ```

2. **Create and push tag**:
   ```bash
   git add package.json Cargo.toml
   git commit -m "chore: bump version to 1.0.1"
   git tag -a v1.0.1 -m "Release 1.0.1"
   git push origin main
   git push origin v1.0.1
   ```

## What Happens During Release

1. **GitHub Actions triggers** on the new tag
2. **Build process**:
   - Sets up Rust toolchain with wasm32-wasip1 target
   - Builds the plugin using `cargo build --release --target wasm32-wasip1`
   - Copies the built `.wasm` file to the root directory
3. **Version synchronization**:
   - Extracts version from git tag
   - Updates both `package.json` and `Cargo.toml` to match
4. **Testing**:
   - Runs the test suite to ensure everything works
5. **Publishing**:
   - Publishes to npm using the `NPM_TOKEN` secret
   - Creates a GitHub release with the tag

## Troubleshooting

### Build Failures
- Check that Rust toolchain is properly installed
- Verify wasm32-wasip1 target is available
- Ensure all dependencies are correctly specified

### NPM Publishing Failures
- Verify `NPM_TOKEN` secret is correctly set
- Check that the package name is available on npm
- Ensure version number is higher than the current published version

### Version Mismatch
- The workflow automatically syncs versions between `package.json` and `Cargo.toml`
- If there are conflicts, the workflow will fail and you'll need to fix them manually

## Rollback Process

If a release has issues:

1. **Unpublish from npm** (if within 24 hours):
   ```bash
   npm unpublish swc-plugin-react-intl-auto@1.0.1
   ```

2. **Delete the git tag**:
   ```bash
   git tag -d v1.0.1
   git push origin :refs/tags/v1.0.1
   ```

3. **Revert version changes**:
   ```bash
   git revert <commit-hash>
   git push origin main
   ```

## Token Management

### NPM Token 90-Day Expiration

NPM tokens expire after 90 days, which can cause publishing failures. Here are several solutions:

#### Solution 1: Manual Token Renewal (Recommended)
1. **Set up a calendar reminder** for every 80 days
2. **Create a new token** before the current one expires:
   - Go to [npmjs.com](https://www.npmjs.com) → Access Tokens
   - Generate a new "Automation" token
   - Update the `NPM_TOKEN` secret in GitHub repository settings
3. **Test the new token** by triggering a manual release

#### Solution 2: Use GitHub Packages (Alternative)
Instead of publishing to npm, you can publish to GitHub Packages:

1. **Update the publish workflow** to use GitHub Packages:
   ```yaml
   - name: Publish to GitHub Packages
     run: npm publish
     env:
       NODE_AUTH_TOKEN: ${{ secrets.GITHUB_TOKEN }}
   ```

2. **Update package.json** to include GitHub Packages registry:
   ```json
   {
     "publishConfig": {
       "registry": "https://npm.pkg.github.com"
     }
   }
   ```

#### Solution 3: Automated Token Renewal Script
Create a script to check token expiration and send notifications:

```bash
#!/bin/bash
# scripts/check-npm-token.sh
# Add this to a cron job or GitHub Actions scheduled workflow

TOKEN_EXPIRY=$(curl -s -H "Authorization: Bearer $NPM_TOKEN" \
  https://registry.npmjs.org/-/whoami | jq -r '.expires')

if [ "$TOKEN_EXPIRY" != "null" ]; then
  EXPIRY_DATE=$(date -d "$TOKEN_EXPIRY" +%s)
  CURRENT_DATE=$(date +%s)
  DAYS_LEFT=$(( (EXPIRY_DATE - CURRENT_DATE) / 86400 ))
  
  if [ $DAYS_LEFT -lt 30 ]; then
    echo "⚠️ NPM token expires in $DAYS_LEFT days"
    # Send notification (email, Slack, etc.)
  fi
fi
```

#### Solution 4: Use NPM Automation Tokens with Longer Expiry
- Use "Publish" tokens instead of "Automation" tokens when possible
- Publish tokens can have longer expiration periods
- Consider using organization-level tokens if you have an npm organization

### Token Renewal Checklist

When renewing your NPM token:

1. ✅ Create new token in npm dashboard
2. ✅ Update `NPM_TOKEN` secret in GitHub repository
3. ✅ Test with a patch release (e.g., `npm run release 1.0.1`)
4. ✅ Verify the package appears on npmjs.com
5. ✅ Update your calendar reminder for the next renewal

## Best Practices

1. **Always test locally** before releasing
2. **Use semantic versioning** (semver)
3. **Write meaningful commit messages**
4. **Update CHANGELOG.md** if you maintain one
5. **Test the published package** after release
6. **Monitor the GitHub Actions logs** for any issues
7. **Set up token expiration reminders** to avoid publishing failures

## Monitoring

- Check the Actions tab in your GitHub repository to monitor workflow runs
- Monitor npm package downloads and any issues reported by users
- Keep an eye on the GitHub releases page for successful releases
- Set up notifications for token expiration warnings
