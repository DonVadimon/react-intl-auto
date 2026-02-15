import { injectIntl } from 'react-intl';
import { message } from './messages';

function App({ intl }) {
    return <div>{intl.formatMessage(message)}</div>;
}

export default injectIntl(App);
