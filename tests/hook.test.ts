import { cases, createConfigurationSuites } from './testUtils';

const normal = {
    title: 'with useIntl hook imported',
    code: `
import { useIntl } from 'react-intl';

intl.formatMessage({ defaultMessage: "hello" });
  `,
};

const multiUse = {
    title: 'multiple uses',
    code: `
import { useIntl } from 'react-intl';

intl.formatMessage({ defaultMessage: "hello" });
intl.formatMessage({ defaultMessage: "hello" });
intl.formatMessage({ defaultMessage: "some other message" });
`,
};

const withValueInMessage = {
    title: 'with a value interpolated in the message',
    code: `
import { useIntl } from 'react-intl';

intl.formatMessage({ defaultMessage: \`template string ${1 + 1}\` });
`,
};

const withVariableMessage = {
    title: 'with a variable as the defaultMessage',
    code: `
import { useIntl } from 'react-intl';

const message = "variable message";

intl.formatMessage({ defaultMessage: message });
`,
};

const withVariableMessageDescriptor = {
    title: 'with a variable as the defaultMessage',
    code: `
import { useIntl } from 'react-intl';
import { message } from './messages'

intl.formatMessage(messages);
`,
};

const withCustomProperties = {
    title: 'with custom properties in formatMessage call',
    code: `
import { useIntl } from 'react-intl';

intl.formatMessage({ defaultMessage: "custom prop", other: 123 });
`,
};

const someSupportedUseCases = {
    title: 'some supported use cases',
    code: `
import { useIntl } from 'react-intl';

const Component2 = () => {
  const intl = useIntl();
  const label = intl.formatMessage({ defaultMessage: "hello" });
  return (
    <button aria-label={intl.formatMessage({ defaultMessage: "hello" })}>
      {intl.formatMessage({ defaultMessage: "hello" })}
    </button>
  );
};
  `,
};

const importAs = {
    title: 'with FormattedMessage imported as something else',
    code: `
import { useIntl as i18n } from 'react-intl';

intl.formatMessage({ defaultMessage: "i18n" });
`,
};

const notTransformIfNotImported = {
    title: 'does nothing if react-intl is not imported',
    
    code: `
import any from 'any-module';
intl.formatMessage({
  defaultMessage: "hello"
});
`,
};

const notTransformIfIdIsProvided = {
    title: 'does nothing if id is already provided',
    
    code: `
import { useIntl } from 'react-intl';
intl.formatMessage({
  id: "my.custom.id",
  defaultMessage: "hello"
});
`,
};


const useIntlDifferentModuleSource = {
    title: 'useIntl with different module source',
    code: `
import { useIntl } from 'gatsby-plugin-intl';
intl.formatMessage({
  id: "my.custom.id",
  defaultMessage: "hello"
});
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
    useIntlDifferentModuleSource,
];

cases(createConfigurationSuites('hooks', tests));
