import { injectIntl } from 'react-intl';

const Component2 = ({ intl }) => {
    const label = intl.formatMessage({ defaultMessage: 'hello' });
    return (
        <button aria-label={intl.formatMessage({ defaultMessage: 'hello' })}>
            {intl.formatMessage({ defaultMessage: 'hello' })}
        </button>
    );
};

injectIntl(Components2);
