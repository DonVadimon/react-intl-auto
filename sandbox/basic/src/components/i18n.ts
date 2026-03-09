import { createIntl } from 'react-intl';

const intl = createIntl({ locale: 'en' });

export const i18n = (...args: Parameters<typeof intl.formatMessage>) =>
    intl.formatMessage(...args);
