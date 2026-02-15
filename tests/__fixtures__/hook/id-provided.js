import { useIntl } from 'react-intl';

const Component = () => {
    return (
        <div>
            {intl.formatMessage({
                id: 'my.custom.id',
                defaultMessage: 'hello',
            })}
        </div>
    );
};
