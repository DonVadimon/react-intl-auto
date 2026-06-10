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
    shouldError: true,
};

const withVariableMessage = {
    title: 'with a variable as the defaultMessage',
    fixture: 'injection/with-variable-message.js',
    shouldError: true,
};

const withTemplateVarMessage = {
    title: 'with a template literal with variable as defaultMessage',
    fixture: 'injection/with-template-var-message.js',
    shouldError: true,
};

const validTemplateLiteralDefaultMessage = {
    title: 'valid template literal in defaultMessage',
    fixture: 'injection/valid-template-literal-defaultMessage.js',
};

const validTemplateLiteralDescription = {
    title: 'valid template literal in description',
    fixture: 'injection/valid-template-literal-description.js',
};

const defaultMessageWithShorthand = {
    title: 'shorthand property in defaultMessage',
    fixture: 'injection/defaultMessage-with-shorthand.js',
    shouldError: true,
};

const descriptionWithTemplateExpr = {
    title: 'template literal with expression in description',
    fixture: 'injection/description-with-template-expr.js',
    shouldError: true,
};

const descriptionWithTemplateVar = {
    title: 'template literal with variable in description',
    fixture: 'injection/description-with-template-var.js',
    shouldError: true,
};

const descriptionWithVariable = {
    title: 'variable in description',
    fixture: 'injection/description-with-variable.js',
    shouldError: true,
};

const descriptionWithShorthand = {
    title: 'shorthand property in description',
    fixture: 'injection/description-with-shorthand.js',
    shouldError: true,
};

const withVariableMessageDescriptor = {
    title: 'with a variable as message descriptor',
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
    title: 'with injectIntl imported as something else',
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

export const injectionTests = [
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
    injectIntlWithProps,
];
