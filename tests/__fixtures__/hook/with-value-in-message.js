import { useIntl } from 'react-intl';

const Component = () => {
    const intl = useIntl();
    return (
        <div>
            {intl.formatMessage({ defaultMessage: `template string ${1 + 1}` })}
        </div>
    );
};
