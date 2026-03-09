import { useIntl as useI18n } from 'react-intl';

const Component = () => {
    const intl = useI18n();
    return <div>{intl.formatMessage({ defaultMessage: 'i18n' })}</div>;
};
