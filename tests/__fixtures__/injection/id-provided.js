import { injectIntl } from 'react-intl';

function App({ intl }) {
    return (
        <div>
            {intl.formatMessage({
                id: 'my.custom.id',
                defaultMessage: 'hello',
            })}
        </div>
    );
}

export default injectIntl(App);
