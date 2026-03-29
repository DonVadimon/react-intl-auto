#!/usr/bin/env ts-node

import { execSync } from 'node:child_process';
import { parseArgs } from 'node:util';

function getCurrentBranch(): string {
    try {
        return execSync('git branch --show-current', {
            encoding: 'utf-8',
        }).trim();
    } catch (error) {
        throw new Error(
            'Failed to get current git branch. Make sure you are in a git repository.',
        );
    }
}

function bumpVersion(versionType: string): void {
    console.log(`Bumping version with: npm version ${versionType}`);
    execSync(`npm version ${versionType}`, { stdio: 'inherit' });
}

function bumpPreRelease(): void {
    console.log('Bumping pre-release version: npm version prepatch --preid=rc');
    execSync('npm version prepatch --preid=rc', { stdio: 'inherit' });
}

function showHelp(): void {
    console.log(`
Usage: ts-node cli/version.ts [options] [version-type]

Bump npm package version based on current git branch.

Options:
  -h, --help              Show this help message

Arguments:
  version-type            Version bump type: patch, minor, or major
                          Required when on master branch

Behavior:
  - On master branch: requires version-type argument
    Examples:
      ts-node cli/version.ts patch    # 1.0.0 -> 1.0.1
      ts-node cli/version.ts minor    # 1.0.0 -> 1.1.0
      ts-node cli/version.ts major    # 1.0.0 -> 2.0.0

  - On other branches: creates pre-release version
    Example:
      ts-node cli/version.ts          # 1.0.0 -> 1.0.1-rc.0

Examples:
  ts-node cli/version.ts --help
  ts-node cli/version.ts patch
  ts-node cli/version.ts minor
`);
}

function main(): void {
    const { values, positionals } = parseArgs({
        options: {
            help: {
                type: 'boolean',
                short: 'h',
            },
        },
        allowPositionals: true,
    });

    if (values.help) {
        showHelp();
        process.exit(0);
    }

    const branch = getCurrentBranch();
    console.log(`Current branch: ${branch}`);

    if (branch === 'master') {
        const versionType = positionals[0];

        if (!versionType) {
            console.error(
                'Error: Version type is required when on master branch.',
            );
            console.error('Usage: ts-node cli/version.ts <patch|minor|major>');
            process.exit(1);
        }

        const validTypes = ['patch', 'minor', 'major'];
        if (!validTypes.includes(versionType)) {
            console.error(`Error: Invalid version type "${versionType}".`);
            console.error(`Valid types are: ${validTypes.join(', ')}`);
            process.exit(1);
        }

        bumpVersion(versionType);
    } else {
        if (positionals.length > 0) {
            console.warn(
                `Warning: Version type argument is ignored on non-master branch.`,
            );
            console.warn(`Creating pre-release version instead.`);
        }

        bumpPreRelease();
    }
}

try {
    main();
} catch (error) {
    if (error instanceof Error) {
        console.error('Error:', error.message);
    } else {
        console.error('Error:', error);
    }
    process.exit(1);
}
