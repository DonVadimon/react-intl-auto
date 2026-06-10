import { useIntl } from 'react-intl';

const Component = () => {
    const intl = useIntl();
    return (
        <div>
            {intl.formatMessage({
                defaultMessage: 'hello',
                description: `greeting ${1 + 1}`,
            })}
        </div>
    );
};
