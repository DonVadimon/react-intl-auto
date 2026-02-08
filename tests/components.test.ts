import { cases, createConfigurationSuites } from './testUtils';

const normal = {
    title: 'default',
    code: `
import { FormattedMessage } from 'react-intl';

<FormattedMessage defaultMessage="hello" />;
`,
};

const multiUse = {
    title: 'multiple uses',
    code: `
import { FormattedMessage } from 'react-intl';

<FormattedMessage defaultMessage="hello" />;
<FormattedMessage defaultMessage="another" />;
`,
};

const conditionalRendering = {
    title: 'conditional rendering',
    code: `
 import { FormattedMessage } from 'react-intl';
        
function App({ showMessage }) {
    return (
    <div>
        {showMessage && <FormattedMessage defaultMessage="Conditional message" />}
    </div>
    );
}`,
};

const allProps = {
    title: 'all props',
    code: `
import { FormattedMessage } from 'react-intl';
        
function App() {
    return (
    <FormattedMessage 
        defaultMessage="Hello {name}"
        description="A greeting message"
        values={{ name: 'World' }}
        tagName="span"
    />
    );
}`,
};

const allSupportedComponents = {
    title: 'import all supported components',
    code: `
import { FormattedHTMLMessage, FormattedMessage } from 'react-intl';

<FormattedHTMLMessage defaultMessage="<span>hello</span>" />;
<FormattedMessage defaultMessage="hello" />;
`,
};

const withValueInMessage = {
    title: 'with a value interpolated in the message',
    code: `
import { FormattedMessage } from 'react-intl';

<FormattedMessage defaultMessage={\`hello world ${1 + 1}\`} />;
`,
};

const withVariableMessage = {
    title: 'with a variable as the defaultMessage',
    code: `
import { FormattedMessage } from 'react-intl';

const message = "variable message";

<FormattedMessage defaultMessage={message} />;
`,
};

const importAsTest = {
    title: 'with FormattedMessage imported as something else',
    code: `
import { FormattedMessage as T } from 'react-intl';

<T defaultMessage="hello" />;
`,
};

const nestedJSXTest = {
    title: 'with FormattedMessage nested in other JSX',
    code: `
import { FormattedMessage } from 'react-intl';

<div>
  <FormattedMessage defaultMessage="hello" />
</div>
`,
};

const notTransformIfNotImported = {
    title: 'does nothing if components not imported from react-intl',
    code: `
import any from 'any-module';
<FormattedMessage defaultMessage="hello" />;
`,
};

const notTransformIfSpreadAttribute = {
    title: 'does nothing if component props are spread',    
    code: `
import { FormattedMessage } from 'react-intl';
const props = {
  defaultMessage: 'hello'
};
<FormattedMessage {...props} />;
`,
};

const keyProp = {
    title: 'using key',
    code: `
import { FormattedMessage } from 'react-intl';

<FormattedMessage key="foobar" defaultMessage="hello" />;
`,
};

const userId = {
    title: 'user id',
    code: `
import { FormattedMessage } from 'react-intl';

<FormattedMessage id="foobar" defaultMessage="hello" />;
`,
};

const differentModuleSource = {
    title: 'different module source',
    code: `
import { FormattedMessage } from 'gatsby-plugin-intl';

<FormattedMessage defaultMessage="hello" />;
`,
};

const tests = [
    normal,
    multiUse,
    conditionalRendering,
    allProps,
    allSupportedComponents,
    withValueInMessage,
    withVariableMessage,
    importAsTest,
    nestedJSXTest,
    notTransformIfNotImported,
    notTransformIfSpreadAttribute,
    keyProp,
    userId,
    differentModuleSource,
];

cases(createConfigurationSuites('components', tests));
