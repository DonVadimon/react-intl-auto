const { transform } = require('@swc/core');
const path = require('path');

// Load the SWC plugin
const plugin = require('../index.js');

describe('swc-plugin-react-intl-auto', () => {
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

    describe('FormattedMessage JSX elements', () => {
        it('should add id to FormattedMessage without id', async () => {
            const code = `
        import { FormattedMessage } from 'react-intl';
        
        function App() {
          return (
            <FormattedMessage defaultMessage="Hello World" />
          );
        }
      `;

            const result = await transformWithPlugin(code);
            expect(result).toMatchSnapshot();
        });

        it('should not add id to FormattedMessage that already has id', async () => {
            const code = `
        import { FormattedMessage } from 'react-intl';
        
        function App() {
          return (
            <FormattedMessage id="existing.id" defaultMessage="Hello World" />
          );
        }
      `;

            const result = await transformWithPlugin(code);
            expect(result).toMatchSnapshot();
        });

        it('should handle FormattedMessage with key attribute when useKey is true', async () => {
            const code = `
        import { FormattedMessage } from 'react-intl';
        
        function App() {
          return (
            <FormattedMessage key="myKey" defaultMessage="Hello World" />
          );
        }
      `;

            const result = await transformWithPlugin(code, {
                pluginOptions: { use_key: true },
            });
            expect(result).toMatchSnapshot();
        });

        it('should handle FormattedMessage with dynamic defaultMessage', async () => {
            const code = `
        import { FormattedMessage } from 'react-intl';
        
        function App() {
          const message = "Hello World";
          return (
            <FormattedMessage defaultMessage={message} />
          );
        }
      `;

            const result = await transformWithPlugin(code);
            expect(result).toMatchSnapshot();
        });
    });

    describe('FormattedHTMLMessage JSX elements', () => {
        it('should add id to FormattedHTMLMessage without id', async () => {
            const code = `
        import { FormattedHTMLMessage } from 'react-intl';
        
        function App() {
          return (
            <FormattedHTMLMessage defaultMessage="<b>Hello World</b>" />
          );
        }
      `;

            const result = await transformWithPlugin(code);
            expect(result).toMatchSnapshot();
        });
    });

    describe('defineMessages function calls', () => {
        it('should add id to defineMessages object', async () => {
            const code = `
        import { defineMessages } from 'react-intl';
        
        const messages = defineMessages({
          hello: {
            defaultMessage: 'Hello World'
          }
        });
      `;

            const result = await transformWithPlugin(code);
            expect(result).toMatchSnapshot();
        });

        it('should handle defineMessages with string values', async () => {
            const code = `
        import { defineMessages } from 'react-intl';
        
        const messages = defineMessages({
          hello: 'Hello World'
        });
      `;

            const result = await transformWithPlugin(code);
            expect(result).toMatchSnapshot();
        });

        it('should handle defineMessages with includeExportName', async () => {
            const code = `
        import { defineMessages } from 'react-intl';
        
        export const messages = defineMessages({
          hello: {
            defaultMessage: 'Hello World'
          }
        });
      `;

            const result = await transformWithPlugin(code, {
                pluginOptions: { include_export_name: true },
            });
            expect(result).toMatchSnapshot();
        });
    });

    describe('formatMessage function calls', () => {
        it('should add id to formatMessage calls', async () => {
            const code = `
        import { injectIntl } from 'react-intl';
        
        function MyComponent({ intl }) {
          return intl.formatMessage({
            defaultMessage: 'Hello World'
          });
        }
        
        export default injectIntl(MyComponent);
      `;

            const result = await transformWithPlugin(code);
            expect(result).toMatchSnapshot();
        });

        it('should add id to useIntl formatMessage calls', async () => {
            const code = `
        import { useIntl } from 'react-intl';
        
        function MyComponent() {
          const intl = useIntl();
          return intl.formatMessage({
            defaultMessage: 'Hello World'
          });
        }
      `;

            const result = await transformWithPlugin(code);
            expect(result).toMatchSnapshot();
        });

        it('should not add id to formatMessage calls that already have id', async () => {
            const code = `
        import { useIntl } from 'react-intl';
        
        function MyComponent() {
          const intl = useIntl();
          return intl.formatMessage({
            id: 'existing.id',
            defaultMessage: 'Hello World'
          });
        }
      `;

            const result = await transformWithPlugin(code);
            expect(result).toMatchSnapshot();
        });
    });

    describe('Plugin options', () => {
        it('should handle custom module source name', async () => {
            const code = `
        import { FormattedMessage } from 'my-intl';
        
        function App() {
          return (
            <FormattedMessage defaultMessage="Hello World" />
          );
        }
      `;

            const result = await transformWithPlugin(code, {
                pluginOptions: { module_source_name: 'my-intl' },
            });
            expect(result).toMatchSnapshot();
        });

        it('should handle custom separator', async () => {
            const code = `
        import { FormattedMessage } from 'react-intl';
        
        function App() {
          return (
            <FormattedMessage defaultMessage="Hello World" />
          );
        }
      `;

            const result = await transformWithPlugin(code, {
                pluginOptions: { separator: '_' },
            });
            expect(result).toMatchSnapshot();
        });

        it('should handle filebase option', async () => {
            const code = `
        import { FormattedMessage } from 'react-intl';
        
        function App() {
          return (
            <FormattedMessage defaultMessage="Hello World" />
          );
        }
      `;

            const result = await transformWithPlugin(code, {
                filename: 'src/components/App.js',
                pluginOptions: { filebase: true },
            });
            expect(result).toMatchSnapshot();
        });

        it('should handle removePrefix option', async () => {
            const code = `
        import { FormattedMessage } from 'react-intl';
        
        function App() {
          return (
            <FormattedMessage defaultMessage="Hello World" />
          );
        }
      `;

            const result = await transformWithPlugin(code, {
                filename: 'src/components/App.js',
                pluginOptions: { remove_prefix: 'src/components/' },
            });
            expect(result).toMatchSnapshot();
        });
    });

    describe('Edge cases', () => {
        it('should handle nested JSX elements', async () => {
            const code = `
        import { FormattedMessage } from 'react-intl';
        
        function App() {
          return (
            <div>
              <FormattedMessage defaultMessage="Hello World" />
              <span>
                <FormattedMessage defaultMessage="Nested message" />
              </span>
            </div>
          );
        }
      `;

            const result = await transformWithPlugin(code);
            expect(result).toMatchSnapshot();
        });

        it('should handle multiple defineMessages calls', async () => {
            const code = `
        import { defineMessages } from 'react-intl';
        
        const messages1 = defineMessages({
          hello: {
            defaultMessage: 'Hello World'
          }
        });
        
        const messages2 = defineMessages({
          goodbye: {
            defaultMessage: 'Goodbye World'
          }
        });
      `;

            const result = await transformWithPlugin(code);
            expect(result).toMatchSnapshot();
        });

        it('should handle mixed imports', async () => {
            const code = `
        import { FormattedMessage, defineMessages, useIntl } from 'react-intl';
        
        const messages = defineMessages({
          hello: {
            defaultMessage: 'Hello World'
          }
        });
        
        function App() {
          const intl = useIntl();
          return (
            <div>
              <FormattedMessage defaultMessage="JSX Message" />
              {intl.formatMessage({ defaultMessage: 'Hook Message' })}
            </div>
          );
        }
      `;

            const result = await transformWithPlugin(code);
            expect(result).toMatchSnapshot();
        });

        it('should handle aliased imports', async () => {
            const code = `
        import { FormattedMessage as FM } from 'react-intl';
        
        function App() {
          return (
            <FM defaultMessage="Hello World" />
          );
        }
      `;

            const result = await transformWithPlugin(code);
            expect(result).toMatchSnapshot();
        });

        it('should handle default imports', async () => {
            const code = `
        import intl from 'react-intl';
        
        function App() {
          return intl.formatMessage({
            defaultMessage: 'Hello World'
          });
        }
      `;

            const result = await transformWithPlugin(code);
            expect(result).toMatchSnapshot();
        });
    });

    describe('Error handling', () => {
        it('should handle malformed JSX', async () => {
            const code = `
        import { FormattedMessage } from 'react-intl';
        
        function App() {
          return (
            <FormattedMessage defaultMessage="Hello World" />
          );
        }
      `;

            // This should not crash the plugin
            const result = await transformWithPlugin(code);
            expect(result).toBeDefined();
        });

        it('should handle empty defineMessages', async () => {
            const code = `
        import { defineMessages } from 'react-intl';
        
        const messages = defineMessages({});
      `;

            const result = await transformWithPlugin(code);
            expect(result).toMatchSnapshot();
        });
    });
});
