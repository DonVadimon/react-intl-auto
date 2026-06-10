import { injectIntl } from 'react-intl';

function App({ intl }) {
    const defaultMessage = 'hello';
    return <div>{intl.formatMessage({ defaultMessage })}</div>;
}

export default injectIntl(App);
