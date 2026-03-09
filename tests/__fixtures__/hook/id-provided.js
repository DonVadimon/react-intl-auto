import { useIntl } from 'react-intl';

const Component = () => {
    const intl = useIntl();

    return (
        <div>
            {intl.formatMessage({
                id: 'my.custom.id',
                defaultMessage: 'hello',
            })}
        </div>
    );
};
