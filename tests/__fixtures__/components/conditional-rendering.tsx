import { FormattedMessage } from 'react-intl';

function App({ showMessage }) {
    return (
        <div>
            {showMessage && (
                <FormattedMessage defaultMessage="Conditional message" />
            )}
        </div>
    );
}
