const normal = {
    title: 'default',
    fixture: 'components/default.tsx',
};

const multiUse = {
    title: 'multiple uses',
    fixture: 'components/multiple-uses.tsx',
};

const conditionalRendering = {
    title: 'conditional rendering',
    fixture: 'components/conditional-rendering.tsx',
};

const allProps = {
    title: 'all props',
    fixture: 'components/all-props.tsx',
};

const allSupportedComponents = {
    title: 'import all supported components',
    fixture: 'components/all-supported-components.tsx',
};

const withValueInMessage = {
    title: 'with a value interpolated in the message',
    fixture: 'components/with-value-in-message.tsx',
};

const withVariableMessage = {
    title: 'with a variable as the defaultMessage',
    fixture: 'components/with-variable-message.tsx',
};

const importAsTest = {
    title: 'with FormattedMessage imported as something else',
    fixture: 'components/import-as.tsx',
};

const nestedJSXTest = {
    title: 'with FormattedMessage nested in other JSX',
    fixture: 'components/nested-jsx.tsx',
};

const notTransformIfNotImported = {
    title: 'does nothing if components not imported from react-intl',
    fixture: 'components/not-imported.tsx',
};

const notTransformIfSpreadAttribute = {
    title: 'does nothing if component props are spread',
    fixture: 'components/spread-attribute.tsx',
};

const userId = {
    title: 'user id',
    fixture: 'components/user-id.tsx',
};

const differentModuleSource = {
    title: 'different module source',
    fixture: 'components/different-module-source.tsx',
};

export const componentsTests = [
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
    userId,
    differentModuleSource,
];
