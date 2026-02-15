import { cases, createConfigurationSuites } from './testUtils';

const normal = {
    title: 'with Injection API HOC imported',
    fixture: 'injection/default.js',
};

const multiUse = {
    title: 'multiple uses',
    fixture: 'injection/multiple-uses.js',
};

const withValueInMessage = {
    title: 'with a value interpolated in the message',
    fixture: 'injection/with-value-in-message.js',
};

const withVariableMessage = {
    title: 'with a variable as the defaultMessage',
    fixture: 'injection/with-variable-message.js',
};

const withVariableMessageDescriptor = {
    title: 'with a variable as the defaultMessage',
    fixture: 'injection/with-variable-message-descriptor.js',
};

const withCustomProperties = {
    title: 'with custom properties in formatMessage call',
    fixture: 'injection/with-custom-properties.js',
};

const someSupportedUseCases = {
    title: 'some supported use cases',
    fixture: 'injection/some-supported-use-cases.js',
};

const importAs = {
    title: 'with FormattedMessage imported as something else',
    fixture: 'injection/import-as.js',
};

const notTransformIfNotImported = {
    title: 'does nothing if react-intl is not imported',
    fixture: 'injection/not-imported.js',
};

const notTransformIfIdIsProvided = {
    title: 'does nothing if id is already provided',
    fixture: 'injection/id-provided.js',
};

const injectIntlWithProps = {
    title: 'with injectIntl',
    fixture: 'injection/with-injectIntl.js',
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
