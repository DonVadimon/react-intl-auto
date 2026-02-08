import { transform, WasmPlugin } from '@swc/core';

type TestCase = {
    title: string;
    code: string;
    error?: RegExp;
    snapshot?: boolean;
};

type PluginOptions = {
    /** Remove prefix from file path */
    removePrefix?: boolean | string | RegExp;
    /** Include filename in ID */
    filebase?: boolean;
    /** Include export name in ID */
    includeExportName?: boolean | 'all';
    /** Extract comments as descriptions */
    extractComments?: boolean;
    /** Use key property instead of hash */
    useKey?: boolean;
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
const filename = 'Users/username/repo/src/components/App.tsx';

export async function cases(suites: TestSuite[]) {
    for (const suite of suites) {
        describe(suite.title, () => {
            for (const test of suite.tests) {
                it(test.title, async () => {
                    const options = {
                        filename,
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

                    const promise = transform(test.code, options);

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
    const withPrefix = (t: string) => `${title} - ${t}`;

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
            title: 'filebase = true',
            tests,
            pluginOptions: { filebase: true },
        },

        {
            title: 'includeExportName = true',
            tests,
            pluginOptions: { includeExportName: true },
        },

        {
            title: 'includeExportName = all',
            tests,
            pluginOptions: { includeExportName: 'all' },
        },

        {
            title: 'removePrefix = true, includeExportName = true',
            tests,
            pluginOptions: { removePrefix: true, includeExportName: true },
        },

        {
            title: 'removePrefix = false',
            tests,
            pluginOptions: { removePrefix: false },
        },

        {
            title: 'removePrefix = true, includeExportName = all',
            tests,
            pluginOptions: { removePrefix: true, includeExportName: 'all' },
        },

        {
            title: 'extractComments = false',
            tests,
            pluginOptions: { extractComments: false },
        },

        {
            title: 'removePrefix = /__fixtures__/',
            tests,
            pluginOptions: {
                removePrefix: /src[\\/]__f.+?_/u,
                includeExportName: true,
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
            title: 'removePrefix = "src.__fixtures__", includeExportName = true',
            tests,
            pluginOptions: {
                removePrefix: 'src.__fixtures__',
                includeExportName: true,
            },
        },

        {
            title: 'moduleSourceNameTest',
            tests,
            pluginOptions: {
                moduleSourceName: 'gatsby-plugin-intl',
            },
        },

        {
            title: 'separator = ""',
            tests,
            pluginOptions: {
                separator: '',
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
            title: 'separator = "foo"',
            tests,
            pluginOptions: {
                separator: 'foo',
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
            title: 'useKey = true',
            tests,
            pluginOptions: {
                useKey: true,
            },
        },

        {
            title: 'extractComments = true',
            tests,
            pluginOptions: {
                extractComments: true,
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
            title: 'hash + includeExportName = true',
            tests,
            pluginOptions: {
                hashId: true,
                hashAlgorithm: 'base64',
                includeExportName: true,
            },
        },

        {
            title: 'hash + filebase = true',
            tests,
            pluginOptions: {
                hashId: true,
                hashAlgorithm: 'base64',
                filebase: true,
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
