import { injectIntl as i18n } from 'react-intl';

function App({ intl }) {
    return (
        <div>
            {intl.formatMessage({
                defaultMessage: 'i18n',
            })}
        </div>
    );
}

export default i18n(App);
