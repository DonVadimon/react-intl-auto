import { useIntl } from 'react-intl';

const variable = 'world';

const Component = () => {
    const intl = useIntl();
    return (
        <div>{intl.formatMessage({ defaultMessage: `hello ${variable}` })}</div>
    );
};
