import { defineMessages } from 'react-intl';

export const messagesOne = {
    ...defineMessages({
        hello: 'Hello {name}!',
        welcome: 'Welcome to our app',
        goodbye: 'Goodbye {name}',
        createOrSaveAlert: 'createOrSaveAlert {id}',
    }),
};

export const messagesTwo = defineMessages({
    hello: 'Hello {name}!',
    welcome: 'Welcome to our app',
    goodbye: 'Goodbye {name}',
    createOrSaveAlert: 'createOrSaveAlert {id}',
    obj: {
        defaultMessage: 'Test object',
    },
});
