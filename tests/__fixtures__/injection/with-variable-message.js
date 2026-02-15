import { injectIntl } from 'react-intl';

const message = 'variable message';

function App({ intl }) {
    return <div>{intl.formatMessage({ defaultMessage: message })}</div>;
}

export default injectIntl(App);
