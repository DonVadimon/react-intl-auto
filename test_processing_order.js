const { transform } = require('@swc/core');
const plugin = require('./index.js');

async function testProcessingOrder() {
  const code = `
    import { FormattedMessage } from 'react-intl';
    
    function App() {
      return <FormattedMessage defaultMessage="Hello World" />;
    }
  `;

  console.log('Testing processing order: relative_to first, then remove_prefix...\n');

  // Test 1: relative_to + remove_prefix combination
  console.log('1. relative_to: "src" + remove_prefix: "components/"');
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
          [plugin, { 
            relative_to: 'src',
            remove_prefix: 'components/'
          }]
        ]
      }
    }
  });
  
  const id1 = result1.code.match(/id:\s*"([^"]+)"/)?.[1];
  console.log(`   ID: ${id1}`);
  console.log(`   Expected: App.427197390 (relative_to first removes src, then remove_prefix removes components/)`);
  console.log(`   Match: ${id1 === 'App.427197390' ? '✅' : '❌'}\n`);

  // Test 2: Only relative_to
  console.log('2. Only relative_to: "src"');
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
          [plugin, { 
            relative_to: 'src'
          }]
        ]
      }
    }
  });
  
  const id2 = result2.code.match(/id:\s*"([^"]+)"/)?.[1];
  console.log(`   ID: ${id2}`);
  console.log(`   Expected: components.App.427197390 (relative_to removes src, leaving components.App)`);
  console.log(`   Match: ${id2 === 'components.App.427197390' ? '✅' : '❌'}\n`);

  // Test 3: Only remove_prefix
  console.log('3. Only remove_prefix: "src/"');
  const result3 = await transform(code, {
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
          [plugin, { 
            remove_prefix: 'src/'
          }]
        ]
      }
    }
  });
  
  const id3 = result3.code.match(/id:\s*"([^"]+)"/)?.[1];
  console.log(`   ID: ${id3}`);
  console.log(`   Expected: components.App.427197390 (remove_prefix removes src/ from absolute path)`);
  console.log(`   Match: ${id3 === 'components.App.427197390' ? '✅' : '❌'}\n`);

  // Test 4: relative_to with absolute path
  console.log('4. relative_to: "/Users/ryan/project/src" (absolute path)');
  const result4 = await transform(code, {
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
          [plugin, { 
            relative_to: '/Users/ryan/project/src'
          }]
        ]
      }
    }
  });
  
  const id4 = result4.code.match(/id:\s*"([^"]+)"/)?.[1];
  console.log(`   ID: ${id4}`);
  console.log(`   Expected: components.App.427197390 (relative_to removes absolute path prefix)`);
  console.log(`   Match: ${id4 === 'components.App.427197390' ? '✅' : '❌'}\n`);
}

testProcessingOrder().catch(console.error);
