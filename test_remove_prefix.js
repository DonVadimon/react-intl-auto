const { transform } = require('@swc/core');
const plugin = require('./index.js');

async function testRemovePrefix() {
  const code = `
    import { FormattedMessage } from 'react-intl';
    
    function App() {
      return <FormattedMessage defaultMessage="Hello World" />;
    }
  `;

  console.log('Testing remove_prefix with different values...\n');

  // Test 1: remove_prefix: true
  console.log('1. remove_prefix: true');
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
          [plugin, { remove_prefix: true }]
        ]
      }
    }
  });
  
  const id1 = result1.code.match(/id:\s*"([^"]+)"/)?.[1];
  console.log(`   ID: ${id1}`);
  console.log(`   Expected: 427197390 (empty prefix)`);
  console.log(`   Match: ${id1 === '427197390' ? '✅' : '❌'}\n`);

  // Test 2: remove_prefix: false
  console.log('2. remove_prefix: false');
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
          [plugin, { remove_prefix: false }]
        ]
      }
    }
  });
  
  const id2 = result2.code.match(/id:\s*"([^"]+)"/)?.[1];
  console.log(`   ID: ${id2}`);
  console.log(`   Expected: src.components.App.427197390`);
  console.log(`   Match: ${id2 === 'src.components.App.427197390' ? '✅' : '❌'}\n`);

  // Test 3: remove_prefix: "src/"
  console.log('3. remove_prefix: "src/"');
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
          [plugin, { remove_prefix: 'src/' }]
        ]
      }
    }
  });
  
  const id3 = result3.code.match(/id:\s*"([^"]+)"/)?.[1];
  console.log(`   ID: ${id3}`);
  console.log(`   Expected: components.App.427197390`);
  console.log(`   Match: ${id3 === 'components.App.427197390' ? '✅' : '❌'}\n`);

  // Test 4: remove_prefix: "src/components/"
  console.log('4. remove_prefix: "src/components/"');
  const result4 = await transform(code, {
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
          [plugin, { remove_prefix: 'src/components/' }]
        ]
      }
    }
  });
  
  const id4 = result4.code.match(/id:\s*"([^"]+)"/)?.[1];
  console.log(`   ID: ${id4}`);
  console.log(`   Expected: App.427197390`);
  console.log(`   Match: ${id4 === 'App.427197390' ? '✅' : '❌'}\n`);

  // Test 5: remove_prefix: "src/message" (with different path)
  console.log('5. remove_prefix: "src/message" (with different path)');
  const result5 = await transform(code, {
    filename: 'src/message/App.js',
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
          [plugin, { remove_prefix: 'src/message' }]
        ]
      }
    }
  });
  
  const id5 = result5.code.match(/id:\s*"([^"]+)"/)?.[1];
  console.log(`   ID: ${id5}`);
  console.log(`   Expected: App.427197390`);
  console.log(`   Match: ${id5 === 'App.427197390' ? '✅' : '❌'}\n`);

  // Test 6: remove_prefix: "src/message/" (with trailing slash)
  console.log('6. remove_prefix: "src/message/" (with trailing slash)');
  const result6 = await transform(code, {
    filename: 'src/message/App.js',
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
          [plugin, { remove_prefix: 'src/message/' }]
        ]
      }
    }
  });
  
  const id6 = result6.code.match(/id:\s*"([^"]+)"/)?.[1];
  console.log(`   ID: ${id6}`);
  console.log(`   Expected: App.427197390`);
  console.log(`   Match: ${id6 === 'App.427197390' ? '✅' : '❌'}\n`);

  // Test 7: remove_prefix: "src/.*/" (regex pattern)
  console.log('7. remove_prefix: "src/.*/" (regex pattern)');
  const result7 = await transform(code, {
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
          [plugin, { remove_prefix: 'src/.*/' }]
        ]
      }
    }
  });
  
  const id7 = result7.code.match(/id:\s*"([^"]+)"/)?.[1];
  console.log(`   ID: ${id7}`);
  console.log(`   Expected: App.427197390`);
  console.log(`   Match: ${id7 === 'App.427197390' ? '✅' : '❌'}\n`);

  // Test 8: remove_prefix: "src.*" (regex pattern that matches everything)
  console.log('8. remove_prefix: "src.*" (regex pattern that matches everything)');
  const result8 = await transform(code, {
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
          [plugin, { remove_prefix: 'src.*' }]
        ]
      }
    }
  });
  
  const id8 = result8.code.match(/id:\s*"([^"]+)"/)?.[1];
  console.log(`   ID: ${id8}`);
  console.log(`   Expected: 427197390 (empty prefix due to full match)`);
  console.log(`   Match: ${id8 === '427197390' ? '✅' : '❌'}\n`);
}

testRemovePrefix().catch(console.error);
