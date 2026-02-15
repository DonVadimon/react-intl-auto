import { FormattedMessage } from 'react-intl';

function App() {
    return (
        <FormattedMessage
            defaultMessage="Hello {name}"
            description="A greeting message"
            values={{ name: 'World' }}
            tagName="span"
        />
    );
}
