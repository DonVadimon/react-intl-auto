const { transform } = require('@swc/core');

const plugin = require('../index.js');

describe('formatMessage', () => {
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

    describe('injectIntl formatMessage', () => {
        describe('object no id', () => {
            it('should add generated id to formatMessage object without id', async () => {
                const code = `
          import { injectIntl } from 'react-intl';
          
          function MyComponent({ intl }) {
            return intl.formatMessage({
              defaultMessage: 'Привет'
            });
          }
          
          export default injectIntl(MyComponent);
        `;

                const result = await transformWithPlugin(code);
                // Verify id is generated
                expect(result).toMatch(/["']id["']/);
                expect(result).toContain('Привет');
                expect(result).toMatchSnapshot();
            });
        });

        describe('object with id', () => {
            it('should preserve existing user id in formatMessage object', async () => {
                const code = `
          import { injectIntl } from 'react-intl';
          
          function MyComponent({ intl }) {
            return intl.formatMessage({
              id: 'my-id',
              defaultMessage: 'Привет'
            });
          }
          
          export default injectIntl(MyComponent);
        `;

                const result = await transformWithPlugin(code);
                // Verify user id is preserved
                expect(result).toContain('my-id');
                expect(result).toContain('Привет');
                expect(result).toMatchSnapshot();
            });
        });

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

        it('should add id to formatMessage calls with values', async () => {
            const code = `
        import { injectIntl } from 'react-intl';
        
        function MyComponent({ intl, name }) {
          return intl.formatMessage({
            defaultMessage: 'Hello {name}',
            values: { name }
          });
        }
        
        export default injectIntl(MyComponent);
      `;

            const result = await transformWithPlugin(code);
            expect(result).toMatchSnapshot();
        });

        it('should add id to formatMessage calls with description', async () => {
            const code = `
        import { injectIntl } from 'react-intl';
        
        function MyComponent({ intl }) {
          return intl.formatMessage({
            defaultMessage: 'Hello World',
            description: 'A greeting message'
          });
        }
        
        export default injectIntl(MyComponent);
      `;

            const result = await transformWithPlugin(code);
            expect(result).toMatchSnapshot();
        });

        it('should not add id to formatMessage calls that already have id', async () => {
            const code = `
        import { injectIntl } from 'react-intl';
        
        function MyComponent({ intl }) {
          return intl.formatMessage({
            id: 'existing.id',
            defaultMessage: 'Hello World'
          });
        }
        
        export default injectIntl(MyComponent);
      `;

            const result = await transformWithPlugin(code);
            expect(result).toMatchSnapshot();
        });
    });

    describe('useIntl formatMessage', () => {
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

        it('should add id to useIntl formatMessage calls with values', async () => {
            const code = `
        import { useIntl } from 'react-intl';
        
        function MyComponent({ name }) {
          const intl = useIntl();
          return intl.formatMessage({
            defaultMessage: 'Hello {name}',
            values: { name }
          });
        }
      `;

            const result = await transformWithPlugin(code);
            expect(result).toMatchSnapshot();
        });

        it('should handle multiple useIntl formatMessage calls', async () => {
            const code = `
        import { useIntl } from 'react-intl';
        
        function MyComponent() {
          const intl = useIntl();
          return (
            <div>
              {intl.formatMessage({ defaultMessage: 'First message' })}
              {intl.formatMessage({ defaultMessage: 'Second message' })}
            </div>
          );
        }
      `;

            const result = await transformWithPlugin(code);
            expect(result).toMatchSnapshot();
        });
    });

    describe('formatMessage with different patterns', () => {
        it('should handle formatMessage with template literals', async () => {
            const code = `
        import { useIntl } from 'react-intl';
        
        function MyComponent({ name }) {
          const intl = useIntl();
          return intl.formatMessage({
            defaultMessage: \`Hello \${name}\`
          });
        }
      `;

            const result = await transformWithPlugin(code);
            expect(result).toMatchSnapshot();
        });

        it('should handle formatMessage with dynamic defaultMessage', async () => {
            const code = `
        import { useIntl } from 'react-intl';
        
        function MyComponent({ message }) {
          const intl = useIntl();
          return intl.formatMessage({
            defaultMessage: message
          });
        }
      `;

            const result = await transformWithPlugin(code);
            expect(result).toMatchSnapshot();
        });

        it('should handle formatMessage with computed properties', async () => {
            const code = `
        import { useIntl } from 'react-intl';
        
        function MyComponent() {
          const intl = useIntl();
          const messageObj = {
            defaultMessage: 'Hello World'
          };
          return intl.formatMessage(messageObj);
        }
      `;

            const result = await transformWithPlugin(code);
            expect(result).toMatchSnapshot();
        });
    });

    describe('formatMessage edge cases', () => {
        it('should handle formatMessage with empty object', async () => {
            const code = `
        import { useIntl } from 'react-intl';
        
        function MyComponent() {
          const intl = useIntl();
          return intl.formatMessage({});
        }
      `;

            const result = await transformWithPlugin(code);
            expect(result).toMatchSnapshot();
        });

        it('should handle formatMessage with non-string defaultMessage', async () => {
            const code = `
        import { useIntl } from 'react-intl';
        
        function MyComponent() {
          const intl = useIntl();
          return intl.formatMessage({
            defaultMessage: 123
          });
        }
      `;

            const result = await transformWithPlugin(code);
            expect(result).toMatchSnapshot();
        });

        it('should handle formatMessage with missing defaultMessage', async () => {
            const code = `
        import { useIntl } from 'react-intl';
        
        function MyComponent() {
          const intl = useIntl();
          return intl.formatMessage({
            id: 'some.id'
          });
        }
      `;

            const result = await transformWithPlugin(code);
            expect(result).toMatchSnapshot();
        });
    });

    describe('formatMessage with useKey option', () => {
        it('should handle formatMessage with key when useKey is true', async () => {
            const code = `
        import { useIntl } from 'react-intl';
        
        function MyComponent() {
          const intl = useIntl();
          return intl.formatMessage({
            key: 'myKey',
            defaultMessage: 'Hello World'
          });
        }
      `;

            const result = await transformWithPlugin(code, {
                pluginOptions: { use_key: true },
            });
            expect(result).toMatchSnapshot();
        });
    });

    describe('formatMessage in different contexts', () => {
        it('should handle formatMessage in arrow function', async () => {
            const code = `
        import { useIntl } from 'react-intl';
        
        const MyComponent = () => {
          const intl = useIntl();
          return intl.formatMessage({
            defaultMessage: 'Hello World'
          });
        };
      `;

            const result = await transformWithPlugin(code);
            expect(result).toMatchSnapshot();
        });

        it('should handle formatMessage in class method', async () => {
            const code = `
        import { injectIntl } from 'react-intl';
        
        class MyComponent {
          render() {
            return this.props.intl.formatMessage({
              defaultMessage: 'Hello World'
            });
          }
        }
        
        export default injectIntl(MyComponent);
      `;

            const result = await transformWithPlugin(code);
            expect(result).toMatchSnapshot();
        });

        it('should handle formatMessage in callback', async () => {
            const code = `
        import { useIntl } from 'react-intl';
        
        function MyComponent() {
          const intl = useIntl();
          
          const handleClick = () => {
            const message = intl.formatMessage({
              defaultMessage: 'Button clicked'
            });
            console.log(message);
          };
          
          return <button onClick={handleClick}>Click me</button>;
        }
      `;

            const result = await transformWithPlugin(code);
            expect(result).toMatchSnapshot();
        });
    });
});
