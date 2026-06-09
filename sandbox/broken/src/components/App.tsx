import React from 'react';
import { FormattedMessage, useIntl } from 'react-intl';
import { messagesBrokenShort } from '../messages/broken-shorthand';
import { messagesBrokenVar } from '../messages/broken';
import { i18n } from './i18n';

export const App = () => {
    const intl = useIntl();

    return (
        <div>
            {/* variables usage */}
            <FormattedMessage {...messagesBrokenVar.brokenVar} />
            {intl.formatMessage(messagesBrokenVar.brokenObjVar)}
            {i18n(messagesBrokenVar.brokenObjDescVar)}

            {/* shorthand variables usage */}
            {intl.formatMessage(messagesBrokenShort.brokenObj)}
            {i18n(messagesBrokenShort.brokenObjDesc)}
        </div>
    );
};
