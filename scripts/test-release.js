#!/usr/bin/env node

const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

// Colors for console output
const colors = {
    red: '\x1b[31m',
    yellow: '\x1b[33m',
    green: '\x1b[32m',
    blue: '\x1b[34m',
    reset: '\x1b[0m',
    bold: '\x1b[1m',
};

function log(message, color = 'reset') {
    console.log(`${colors[color]}${message}${colors.reset}`);
}

function checkGitHubCLI() {
    try {
        const version = execSync('gh --version', { encoding: 'utf8' });
        log(`✅ GitHub CLI found: ${version.trim()}`, 'green');
        return true;
    } catch (error) {
        log('❌ GitHub CLI not found. Please install it first:', 'red');
        log('  https://cli.github.com/', 'blue');
        return false;
    }
}

function checkGitHubAuth() {
    try {
        const user = execSync('gh auth status', { encoding: 'utf8' });
        log(`✅ GitHub CLI authenticated: ${user.trim()}`, 'green');
        return true;
    } catch (error) {
        log('❌ GitHub CLI not authenticated. Please run:', 'red');
        log('  gh auth login', 'blue');
        return false;
    }
}

function checkRepository() {
    try {
        const repo = execSync('gh repo view --json nameWithOwner', {
            encoding: 'utf8',
        });
        const repoInfo = JSON.parse(repo);
        log(`✅ Repository: ${repoInfo.nameWithOwner}`, 'green');
        return true;
    } catch (error) {
        log(
            '❌ Could not access repository. Make sure you have access.',
            'red',
        );
        return false;
    }
}

function checkWorkflowFiles() {
    const workflowsDir = path.join(__dirname, '..', '.github', 'workflows');
    const requiredFiles = [
        'publish.yml',
        'publish-github-packages.yml',
        'ci.yml',
        'test-matrix.yml',
    ];

    let allExist = true;
    for (const file of requiredFiles) {
        const filePath = path.join(workflowsDir, file);
        if (fs.existsSync(filePath)) {
            log(`✅ Found workflow: ${file}`, 'green');
        } else {
            log(`❌ Missing workflow: ${file}`, 'red');
            allExist = false;
        }
    }
    return allExist;
}

function checkPackageJson() {
    const packageJsonPath = path.join(__dirname, '..', 'package.json');
    if (!fs.existsSync(packageJsonPath)) {
        log('❌ package.json not found', 'red');
        return false;
    }

    const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, 'utf8'));

    // Check required fields
    const requiredFields = ['name', 'version', 'main', 'scripts'];
    let allFieldsExist = true;

    for (const field of requiredFields) {
        if (packageJson[field]) {
            log(`✅ package.json has ${field}`, 'green');
        } else {
            log(`❌ package.json missing ${field}`, 'red');
            allFieldsExist = false;
        }
    }

    // Check scripts
    const requiredScripts = ['build', 'test', 'release'];
    for (const script of requiredScripts) {
        if (packageJson.scripts && packageJson.scripts[script]) {
            log(`✅ Script '${script}' found`, 'green');
        } else {
            log(`❌ Script '${script}' missing`, 'red');
            allFieldsExist = false;
        }
    }

    return allFieldsExist;
}

function checkRustSetup() {
    try {
        const rustVersion = execSync('rustc --version', { encoding: 'utf8' });
        log(`✅ Rust found: ${rustVersion.trim()}`, 'green');
    } catch (error) {
        log('❌ Rust not found. Please install Rust toolchain.', 'red');
        return false;
    }

    try {
        const targets = execSync('rustup target list --installed', {
            encoding: 'utf8',
        });
        if (targets.includes('wasm32-wasip1')) {
            log('✅ wasm32-wasip1 target installed', 'green');
        } else {
            log(
                '⚠️ wasm32-wasip1 target not installed. Run: rustup target add wasm32-wasip1',
                'yellow',
            );
        }
    } catch (error) {
        log('⚠️ Could not check Rust targets', 'yellow');
    }

    return true;
}

function checkNPMToken() {
    const token = process.env.NPM_TOKEN;
    if (token) {
        log('✅ NPM_TOKEN environment variable is set', 'green');
        return true;
    } else {
        log('⚠️ NPM_TOKEN environment variable not set', 'yellow');
        log('  This is required for npm publishing', 'yellow');
        return false;
    }
}

async function main() {
    log('🧪 Testing Release Workflow Setup', 'bold');
    log('================================', 'bold');
    log('');

    let allChecksPassed = true;

    // Check GitHub CLI
    log('📋 Checking GitHub CLI...', 'blue');
    if (!checkGitHubCLI()) {
        allChecksPassed = false;
    }
    log('');

    // Check GitHub authentication
    log('🔐 Checking GitHub authentication...', 'blue');
    if (!checkGitHubAuth()) {
        allChecksPassed = false;
    }
    log('');

    // Check repository access
    log('📁 Checking repository access...', 'blue');
    if (!checkRepository()) {
        allChecksPassed = false;
    }
    log('');

    // Check workflow files
    log('⚙️ Checking workflow files...', 'blue');
    if (!checkWorkflowFiles()) {
        allChecksPassed = false;
    }
    log('');

    // Check package.json
    log('📦 Checking package.json...', 'blue');
    if (!checkPackageJson()) {
        allChecksPassed = false;
    }
    log('');

    // Check Rust setup
    log('🦀 Checking Rust setup...', 'blue');
    if (!checkRustSetup()) {
        allChecksPassed = false;
    }
    log('');

    // Check NPM token
    log('🔑 Checking NPM token...', 'blue');
    if (!checkNPMToken()) {
        allChecksPassed = false;
    }
    log('');

    // Summary
    log('📊 Summary', 'bold');
    log('==========', 'bold');

    if (allChecksPassed) {
        log(
            '🎉 All checks passed! Your release workflow should work correctly.',
            'green',
        );
        log('');
        log('Next steps:', 'blue');
        log('1. Create a test tag: git tag v1.0.1-test', 'blue');
        log('2. Push the tag: git push origin v1.0.1-test', 'blue');
        log('3. Check GitHub Actions for the workflow run', 'blue');
        log(
            '4. Delete the test tag: git tag -d v1.0.1-test && git push origin :v1.0.1-test',
            'blue',
        );
    } else {
        log(
            '❌ Some checks failed. Please fix the issues above before releasing.',
            'red',
        );
        process.exit(1);
    }
}

main().catch(console.error);
