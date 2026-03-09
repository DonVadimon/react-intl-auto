import React from 'react';
import { defineMessages } from 'react-intl';
import { i18n } from './i18n';

const messages = defineMessages({
    button: 'Кнопка',
});

export const Button = () => {
    return (
        <button type="button">
            <span>{i18n(messages.button)}</span>
        </button>
    );
};
