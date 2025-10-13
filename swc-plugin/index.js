const { transform } = require('@swc/core');

/**
 * SWC plugin for automatic react-intl ID management
 * @param {Object} options - Plugin options
 * @param {string|boolean|RegExp} [options.removePrefix] - Remove prefix from file path
 * @param {boolean} [options.filebase=false] - Include filename in ID
 * @param {boolean|'all'} [options.includeExportName=false] - Include export name in ID
 * @param {boolean} [options.extractComments=true] - Extract comments as descriptions
 * @param {boolean} [options.useKey=false] - Use key property instead of hash
 * @param {string} [options.moduleSourceName='react-intl'] - Module source name
 * @param {string} [options.separator='.'] - ID separator
 * @param {string} [options.relativeTo] - Relative path for ID generation
 */
function swcPluginReactIntlAuto(options = {}) {
  return {
    name: 'swc-plugin-react-intl-auto',
    config: () => {
      return {
        jsc: {
          parser: {
            syntax: 'typescript',
            tsx: true,
            decorators: true,
          },
          transform: {
            react: {
              runtime: 'automatic',
            },
          },
          experimental: {
            plugins: [
              [
                require.resolve('./swc_plugin_react_intl_auto.wasm'),
                options
              ]
            ]
          }
        }
      };
    }
  };
}

module.exports = swcPluginReactIntlAuto;
