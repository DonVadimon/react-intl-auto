import { useIntl } from 'react-intl';

const Component = () => {
    const intl = useIntl();
    const description = 'greeting';
    return (
        <div>
            {intl.formatMessage({
                defaultMessage: 'hello',
                description,
            })}
        </div>
    );
};
