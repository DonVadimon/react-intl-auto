import { extractIdsFromCode } from './testUtils';

describe('extractIdsFromCode', () => {
    it('should work with defineMessages', () => {
        const code = `
import { defineMessages } from 'react-intl';

defineMessages({
    hello: {
        id: 'test.hello',
        defaultMessage: 'hello',
    }
});

defineMessages({
    world: {
        'id': 'test.world',
        defaultMessage: 'world',
    }
});

defineMessages({
    foo: {
        "id": 'test.foo',
        defaultMessage: 'foo',
    },
    bar: {
        'id': "test.bar",
        defaultMessage: 'bar',
    }
});

hello({
    id: 'test.no-match',
});

hello({
    noMatch: {
        id: 'test.no-match-2',
        defaultMessage: 'no-match-2',
    }
});`;

        const messages = extractIdsFromCode(code);

        expect(messages).toEqual([
            'test.hello',
            'test.world',
            'test.foo',
            'test.bar',
        ]);
    });

    it('should work with jsx', () => {
        const code = `
import { jsx as _jsx } from "react/jsx-runtime";
import { FormattedMessage } from 'react-intl';
/*#__PURE__*/ _jsx(FormattedMessage, {
    id: "test.foo",
    defaultMessage: "<span>hello</span>"
});
/*#__PURE__*/ _jsx(FormattedMessage, {
    id: "test.bar",
    defaultMessage: "hello"
});
/*#__PURE__*/ _jsx("div", {
children: /*#__PURE__*/ _jsx(FormattedMessage, {
        id: "test.nested",
        defaultMessage: "hello"
    })
});
/*#__PURE__*/ _jsx(NotFormattedMessage, {
    id: "test.no-match",
    defaultMessage: "hello"
});
`;

        const messages = extractIdsFromCode(code);

        expect(messages).toEqual(['test.foo', 'test.bar', 'test.nested']);
    });
});
