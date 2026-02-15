import { useIntl } from 'react-intl';

const Component = () => {
    const intl = useIntl();
    return (
        <div>
            {intl.formatMessage({ defaultMessage: 'custom prop', other: 123 })}
        </div>
    );
};
