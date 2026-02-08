# Модернизация пакета и расширение функциональности

Глобальные задачи:
- переименовать пакет swc-plugin-react-intl-auto -> swc-plugin-react-intl-auto-fs
- поднять версии зависимостей
- добавить возможность хэшировать id сообщений
- добавить возможность записывать сообщения в файл

Детализация:
1. Переименование
Этот репозиторий - форк пакета [swc-plugin-react-intl-auto](https://github.com/lcl9288/swc-plugin-react-intl-auto). Переименование в swc-plugin-react-intl-auto-fs необходимо для возможности публикации пакета после модернизации

2. Поднятие зависимостей
Данный плагин предполагается использовать совместно с плагином @swc/plugin-formatjs, который использует @swc/core: ^1.15.0
Для совместимости надо поднять используемые зависимости до послежних версий

3. Хэширование id сообщений
Необходимо добавить возможность преобразовывать id сообщения (тот что дал пользователь и тот что мы сгенерировали на основе пути до файла) в хэш. Должна быть возможность выбора алгоритма хэширования, в том числе murmur3. Пример:
Before
```js
export default defineMessages({
  hello: {
    id: 'App.Components.Greeting.hello',
    defaultMessage: 'hello {name}'
  }
})
```
After
```js
export default defineMessages({
  hello: {
    id: 'GSplhw==',
    defaultMessage: 'hello {name}'
  }
})
```

4. Экстракт сообщений в файл
Необходимо добавить функционал записи выделенных сообщений в json файл после всех преобразований. Директория и название файла должны быть настраиваемыми. Пример содержимого файла:
```json
[{"id":"o9zTCA==","defaultMessage":"Что-то пошло не так","file":"src/components/ErrorBoundary/ErrorFallback/ErrorFallback.tsx"},{"id":"V/1eFA==","defaultMessage":"Обновить страницу","file":"src/components/ErrorBoundary/ErrorFallback/ErrorFallback.tsx"}]
```

Критерий выполнения:
Должно стать возможным для проекта с исходным кодом
```ts
// components/Hello/Hello.tsx
const messages = defineMessages({
  hello: "Привет мир",
});
```

Использовать swc с данным плагином и после сборки получать директорию с файлом, в котором будут содержаться все используемые сообщения. Из предыдущего примера:
```json
[{"id":"o9zTCA==","defaultMessage":"Привет мир","file":"components/Hello/Hello.tsx"}]
```

Также должна быть возможность без проблем использовать плагин `@swc/plugin-formatjs`. Например так

```js
 {
    test: /\.(tsx|ts|js|mjs|cjs|jsx)$/,
    loader: 'swc-loader',
    options: {
        jsc: {
            parser: {
                syntax: 'typescript',
                tsx: true,
                transform: {
                    react: {
                        runtime: 'automatic',
                    },
                },
            },
            externalHelpers: true,
            experimental: {
                plugins: [
                  [
                    'swc-plugin-react-intl-auto',
                    {}
                  ],
                  [
                    "@swc/plugin-formatjs",
                    {
                      idInterpolationPattern: '[md5:contenthash:hex:10]',
                      ast: true,
                    },
                  ]
                ]
            },
        },
    },
  }
```
