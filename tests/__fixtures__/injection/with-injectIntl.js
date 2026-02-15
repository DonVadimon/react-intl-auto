import { injectIntl } from 'react-intl';

function App({ intl }) {
    return <div>{intl.formatMessage({ defaultMessage: 'hello' })}</div>;
}

export default injectIntl(App);
