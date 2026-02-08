const { transform } = require('@swc/core');

const plugin = require('../index.js');

describe('Hash ID', () => {
    const transformWithPlugin = async (code, options = {}) => {
        const result = await transform(code, {
            filename: options.filename || 'test.js',
            jsc: {
                parser: {
                    syntax: 'ecmascript',
                    jsx: true,
                },
                transform: {
                    react: {
                        runtime: 'automatic',
                    },
                },
                experimental: {
                    plugins: [[plugin, options.pluginOptions || {}]],
                },
            },
        });
        return result.code;
    };

    describe('Backward compatibility', () => {
        it('should work without hash_id option (default false)', async () => {
            const code = `
        import { FormattedMessage } from 'react-intl';
        
        function App() {
          return <FormattedMessage defaultMessage="Hello World" />;
        }
      `;

            const result = await transformWithPlugin(code);
            expect(result).toMatchSnapshot();
        });

        it('should work with hash_id: false explicitly', async () => {
            const code = `
        import { FormattedMessage } from 'react-intl';
        
        function App() {
          return <FormattedMessage defaultMessage="Hello World" />;
        }
      `;

            const result = await transformWithPlugin(code, {
                pluginOptions: { hashId: false },
            });
            expect(result).toMatchSnapshot();
        });
    });

    describe('Murmur3 hashing', () => {
        it('should hash ID with murmur3 algorithm', async () => {
            const code = `
        import { FormattedMessage } from 'react-intl';
        
        function App() {
          return <FormattedMessage defaultMessage="Hello World" />;
        }
      `;

            const result = await transformWithPlugin(code, {
                pluginOptions: { hashId: true, hashAlgorithm: 'murmur3' },
            });
            expect(result).toMatchSnapshot();
        });

        it('should handle defaultMessage with variables using murmur3', async () => {
            const code = `
        import { FormattedMessage } from 'react-intl';
        
        function App() {
          return <FormattedMessage defaultMessage="Hello {name}" values={{ name: 'World' }} />;
        }
      `;

            const result = await transformWithPlugin(code, {
                pluginOptions: { hashId: true, hashAlgorithm: 'murmur3' },
            });
            expect(result).toMatchSnapshot();
        });
    });

    describe('Base64 hashing', () => {
        it('should hash ID with base64 algorithm', async () => {
            const code = `
        import { FormattedMessage } from 'react-intl';
        
        function App() {
          return <FormattedMessage defaultMessage="Hello World" />;
        }
      `;

            const result = await transformWithPlugin(code, {
                pluginOptions: { hashId: true, hashAlgorithm: 'base64' },
            });
            expect(result).toMatchSnapshot();
        });

        it('should handle defaultMessage with variables using base64', async () => {
            const code = `
        import { FormattedMessage } from 'react-intl';
        
        function App() {
          return <FormattedMessage defaultMessage="Hello {name}" values={{ name: 'World' }} />;
        }
      `;

            const result = await transformWithPlugin(code, {
                pluginOptions: { hashId: true, hashAlgorithm: 'base64' },
            });
            expect(result).toMatchSnapshot();
        });

        it('should produce valid base64 IDs', async () => {
            const code = `
        import { FormattedMessage } from 'react-intl';
        
        function App() {
          return <FormattedMessage defaultMessage="Test message" />;
        }
      `;

            const result = await transformWithPlugin(code, {
                pluginOptions: { hashId: true, hashAlgorithm: 'base64' },
            });

            // Extract ID from the result
            const idMatch = result.match(/id:\s*"([^"]+)"/);
            expect(idMatch).toBeTruthy();

            const id = idMatch[1];
            // Base64 strings should be alphanumeric with +, /, and = padding
            expect(id).toMatch(/^[A-Za-z0-9+/=]+$/);
        });
    });

    describe('formatMessage hashing', () => {
        it('should work with formatMessage and murmur3', async () => {
            const code = `
        import { useIntl } from 'react-intl';
        
        function App() {
          const intl = useIntl();
          return intl.formatMessage({ defaultMessage: 'Hello World' });
        }
      `;

            const result = await transformWithPlugin(code, {
                pluginOptions: { hashId: true, hashAlgorithm: 'murmur3' },
            });
            expect(result).toMatchSnapshot();
        });

        it('should work with formatMessage and base64', async () => {
            const code = `
        import { useIntl } from 'react-intl';
        
        function App() {
          const intl = useIntl();
          return intl.formatMessage({ defaultMessage: 'Hello World' });
        }
      `;

            const result = await transformWithPlugin(code, {
                pluginOptions: { hashId: true, hashAlgorithm: 'base64' },
            });
            expect(result).toMatchSnapshot();
        });
    });

    describe('defineMessages hashing', () => {
        it('should work with defineMessages and murmur3', async () => {
            const code = `
        import { defineMessages } from 'react-intl';
        
        const messages = defineMessages({
          hello: {
            defaultMessage: 'Hello World',
          },
          goodbye: {
            defaultMessage: 'Goodbye World',
          },
        });
        
        export default messages;
      `;

            const result = await transformWithPlugin(code, {
                pluginOptions: { hashId: true, hashAlgorithm: 'murmur3' },
            });
            expect(result).toMatchSnapshot();
        });

        it('should work with defineMessages and base64', async () => {
            const code = `
        import { defineMessages } from 'react-intl';
        
        const messages = defineMessages({
          hello: 'Hello World',
          goodbye: 'Goodbye World',
        });
        
        export default messages;
      `;

            const result = await transformWithPlugin(code, {
                pluginOptions: { hashId: true, hashAlgorithm: 'base64' },
            });
            expect(result).toMatchSnapshot();
        });

        it('should handle shorthand defineMessages with hashing', async () => {
            const code = `
        import { defineMessages } from 'react-intl';
        
        const messages = defineMessages({
          greeting: "Привет мир",
        });
      `;

            const result = await transformWithPlugin(code, {
                pluginOptions: { hashId: true, hashAlgorithm: 'base64' },
            });
            expect(result).toMatchSnapshot();
        });

        it('should handle numeric keys with hashing', async () => {
            const code = `
        import { defineMessages } from 'react-intl';
        
        const messages = defineMessages({
          0: "First",
          1: "Second",
        });
      `;

            const result = await transformWithPlugin(code, {
                pluginOptions: { hashId: true, hashAlgorithm: 'base64' },
            });
            expect(result).toMatchSnapshot();
        });
    });

    describe('Combined options', () => {
        it('should work with removePrefix and hash_id', async () => {
            const code = `
        import { FormattedMessage } from 'react-intl';
        
        function App() {
          return <FormattedMessage defaultMessage="Hello World" />;
        }
      `;

            const result = await transformWithPlugin(code, {
                filename: 'src/components/App.js',
                pluginOptions: {
                    hashId: true,
                    hashAlgorithm: 'base64',
                    removePrefix: 'src/',
                },
            });
            expect(result).toMatchSnapshot();
        });

        it('should work with includeExportName and hash_id', async () => {
            const code = `
        import { defineMessages } from 'react-intl';
        
        const messages = defineMessages({
          hello: 'Hello World',
        });
        
        export default messages;
      `;

            const result = await transformWithPlugin(code, {
                pluginOptions: {
                    hashId: true,
                    hashAlgorithm: 'base64',
                    includeExportName: true,
                },
            });
            expect(result).toMatchSnapshot();
        });

        it('should work with filebase and hash_id', async () => {
            const code = `
        import { FormattedMessage } from 'react-intl';
        
        function App() {
          return <FormattedMessage defaultMessage="Hello World" />;
        }
      `;

            const result = await transformWithPlugin(code, {
                filename: 'src/components/App.js',
                pluginOptions: {
                    hashId: true,
                    hashAlgorithm: 'base64',
                    filebase: true,
                },
            });
            expect(result).toMatchSnapshot();
        });

        it('should work with relativeTo and hash_id', async () => {
            const code = `
        import { FormattedMessage } from 'react-intl';
        
        function App() {
          return <FormattedMessage defaultMessage="Hello World" />;
        }
      `;

            const result = await transformWithPlugin(code, {
                filename: 'src/components/App.js',
                pluginOptions: {
                    hashId: true,
                    hashAlgorithm: 'base64',
                    relativeTo: 'src',
                },
            });
            expect(result).toMatchSnapshot();
        });
    });

    describe('Unknown algorithm fallback', () => {
        it('should fallback to murmur3 for unknown algorithm', async () => {
            const code = `
        import { FormattedMessage } from 'react-intl';
        
        function App() {
          return <FormattedMessage defaultMessage="Hello World" />;
        }
      `;

            const result = await transformWithPlugin(code, {
                pluginOptions: { hashId: true, hashAlgorithm: 'unknown' },
            });
            expect(result).toMatchSnapshot();
        });
    });
});
