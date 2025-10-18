const { transform } = require('@swc/core');

const plugin = require('../index.js');

describe('defineMessages', () => {
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

  describe('Basic defineMessages', () => {
    it('should add id to defineMessages with object values', async () => {
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

    it('should add id to defineMessages with string values', async () => {
      const code = `
        import { defineMessages } from 'react-intl';
        
        const messages = defineMessages({
          hello: 'Hello World'
        });
      `;

      const result = await transformWithPlugin(code);
      expect(result).toMatchSnapshot();
    });

    it('should handle defineMessages with multiple keys', async () => {
      const code = `
        import { defineMessages } from 'react-intl';
        
        const messages = defineMessages({
          hello: {
            defaultMessage: 'Hello World'
          },
          goodbye: {
            defaultMessage: 'Goodbye World'
          }
        });
      `;

      const result = await transformWithPlugin(code);
      expect(result).toMatchSnapshot();
    });

    it('should handle defineMessages with mixed string and object values', async () => {
      const code = `
        import { defineMessages } from 'react-intl';
        
        const messages = defineMessages({
          hello: 'Hello World',
          goodbye: {
            defaultMessage: 'Goodbye World'
          }
        });
      `;

      const result = await transformWithPlugin(code);
      expect(result).toMatchSnapshot();
    });
  });

  describe('defineMessages with additional properties', () => {
    it('should handle defineMessages with description', async () => {
      const code = `
        import { defineMessages } from 'react-intl';
        
        const messages = defineMessages({
          hello: {
            defaultMessage: 'Hello World',
            description: 'A greeting message'
          }
        });
      `;

      const result = await transformWithPlugin(code);
      expect(result).toMatchSnapshot();
    });

    it('should handle defineMessages with id already present', async () => {
      const code = `
        import { defineMessages } from 'react-intl';
        
        const messages = defineMessages({
          hello: {
            id: 'existing.id',
            defaultMessage: 'Hello World'
          }
        });
      `;

      const result = await transformWithPlugin(code);
      expect(result).toMatchSnapshot();
    });

    it('should handle defineMessages with all properties', async () => {
      const code = `
        import { defineMessages } from 'react-intl';
        
        const messages = defineMessages({
          hello: {
            id: 'custom.id',
            defaultMessage: 'Hello World',
            description: 'A greeting message'
          }
        });
      `;

      const result = await transformWithPlugin(code);
      expect(result).toMatchSnapshot();
    });
  });

  describe('defineMessages with export', () => {
    it('should handle exported defineMessages', async () => {
      const code = `
        import { defineMessages } from 'react-intl';
        
        export const messages = defineMessages({
          hello: {
            defaultMessage: 'Hello World'
          }
        });
      `;

      const result = await transformWithPlugin(code);
      expect(result).toMatchSnapshot();
    });

    it('should handle default export defineMessages', async () => {
      const code = `
        import { defineMessages } from 'react-intl';
        
        export default defineMessages({
          hello: {
            defaultMessage: 'Hello World'
          }
        });
      `;

      const result = await transformWithPlugin(code);
      expect(result).toMatchSnapshot();
    });

    it('should handle defineMessages with includeExportName option', async () => {
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
  });

  describe('defineMessages with complex keys', () => {
    it('should handle defineMessages with string keys', async () => {
      const code = `
        import { defineMessages } from 'react-intl';
        
        const messages = defineMessages({
          'hello.world': {
            defaultMessage: 'Hello World'
          },
          'goodbye.world': {
            defaultMessage: 'Goodbye World'
          }
        });
      `;

      const result = await transformWithPlugin(code);
      expect(result).toMatchSnapshot();
    });

    it('should handle defineMessages with numeric keys', async () => {
      const code = `
        import { defineMessages } from 'react-intl';
        
        const messages = defineMessages({
          0: {
            defaultMessage: 'First message'
          },
          1: {
            defaultMessage: 'Second message'
          }
        });
      `;

      const result = await transformWithPlugin(code);
      expect(result).toMatchSnapshot();
    });
  });

  describe('defineMessages edge cases', () => {
    it('should handle empty defineMessages', async () => {
      const code = `
        import { defineMessages } from 'react-intl';
        
        const messages = defineMessages({});
      `;

      const result = await transformWithPlugin(code);
      expect(result).toMatchSnapshot();
    });

    it('should handle defineMessages with non-string values', async () => {
      const code = `
        import { defineMessages } from 'react-intl';
        
        const messages = defineMessages({
          hello: {
            defaultMessage: 123
          }
        });
      `;

      const result = await transformWithPlugin(code);
      expect(result).toMatchSnapshot();
    });

    it('should handle defineMessages with computed keys', async () => {
      const code = `
        import { defineMessages } from 'react-intl';
        
        const key = 'hello';
        const messages = defineMessages({
          [key]: {
            defaultMessage: 'Hello World'
          }
        });
      `;

      const result = await transformWithPlugin(code);
      expect(result).toMatchSnapshot();
    });
  });

  describe('Multiple defineMessages calls', () => {
    it('should handle multiple defineMessages in same file', async () => {
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

    it('should handle defineMessages in different scopes', async () => {
      const code = `
        import { defineMessages } from 'react-intl';
        
        function createMessages() {
          return defineMessages({
            hello: {
              defaultMessage: 'Hello World'
            }
          });
        }
        
        const messages = defineMessages({
          goodbye: {
            defaultMessage: 'Goodbye World'
          }
        });
      `;

      const result = await transformWithPlugin(code);
      expect(result).toMatchSnapshot();
    });

    it('should handle defineMessages with numeric keys', async () => {
      const code = `
        import { defineMessages } from 'react-intl';
        
        const messages = defineMessages({
          1: {
            defaultMessage: 'Message for key 1'
          },
          2: {
            defaultMessage: 'Message for key 2'
          },
          '3': {
            defaultMessage: 'Message for key 3'
          }
        });
      `;

      const result = await transformWithPlugin(code);
      expect(result).toMatchSnapshot();
    });
  });
});
