const { transform } = require('@swc/core');
const plugin = require('./index.js');

async function debugTest() {
  const code = `
    import { FormattedMessage } from 'react-intl';
    
    function App() {
      return <FormattedMessage defaultMessage="Hello World" />;
    }
  `;

  console.log('Testing with relative_to: "src"');
  const result1 = await transform(code, {
    filename: 'src/components/App.js',
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
          [plugin, { relative_to: 'src' }]
        ]
      }
    }
  });
  
  console.log('Result 1:');
  console.log(result1.code);
  
  console.log('\nTesting with relative_to: "src/"');
  const result2 = await transform(code, {
    filename: 'src/components/App.js',
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
          [plugin, { relative_to: 'src/' }]
        ]
      }
    }
  });
  
  console.log('Result 2:');
  console.log(result2.code);
}

debugTest().catch(console.error);
