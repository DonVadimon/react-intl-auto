import { defineMessages } from 'react-intl';

const variable = 'broken';

export const messagesBrokenLit = defineMessages({
    inline: `broken ${variable}`,
    objDefaultMessage: {
        defaultMessage: `hello ${variable}`,
    },
    objDescription: {
        defaultMessage: 'Hello',
        description: `hello ${variable}`,
    },
});
