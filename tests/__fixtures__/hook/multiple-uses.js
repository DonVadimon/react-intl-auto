import { useIntl } from 'react-intl';

const Component = () => {
    const intl = useIntl();

    return (
        <div>
            {intl.formatMessage({ defaultMessage: 'hello' })}
            {intl.formatMessage({ defaultMessage: 'hello' })}
            {intl.formatMessage({ defaultMessage: 'some other message' })}
        </div>
    );
};
