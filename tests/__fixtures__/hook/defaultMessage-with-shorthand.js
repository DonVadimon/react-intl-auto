import { useIntl } from 'react-intl';

const Component = () => {
    const intl = useIntl();
    const defaultMessage = 'hello';
    return <div>{intl.formatMessage({ defaultMessage })}</div>;
};
