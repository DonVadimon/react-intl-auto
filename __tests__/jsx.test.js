const { transform } = require('@swc/core');

const plugin = require('../index.js');

describe('JSX Elements', () => {
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
          plugins: [
            [plugin, options.pluginOptions || {}]
          ]
        }
      }
    });
    return result.code;
  };

  describe('FormattedMessage', () => {
    it('should add id to FormattedMessage with string defaultMessage', async () => {
      const code = `
        import { FormattedMessage } from 'react-intl';
        
        function App() {
          return <FormattedMessage defaultMessage="Hello World" />;
        }
      `;

      const result = await transformWithPlugin(code);
      expect(result).toMatchSnapshot();
    });

    it('should add id to FormattedMessage with template literal defaultMessage', async () => {
      const code = `
        import { FormattedMessage } from 'react-intl';
        
        function App() {
          return <FormattedMessage defaultMessage={\`Hello \${name}\`} />;
        }
      `;

      const result = await transformWithPlugin(code);
      expect(result).toMatchSnapshot();
    });

    it('should handle FormattedMessage with values prop', async () => {
      const code = `
        import { FormattedMessage } from 'react-intl';
        
        function App() {
          return (
            <FormattedMessage 
              defaultMessage="Hello {name}" 
              values={{ name: 'World' }}
            />
          );
        }
      `;

      const result = await transformWithPlugin(code);
      expect(result).toMatchSnapshot();
    });

    it('should handle FormattedMessage with description prop', async () => {
      const code = `
        import { FormattedMessage } from 'react-intl';
        
        function App() {
          return (
            <FormattedMessage 
              defaultMessage="Hello World"
              description="A greeting message"
            />
          );
        }
      `;

      const result = await transformWithPlugin(code);
      expect(result).toMatchSnapshot();
    });

    it('should handle FormattedMessage with all props', async () => {
      const code = `
        import { FormattedMessage } from 'react-intl';
        
        function App() {
          return (
            <FormattedMessage 
              defaultMessage="Hello {name}"
              description="A greeting message"
              values={{ name: 'World' }}
              tagName="span"
            />
          );
        }
      `;

      const result = await transformWithPlugin(code);
      expect(result).toMatchSnapshot();
    });
  });

  describe('FormattedHTMLMessage', () => {
    it('should add id to FormattedHTMLMessage', async () => {
      const code = `
        import { FormattedHTMLMessage } from 'react-intl';
        
        function App() {
          return <FormattedHTMLMessage defaultMessage="<b>Hello World</b>" />;
        }
      `;

      const result = await transformWithPlugin(code);
      expect(result).toMatchSnapshot();
    });

    it('should handle FormattedHTMLMessage with values', async () => {
      const code = `
        import { FormattedHTMLMessage } from 'react-intl';
        
        function App() {
          return (
            <FormattedHTMLMessage 
              defaultMessage="<b>Hello {name}</b>" 
              values={{ name: 'World' }}
            />
          );
        }
      `;

      const result = await transformWithPlugin(code);
      expect(result).toMatchSnapshot();
    });
  });

  describe('Self-closing elements', () => {
    it('should handle self-closing FormattedMessage', async () => {
      const code = `
        import { FormattedMessage } from 'react-intl';
        
        function App() {
          return <FormattedMessage defaultMessage="Hello World" />;
        }
      `;

      const result = await transformWithPlugin(code);
      expect(result).toMatchSnapshot();
    });
  });

  describe('Nested elements', () => {
    it('should handle nested FormattedMessage elements', async () => {
      const code = `
        import { FormattedMessage } from 'react-intl';
        
        function App() {
          return (
            <div>
              <FormattedMessage defaultMessage="Outer message" />
              <span>
                <FormattedMessage defaultMessage="Inner message" />
              </span>
            </div>
          );
        }
      `;

      const result = await transformWithPlugin(code);
      expect(result).toMatchSnapshot();
    });

    it('should handle FormattedMessage inside conditional rendering', async () => {
      const code = `
        import { FormattedMessage } from 'react-intl';
        
        function App({ showMessage }) {
          return (
            <div>
              {showMessage && <FormattedMessage defaultMessage="Conditional message" />}
            </div>
          );
        }
      `;

      const result = await transformWithPlugin(code);
      expect(result).toMatchSnapshot();
    });
  });

  describe('Dynamic attributes', () => {
    it('should handle FormattedMessage with dynamic defaultMessage', async () => {
      const code = `
        import { FormattedMessage } from 'react-intl';
        
        function App({ message }) {
          return <FormattedMessage defaultMessage={message} />;
        }
      `;

      const result = await transformWithPlugin(code);
      expect(result).toMatchSnapshot();
    });

    it('should handle FormattedMessage with computed defaultMessage', async () => {
      const code = `
        import { FormattedMessage } from 'react-intl';
        
        function App() {
          const messages = {
            hello: 'Hello World',
            goodbye: 'Goodbye World'
          };
          return <FormattedMessage defaultMessage={messages.hello} />;
        }
      `;

      const result = await transformWithPlugin(code);
      expect(result).toMatchSnapshot();
    });
  });
});
