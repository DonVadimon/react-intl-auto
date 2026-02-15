import { cases, createConfigurationSuites } from './testUtils';

const defaultTest = {
    title: 'default',
    fixture: 'definition/default.js',
};

const multiExport = {
    title: 'multi export',
    fixture: 'definition/multi-export.js',
};

const leadingCommentTest = {
    title: 'leading comment',
    fixture: 'definition/leading-comment.js',
};

const leadingCommentWithDescriptionTest = {
    title: 'leading comment with description',
    fixture: 'definition/leading-comment-with-description.js',
};

const templateString = {
    title: 'with include value',
    fixture: 'definition/with-template-literal.js',
};

const stringKey = {
    title: 'string literal',
    fixture: 'definition/string-literal.js',
};

const objectArg = {
    title: 'Object',
    fixture: 'definition/object-arg.js',
};

const importAs = {
    title: 'import as',
    fixture: 'definition/import-as.js',
};

const withOtherFn = {
    title: 'with other func',
    fixture: 'definition/with-other-func.js',
};

const notImported = {
    title: 'not transform if defineMessages is not imported',
    fixture: 'definition/not-imported.js',
};

const argsNonObject = {
    title: 'not transform when defineMessages argumens is not object',
    fixture: 'definition/args-non-object.js',
};

const useVariable = {
    title: 'when using the variable',
    fixture: 'definition/use-variable.js',
};

const argsNotFound = {
    title: 'not transfrom when the variable can not be found',
    fixture: 'definition/args-not-found.js',
};

const emptyArgs = {
    title: 'not transform when defineMessages argumens is empty',
    fixture: 'definition/empty-args.js',
};

const calleeNotIdentifier = {
    title: 'not transform if callee is not identifier',
    fixture: 'definition/callee-not-identifier.js',
};

const withOtherSpecifier = {
    title: 'with other specifier',
    fixture: 'definition/with-other-specifier.js',
};

const evalString = {
    title: 'eval string',
    fixture: 'definition/eval-string.js',
};

const differentModuleSource = {
    title: 'moduleSourceName',
    fixture: 'definition/module-source-name.js',
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
