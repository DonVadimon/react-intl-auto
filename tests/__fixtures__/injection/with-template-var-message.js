import { injectIntl } from 'react-intl';

const variable = 'world';

function App({ intl }) {
    return (
        <div>{intl.formatMessage({ defaultMessage: `hello ${variable}` })}</div>
    );
}

export default injectIntl(App);
