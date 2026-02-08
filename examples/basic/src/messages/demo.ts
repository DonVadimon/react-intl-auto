import { defineMessages } from 'react-intl';

const messages = {
    ...defineMessages({
        hello: 'Hello {name}!',
        welcome: 'Welcome to our app',
        goodbye: 'Goodbye {name}',
        createOrSaveAlert: `{id, select,
      undefined { 创建 }
      other { 保存 }
    }成功`,
    }),
};

export default messages;
