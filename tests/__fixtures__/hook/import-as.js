import { useIntl as i18n } from 'react-intl';

const Component = () => {
    return <div>{intl.formatMessage({ defaultMessage: 'i18n' })}</div>;
};
