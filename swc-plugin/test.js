const { transform } = require('@swc/core');
const fs = require('fs');
const path = require('path');

// Test cases
const testCases = [
  {
    name: 'defineMessages with string values',
    input: `
import { defineMessages } from 'react-intl';

export default defineMessages({
  hello: 'Hello {name}!',
  welcome: 'Welcome to our app',
});
    `,
    expected: 'id'
  },
  {
    name: 'FormattedMessage without ID',
    input: `
import { FormattedMessage } from 'react-intl';

const Component = () => (
  <FormattedMessage defaultMessage="Hello World" />
);
    `,
    expected: 'id'
  },
  {
    name: 'intl.formatMessage call',
    input: `
import { useIntl } from 'react-intl';

const Component = () => {
  const intl = useIntl();
  const message = intl.formatMessage({ defaultMessage: 'Hello' });
  return <div>{message}</div>;
};
    `,
    expected: 'id'
  }
];

async function runTests() {
  console.log('Running SWC Plugin Tests...\n');
  
  let passed = 0;
  let failed = 0;
  
  for (const testCase of testCases) {
    try {
      console.log(`Testing: ${testCase.name}`);
      
      const result = await transform(testCase.input, {
        filename: 'test.tsx',
        jsc: {
          parser: {
            syntax: 'typescript',
            tsx: true,
          },
          experimental: {
            plugins: [
              [
                'swc-plugin-react-intl-auto',
                {
                  removePrefix: '',
                  separator: '.',
                }
              ]
            ]
          }
        }
      });
      
      // Check if the expected transformation occurred
      if (testCase.expected === 'id') {
        if (result.code.includes('id:') || result.code.includes('id=')) {
          console.log('✅ PASSED\n');
          passed++;
        } else {
          console.log('❌ FAILED - No ID found in output\n');
          failed++;
        }
      }
      
      // Log the transformed code for debugging
      console.log('Transformed code:');
      console.log(result.code);
      console.log('---\n');
      
    } catch (error) {
      console.log(`❌ FAILED - Error: ${error.message}\n`);
      failed++;
    }
  }
  
  console.log(`\nTest Results: ${passed} passed, ${failed} failed`);
  
  if (failed > 0) {
    process.exit(1);
  }
}

// Only run tests if this file is executed directly
if (require.main === module) {
  runTests().catch(console.error);
}

module.exports = { runTests };
