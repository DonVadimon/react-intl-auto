import { useIntl } from 'react-intl';

const Component = () => {
    const intl = useIntl();
    const label = intl.formatMessage({ defaultMessage: 'hello' });
    return (
        <div aria-label={intl.formatMessage({ defaultMessage: 'hello' })}>
            {intl.formatMessage({ defaultMessage: 'hello' })}
        </div>
    );
};
