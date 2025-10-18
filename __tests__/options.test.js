const { transform } = require('@swc/core');

const plugin = require('../index.js');

describe('Plugin Options', () => {
  const transformWithPlugin = async (code, options = {}) => {
    // Convert regex objects to strings for serialization
    const pluginOptions = options.pluginOptions || {};
    const serializedOptions = {};
    
    for (const [key, value] of Object.entries(pluginOptions)) {
      if (value instanceof RegExp) {
        serializedOptions[key] = value.source;
      } else {
        serializedOptions[key] = value;
      }
    }
    
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
            [plugin, serializedOptions]
          ]
        }
      }
    });
    return result.code;
  };

  describe('module_source_name option', () => {
    it('should work with custom module source name', async () => {
      const code = `
        import { FormattedMessage } from 'my-intl';
        
        function App() {
          return <FormattedMessage defaultMessage="Hello World" />;
        }
      `;

      const result = await transformWithPlugin(code, {
        pluginOptions: { module_source_name: 'my-intl' }
      });
      expect(result).toMatchSnapshot();
    });

    it('should work with react-intl by default', async () => {
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

  describe('separator option', () => {
    it('should use custom separator', async () => {
      const code = `
        import { FormattedMessage } from 'react-intl';
        
        function App() {
          return <FormattedMessage defaultMessage="Hello World" />;
        }
      `;

      const result = await transformWithPlugin(code, {
        pluginOptions: { separator: '_' }
      });
      expect(result).toMatchSnapshot();
    });

    it('should use dot separator by default', async () => {
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

  describe('filebase option', () => {
    it('should include filename in id when filebase is true', async () => {
      const code = `
        import { FormattedMessage } from 'react-intl';
        
        function App() {
          return <FormattedMessage defaultMessage="Hello World" />;
        }
      `;

      const result = await transformWithPlugin(code, {
        filename: 'src/components/App.js',
        pluginOptions: { filebase: true }
      });
      expect(result).toMatchSnapshot();
    });

    it('should not include filename in id when filebase is false', async () => {
      const code = `
        import { FormattedMessage } from 'react-intl';
        
        function App() {
          return <FormattedMessage defaultMessage="Hello World" />;
        }
      `;

      const result = await transformWithPlugin(code, {
        filename: 'src/components/App.js',
        pluginOptions: { filebase: false }
      });
      expect(result).toMatchSnapshot();
    });
  });

  describe('remove_prefix option', () => {
    it('should remove string prefix from filename', async () => {
      const code = `
        import { FormattedMessage } from 'react-intl';
        
        function App() {
          return <FormattedMessage defaultMessage="Hello World" />;
        }
      `;

      const result = await transformWithPlugin(code, {
        filename: 'src/components/App.js',
        pluginOptions: { remove_prefix: 'src/' }
      });
      expect(result).toMatchSnapshot();
    });

    it('should remove regex prefix from filename', async () => {
      const code = `
        import { FormattedMessage } from 'react-intl';
        
        function App() {
          return <FormattedMessage defaultMessage="Hello World" />;
        }
      `;

      const result = await transformWithPlugin(code, {
        filename: 'src/components/App.js',
        pluginOptions: { remove_prefix: 'src/.*/' }
      });
      expect(result).toMatchSnapshot();
    });

    it('should remove regex prefix from filename with regex object', async () => {
      const code = `
        import { FormattedMessage } from 'react-intl';
        
        function App() {
          return <FormattedMessage defaultMessage="Hello World" />;
        }
      `;

      const result = await transformWithPlugin(code, {
        filename: 'src/components/App.js',
        pluginOptions: { remove_prefix: /src\/.*\// }
      });
      expect(result).toMatchSnapshot();
    });

    it('should handle remove_prefix with filebase', async () => {
      const code = `
        import { FormattedMessage } from 'react-intl';
        
        function App() {
          return <FormattedMessage defaultMessage="Hello World" />;
        }
      `;

      const result = await transformWithPlugin(code, {
        filename: 'src/components/App.js',
        pluginOptions: { 
          filebase: true,
          remove_prefix: 'src/'
        }
      });
      expect(result).toMatchSnapshot();
    });
  });

  describe('include_export_name option', () => {
    it('should include export name in defineMessages id', async () => {
      const code = `
        import { defineMessages } from 'react-intl';
        
        export const messages = defineMessages({
          hello: {
            defaultMessage: 'Hello World'
          }
        });
      `;

      const result = await transformWithPlugin(code, {
        pluginOptions: { include_export_name: true }
      });
      expect(result).toMatchSnapshot();
    });

    it('should not include export name when include_export_name is false', async () => {
      const code = `
        import { defineMessages } from 'react-intl';
        
        export const messages = defineMessages({
          hello: {
            defaultMessage: 'Hello World'
          }
        });
      `;

      const result = await transformWithPlugin(code, {
        pluginOptions: { include_export_name: false }
      });
      expect(result).toMatchSnapshot();
    });

    it('should handle include_export_name with default export', async () => {
      const code = `
        import { defineMessages } from 'react-intl';
        
        export default defineMessages({
          hello: {
            defaultMessage: 'Hello World'
          }
        });
      `;

      const result = await transformWithPlugin(code, {
        pluginOptions: { include_export_name: true }
      });
      expect(result).toMatchSnapshot();
    });
  });

  describe('use_key option', () => {
    it('should use key attribute when use_key is true', async () => {
      const code = `
        import { FormattedMessage } from 'react-intl';
        
        function App() {
          return <FormattedMessage key="myKey" defaultMessage="Hello World" />;
        }
      `;

      const result = await transformWithPlugin(code, {
        pluginOptions: { use_key: true }
      });
      expect(result).toMatchSnapshot();
    });

    it('should use defaultMessage hash when use_key is false', async () => {
      const code = `
        import { FormattedMessage } from 'react-intl';
        
        function App() {
          return <FormattedMessage key="myKey" defaultMessage="Hello World" />;
        }
      `;

      const result = await transformWithPlugin(code, {
        pluginOptions: { use_key: false }
      });
      expect(result).toMatchSnapshot();
    });

    it('should use key in formatMessage when use_key is true', async () => {
      const code = `
        import { useIntl } from 'react-intl';
        
        function App() {
          const intl = useIntl();
          return intl.formatMessage({
            key: 'myKey',
            defaultMessage: 'Hello World'
          });
        }
      `;

      const result = await transformWithPlugin(code, {
        pluginOptions: { use_key: true }
      });
      expect(result).toMatchSnapshot();
    });
  });

  describe('extract_comments option', () => {
    it('should handle extract_comments option', async () => {
      const code = `
        import { FormattedMessage } from 'react-intl';
        
        function App() {
          return (
            <FormattedMessage 
              // This is a comment
              defaultMessage="Hello World" 
            />
          );
        }
      `;

      const result = await transformWithPlugin(code, {
        pluginOptions: { extract_comments: true }
      });
      expect(result).toMatchSnapshot();
    });
  });

  describe('relative_to option', () => {
    it('should handle relative_to option', async () => {
      const code = `
        import { FormattedMessage } from 'react-intl';
        
        function App() {
          return <FormattedMessage defaultMessage="Hello World" />;
        }
      `;

      const result = await transformWithPlugin(code, {
        filename: 'src/components/App.js',
        pluginOptions: { relative_to: 'src' }
      });
      expect(result).toMatchSnapshot();
    });
  });

  describe('Combined options', () => {
    it('should handle multiple options together', async () => {
      const code = `
        import { FormattedMessage, defineMessages } from 'react-intl';
        
        export const messages = defineMessages({
          hello: {
            defaultMessage: 'Hello World'
          }
        });
        
        function App() {
          return <FormattedMessage key="myKey" defaultMessage="Hello World" />;
        }
      `;

      const result = await transformWithPlugin(code, {
        filename: 'src/components/App.js',
        pluginOptions: {
          filebase: true,
          remove_prefix: 'src/',
          separator: '_',
          use_key: true,
          include_export_name: true
        }
      });
      expect(result).toMatchSnapshot();
    });
  });
});
