import React from 'react';
import { defineMessages, FormattedMessage, useIntl } from 'react-intl';
import {messagesOne, messagesTwo} from '../messages/demo';

// This will be transformed to include IDs
export const messages1 = defineMessages({
    hello: 'Hello {name}!',
    welcome: 'Welcome to our app',
    goodbye: 'Goodbye {name}',
    spreadMe: 'spreadMe',
});

// This will also be transformed
export default defineMessages({
    title: 'My App',
    description: 'A sample application',
});

export const App: React.FC = () => {
    const intl = useIntl();

    // This will be transformed to include ID
    const label = intl.formatMessage({
        defaultMessage: 'Click me!',
    });

    return (
        <div>
            <h1>
                <FormattedMessage defaultMessage="Welcome to React Intl Auto" />
            </h1>

            <p>
                <FormattedMessage {...messages1.spreadMe} />
                <FormattedMessage
                    defaultMessage="Hello {name}"
                    values={{ name: 'World' }}
                />
                {/* <FormattedMessage
                    defaultMessage="commentedMessage"
                    values={{ id: '123' }}
                /> */}
                {intl.formatMessage(messagesOne.createOrSaveAlert, { id: '123' })}
            </p>
            {intl.formatMessage(messagesTwo.hello, { name: 'World' })}
            <button aria-label={label}>
                <FormattedMessage defaultMessage="Submit" />
            </button>
        </div>
    );
};
