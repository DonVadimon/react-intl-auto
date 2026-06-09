import { defineMessages } from 'react-intl';

const defaultMessageVar = 'broken';
const descriptionVar = 'broken';

export const messagesBrokenVar = defineMessages({
    brokenVar: defaultMessageVar,
    brokenObjVar: {
        defaultMessage: defaultMessageVar,
    },
    brokenObjDescVar: {
        defaultMessage: 'Hello',
        description: descriptionVar,
    },
});
