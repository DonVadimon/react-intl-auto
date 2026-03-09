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
            title: 'removePrefix = "tests"',
            tests,
            pluginOptions: { removePrefix: 'tests' },
        },

        {
            title: 'removePrefix = "tests/"',
            tests,
            pluginOptions: { removePrefix: 'tests/' },
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
                removePrefix: /tests[\\/]__f.+?_/u,
            },
        },

        {
            title: 'removePrefix = "tests.__fixtures__"',
            tests,
            pluginOptions: {
                removePrefix: 'tests.__fixtures__',
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
            title: 'hashAlgorithm = "murmur3"',
            tests,
            pluginOptions: { hashId: true, hashAlgorithm: 'murmur3' },
        },

        {
            title: 'hashAlgorithm = "base64"',
            tests,
            pluginOptions: { hashId: true, hashAlgorithm: 'base64' },
        },

        {
            title: 'hashAlgorithm = "unknown"',
            tests,
            pluginOptions: { hashId: true, hashAlgorithm: 'unknown' as any },
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
        const patch =
            value === true && key !== 'removePrefix'
                ? [arg]
                : [arg, `${value}`];
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

function extractDefineMessagesCalls(code: string) {
    const calls = [] as string[];
    const regex =
        /(defineMessages|formatMessage|testDefineMessages)\s*\(([^)]*)\)/g;
    let match;
    while ((match = regex.exec(code)) !== null) {
        if (match[2]) {
            calls.push(match[2]);
        }
    }

    return calls;
}

function extractDefineMessagesIds(code: string) {
    const ids: string[] = [];
    const calls = extractDefineMessagesCalls(code);

    for (const call of calls) {
        // Match both "id": "value" and id: "value"
        const regex = /(?:"id"|\bid|'id')\s*:\s*("|')([^"']+)("|')/g;
        let match;

        while ((match = regex.exec(call)) !== null) {
            if (match[2]) {
                ids.push(match[2]);
            }
        }
    }

    return ids;
}

function extractJsxMessagesCalls(code: string) {
    const calls = [] as string[];
    const regex =
        /_jsxs?\s*\((FormattedMessage|FormattedHTMLMessage|TestFormattedMessage)([^)]*)\)/g;

    let match;
    while ((match = regex.exec(code)) !== null) {
        if (match[2]) {
            calls.push(match[2]);
        }
    }

    return calls;
}

function extractJsxMessagesIds(code: string) {
    const ids: string[] = [];
    const calls = extractJsxMessagesCalls(code);

    for (const call of calls) {
        // Match both "id": "value" and id: "value"
        const regex = /(?:"id"|\bid|'id')\s*:\s*("|')([^"']+)("|')/g;
        let match;

        while ((match = regex.exec(call)) !== null) {
            if (match[2]) {
                ids.push(match[2]);
            }
        }
    }

    return ids;
}

/**
 * Extract IDs from transformed code
 */
export function extractIdsFromCode(code: string): string[] {
    return [...extractDefineMessagesIds(code), ...extractJsxMessagesIds(code)];
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

const CLI_CONSISTENCY_SKIP = [
    /**
     * extractIdsFromCode вытягивает айди, тк он не проверяет, что функции
     * импортнуты не из gatsby-plugin-intl, а cli не вытягивает
     */
    {
        suite: /cli consistency - components | CONFIGURATION: moduleSourceName = "gatsby-plugin-intl"/,
        tests: [/user id/],
    },
    /**
     * extractIdsFromCode вытягивает айди, тк он не проверяет, что функции
     * импортнуты не из gatsby-plugin-intl, а cli не вытягивает
     */
    {
        suite: /cli consistency - definition | CONFIGURATION: moduleSourceName = "gatsby-plugin-intl"/,
        tests: [/leading comment/, /Object/],
    },
] as const;

const isSkipped = (suite: TestSuite, test: TestCase) => {
    return CLI_CONSISTENCY_SKIP.some(
        (skip) =>
            skip.suite.test(suite.title) &&
            skip.tests.some((regexp) => regexp.test(test.title)),
    );
};

export const cliConsistencyCases = async (suites: TestSuite[]) => {
    for (const suite of suites) {
        describe(`cli consistency - ${suite.title}`, () => {
            for (const test of suite.tests) {
                if (isSkipped(suite, test)) {
                    it.skip(test.title, () => {});
                } else {
                    it(test.title, async () => {
                        // Run CLI and Plugin
                        const [cliMessages, transformResult] =
                            await Promise.all([
                                runCli(test, suite),
                                runPlugin(test, suite),
                            ]);

                        // Extract IDs from transformed code
                        const pluginIds = extractIdsFromCode(
                            transformResult.code,
                        ).sort();

                        // Extract IDs from cli messages
                        const cliIds = cliMessages.map(({ id }) => id).sort();

                        // Verify equality
                        expect(pluginIds).toEqual(cliIds);
                    });
                }
            }
        });
    }
};
