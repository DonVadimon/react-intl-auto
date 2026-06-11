import { defineMessages } from 'react-intl';

const defaultMessageVar = 'broken';
const descriptionVar = 'broken';

export const messagesBrokenVar = defineMessages({
    inline: defaultMessageVar,
    objDefaultMessage: {
        defaultMessage: defaultMessageVar,
    },
    objDescription: {
        defaultMessage: 'Hello',
        description: descriptionVar,
    },
});
