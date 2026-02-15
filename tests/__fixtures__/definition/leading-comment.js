import { defineMessages } from 'react-intl';

export default defineMessages({
    // The main Hello of our app.
    hello: 'hello',

    // Another Hello,
    // multiline this time
    world: {
        id: 'hello.world',
        defaultMessage: 'hello world',
    },
});
