import { FormattedMessage } from 'react-intl';

const variable = 'greeting';

<FormattedMessage
    defaultMessage="hello"
    description={`greeting ${variable}`}
/>;
