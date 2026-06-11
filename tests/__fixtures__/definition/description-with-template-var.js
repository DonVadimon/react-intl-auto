import { defineMessages } from 'react-intl';

const variable = 'greeting';

export default defineMessages({
    hello: {
        defaultMessage: 'hello',
        description: `greeting ${variable}`,
    },
});
