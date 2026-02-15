import { injectIntl } from 'react-intl';

function App({ intl }) {
    return (
        <div>
            {intl.formatMessage({ defaultMessage: 'hello' })}
            {intl.formatMessage({ defaultMessage: 'hello' })}
            {intl.formatMessage({ defaultMessage: 'some other message' })}
        </div>
    );
}

export default injectIntl(App);
