import { defineMessages } from 'react-intl';

export const messagesBrokenExpr = defineMessages({
    inline: `broken ${1 + 1}`,
    objDefaultMessage: {
        defaultMessage: `broken ${1 + 1}`,
    },
    objDescription: {
        defaultMessage: 'Hello',
        description: `broken ${1 + 1}`,
    },
});
