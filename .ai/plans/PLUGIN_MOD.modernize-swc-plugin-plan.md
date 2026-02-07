# PLUGIN_MOD: 🚀 Модернизация SWC плагина react-intl-auto

## 🎯 Цель
Модернизировать SWC плагин для автоматической генерации ID сообщений React Intl:
1. Переименовать пакет в `swc-plugin-react-intl-auto-fs` для публикации на npm
2. Обновить зависимости для совместимости с `@swc/core: ^1.15.0`
3. Добавить опциональное хэширование ID сообщений (включая murmur3)
4. Добавить функционал записи сообщений в JSON файл с настраиваемым путем

---

## 🔍 Исходное состояние

### ✅ Существующие компоненты
- **Плагин SWC** (`src/lib.rs`): Основной entry point с transform функцией
- **Типы** (`src/types.rs`): `PluginOptions`, `PluginState`, `RemovePrefix`, `IncludeExportName`
- **Утилиты** (`src/utils.rs`): `create_hash()` (murmur3), `get_prefix()`, `find_project_root()`, `dot_path()`
- **Посетители AST** (`src/visitors.rs`): 
  - `JSXVisitor` - обработка `<FormattedMessage>` и `<FormattedHTMLMessage>`
  - `CallExpressionVisitor` - обработка `formatMessage()` и `defineMessages()`
  - `ImportVisitor` - сбор информации об импортах
- **Тесты**: Интеграционные тесты в `__tests__/*.test.js`
- **Сборка**: `build.js`, поддержка wasm32-wasip1 и wasm32-unknown-unknown

### ⚠️ Ограничения текущей реализации
1. **Имя пакета**: `swc-plugin-react-intl-auto` конфликтует с оригинальным пакетом (форк)
2. **Зависимости**: `swc_core = "43.0.*"` может быть несовместим с `@swc/core: ^1.15.0`
3. **ID сообщений**: Всегда генерируются как читаемые строки (путь + hash), нет опции хэширования самого ID
4. **Нет экстракта**: Сообщения не сохраняются во внешний файл для последующего использования
5. **Совместимость**: Не тестировалась совместная работа с `@swc/plugin-formatjs`

### 📍 Найденные проблемные места
1. **`Cargo.toml:2`**: `name = "swc-plugin-react-intl-auto"` - нужно переименовать
2. **`Cargo.toml:15`**: `swc_core = { version = "43.0.*"` - устаревшая версия
3. **`package.json:2`**: `"name": "swc-plugin-react-intl-auto"` - нужно переименовать
4. **`src/types.rs:5-22`**: `PluginOptions` не содержит опций для хэширования ID и экстракта
5. **`src/utils.rs:7-11`**: `create_hash()` используется только для суффикса, не для финального ID
6. **`src/visitors.rs:99-143`**: `generate_id()` создает ID как строку, без опции хэширования
7. **Нет модуля экстракта**: Нужно создать новый функционал для записи в файл

---

## 📋 Список задач

- [x] PLUGIN_MOD-001: Переименование пакета в swc-plugin-react-intl-auto-fs
- [x] PLUGIN_MOD-002: Обновление зависимостей для совместимости с @swc/core ^1.15.0
- [ ] PLUGIN_MOD-003: Добавление опционального хэширования ID сообщений
- [ ] PLUGIN_MOD-004: Добавление функционала записи сообщений в JSON файл

---

## [x] PLUGIN_MOD-001: Переименование пакета

### 📋 Metadata
- **status:** `ready`
- **depends:** `-`
- **priority:** `P0`
- **files:** `Cargo.toml`, `package.json`, `build.js`, `index.js`, `readme.md`

### 📝 Details

Переименовать пакет для возможности публикации на npm как отдельный пакет.

**Требования:**
- ✅ Изменить имя пакета в `Cargo.toml` на `swc-plugin-react-intl-auto-fs`
- ✅ Изменить имя пакета в `package.json` на `swc-plugin-react-intl-auto-fs`
- ✅ Обновить `main` в `package.json` на `swc-plugin-react-intl-auto-fs.wasm`
- ✅ Обновить `build.js` для генерации правильного имени WASM файла
- ✅ Обновить `index.js`
- ✅ Обновить README с новым именем пакета

**Выполненные изменения:**
1. ✅ `Cargo.toml:2` - изменено имя пакета
2. ✅ `package.json:2` - изменено имя npm пакета
3. ✅ `package.json:5` - обновлен main entry point
4. ✅ `package.json:30-33` - обновлен массив files
5. ✅ `build.js:12-13` - обновлены пути к WASM файлам
6. ✅ `build.js:17` - обновлено сообщение в консоли
7. ✅ `index.js:4` - обновлен путь к WASM файлу
8. ✅ `readme.md:16,25,39` - обновлены упоминания пакета
9. ✅ `readme.md:221` - обновлен GitHub Packages путь

**Влияние:**
- После переименования пакет можно будет опубликовать на npm
- Пользователи должны будут обновить имя пакета в своих проектах

### 📊 ActionLog:
- `2026-02-07 01:58` План задачи создан
- `2026-02-07 02:05` Данные актуализированы: проверены файлы Cargo.toml, package.json, build.js, index.js, readme.md
- `2026-02-07 02:05` Составлен план выполнения: обновление 5 файлов
- `2026-02-07 02:05` План согласован с USER
- `2026-02-07 02:05` Выполнен шаг 1: обновлен Cargo.toml
- `2026-02-07 02:05` Выполнен шаг 2: обновлен package.json
- `2026-02-07 02:05` Выполнен шаг 3: обновлен build.js
- `2026-02-07 02:05` Выполнен шаг 4: обновлен index.js
- `2026-02-07 02:05` Выполнен шаг 5: обновлен readme.md
- `2026-02-07 02:05` Проверка: cargo check прошел успешно
- `2026-02-07 02:05` Определены критерии приёмки: все файлы обновлены, код компилируется
- `2026-02-07 02:05` Review: одобрено USER
- `2026-02-07 02:05` Задача завершена, статус изменен на `ready`

---

## [x] PLUGIN_MOD-002: Обновление зависимостей

### 📋 Metadata
- **status:** `ready`
- **depends:** `PLUGIN_MOD-001`
- **priority:** `P0`
- **files:** `Cargo.toml`, `package.json`, `src/visitors.rs`, `readme.md`, `AGENTS.md`

### 📝 Details

Обновить зависимости Rust и JavaScript для совместимости с `@swc/core: ^1.15.0`.

**Требования (все выполнены):**
- ✅ Установить `@swc/core` версии `^1.15.0` (через `npm install`)
- ✅ Установить `swc_core` версии `47.0.*` (совместимой с @swc/core ^1.15.0) через `cargo add`
- ✅ Обновить остальные Rust зависимости при необходимости
- ✅ Адаптировать код под новое API SWC (исправлены breaking changes)
- ✅ Убедиться в работоспособности: сборка проходит, все тесты проходят
- ✅ Добавить информацию о сборке и тестировании в документацию

**Выполненные изменения:**
1. ✅ `npm install @swc/core@^1.15.0 --save-dev` - установлена версия 1.15.11
2. ✅ `cargo add 'swc_core@47.0.*'` с features - установлена версия 47.0.9
3. ✅ Исправлен код в `src/visitors.rs` для совместимости с новым API:
   - `JSXAttrValue::Lit` → `JSXAttrValue::Str`
   - `.to_string()` → `.to_string_lossy().to_string()` для Wtf8Atom
   - Исправлены методы работы со строками
4. ✅ Обновлен `readme.md` - добавлен раздел Development Workflow
5. ✅ Обновлен `AGENTS.md` - расширен раздел Build Commands

**Известные ограничения совместимости:**
- Для @swc/core ^1.15.0 требуется swc_core версии 47.0.0 - 51.0.0
- Версии swc_core < 47.0.0 несовместимы с @swc/core 1.15.0
- Версии swc_core >= 52.0.0 имеют breaking changes в API плагинов

**Результаты тестирования:**
- ✅ `npm run build` - успешно
- ✅ `cargo test` - 25 tests passed
- ✅ `npm test` - 88 tests passed, 87 snapshots passed

### 📊 ActionLog:
- `2026-02-07 13:08` План задачи скорректирован с учетом требований (swc_core 47-51, @swc/core ^1.15.0)
- `2026-02-07 13:08` Установлен @swc/core ^1.15.0 через npm install
- `2026-02-07 13:08` Установлен swc_core 47.0.* через cargo add
- `2026-02-07 13:08` Попытка сборки - ошибки компиляции из-за breaking changes в API
- `2026-02-07 13:08` Исправлен код для совместимости с swc_core 47.0.*
- `2026-02-07 13:08` Сборка прошла успешно
- `2026-02-07 13:08` Rust тесты: 25 passed
- `2026-02-07 13:08` npm тесты: 88 passed, 87 snapshots passed
- `2026-02-07 13:08` Обновлена документация (readme.md, AGENTS.md)
- `2026-02-07 13:08` Задача завершена, статус изменен на `ready`

---

## [ ] PLUGIN_MOD-003: Добавление хэширования ID сообщений

### 📋 Metadata
- **status:** `todo`
- **depends:** `PLUGIN_MOD-002`
- **priority:** `P1`
- **files:** `src/types.rs`, `src/utils.rs`, `src/visitors.rs`

### 📝 Details

Добавить возможность хэшировать финальные ID сообщений с выбором алгоритма (включая murmur3).

**Требования:**
- Добавить опцию `hash_id` в `PluginOptions` (boolean)
- Добавить опцию `hash_algorithm` в `PluginOptions` (enum: "murmur3", "base64", "md5", etc.)
- При `hash_id: true` преобразовывать сгенерированный ID в хэш
- Поддержать несколько алгоритмов хэширования:
  - `murmur3` - существующая реализация
  - `base64` - Base64 кодирование строки
  - Возможность расширения другими алгоритмами
- Сохранять обратную совместимость (по умолчанию `hash_id: false`)

**Проблемные места:**
- `src/types.rs:5-22`: Нужно добавить поля `hash_id` и `hash_algorithm`
- `src/utils.rs:7-11`: `create_hash()` только для murmur3, нужно обобщить
- `src/visitors.rs:99-143`: `generate_id()` создает ID как строку, нужно добавить хэширование
- `src/visitors.rs:322-408`: `process_format_message_object()` - аналогично
- `src/visitors.rs:411-592`: `process_define_messages_object()` - аналогично

**Изменения:**
1. Обновить `src/types.rs`:
   ```rust
   #[derive(Debug, Clone, Serialize, Deserialize)]
   pub struct PluginOptions {
       // ... существующие поля ...
       #[serde(default)]
       pub hash_id: bool,
       #[serde(default = "default_hash_algorithm")]
       pub hash_algorithm: String, // "murmur3", "base64"
   }
   ```

2. Обновить `src/utils.rs`:
   ```rust
   pub fn hash_string(input: &str, algorithm: &str) -> String {
       match algorithm {
           "murmur3" => create_hash(input), // существующая функция
           "base64" => base64::encode(input),
           _ => create_hash(input),
       }
   }
   ```

3. Обновить `src/visitors.rs`:
   - В `generate_id()` и других методах добавить проверку `state.opts.hash_id`
   - Если `hash_id: true`, применить хэширование к финальному ID

**Пример:**
```javascript
// Before (hash_id: false)
export default defineMessages({
  hello: {
    id: 'App.Components.Greeting.hello',
    defaultMessage: 'hello {name}'
  }
})

// After (hash_id: true, hash_algorithm: "base64")
export default defineMessages({
  hello: {
    id: 'R3NwLkNvbXBvbmVudHMuR3JlZXRpbmcuaGVsbG8=',
    defaultMessage: 'hello {name}'
  }
})
```

**Влияние:**
- Добавляет гибкость в форматировании ID
- Может использоваться для сокращения длины ID
- Необходимо обновить тесты для новых опций

### 📊 ActionLog:
- `2026-02-07 01:58` План задачи создан

---

## [ ] PLUGIN_MOD-004: Добавление функционала записи сообщений в файл

### 📋 Metadata
- **status:** `todo`
- **depends:** `PLUGIN_MOD-003`
- **priority:** `P1`
- **files:** `src/types.rs`, `src/lib.rs`, `src/utils.rs` (новый модуль extract.rs)

### 📝 Details

Добавить функционал записи всех обработанных сообщений в JSON файл с настраиваемым путем.

**Требования:**
- Добавить опцию `extract_to_file` в `PluginOptions` (boolean)
- Добавить опцию `extract_output_dir` в `PluginOptions` (String, путь к директории)
- Добавить опцию `extract_filename` в `PluginOptions` (String, имя файла, default: "messages.json")
- Собирать все сообщения во время трансформации (id, defaultMessage, file)
- Записывать собранные сообщения в JSON файл после обработки всех файлов
- Формат JSON: массив объектов `[{"id": "...", "defaultMessage": "...", "file": "..."}]`
- Обеспечить потокобезопасность при параллельной сборке

**Проблемные места:**
- `src/types.rs:5-22`: Нужно добавить поля для конфигурации экстракта
- `src/lib.rs:16-24`: `TransformVisitor` нужно добавить хранилище сообщений
- `src/lib.rs:54-72`: `process_transform` нужно добавить запись в файл после обработки
- `src/visitors.rs`: Нужно собирать информацию о сообщениях во время обхода AST
- Новый модуль: нужно создать `src/extract.rs` для управления записью файлов

**Изменения:**
1. Обновить `src/types.rs`:
   ```rust
   #[derive(Debug, Clone, Serialize, Deserialize)]
   pub struct PluginOptions {
       // ... существующие поля ...
       #[serde(default)]
       pub extract_to_file: bool,
       #[serde(default)]
       pub extract_output_dir: Option<String>,
       #[serde(default = "default_extract_filename")]
       pub extract_filename: String,
   }

   #[derive(Debug, Clone, Serialize)]
   pub struct ExtractedMessage {
       pub id: String,
       pub default_message: String,
       pub file: String,
   }
   ```

2. Создать `src/extract.rs`:
   ```rust
   use std::sync::Mutex;
   use std::collections::HashMap;
   use crate::types::ExtractedMessage;

   lazy_static! {
       static ref EXTRACTED_MESSAGES: Mutex<HashMap<String, Vec<ExtractedMessage>>> = 
           Mutex::new(HashMap::new());
   }

   pub fn add_message(file: &str, message: ExtractedMessage) {
       let mut messages = EXTRACTED_MESSAGES.lock().unwrap();
       messages.entry(file.to_string())
           .or_insert_with(Vec::new)
           .push(message);
   }

   pub fn write_to_file(output_dir: &str, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
       let messages = EXTRACTED_MESSAGES.lock().unwrap();
       let all_messages: Vec<&ExtractedMessage> = messages.values()
           .flatten()
           .collect();
       
       let json = serde_json::to_string_pretty(&all_messages)?;
       let output_path = std::path::Path::new(output_dir).join(filename);
       std::fs::write(output_path, json)?;
       Ok(())
   }
   ```

3. Обновить `src/lib.rs`:
   - Добавить вызов записи в файл в конце `process_transform`
   - Добавить `ExtractedMessage` в состояние

4. Обновить `src/visitors.rs`:
   - В `generate_id()` и других методах добавить сбор сообщений
   - Вызывать `extract::add_message()` при обработке каждого сообщения

**Пример:**
```javascript
// Исходный код
const messages = defineMessages({
  hello: "Привет мир",
});
```

```json
// Результат (extract_output_dir: "./i18n", extract_filename: "messages.json")
[
  {
    "id": "o9zTCA==",
    "defaultMessage": "Привет мир",
    "file": "components/Hello/Hello.tsx"
  }
]
```

**Влияние:**
- Позволяет извлекать все сообщения для последующей обработки (переводы, анализ)
- Требует аккуратной работы с параллельной сборкой (webpack, etc.)
- Увеличивает сложность плагина

### 📊 ActionLog:
- `2026-02-07 01:58` План задачи создан

---

## ✅ EPIC план создан

**Файл:** `.ai/plans/PLUGIN_MOD.modernize-swc-plugin-plan.md`

**Следующие шаги:**
- Используйте `tasks` для просмотра всех задач
- Используйте `task PLUGIN_MOD-001` для начала работы с первой задачей
- Рекомендуется начать с задач приоритета P0 без зависимостей (PLUGIN_MOD-001)

**Критерий выполнения эпика:**
Для проекта с исходным кодом:
```ts
// components/Hello/Hello.tsx
const messages = defineMessages({
  hello: "Привет мир",
});
```

После сборки с использованием SWC и данного плагина должна быть создана директория с файлом:
```json
[{"id":"o9zTCA==","defaultMessage":"Привет мир","file":"components/Hello/Hello.tsx"}]
```

Также плагин должен корректно работать совместно с `@swc/plugin-formatjs`:
```js
{
  experimental: {
    plugins: [
      ['swc-plugin-react-intl-auto-fs', {}],
      ["@swc/plugin-formatjs", {
        idInterpolationPattern: '[md5:contenthash:hex:10]',
        ast: true,
      }]
    ]
  }
}
```
