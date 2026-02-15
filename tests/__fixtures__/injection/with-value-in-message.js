import { injectIntl } from 'react-intl';

function App({ intl }) {
    return (
        <div>
            {intl.formatMessage({ defaultMessage: `template string ${1 + 1}` })}
        </div>
    );
}

export default injectIntl(App);
