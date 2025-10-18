import { jsx as _jsx, jsxs as _jsxs } from "react/jsx-runtime";
import React from 'react';
import { defineMessages, FormattedMessage, useIntl } from 'react-intl';
// This will be transformed to include IDs
export var messages = defineMessages({
    hello: 'Hello {name}!',
    welcome: 'Welcome to our app',
    goodbye: 'Goodbye {name}'
});
// This will also be transformed
export default defineMessages({
    title: 'My App',
    description: 'A sample application'
});
export var App = function() {
    var intl = useIntl();
    // This will be transformed to include ID
    var label = intl.formatMessage({
        defaultMessage: 'Click me!'
    });
    return /*#__PURE__*/ _jsxs("div", {
        children: [
            /*#__PURE__*/ _jsx("h1", {
                children: /*#__PURE__*/ _jsx(FormattedMessage, {
                    id: "components.4109821222",
                    defaultMessage: "Welcome to React Intl Auto"
                })
            }),
            /*#__PURE__*/ _jsx("p", {
                children: /*#__PURE__*/ _jsx(FormattedMessage, {
                    id: "components.471047910",
                    defaultMessage: "Hello {name}",
                    values: {
                        name: 'World'
                    }
                })
            }),
            /*#__PURE__*/ _jsx("button", {
                "aria-label": label,
                children: /*#__PURE__*/ _jsx(FormattedMessage, {
                    id: "components.751710530",
                    defaultMessage: "Submit"
                })
            }),
            /*#__PURE__*/ _jsx(FormattedMessage, {
                id: "components.1860133942",
                defaultMessage: "This is a test message"
            }, "test-message")
        ]
    });
};
