module.exports = {
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
          'swc-plugin-react-intl-auto',
          {
            removePrefix: 'src/',
            separator: '.',
            filebase: false,
            extractComments: true,
          }
        ]
      ]
    }
  },
  module: {
    type: 'es6',
  },
};
