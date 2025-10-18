const { transform } = require('@swc/core');
const plugin = require('./index.js');

async function testRelativeToPaths() {
  const code = `
    import { FormattedMessage } from 'react-intl';
    
    function App() {
      return <FormattedMessage defaultMessage="Hello World" />;
    }
  `;

  console.log('Testing relative_to with different path types...\n');

  // Test 1: relative_to as absolute path
  console.log('1. relative_to as absolute path:');
  const result1 = await transform(code, {
    filename: '/Users/ryan/project/src/components/App.js',
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
          [plugin, { relative_to: '/Users/ryan/project' }]
        ]
      }
    }
  });
  
  const id1 = result1.code.match(/id:\s*"([^"]+)"/)?.[1];
  console.log(`   ID: ${id1}`);
  console.log(`   Expected: src.components.App.427197390`);
  console.log(`   Match: ${id1 === 'src.components.App.427197390' ? '✅' : '❌'}\n`);

  // Test 2: relative_to as relative path
  console.log('2. relative_to as relative path:');
  const result2 = await transform(code, {
    filename: '/Users/ryan/project/src/components/App.js',
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
  
  const id2 = result2.code.match(/id:\s*"([^"]+)"/)?.[1];
  console.log(`   ID: ${id2}`);
  console.log(`   Expected: components.App.427197390`);
  console.log(`   Match: ${id2 === 'components.App.427197390' ? '✅' : '❌'}\n`);

  // Test 3: relative_to with nested relative path
  console.log('3. relative_to with nested relative path:');
  const result3 = await transform(code, {
    filename: '/Users/ryan/project/src/components/App.js',
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
          [plugin, { relative_to: 'src/components' }]
        ]
      }
    }
  });
  
  const id3 = result3.code.match(/id:\s*"([^"]+)"/)?.[1];
  console.log(`   ID: ${id3}`);
  console.log(`   Expected: App.427197390`);
  console.log(`   Match: ${id3 === 'App.427197390' ? '✅' : '❌'}\n`);
}

testRelativeToPaths().catch(console.error);
