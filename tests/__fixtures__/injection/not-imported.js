import any from 'any-module';

function App({ intl }) {
    return (
        <div>
            {intl.formatMessage({
                defaultMessage: 'hello',
            })}
        </div>
    );
}

export default injectIntl(App);
