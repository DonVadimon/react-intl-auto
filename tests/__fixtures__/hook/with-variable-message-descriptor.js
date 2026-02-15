import { useIntl } from 'react-intl';
import { message } from './messages';

const Component = () => {
    const intl = useIntl();
    return <div>{intl.formatMessage(messages)}</div>;
};
