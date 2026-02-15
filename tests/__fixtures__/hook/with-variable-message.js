import { useIntl } from 'react-intl';

const message = 'variable message';

const Component = () => {
    const intl = useIntl();
    return <div>{intl.formatMessage({ defaultMessage: message })}</div>;
};
