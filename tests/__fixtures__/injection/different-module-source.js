import { injectIntl } from 'gatsby-plugin-intl';

function App({ intl }) {
    return <div>{intl.formatMessage({ defaultMessage: 'hello' })}</div>;
}

export default injectIntl(App);
