import { injectIntl } from 'react-intl';

function App({ intl }) {
    const description = 'greeting';
    return (
        <div>
            {intl.formatMessage({
                defaultMessage: 'hello',
                description,
            })}
        </div>
    );
}

export default injectIntl(App);
