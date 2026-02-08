#!/usr/bin/env node

const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

const packageJsonPath = path.join(__dirname, '..', 'package.json');
const cargoTomlPath = path.join(__dirname, '..', 'Cargo.toml');

function getCurrentVersion() {
    const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, 'utf8'));
    return packageJson.version;
}

function updateVersion(newVersion) {
    // Update package.json
    const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, 'utf8'));
    packageJson.version = newVersion;
    fs.writeFileSync(
        packageJsonPath,
        JSON.stringify(packageJson, null, 2) + '\n',
    );

    // Update Cargo.toml
    let cargoToml = fs.readFileSync(cargoTomlPath, 'utf8');
    cargoToml = cargoToml.replace(
        /^version = ".*"$/m,
        `version = "${newVersion}"`,
    );
    fs.writeFileSync(cargoTomlPath, cargoToml);

    console.log(
        `✅ Updated version to ${newVersion} in both package.json and Cargo.toml`,
    );
}

function createGitTag(version) {
    const tagName = `v${version}`;
    try {
        execSync(`git tag -a ${tagName} -m "Release ${version}"`, {
            stdio: 'inherit',
        });
        console.log(`✅ Created git tag: ${tagName}`);
    } catch (error) {
        console.error(`❌ Failed to create git tag: ${error.message}`);
        process.exit(1);
    }
}

function pushToGit() {
    try {
        execSync('git add package.json Cargo.toml', { stdio: 'inherit' });
        execSync(
            `git commit -m "chore: bump version to ${getCurrentVersion()}"`,
            { stdio: 'inherit' },
        );
        execSync('git push origin main', { stdio: 'inherit' });
        execSync(`git push origin v${getCurrentVersion()}`, {
            stdio: 'inherit',
        });
        console.log('✅ Pushed changes and tag to GitHub');
    } catch (error) {
        console.error(`❌ Failed to push to git: ${error.message}`);
        process.exit(1);
    }
}

function main() {
    const args = process.argv.slice(2);

    if (args.length === 0) {
        console.log('Usage: node scripts/release.js <version>');
        console.log('Example: node scripts/release.js 1.0.1');
        process.exit(1);
    }

    const newVersion = args[0];

    // Validate version format (basic semver check)
    if (!/^\d+\.\d+\.\d+/.test(newVersion)) {
        console.error(
            '❌ Invalid version format. Use semantic versioning (e.g., 1.0.1)',
        );
        process.exit(1);
    }

    console.log(`🚀 Releasing version ${newVersion}...`);

    // Check if working directory is clean
    try {
        const status = execSync('git status --porcelain', { encoding: 'utf8' });
        if (status.trim()) {
            console.error(
                '❌ Working directory is not clean. Please commit or stash changes first.',
            );
            process.exit(1);
        }
    } catch (error) {
        console.error('❌ Failed to check git status:', error.message);
        process.exit(1);
    }

    updateVersion(newVersion);
    createGitTag(newVersion);

    console.log('\n📝 Next steps:');
    console.log('1. Review the changes');
    console.log(
        '2. Run: git push origin main && git push origin v' + newVersion,
    );
    console.log('3. GitHub Actions will automatically publish to npm');
}

main();
