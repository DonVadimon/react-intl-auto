import React from 'react';
import { FormattedMessage, useIntl } from 'react-intl';
import { messagesBrokenShort } from '../messages/broken-shorthand';
import { messagesBrokenVar } from '../messages/broken-vars';
import { messagesBrokenLit } from '../messages/broken-lit';
import { messagesBrokenExpr } from '../messages/broken-expr';
import { i18n } from './i18n';

export const App = () => {
    const intl = useIntl();

    return (
        <div>
            {/* variables usage */}
            <FormattedMessage {...messagesBrokenVar.inline} />
            {intl.formatMessage(messagesBrokenVar.objDefaultMessage)}
            {i18n(messagesBrokenVar.objDescription)}

            {/* shorthand variables usage */}
            {intl.formatMessage(messagesBrokenShort.objDefaultMessage)}
            {i18n(messagesBrokenShort.objDescription)}

            {/* varible in literals usage */}
            <FormattedMessage {...messagesBrokenLit.inline} />
            {intl.formatMessage(messagesBrokenLit.objDefaultMessage)}
            {i18n(messagesBrokenLit.objDescription)}

            {/* expression in literals usage */}
            <FormattedMessage {...messagesBrokenExpr.inline} />
            {intl.formatMessage(messagesBrokenExpr.objDefaultMessage)}
            {i18n(messagesBrokenExpr.objDescription)}
        </div>
    );
};
