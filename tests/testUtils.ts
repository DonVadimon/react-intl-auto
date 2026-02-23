import { transform, WasmPlugin } from '@swc/core';
import * as fs from 'fs';
import * as path from 'path';

type TestCase = {
    title: string;
    /** Path to fixture file relative to __fixtures__ directory (e.g., 'definition/default.js') */
    fixture: string;
    error?: RegExp;
    snapshot?: boolean;
};

type PluginOptions = {
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

type TestSuite = {
    title: string;
    tests: TestCase[];
    pluginOptions?: PluginOptions;
};

const plugin = require('../index.js');

export async function cases(suites: TestSuite[]) {
    for (const suite of suites) {
        describe(suite.title, () => {
            for (const test of suite.tests) {
                it(test.title, async () => {
                    // Load fixture content and determine full path
                    const fixtureFullPath = path.resolve(
                        __dirname,
                        '__fixtures__',
                        test.fixture,
                    );
                    const fixtureRelativePath = path.relative(
                        process.cwd(),
                        fixtureFullPath,
                    );
                    const code = fs.readFileSync(fixtureFullPath, 'utf-8');

                    const options = {
                        filename: fixtureRelativePath,
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
                                plugins: [
                                    [plugin, suite.pluginOptions || {}],
                                ] as WasmPlugin[],
                            },
                        },
                        module: {
                            type: 'es6' as const,
                        },
                    };

                    const promise = transform(code, options);

                    // если ждем ошибку - проверяем ошибку
                    if (test.error) {
                        await expect(promise).rejects.toBeDefined();
                        await expect(promise).rejects.toMatch(test.error);
                        return;
                    }

                    const result = await promise;
                    if (test.snapshot !== false) {
                        expect(result.code).toMatchSnapshot();
                    }
                });
            }
        });
    }
}

export const createConfigurationSuites = (title: string, tests: TestCase[]) => {
    const withPrefix = (t: string) => `${title} | CONFIGURATION: ${t} |`;

    const suites: TestSuite[] = [
        { title: 'default', tests },
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
