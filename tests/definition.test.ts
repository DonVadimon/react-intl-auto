import { cases, createConfigurationSuites } from './testUtils';

const defaultTest = {
    title: 'default',
    code: `
import { defineMessages } from 'react-intl'

export default defineMessages({
  hello: 'hello',
})
`,
};

const multiExport = {
    title: 'multi export',
    code: `
import { defineMessages } from 'react-intl'

export const extra = defineMessages({
  hello: 'hello world extra'
})

export default defineMessages({
  hello: 'hello world',
})
`,
};

const leadingCommentTest = {
    title: 'leading comment',
    code: `
import { defineMessages } from 'react-intl'

export default defineMessages({
  // The main Hello of our app.
  hello: 'hello',

  // Another Hello,
  // multiline this time
  world: {
    id: 'hello.world',
    defaultMessage: 'hello world',
  }
})
`,
};

const leadingCommentWithDescriptionTest = {
    title: 'leading comment with description',
    code: `
import { defineMessages } from 'react-intl'

export default defineMessages({

  // This comment should not be used
  world: {
    defaultMessage: 'hello world',
    description: 'The hello world',
  }
})
`,
};

const templateString = {
    title: 'with include value',
    code: `
import { defineMessages } from 'react-intl'

defineMessages({
hello: \`hello world \${1}\`,
})
  `,
};

const stringKey = {
    title: 'string literal',
    code: `
import { defineMessages } from 'react-intl'

defineMessages({
  'hello': 'hello world'
})
      `,
};

const objectArg = {
    title: 'Object',
    code: `
import { defineMessages } from 'react-intl'

defineMessages({
new: {
id: 'this is id',
defaultMessage: 'id',
},
world: {
defaultMessage: 'world',
},
headerTitle: {
defaultMessage: 'Welcome to dashboard {name}!',
description: 'Message to greet the user.',
},
})
  `,
};

const importAs = {
    title: 'import as',
    code: `
import { defineMessages as m } from 'react-intl'

m({
hello: 'hello'
})

`,
};

const withOtherFn = {
    title: 'with other func',
    code: `
import { defineMessages } from 'react-intl'

defineMessages({
hello: 'hello',
})

hello({
id: 'hoge',
})
`,
};

const notImported = {
    title: 'not transform if defineMessages is not imported',
    code: `
import any from 'any-module'

export default defineMessages({
  hello: 'hello'
})
    `,
};

const argsNonObject = {
    title: 'not transform when defineMessages argumens is not object',
    code: `
import { defineMessages } from 'react-intl'

export default defineMessages(1)
    `,
};

const useVariable = {
    title: 'when using the variable',
    code: `
import { defineMessages } from 'react-intl'

const messages = {hello: 'hello'}

export default defineMessages(messages)
    `,
};

const argsNotFound = {
    title: 'not transfrom when the variable can not be found',
    code: `
import { defineMessages } from 'react-intl'

export default defineMessages(messages)
    `,
};

const emptyArgs = {
    title: 'not transform when defineMessages argumens is empty',
    code: `
import { defineMessages } from 'react-intl'

export default defineMessages()
    `,
};

const calleeNotIdentifier = {
    title: 'not transform if callee is not identifier',
    code: `
import { defineMessages } from 'react-intl'

const m = [defineMessages]

export default m[0]({
  hello: 'hello world'
})
    `,
};

const withOtherSpecifier = {
    title: 'with other specifier',
    code: `
import { defineMessages, FormattedMessage } from 'react-intl'

export default defineMessages({
hello: 'hello world',
})
`,
};

const evalString = {
    title: 'eval string',
    code: `
import { defineMessages } from 'react-intl'

export default defineMessages({
hello: 'hello' + 'world',
})
`,
};

const differentModuleSource = {
    title: 'moduleSourceName',
    code: `
import { defineMessages } from 'gatsby-plugin-intl'

export default defineMessages({
  hello: 'hello',
})
  `,
};

const tests = [
    defaultTest,
    multiExport,
    leadingCommentTest,
    leadingCommentWithDescriptionTest,
    templateString,
    stringKey,
    objectArg,
    importAs,
    withOtherFn,
    notImported,
    argsNonObject,
    useVariable,
    argsNotFound,
    emptyArgs,
    calleeNotIdentifier,
    withOtherSpecifier,
    evalString,
    differentModuleSource,
];

cases(createConfigurationSuites('definition', tests));
