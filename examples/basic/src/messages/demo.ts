import { defineMessages } from 'react-intl';

const messages = {
    ...defineMessages({
        hello: 'Hello {name}!',
        welcome: 'Welcome to our app',
        goodbye: 'Goodbye {name}',
        createOrSaveAlert: 'createOrSaveAlert {id}',
    }),
};

export default messages;
