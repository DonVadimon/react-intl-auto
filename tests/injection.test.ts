import { cases, createConfigurationSuites } from './testUtils';

const normal = {
    title: 'with Injection API HOC imported',
    code: `
import { injectIntl } from 'react-intl';

function App({ intl }) {
  return (
    <div>
        {intl.formatMessage({ defaultMessage: "hello" })}
    </div>);
}

export default injectIntl(App)
  `,
};

const multiUse = {
    title: 'multiple uses',
    code: `
import { injectIntl } from 'react-intl';

function App({ intl }) {
  return (
    <div>
        {intl.formatMessage({ defaultMessage: "hello" })}
        {intl.formatMessage({ defaultMessage: "hello" })}
        {intl.formatMessage({ defaultMessage: "some other message" })}
    </div>);
}

export default injectIntl(App)

intl.formatMessage({ defaultMessage: "hello" });
intl.formatMessage({ defaultMessage: "hello" });
intl.formatMessage({ defaultMessage: "some other message" });
`,
};

const withValueInMessage = {
    title: 'with a value interpolated in the message',
    code: `
import { injectIntl } from 'react-intl';

function App({ intl }) {
  return (
    <div>
        {intl.formatMessage({ defaultMessage: \`template string ${1 + 1}\` })}
    </div>);
}

export default injectIntl(App)
`,
};

const withVariableMessage = {
    title: 'with a variable as the defaultMessage',
    code: `
import { injectIntl } from 'react-intl';

const message = "variable message";

function App({ intl }) {
  return (
    <div>
        {intl.formatMessage({ defaultMessage: message })}
    </div>);
}

export default injectIntl(App)
`,
};

const withVariableMessageDescriptor = {
    title: 'with a variable as the defaultMessage',
    code: `
import { injectIntl } from 'react-intl';
import { message } from './messages';

function App({ intl }) {
  return (
    <div>
        {intl.formatMessage(message)}
    </div>);
}

export default injectIntl(App)
`,
};

const withCustomProperties = {
    title: 'with custom properties in formatMessage call',
    code: `
import { injectIntl } from 'react-intl';

function App({ intl }) {
  return (
    <div>
        {intl.formatMessage({ defaultMessage: "custom prop", other: 123 })}
    </div>);
}

export default injectIntl(App)
`,
};

const someSupportedUseCases = {
    title: 'some supported use cases',
    code: `
import { injectIntl } from 'react-intl';

const Component2 = ({ intl }) => {
  const label = intl.formatMessage({ defaultMessage: "hello" });
  return (
    <button aria-label={intl.formatMessage({ defaultMessage: "hello" })}>
      {intl.formatMessage({ defaultMessage: "hello" })}
    </button>
  );
};
injectIntl(Components2);
  `,
};

const importAs = {
    title: 'with FormattedMessage imported as something else',
    code: `
import { injectIntl as i18n } from 'react-intl';
function App({ intl }) {
  return (
    <div>
        {
            intl.formatMessage({
                defaultMessage: "i18n",
            })
        }
    </div>);
}

export default i18n(App)
`,
};


const notTransformIfNotImported = {
    title: 'does nothing if react-intl is not imported',
    
    code: `
import any from 'any-module';
function App({ intl }) {
  return (
    <div>
        {
            intl.formatMessage({
                defaultMessage: "hello"
            })
        }
    </div>);
}

export default injectIntl(App)
`,
};

const notTransformIfIdIsProvided = {
    title: 'does nothing if id is already provided',
    
    code: `
import { injectIntl } from 'react-intl';
import { injectIntl } from 'gatsby-plugin-intl';
function App({ intl }) {
  return (
    <div>
        {
            intl.formatMessage({
                id: "my.custom.id",
                defaultMessage: "hello"
            })
        }
    </div>);
}

export default injectIntl(App)
`,
};

const injectIntlWithProps = {
    title: 'with injectIntl',
    code: `
import { injectIntl } from 'react-intl';
function App({ intl }) {
  return <div>{intl.formatMessage({ defaultMessage: 'hello' })}</div>
}

export default injectIntl(App)
  `,
};

const useIntlDifferentModuleSource = {
    title: 'different module source',
    code: `
import { injectIntl } from 'gatsby-plugin-intl';
function App({ intl }) {
  return <div>{intl.formatMessage({ defaultMessage: 'hello' })}</div>
}

export default injectIntl(App)
  `,
};

const tests = [
    normal,
    multiUse,
    withValueInMessage,
    withVariableMessage,
    withVariableMessageDescriptor,
    withCustomProperties,
    someSupportedUseCases,
    importAs,
    notTransformIfNotImported,
    notTransformIfIdIsProvided,
    injectIntlWithProps,
];

cases(createConfigurationSuites('injection', tests));
