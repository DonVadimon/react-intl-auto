import React from 'react';
import { FormattedMessage, useIntl } from 'react-intl';
import { messagesBroken } from '../messages/broken';
import { i18n } from './i18n';

export const App = () => {
    const intl = useIntl();

    return (
        <div>
            <FormattedMessage {...messagesBroken.broken} />
            {intl.formatMessage(messagesBroken.brokenObj)}
            {i18n(messagesBroken.brokenObjDesc)}
        </div>
    );
};
