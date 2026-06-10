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
    shouldError: true,
};

const withVariableMessage = {
    title: 'with a variable as the defaultMessage',
    fixture: 'hook/with-variable-message.js',
    shouldError: true,
};

const withTemplateVarMessage = {
    title: 'with a template literal with variable as defaultMessage',
    fixture: 'hook/with-template-var-message.js',
    shouldError: true,
};

const validTemplateLiteralDefaultMessage = {
    title: 'valid template literal in defaultMessage',
    fixture: 'hook/valid-template-literal-defaultMessage.js',
};

const validTemplateLiteralDescription = {
    title: 'valid template literal in description',
    fixture: 'hook/valid-template-literal-description.js',
};

const defaultMessageWithShorthand = {
    title: 'shorthand property in defaultMessage',
    fixture: 'hook/defaultMessage-with-shorthand.js',
    shouldError: true,
};

const descriptionWithTemplateExpr = {
    title: 'template literal with expression in description',
    fixture: 'hook/description-with-template-expr.js',
    shouldError: true,
};

const descriptionWithTemplateVar = {
    title: 'template literal with variable in description',
    fixture: 'hook/description-with-template-var.js',
    shouldError: true,
};

const descriptionWithVariable = {
    title: 'variable in description',
    fixture: 'hook/description-with-variable.js',
    shouldError: true,
};

const descriptionWithShorthand = {
    title: 'shorthand property in description',
    fixture: 'hook/description-with-shorthand.js',
    shouldError: true,
};

const withVariableMessageDescriptor = {
    title: 'with a variable as the message descriptor',
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
    title: 'with useIntl imported as something else',
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

export const hookTests = [
    normal,
    multiUse,
    withValueInMessage,
    withVariableMessage,
    withTemplateVarMessage,
    validTemplateLiteralDefaultMessage,
    validTemplateLiteralDescription,
    defaultMessageWithShorthand,
    descriptionWithTemplateExpr,
    descriptionWithTemplateVar,
    descriptionWithVariable,
    descriptionWithShorthand,
    withVariableMessageDescriptor,
    withCustomProperties,
    someSupportedUseCases,
    importAs,
    notTransformIfNotImported,
    notTransformIfIdIsProvided,
    useIntlDifferentModuleSource,
];
