import { injectIntl } from 'react-intl';

function App({ intl }) {
    return <div>{intl.formatMessage({ defaultMessage: `hello world` })}</div>;
}

export default injectIntl(App);
