import { useIntl } from 'gatsby-plugin-intl';

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
