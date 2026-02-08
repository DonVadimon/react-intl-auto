#!/usr/bin/env node

const https = require('https');
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

function makeRequest(url, token) {
    return new Promise((resolve, reject) => {
        const options = {
            headers: {
                Authorization: `Bearer ${token}`,
                'User-Agent': 'npm-token-checker/1.0.0',
            },
        };

        https
            .get(url, options, (res) => {
                let data = '';
                res.on('data', (chunk) => (data += chunk));
                res.on('end', () => {
                    try {
                        resolve(JSON.parse(data));
                    } catch (e) {
                        resolve({ error: 'Invalid JSON response' });
                    }
                });
            })
            .on('error', reject);
    });
}

async function checkNpmToken(token) {
    if (!token) {
        log('❌ No NPM token provided', 'red');
        log('Usage: node scripts/check-npm-token.js <token>', 'yellow');
        log('Or set NPM_TOKEN environment variable', 'yellow');
        process.exit(1);
    }

    log('🔍 Checking NPM token...', 'blue');

    try {
        // Check token validity
        const whoami = await makeRequest(
            'https://registry.npmjs.org/-/whoami',
            token,
        );

        if (whoami.error) {
            log('❌ Invalid or expired NPM token', 'red');
            log(
                'Please create a new token at https://www.npmjs.com/settings/tokens',
                'yellow',
            );
            process.exit(1);
        }

        log(`✅ Token is valid for user: ${whoami.username}`, 'green');

        // Check if we can access package info
        const packageInfo = await makeRequest(
            'https://registry.npmjs.org/swc-plugin-react-intl-auto',
            token,
        );

        if (packageInfo.error) {
            log('⚠️ Token cannot access package information', 'yellow');
            log(
                'This might be normal if the package does not exist yet',
                'yellow',
            );
        } else {
            log('✅ Token can access package information', 'green');
        }

        // Check token expiry (if available)
        if (whoami.expires) {
            const expiryDate = new Date(whoami.expires);
            const now = new Date();
            const daysLeft = Math.ceil(
                (expiryDate - now) / (1000 * 60 * 60 * 24),
            );

            log(
                `📅 Token expires on: ${expiryDate.toISOString().split('T')[0]}`,
                'blue',
            );

            if (daysLeft < 0) {
                log('❌ Token has expired!', 'red');
                process.exit(1);
            } else if (daysLeft < 7) {
                log(`⚠️ Token expires in ${daysLeft} days - URGENT!`, 'red');
                log('Please update the NPM_TOKEN secret immediately', 'yellow');
            } else if (daysLeft < 30) {
                log(`⚠️ Token expires in ${daysLeft} days`, 'yellow');
                log('Consider updating the NPM_TOKEN secret soon', 'yellow');
            } else {
                log(`✅ Token is valid for ${daysLeft} more days`, 'green');
            }
        } else {
            log('ℹ️ Token expiry information not available', 'blue');
        }

        // Test publish permissions (dry run)
        log('🧪 Testing publish permissions...', 'blue');

        // This is a simplified test - in reality, you'd need to test with a real package
        const testPackage = await makeRequest(
            'https://registry.npmjs.org/@test-package-that-should-not-exist',
            token,
        );

        if (testPackage.error && testPackage.error.includes('404')) {
            log('✅ Token has proper registry access', 'green');
        } else {
            log('⚠️ Token registry access test inconclusive', 'yellow');
        }
    } catch (error) {
        log(`❌ Error checking token: ${error.message}`, 'red');
        process.exit(1);
    }
}

async function main() {
    const args = process.argv.slice(2);
    const token = args[0] || process.env.NPM_TOKEN;

    await checkNpmToken(token);

    log('\n📋 Next steps:', 'bold');
    log(
        '1. If token is expired, create a new one at https://www.npmjs.com/settings/tokens',
        'blue',
    );
    log(
        '2. Update the NPM_TOKEN secret in your GitHub repository settings',
        'blue',
    );
    log('3. Test the new token with: npm run release 1.0.1', 'blue');
}

main().catch(console.error);
