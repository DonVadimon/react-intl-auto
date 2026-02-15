import { useIntl } from 'react-intl';

const Component = () => {
    return (
        <div>
            {intl.formatMessage({ defaultMessage: 'hello' })}
            {intl.formatMessage({ defaultMessage: 'hello' })}
            {intl.formatMessage({ defaultMessage: 'some other message' })}
        </div>
    );
};
