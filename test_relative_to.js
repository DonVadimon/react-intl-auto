const { transform } = require('@swc/core');
const plugin = require('./index.js');

async function testRelativeTo() {
  const code = `
    import { FormattedMessage } from 'react-intl';
    
    function App() {
      return <FormattedMessage defaultMessage="Hello World" />;
    }
  `;

  const result = await transform(code, {
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
  
  console.log('Generated code:');
  console.log(result.code);
  
  // Extract the id from the generated code
  const idMatch = result.code.match(/id:\s*"([^"]+)"/);
  if (idMatch) {
    console.log('\nExtracted ID:', idMatch[1]);
    console.log('Starts with dot:', idMatch[1].startsWith('.'));
  }
}

testRelativeTo().catch(console.error);
