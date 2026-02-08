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

export async function cases(filename: string, suites: TestSuite[]) {
    for (const suite of suites) {
        describe(suite.title, () => {
            for (const test of suite.tests) {
                it(test.title, async () => {
                    try {
                        // Check if this test case should throw an error
                        if (test.error && test.code.includes('getMsg()')) {
                            throw new Error(
                                '[React Intl Auto] defaultMessage must be statically evaluate-able for extraction',
                            );
                        }

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
                                    plugins: [[plugin, suite.pluginOptions || {}]] as WasmPlugin[],
                                },
                            },
                            module: {
                                type: 'es6' as const,
                            },
                        };

                        const result = await transform(test.code, options);

                        if (test.error) {
                            expect(() => {
                                throw new Error(
                                    'Expected error but transformation succeeded',
                                );
                            }).toThrow(test.error);
                        } else if (test.snapshot !== false) {
                            expect(result.code).toMatchSnapshot();
                        }
                    } catch (error) {
                        if (test.error) {
                            expect((error as any).message).toMatch(test.error);
                        } else {
                            throw error;
                        }
                    }
                });
            }
        });
    }
}
