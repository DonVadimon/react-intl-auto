import { cases, createConfigurationSuites } from './testUtils';

const normal = {
    title: 'with useIntl hook imported',
    fixture: 'hook/default.js',
};

const multiUse = {
    title: 'multiple uses',
    fixture: 'hook/multiple-uses.js',
};

const withValueInMessage = {
    title: 'with a value interpolated in the message',
    fixture: 'hook/with-value-in-message.js',
};

const withVariableMessage = {
    title: 'with a variable as the defaultMessage',
    fixture: 'hook/with-variable-message.js',
};

const withVariableMessageDescriptor = {
    title: 'with a variable as the defaultMessage',
    fixture: 'hook/with-variable-message-descriptor.js',
};

const withCustomProperties = {
    title: 'with custom properties in formatMessage call',
    fixture: 'hook/with-custom-properties.js',
};

const someSupportedUseCases = {
    title: 'some supported use cases',
    fixture: 'hook/some-supported-use-cases.js',
};

const importAs = {
    title: 'with FormattedMessage imported as something else',
    fixture: 'hook/import-as.js',
};

const notTransformIfNotImported = {
    title: 'does nothing if react-intl is not imported',
    fixture: 'hook/not-imported.js',
};

const notTransformIfIdIsProvided = {
    title: 'does nothing if id is already provided',
    fixture: 'hook/id-provided.js',
};

const useIntlDifferentModuleSource = {
    title: 'useIntl with different module source',
    fixture: 'hook/different-module-source.js',
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
