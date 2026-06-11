import { injectIntl } from 'react-intl';

function App({ intl }) {
    return (
        <div>
            {intl.formatMessage({
                defaultMessage: 'hello',
                description: `greeting ${1 + 1}`,
            })}
        </div>
    );
}

export default injectIntl(App);
