import { defineMessages } from 'react-intl';

const defaultMessage = 'broken';
const description = 'broken';

export const messagesBroken = defineMessages({
    broken: defaultMessage,
    brokenObj: { defaultMessage },
    brokenObjDesc: { defaultMessage: 'Hello', description },
});
