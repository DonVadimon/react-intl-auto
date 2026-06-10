import { injectIntl } from 'react-intl';

const variable = 'greeting';

function App({ intl }) {
    return (
        <div>
            {intl.formatMessage({
                defaultMessage: 'hello',
                description: `greeting ${variable}`,
            })}
        </div>
    );
}

export default injectIntl(App);
