import { injectIntl } from 'react-intl';

function App({ intl }) {
    return (
        <div>
            {intl.formatMessage({
                defaultMessage: 'hello',
                description: `greeting message`,
            })}
        </div>
    );
}

export default injectIntl(App);
