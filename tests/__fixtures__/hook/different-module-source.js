import { useIntl } from 'gatsby-plugin-intl';

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
