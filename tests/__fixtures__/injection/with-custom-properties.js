import { injectIntl } from 'react-intl';

function App({ intl }) {
    return (
        <div>
            {intl.formatMessage({ defaultMessage: 'custom prop', other: 123 })}
        </div>
    );
}

export default injectIntl(App);
