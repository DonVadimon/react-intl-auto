import { transform, WasmPlugin } from '@swc/core';
import * as fs from 'fs';
import * as path from 'path';

import { spawn } from 'child_process';

export type PluginOptions = {
    /** Remove prefix from file path */
    removePrefix?: boolean | string | RegExp;
    /** Module source name */
    moduleSourceName?: string;
    /** ID separator */
    separator?: string;
    /** Relative path for ID generation */
    relativeTo?: string;
    /** Apply hash fn to id */
    hashId?: boolean;
    /** Hash fn for id */
    hashAlgorithm?: 'murmur3' | 'base64';
};

export type TestCase = {
    title: string;
    /** Path to fixture file relative to __fixtures__ directory (e.g., 'definition/default.js') */
    fixture: string;
};

export type TestSuite = {
    title: string;
    tests: TestCase[];
    pluginOptions: PluginOptions;
};

export type CliMessage = {
    id: string;
    defaultMessage: string;
    description?: string;
    file?: string;
};

const plugin = require('../index.js');

export const createConfigurationSuites = (title: string, tests: TestCase[]) => {
    const withPrefix = (t: string) => `${title} | CONFIGURATION: ${t} |`;

    const suites: TestSuite[] = [
        { title: 'default', tests, pluginOptions: {} },
        {
            title: 'removePrefix = "src"',
            tests,
            pluginOptions: { removePrefix: 'src' },
        },

        {
            title: 'removePrefix = "src/" -- with slash',
            tests,
            pluginOptions: { removePrefix: 'src/' },
        },

        {
            title: 'removePrefix = true',
            tests,
            pluginOptions: { removePrefix: true },
        },

        {
            title: 'removePrefix = /__fixtures__/',
            tests,
            pluginOptions: {
                removePrefix: /src[\\/]__f.+?_/u,
            },
        },

        {
            title: 'removePrefix = "src.__fixtures__"',
            tests,
            pluginOptions: {
                removePrefix: 'src.__fixtures__',
            },
        },

        {
            title: 'removePrefix = "src.__fixtures__"',
            tests,
            pluginOptions: {
                removePrefix: 'src.__fixtures__',
            },
        },

        {
            title: 'moduleSourceName = "gatsby-plugin-intl"',
            tests,
            pluginOptions: {
                moduleSourceName: 'gatsby-plugin-intl',
            },
        },

        {
            title: 'separator = "_"',
            // tests,
            tests,
            pluginOptions: {
                separator: '_',
            },
        },

        {
            title: 'relativeTo = "../"',
            tests,
            pluginOptions: {
                relativeTo: '..',
            },
        },

        {
            title: 'relativeTo = ""',
            tests,
            pluginOptions: {
                relativeTo: '',
            },
        },

        {
            title: 'hash murmur3',
            tests,
            pluginOptions: { hashId: true, hashAlgorithm: 'murmur3' },
        },

        {
            title: 'hash base64',
            tests,
            pluginOptions: { hashId: true, hashAlgorithm: 'base64' },
        },

        {
            title: 'hash unknown fallback to murmur3',
            tests,
            pluginOptions: { hashId: true, hashAlgorithm: 'unknown' as any },
        },

        {
            title: 'hash + removePrefix = "src/"',
            tests,
            pluginOptions: {
                hashId: true,
                hashAlgorithm: 'base64',
                removePrefix: 'src/',
            },
        },

        {
            title: 'hash + relativeTo = "src"',
            tests,
            pluginOptions: {
                hashId: true,
                hashAlgorithm: 'base64',
                relativeTo: 'src',
            },
        },
    ];

    return suites.map((suite) => ({
        ...suite,
        title: withPrefix(suite.title),
    }));
};

const getFixturePaths = (fixture: string) => {
    const absolute = path.resolve(__dirname, '__fixtures__', fixture);
    const relative = path.relative(process.cwd(), absolute);

    return { absolute, relative };
};

const runPlugin = async (test: TestCase, suite: TestSuite) => {
    // Load fixture content and determine full path
    const { absolute, relative } = getFixturePaths(test.fixture);
    const code = fs.readFileSync(absolute, 'utf-8');

    const options = {
        filename: relative,
        jsc: {
            parser: {
                syntax: 'typescript' as const,
                tsx: true,
                decorators: true,
            },
            transform: {
                react: {
                    runtime: 'automatic' as const,
                },
            },
            experimental: {
                plugins: [[plugin, suite.pluginOptions]] as WasmPlugin[],
            },
        },
        module: {
            type: 'es6' as const,
        },
    };

    return await transform(code, options);
};

const CLI_OUT = path.resolve(__dirname, '.tmp', 'cli-out');
if (fs.existsSync(CLI_OUT)) {
    fs.rmdirSync(CLI_OUT, { recursive: true });
}

const CLI_OPTIONS_MAP: Record<keyof PluginOptions, string> = {
    removePrefix: '--remove-prefix',
    hashAlgorithm: '--hash-algorithm',
    hashId: '--hash-id',
    moduleSourceName: '--module-source-name',
    relativeTo: '--relative-to',
    separator: '--separator',
};

const CLI_PATH = path.resolve(
    __dirname,
    '../target/release/react-intl-extract',
);

/**
 * Run CLI with given options and return extracted messages
 */
const runCli = async (
    test: TestCase,
    suite: TestSuite,
): Promise<CliMessage[]> => {
    const { absolute } = getFixturePaths(test.fixture);
    const jsonName = path
        .basename(absolute)
        .replace(path.extname(absolute), '.json');
    const filename = `${suite.title}_${jsonName}`
        .replace(/\W/gi, '_')
        .toLowerCase();
    const outputFile = path.resolve(CLI_OUT, filename);
    const args: string[] = [absolute, '--output', outputFile];

    Object.entries(suite.pluginOptions).forEach((entry) => {
        const [key, value] = entry as [keyof PluginOptions, any];
        const arg = CLI_OPTIONS_MAP[key];
        const patch = value === true ? [arg] : [arg, `${value}`];
        args.push(...patch);
    });

    // Always extract source location for consistency
    args.push('--extract-source-location');

    return new Promise((resolve, reject) => {
        const proc = spawn(CLI_PATH, args);
        let stderr = '';

        proc.stderr?.on('data', (data) => {
            stderr += data.toString();
        });

        proc.on('close', (code) => {
            if (code === 0) {
                try {
                    const content = fs.readFileSync(outputFile, 'utf-8');
                    const messages: CliMessage[] = JSON.parse(content);
                    resolve(messages);
                } catch (error) {
                    reject(new Error(`Failed to read CLI output: ${error}`));
                }
            } else {
                reject(
                    new Error(
                        `CLI exited with code ${code}. stderr: ${stderr}`,
                    ),
                );
            }
        });
    });
};

/**
 * Extract IDs from transformed code
 * Handles both quoted and unquoted property names:
 * - "id": "value" (quoted)
 * - id: "value" (unquoted, like in JSX output)
 */
function extractIdsFromCode(code: string): string[] {
    const ids: string[] = [];
    // Match both "id": "value" and id: "value"
    const regex = /(?:"id"|\bid)\s*:\s*"([^"]+)"/g;
    let match;

    while ((match = regex.exec(code)) !== null) {
        ids.push(match[1]!);
    }

    return ids;
}

export const snapCases = async (suites: TestSuite[]) => {
    for (const suite of suites) {
        describe(`snap - ${suite.title}`, () => {
            for (const test of suite.tests) {
                it(test.title, async () => {
                    const result = await runPlugin(test, suite);
                    expect(result.code).toMatchSnapshot();
                });
            }
        });
    }
};

export const cliConsistencyCases = async (suites: TestSuite[]) => {
    for (const suite of suites) {
        describe(`cli consistency - ${suite.title}`, () => {
            for (const test of suite.tests) {
                it(test.title, async () => {
                    // Run CLI and Plugin
                    const [cliMessages, transformResult] = await Promise.all([
                        runCli(test, suite),
                        runPlugin(test, suite),
                    ]);

                    // Extract IDs from transformed code
                    const pluginIds = extractIdsFromCode(transformResult.code);

                    // Extract IDs from cli messages
                    const cliIds = cliMessages.map(({ id }) => id);

                    // Verify equality
                    expect(pluginIds).toEqual(cliIds);
                });
            }
        });
    }
};
