import { useIntl } from 'react-intl';

const variable = 'greeting';

const Component = () => {
    const intl = useIntl();
    return (
        <div>
            {intl.formatMessage({
                defaultMessage: 'hello',
                description: variable,
            })}
        </div>
    );
};
