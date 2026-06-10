# План исправления валидации неподдерживаемых форматов

## Обзор

В процессе анализа обнаружено 3 случая, когда неподдерживаемый код молча игнорируется вместо выдачи ошибки пользователю. Это может привести к тому, что пользователи не узнают о проблемах в коде до production.

---

## Проблема 1: Переменная как значение в defineMessages

### Описание проблемы

При использовании переменной как значения свойства в `defineMessages`, код молча игнорируется:

```typescript
import { defineMessages } from 'react-intl';

const variable = 'test';

export const messages = defineMessages({
    test: variable, // ID не генерируется, нет ошибки
});
```

### Причина

В файле `crates/react-intl-core/src/ast/call.rs`, функция `analyze_define_messages_object_property` обрабатывает случай `Expr::Ident` (строки 214-223):

```rust
Expr::Ident(ident) => {
    if let Some(collector) = var_collector {
        let var_name = ident.sym.to_string();
        if let Some(obj_lit) = collector.get_object(&var_name) {
            // Объект обрабатывается
            return analyze_message_object(obj_lit, state, Some(&key_name))
                .map(|opt| opt.map(|(md, td)| (key_name, md, td)));
        }
    }
    Ok(None) // ← Возвращается None вместо ошибки
}
```

Код проверяет, является ли переменная объектом. Если нет - возвращается `Ok(None)`, что означает "игнорировать".

### Как убедиться в наличии проблемы до исправления

1. Создать тестовый файл:
```bash
cat > /tmp/test-variable-as-value.ts << 'EOF'
import { defineMessages } from 'react-intl';

const variable = 'test';

export const messages = defineMessages({
    test: variable,
});
EOF
```

2. Запустить CLI:
```bash
node cli.js /tmp/test-variable-as-value.ts
```

3. **Ожидается**: Ошибка с сообщением о неподдерживаемом формате
4. **Фактически**: Код выполняется успешно, сообщений 0

### Тесты для написания

1. **Файл**: `tests/__fixtures__/definition/with-variable-as-value.js`
```javascript
import { defineMessages } from 'react-intl';

const variable = 'test';

export default defineMessages({
    test: variable,
});
```

2. **Файл**: `tests/__fixtures__/definition/index.ts` - добавить в массив:
```typescript
const withVariableAsValue = {
    title: 'with variable as value',
    fixture: 'definition/with-variable-as-value.js',
    shouldError: true,
};
```

### Решение

В функции `analyze_define_messages_object_property` для случая `Expr::Ident`:

1. Проверить, является ли переменная объектом через `var_collector`
2. Если да - обработать как объект
3. **Если нет - вернуть ошибку** `Field 'defaultMessage' must be a string literal, but got variable '{var_name}'`

```rust
Expr::Ident(ident) => {
    if let Some(collector) = var_collector {
        let var_name = ident.sym.to_string();
        if let Some(obj_lit) = collector.get_object(&var_name) {
            return analyze_message_object(obj_lit, state, Some(&key_name))
                .map(|opt| opt.map(|(md, td)| (key_name, md, td)));
        }
    }
    // Добавить проверку: если переменная не объект - ошибка
    Err(FieldExtractionError {
        field_name: "defaultMessage".to_string(),
        message: format!(
            "Field 'defaultMessage' must be a string literal, but got variable '{}'",
            ident.sym
        ),
    })
}
```

### Как проверить после исправления

1. Запустить тесты:
```bash
npm run test:jest
```

2. Убедиться, что новый тест проходит с `shouldError: true`

3. Проверить CLI:
```bash
node cli.js /tmp/test-variable-as-value.ts
# Должна быть ошибка:
# × /tmp/test-variable-as-value.ts: Error analyzing defineMessages: Field 'defaultMessage' must be a string literal, but got variable 'variable'
```

---

## Проблема 2: Template literal с выражениями как значение

### Описание проблемы

При использовании template literal с выражениями как значения, код игнорируется:

```typescript
import { defineMessages } from 'react-intl';

export const messages = defineMessages({
    test: `template ${1 + 1}`, // ID не генерируется, нет ошибки
});
```

### Причина

В функции `analyze_define_messages_object_property`, случай `Expr::Tpl` обрабатывается вместе с `Expr::Lit` (строки 190-212):

```rust
Expr::Lit(_) | Expr::Tpl(_) => {
    let default_message_prop = extract_expr_string(value);

    if default_message_prop.is_some() {
        // Успешная обработка
        return Ok(Some((key_name, transformed, true)));
    }

    Ok(None) // ← Template literal с выражениями возвращает None
}
```

Функция `extract_expr_string` возвращает `None` для template literals с выражениями, что приводит к игнорированию.

### Как убедиться в наличии проблемы до исправления

1. Создать тестовый файл:
```bash
cat > /tmp/test-template-expr-as-value.ts << 'EOF'
import { defineMessages } from 'react-intl';

export const messages = defineMessages({
    test: `template ${1 + 1}`,
});
EOF
```

2. Запустить CLI:
```bash
node cli.js /tmp/test-template-expr-as-value.ts
```

3. **Ожидается**: Ошибка
4. **Фактически**: Успешное выполнение, 0 сообщений

### Тесты для написания

1. **Файл**: `tests/__fixtures__/definition/with-template-expr-as-value.js`
```javascript
import { defineMessages } from 'react-intl';

export default defineMessages({
    test: `template ${1 + 1}`,
});
```

2. Добавить в `tests/__fixtures__/definition/index.ts`:
```typescript
const withTemplateExprAsValue = {
    title: 'with template literal with expressions as value',
    fixture: 'definition/with-template-expr-as-value.js',
    shouldError: true,
};
```

### Решение

Разделить обработку `Expr::Lit` и `Expr::Tpl`:

```rust
match value.as_ref() {
    // Строковый литерал - OK
    Expr::Lit(_) => {
        let default_message_prop = extract_expr_string(value);
        if default_message_prop.is_some() {
            // Генерация ID
            return Ok(Some((key_name, transformed, true)));
        }
        Ok(None)
    }
    // Template literal
    Expr::Tpl(tpl) => {
        if tpl.exprs.is_empty() {
            // Без выражений - OK
            let default_message_prop = extract_expr_string(value);
            if default_message_prop.is_some() {
                return Ok(Some((key_name, transformed, true)));
            }
            Ok(None)
        } else {
            // С выражениями - ошибка
            Err(FieldExtractionError {
                field_name: "defaultMessage".to_string(),
                message: "Field 'defaultMessage' must be a string literal, but got template literal with expressions".to_string(),
            })
        }
    }
    // ... остальные случаи
}
```

### Как проверить после исправления

1. Запустить тесты:
```bash
npm run test:jest
```

2. Проверить CLI:
```bash
node cli.js /tmp/test-template-expr-as-value.ts
# Ожидается ошибка:
# × /tmp/test-template-expr-as-value.ts: Error analyzing defineMessages: Field 'defaultMessage' must be a string literal, but got template literal with expressions
```

---

## Проблема 3: Template literal с переменной как значение

### Описание проблемы

Аналогично проблеме 2, но с переменной:

```typescript
import { defineMessages } from 'react-intl';

const variable = 'test';

export const messages = defineMessages({
    test: `template ${variable}`, // Игнорируется
});
```

### Причина

Та же, что и в проблеме 2 - template literal с выражениями игнорируется.

### Как убедиться в наличии проблемы до исправления

1. Создать файл:
```bash
cat > /tmp/test-template-var-as-value.ts << 'EOF'
import { defineMessages } from 'react-intl';

const variable = 'test';

export const messages = defineMessages({
    test: `template ${variable}`,
});
EOF
```

2. Запустить CLI - ожидается ошибка, фактически успех

### Тесты для написания

1. **Файл**: `tests/__fixtures__/definition/with-template-var-as-value.js`
```javascript
import { defineMessages } from 'react-intl';

const variable = 'test';

export default defineMessages({
    test: `template ${variable}`,
});
```

2. Добавить в index.ts

### Решение

То же, что и в проблеме 2 - проверка `tpl.exprs.is_empty()`.

### Как проверить

Аналогично проблеме 2.

---

## План выполнения

### Этап 1: Подготовка тестов

1. Создать 3 новых fixture файла в `tests/__fixtures__/definition/`
2. Обновить `tests/__fixtures__/definition/index.ts` с новыми тестами
3. Запустить `npm run test:jest` - убедиться, что новые тесты падают

### Этап 2: Исправление кода

1. Открыть `crates/react-intl-core/src/ast/call.rs`
2. Найти функцию `analyze_define_messages_object_property`
3. Изменить обработку `Expr::Ident` - добавить ошибку для не-объектов
4. Разделить обработку `Expr::Lit` и `Expr::Tpl`
5. Для `Expr::Tpl` добавить проверку на наличие выражений

### Этап 3: Тестирование

1. Запустить Rust тесты: `cargo test --workspace`
2. Запустить Jest тесты: `npm run test:jest`
3. Проверить CLI с тестовыми файлами из проблем
4. Проверить plugin с тестовыми файлами

### Этап 4: Регрессионное тестирование

1. Запустить `npm run test:all`
2. Убедиться, что все тесты проходят
3. Проверить существующие fixtures в sandbox/basic

---

## Критерии успеха

✅ Все 3 проблемы выдают понятные ошибки  
✅ CLI и Plugin ведут себя одинаково  
✅ Все новые тесты проходят с `shouldError: true`  
✅ Существующие тесты не сломаны  
✅ Сообщения об ошибках содержат путь к файлу  
✅ Ошибки для разных типов значений отличаются

---

## Дополнительные улучшения (опционально)

### Улучшение сообщений об ошибках

Текущее сообщение:
```
Field 'defaultMessage' must be a string literal, but got variable 'variable'
```

Можно улучшить, добавив контекст:
```
Field 'defaultMessage' must be a string literal, but got variable 'variable'. 
Only string literals like 'Hello World' are supported for ID generation.
```

### Проверка других полей

Убедиться, что аналогичные проверки работают для:
- `description` в defineMessages ✓ (уже работает)
- `id` в defineMessages ✓ (уже работает)
- `defaultMessage` и `description` в JSX ✓ (уже работает)
- `defaultMessage` и `description` в formatMessage ✓ (уже работает)

### Документация

Обновить README.md с разделом "Supported Formats":
- Строковые литералы: `'Hello'`
- Template literals без выражений: `` `Hello` ``
- Объекты: `{ defaultMessage: 'Hello', description: 'Greeting' }`
