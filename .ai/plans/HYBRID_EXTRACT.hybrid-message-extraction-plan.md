# HYBRID_EXTRACT: 🌍 Hybrid Solution for React Intl Message Extraction

## 🎯 Цель

Создать гибридное решение для извлечения сообщений React Intl, состоящее из:
1. **SWC Plugin** - трансформация кода (добавление ID) - ✅ УЖЕ РЕАЛИЗОВАН
2. **Shared Rust Core** - библиотека с общей логикой (генерация ID, AST traversal)
3. **CLI Tool** - извлечение сообщений в JSON файлы
4. **JS API** - программный доступ к функционалу из Node.js

**Ключевое требование**: Переиспользование Rust кода между плагином и CLI для гарантии генерации **идентичных ID**.

---

## 🔍 Исходное состояние

### ✅ Существующие компоненты

1. **SWC Plugin** (`src/`)
   - ✅ Трансформация AST для добавления ID
   - ✅ Поддержка `defineMessages()`, `formatMessage()`, `<FormattedMessage>`
   - ✅ Генерация ID (murmur3, base64 хэши)
   - ✅ Опции: hash_id, hash_algorithm, remove_prefix, filebase, relative_to, etc.
   - ✅ 25+ Rust unit tests
   - ✅ 88 Jest интеграционных тестов

2. **Инфраструктура**
   - ✅ Сборка WASM (wasm32-wasip1 target)
   - ✅ CI/CD через GitHub Actions
   - ✅ Публикация в npm

### ⚠️ Ограничения текущей реализации

1. **Нет возможности извлекать сообщения в JSON** - плагин только трансформирует код, WASM не может писать файлы
2. **Нет CLI инструмента** - нельзя извлечь сообщения отдельно от сборки
3. **Нет программного API** - нельзя использовать из кастомных скриптов
4. **Дублирование кода** - если CLI будет отдельным проектом, логика генерации ID может расходиться

### 📍 Найденные проблемные места

1. **Структура проекта** - текущий `Cargo.toml` настроен только для WASM плагина, нужна workspace структура
2. **Зависимости** - нужно разделить зависимости между plugin (WASM) и CLI (native)
3. **Node.js bindings** - нужно настроить napi-rs или neon для JS API
4. **Тестирование консистентности** - нужны тесты, проверяющие идентичность ID между компонентами

---

## 📋 Список задач

- [x] HYBRID_EXTRACT-001: Create Cargo workspace structure with shared core library
- [x] HYBRID_EXTRACT-002: Extract ID generation and path utilities to shared core crate
- [ ] HYBRID_EXTRACT-003: Extract AST traversal logic to shared core crate
- [ ] HYBRID_EXTRACT-004: Create CLI tool crate with message extraction
- [ ] HYBRID_EXTRACT-005: Implement CLI argument parsing and file globbing
- [ ] HYBRID_EXTRACT-006: Implement JSON output format (aggregated and per-file)
- [ ] HYBRID_EXTRACT-007: Add source location extraction option
- [ ] HYBRID_EXTRACT-008: Create JS API with napi-rs bindings
- [ ] HYBRID_EXTRACT-009: Update package.json with CLI bin entry and JS API exports
- [ ] HYBRID_EXTRACT-010: Create integration tests for ID consistency between plugin and CLI
- [ ] HYBRID_EXTRACT-011: Create example projects (webpack, CLI-only, JS API)
- [ ] HYBRID_EXTRACT-012: Update documentation and README

---

## [x] HYBRID_EXTRACT-001: Create Cargo workspace structure with shared core library

### 📋 Metadata
- **status:** `ready`
- **depends:** `-`
- **priority:** `P0`
- **files:** `Cargo.toml`, `crates/react-intl-core/Cargo.toml`

### 📝 Details

Создать workspace структуру Cargo для разделения кода на три crate: plugin (WASM), shared core (lib), CLI (bin).

**Требования:**
- Сохранить обратную совместимость с существующим плагином
- Общие зависимости вынести в workspace
- Каждый crate имеет свои специфичные зависимости

**Проблемные места:**
- Текущий `Cargo.toml` в корне должен стать workspace manifest
- Существующий `src/` нужно переместить или преобразовать в member crate
- Нужно обновить пути в `build.js` и CI скриптах

**Изменения:**
1. Преобразовать `Cargo.toml` в workspace manifest
2. Создать структуру `crates/react-intl-core/`
3. Переместить/переорганизовать существующий plugin код

**Пример:**
```toml
# Cargo.toml (workspace)
[workspace]
members = ["crates/react-intl-core", "crates/swc-plugin", "crates/cli"]
resolver = "2"

[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
murmur3 = "0.5"
base64 = "0.22"
```

**Влияние:**
- Изменение структуры проекта
- Нужно обновить CI/CD пайплайны
- Обновить документацию по сборке

### 📊 ActionLog:
- `2026-02-08 18:48` План задачи создан
- `2026-02-08 19:01` Данные актуализированы: проверены файлы Cargo.toml, src/
- `2026-02-08 19:01` Статус изменен на `in-progress`
- `2026-02-08 19:01` План согласован с USER, начато выполнение
- `2026-02-08 19:02` Выполнен шаг 1: Преобразован Cargo.toml в workspace manifest
- `2026-02-08 19:02` Выполнен шаг 2: Создана структура crates/ с тремя crate
- `2026-02-08 19:02` Выполнен шаг 3: Существующий код перемещен в crates/swc-plugin/
- `2026-02-08 19:03` Выполнен шаг 4: Созданы placeholder модули для react-intl-core
- `2026-02-08 19:03` Выполнен шаг 5: Обновлен build.js для новых путей
- `2026-02-08 19:04` Выполнен шаг 6: Проверка cargo check - успешно
- `2026-02-08 19:04` Выполнен шаг 7: Сборка WASM target - успешно
- `2026-02-08 19:05` Выполнен шаг 8: Jest интеграционные тесты - 225 тестов пройдено
- `2026-02-08 19:05` Выполнен шаг 9: Финальная сборка WASM - успешно
- `2026-02-08 19:05` Определены критерии приёмки: workspace создан, сборка работает, тесты проходят
- `2026-02-08 19:05` Готово к review

---

## [x] HYBRID_EXTRACT-002: Extract ID generation and path utilities to shared core crate

### 📋 Metadata
- **status:** `ready`
- **depends:** `HYBRID_EXTRACT-001`
- **priority:** `P0`
- **files:** `crates/react-intl-core/src/lib.rs`, `crates/react-intl-core/src/id_generator.rs`, `crates/react-intl-core/src/path_utils.rs`, `src/utils.rs`

### 📝 Details

Вынести функции генерации ID и работы с путями в отдельный crate для переиспользования между plugin и CLI.

**Требования:**
- Функции `create_hash()`, `hash_string()` - генерация хэшей
- Функции `get_prefix()`, `dot_path()`, `find_project_root()` - работа с путями
- Типы `PluginOptions`, `PluginState` - конфигурация
- Все функции должны быть platform-agnostic (не зависеть от WASM)

**Проблемные места:**
- `src/utils.rs` содержит 497 строк, нужно выделить общую часть
- Тип `PluginState` зависит от `PathBuf`, нужно сделать его shared
- Тесты нужно перенести или продублировать

**Изменения:**
1. Создать `crates/react-intl-core/src/id_generator.rs`:
   ```rust
   pub fn hash_murmur3(input: &str) -> String
   pub fn hash_base64(input: &str) -> String
   pub fn generate_message_id(prefix: &str, options: &ExtractionOptions) -> String
   ```

2. Создать `crates/react-intl-core/src/path_utils.rs`:
   ```rust
   pub fn get_prefix(filename: &Path, opts: &ExtractionOptions) -> String
   pub fn find_project_root(file_path: &Path) -> Option<PathBuf>
   pub fn dot_path(path: &str, separator: &str) -> String
   ```

3. Обновить `src/utils.rs` - импортировать из shared core

**Влияние:**
- Общий crate используется plugin и CLI
- Гарантия идентичной генерации ID
- Упрощение тестирования (тесты в одном месте)

### 📊 ActionLog:
- `2026-02-08 18:48` План задачи создан
- `2026-02-08 20:11` Данные актуализированы: проверены utils.rs (497 строк), react-intl-core placeholder'ы
- `2026-02-08 20:11` Статус изменен на `in-progress`
- `2026-02-08 20:11` План согласован с USER, начато выполнение
- `2026-02-08 20:12` Выполнен шаг 1: Перенесены ID generation функции в react-intl-core (create_hash, hash_string)
- `2026-02-08 20:13` Выполнен шаг 2: Перенесены path utilities в react-intl-core (get_prefix, find_project_root, dot_path)
- `2026-02-08 20:13` Выполнен шаг 3: Обновлены типы в react-intl-core (CoreOptions, CoreState + aliases)
- `2026-02-08 20:14` Выполнен шаг 4: Обновлен swc-plugin для использования shared core
- `2026-02-08 20:15` Выполнен шаг 5: Перенесены тесты (9 тестов в react-intl-core + doc tests)
- `2026-02-08 20:16` Выполнен шаг 6: Проверка компиляции - успешно
- `2026-02-08 20:17` Выполнен шаг 7: Тесты react-intl-core - 9 тестов + 5 doc tests пройдено
- `2026-02-08 20:17` Выполнен шаг 8: Тесты swc-plugin - 6 тестов пройдено
- `2026-02-08 20:18` Выполнен шаг 9: Jest интеграционные тесты - 225 тестов пройдено
- `2026-02-08 20:19` Выполнен шаг 10: Сборка WASM target - успешно
- `2026-02-08 20:19` Определены критерии приёмки: все выполнены
- `2026-02-08 20:19` Готово к review
- `2026-02-08 20:20` Review: USER одобрил ✓
- `2026-02-08 20:20` Задача завершена, статус изменен на `ready`

---

## [ ] HYBRID_EXTRACT-003: Extract AST traversal logic to shared core crate

### 📋 Metadata
- **status:** `todo`
- **depends:** `HYBRID_EXTRACT-002`
- **priority:** `P0`
- **files:** `crates/react-intl-core/src/message_extractor.rs`, `src/visitors.rs`, `src/types.rs`

### 📝 Details

Вынести логику извлечения сообщений из AST в shared core crate. Это позволит CLI использовать ту же логику обхода AST, что и плагин.

**Требования:**
- Поддержка `defineMessages()` - извлечение всех сообщений из объекта
- Поддержка `<FormattedMessage>` - извлечение из JSX атрибутов
- Поддержка `formatMessage()` - извлечение из вызовов функций
- Работа без трансформации (только чтение AST)
- Возврат структурированных данных о сообщениях

**Проблемные места:**
- Текущий `src/visitors.rs` (34313 байт) содержит visitor'ы для трансформации
- Нужно создать отдельные visitor'ы только для извлечения
- SWC AST типы могут отличаться между версиями

**Изменения:**
1. Создать `crates/react-intl-core/src/message_extractor.rs`:
   ```rust
   pub struct ExtractedMessage {
       pub id: String,
       pub default_message: String,
       pub description: Option<String>,
       pub file: Option<String>,
       pub line: Option<usize>,
   }

   pub fn extract_messages(code: &str, filename: &str, options: &ExtractionOptions) -> Vec<ExtractedMessage>
   ```

2. Создать visitor'ы для извлечения (без трансформации):
   - `MessageExtractorVisitor` - общий visitor
   - `DefineMessagesExtractor` - для defineMessages
   - `JSXMessageExtractor` - для FormattedMessage
   - `CallExpressionExtractor` - для formatMessage

3. Обновить `src/visitors.rs` - использовать shared логику для генерации ID

**Влияние:**
- CLI может извлекать сообщения без дублирования кода
- Единая логика парсинга для plugin и CLI
- Возможность тестировать извлечение отдельно от трансформации

### 📊 ActionLog:
- `2026-02-08 18:48` План задачи создан

---

## [ ] HYBRID_EXTRACT-004: Create CLI tool crate with message extraction

### 📋 Metadata
- **status:** `todo`
- **depends:** `HYBRID_EXTRACT-003`
- **priority:** `P0`
- **files:** `crates/cli/Cargo.toml`, `crates/cli/src/main.rs`

### 📝 Details

Создать CLI инструмент на Rust для извлечения сообщений из исходных файлов.

**Требования:**
- Бинарный crate с точкой входа `main()`
- Использует shared core для извлечения сообщений
- Поддержка glob паттернов для файлов
- Обработка ошибок с понятными сообщениями
- Прогресс-бар для больших проектов (опционально)

**Проблемные места:**
- Нужно парсить glob паттерны (`src/**/*.{ts,tsx}`)
- Обработка множества файлов асинхронно
- Управление памятью при большом количестве файлов

**Изменения:**
1. Создать `crates/cli/Cargo.toml`:
   ```toml
   [package]
   name = "react-intl-extract-cli"
   version = "1.0.0"
   edition = "2021"

   [[bin]]
   name = "react-intl-extract"
   path = "src/main.rs"

   [dependencies]
   react-intl-core = { path = "../react-intl-core" }
   clap = { version = "4", features = ["derive"] }
   glob = "0.3"
   serde_json = "1"
   anyhow = "1"
   ```

2. Создать `crates/cli/src/main.rs`:
   ```rust
   use clap::Parser;
   use react_intl_core::{extract_messages, ExtractionOptions};

   #[derive(Parser)]
   struct Args {
       #[arg(help = "Glob pattern for source files")]
       pattern: String,
       
       #[arg(short, long, help = "Output file or directory")]
       output: String,
       
       #[arg(long, help = "Output format")]
       format: Option<String>,
       
       #[arg(long, help = "Include source file paths")]
       extract_source_location: bool,
       
       #[arg(long, help = "Hash message IDs")]
       hash_id: bool,
       
       #[arg(long, help = "Hash algorithm (murmur3, base64)")]
       hash_algorithm: Option<String>,
   }

   fn main() -> anyhow::Result<()> {
       let args = Args::parse();
       // Implementation...
   }
   ```

**Влияние:**
- Новый бинарный артефакт для npm пакета
- Возможность запускать извлечение отдельно от сборки
- Зависимость от shared core

### 📊 ActionLog:
- `2026-02-08 18:48` План задачи создан

---

## [ ] HYBRID_EXTRACT-005: Implement CLI argument parsing and file globbing

### 📋 Metadata
- **status:** `todo`
- **depends:** `HYBRID_EXTRACT-004`
- **priority:** `P1`
- **files:** `crates/cli/src/main.rs`, `crates/cli/src/glob.rs`

### 📝 Details

Реализовать парсинг аргументов CLI и поиск файлов по glob паттернам.

**Требования:**
- Поддержка glob паттернов: `src/**/*.{ts,tsx,js,jsx}`
- Рекурсивный обход директорий
- Фильтрация по расширениям
- Игнорирование `node_modules`, `.git`
- Поддержка относительных и абсолютных путей

**Проблемные места:**
- Glob библиотеки для Rust имеют разный API
- Нужно обрабатывать ошибки чтения файлов
- Производительность на больших проектах (10k+ файлов)

**Изменения:**
1. Реализовать функцию globbing:
   ```rust
   pub fn find_files(pattern: &str, exclude: &[&str]) -> anyhow::Result<Vec<PathBuf>> {
       // Implementation using glob crate
   }
   ```

2. CLI должен принимать аргументы:
   ```bash
   react-intl-extract 'src/**/*.{ts,tsx}' --output .react-intl/messages.json
   ```

3. Опции CLI:
   - `--output` / `-o` - путь к выходному файлу или директории
   - `--format` - формат вывода (json)
   - `--hash-id` - хэшировать ID
   - `--hash-algorithm` - алгоритм хэширования
   - `--extract-source-location` - добавлять пути к файлам
   - `--relative-to` - базовый путь для относительных путей

**Влияние:**
- CLI становится полнофункциональным инструментом
- Удобство использования через командную строку

### 📊 ActionLog:
- `2026-02-08 18:48` План задачи создан

---

## [ ] HYBRID_EXTRACT-006: Implement JSON output format (aggregated and per-file)

### 📋 Metadata
- **status:** `todo`
- **depends:** `HYBRID_EXTRACT-005`
- **priority:** `P1`
- **files:** `crates/cli/src/output.rs`, `crates/cli/src/main.rs`

### 📝 Details

Реализовать вывод извлеченных сообщений в JSON формате с поддержкой двух режимов: агрегированный файл и отдельные файлы по исходникам.

**Требования:**
- **Агрегированный режим** (когда output - файл):
  ```json
  [
    {"id": "hello", "defaultMessage": "Hello", "file": "src/App.tsx"},
    {"id": "world", "defaultMessage": "World", "file": "src/Button.tsx"}
  ]
  ```

- **Per-file режим** (когда output - директория):
  ```
  .react-intl/
  ├── src/
  │   ├── App.json
  │   └── Button.json
  └── messages.json (опционально, агрегированный)
  ```

**Проблемные места:**
- Создание директорий при per-file режиме
- Корректная сериализация в JSON с pretty print
- Обработка коллизий имен файлов

**Изменения:**
1. Создать `crates/cli/src/output.rs`:
   ```rust
   pub enum OutputMode {
       Aggregated(PathBuf),  // Single file
       PerFile(PathBuf),     // Directory with separate files
   }

   pub fn write_messages(
       messages: Vec<ExtractedMessage>,
       mode: OutputMode,
       options: &OutputOptions,
   ) -> anyhow::Result<()> {
       // Implementation
   }
   ```

2. Примеры использования:
   ```bash
   # Агрегированный файл
   react-intl-extract 'src/**/*.ts' --output messages.json

   # Отдельные файлы
   react-intl-extract 'src/**/*.ts' --output .react-intl/
   ```

**Влияние:**
- Гибкость в использовании (CI/CD может использовать агрегированный, разработчики - per-file)
- Совместимость с различными workflows

### 📊 ActionLog:
- `2026-02-08 18:48` План задачи создан

---

## [ ] HYBRID_EXTRACT-007: Add source location extraction option

### 📋 Metadata
- **status:** `todo`
- **depends:** `HYBRID_EXTRACT-006`
- **priority:** `P1`
- **files:** `crates/react-intl-core/src/message_extractor.rs`, `crates/cli/src/main.rs`

### 📝 Details

Добавить опцию для включения информации о местоположении исходного файла в извлеченные сообщения.

**Требования:**
- Опция `--extract-source-location` включает добавление поля `file`
- Путь должен быть относительным (относительно project root или `relative_to`)
- Должна быть опция отключения для минимизации размера JSON

**Проблемные места:**
- Вычисление относительного пути для каждого сообщения
- Сохранение информации о строке (line number) из AST
- Консистентность путей между Windows и Unix

**Изменения:**
1. Обновить `ExtractedMessage`:
   ```rust
   pub struct ExtractedMessage {
       pub id: String,
       pub default_message: String,
       pub description: Option<String>,
       #[serde(skip_serializing_if = "Option::is_none")]
       pub file: Option<String>,
       #[serde(skip_serializing_if = "Option::is_none")]
       pub line: Option<usize>,
   }
   ```

2. Обновить CLI:
   ```rust
   #[arg(long, help = "Include source file location")]
   extract_source_location: bool,
   ```

3. Пример вывода:
   ```json
   [
     {
       "id": "hello",
       "defaultMessage": "Hello",
       "file": "src/components/App.tsx",
       "line": 42
     }
   ]
   ```

**Влияние:**
- Полезно для отладки и локализации
- Помогает переводчикам найти контекст

### 📊 ActionLog:
- `2026-02-08 18:48` План задачи создан

---

## [ ] HYBRID_EXTRACT-008: Create JS API with napi-rs bindings

### 📋 Metadata
- **status:** `todo`
- **depends:** `HYBRID_EXTRACT-003`
- **priority:** `P1`
- **files:** `src-node/lib.rs`, `index.js`, `package.json`

### 📝 Details

Создать Node.js API для программного доступа к функционалу извлечения сообщений через Rust bindings (napi-rs).

**Требования:**
- Асинхронная функция `extractMessages()` для извлечения из строки кода
- Поддержка всех опций из CLI
- TypeScript definitions для автодополнения
- Производительность через native bindings

**Проблемные места:**
- Настройка napi-rs требует дополнительных зависимостей
- Кросс-компиляция для разных платформ (Windows, macOS, Linux)
- Интеграция с существующей сборкой WASM

**Изменения:**
1. Добавить napi-rs зависимости:
   ```toml
   [dependencies]
   napi = { version = "2", features = ["async"] }
   napi-derive = "2"
   react-intl-core = { path = "../react-intl-core" }

   [build-dependencies]
   napi-build = "2"
   ```

2. Создать `src-node/lib.rs`:
   ```rust
   use napi::bindgen_prelude::*;
   use napi_derive::napi;
   use react_intl_core::{extract_messages, ExtractionOptions};

   #[napi(object)]
   pub struct JsExtractedMessage {
       pub id: String,
       pub default_message: String,
       pub description: Option<String>,
       pub file: Option<String>,
   }

   #[napi]
   pub async fn extract_messages(
       code: String,
       filename: String,
       options: Option<JsExtractionOptions>,
   ) -> Result<Vec<JsExtractedMessage>> {
       // Implementation
   }
   ```

3. Обновить `index.js`:
   ```javascript
   const { extractMessages } = require('./index.node');

   module.exports = {
     extractMessages,
     // ... existing WASM plugin exports
   };
   ```

**Влияние:**
- Возможность использования в кастомных скриптах сборки
- Интеграция с CI/CD пайплайнами
- Требует обновление CI для сборки native bindings

### 📊 ActionLog:
- `2026-02-08 18:48` План задачи создан

---

## [ ] HYBRID_EXTRACT-009: Update package.json with CLI bin entry and JS API exports

### 📋 Metadata
- **status:** `todo`
- **depends:** `HYBRID_EXTRACT-004`, `HYBRID_EXTRACT-008`
- **priority:** `P1`
- **files:** `package.json`, `index.js`

### 📝 Details

Обновить npm пакет для поддержки CLI и JS API экспортов.

**Требования:**
- CLI доступен как `npx swc-plugin-react-intl-auto-fs extract`
- JS API экспортирует `extractMessages`
- Обратная совместимость с существующими импортами WASM плагина
- TypeScript definitions для JS API

**Проблемные места:**
- Нужно скопировать CLI бинарник в npm пакет
- Настройка bin entry в package.json
- Поддержка разных платформ (postinstall scripts)

**Изменения:**
1. Обновить `package.json`:
   ```json
   {
     "name": "swc-plugin-react-intl-auto-fs",
     "main": "index.js",
     "types": "index.d.ts",
     "bin": {
       "react-intl-extract": "./bin/react-intl-extract"
     },
     "scripts": {
       "build": "node build.js && npm run build:native",
       "build:native": "napi build --platform",
       "install": "napi install"
     },
     "napi": {
       "name": "swc-plugin-react-intl-auto-fs",
       "triples": {
         "defaults": true,
         "additional": ["x86_64-pc-windows-msvc", "aarch64-apple-darwin"]
       }
     }
   }
   ```

2. Обновить `index.js`:
   ```javascript
   const { existsSync } = require('fs');
   const { join } = require('path');

   // Try to load native bindings (napi-rs)
   let native;
   try {
     native = require('./index.node');
   } catch (e) {
     // Native bindings not available on this platform
   }

   module.exports = {
     // WASM plugin exports (existing)
     getPluginPath: () => join(__dirname, 'swc-plugin-react-intl-auto-fs.wasm'),
     getDefaultOptions: () => ({ /* ... */ }),
     
     // JS API exports (new)
     extractMessages: native?.extractMessages || (async () => {
       throw new Error('Native bindings not available. Please install from npm.');
     }),
   };
   ```

**Влияние:**
- Пакет становится более функциональным
- Требует обновление CI для сборки native binaries
- Возможны проблемы с совместимостью на разных платформах

### 📊 ActionLog:
- `2026-02-08 18:48` План задачи создан

---

## [ ] HYBRID_EXTRACT-010: Create integration tests for ID consistency between plugin and CLI

### 📋 Metadata
- **status:** `todo`
- **depends:** `HYBRID_EXTRACT-007`, `HYBRID_EXTRACT-009`
- **priority:** `P2`
- **files:** `__tests__/consistency.test.js`, `__tests__/cli.test.js`

### 📝 Details

Создать интеграционные тесты, проверяющие что плагин и CLI генерируют идентичные ID для одного и того же кода.

**Требования:**
- Тесты на Jest
- Сравнение ID между трансформацией плагина и извлечением CLI
- Различные сценарии: defineMessages, FormattedMessage, formatMessage
- Разные опции: hash_id, hash_algorithm, remove_prefix

**Проблемные места:**
- Нужно запускать CLI как subprocess в тестах
- Синхронизация между WASM плагином и CLI
- Тестирование разных платформ

**Изменения:**
1. Создать `__tests__/consistency.test.js`:
   ```javascript
   const { transform } = require('@swc/core');
   const { execSync } = require('child_process');
   const { extractMessages } = require('../index.js');

   describe('ID Consistency between Plugin and CLI', () => {
     const testCode = `
       import { defineMessages } from 'react-intl';
       const messages = defineMessages({
         hello: 'Hello World'
       });
     `;

     it('should generate identical IDs with murmur3 hash', async () => {
       // Transform with plugin
       const pluginResult = await transform(testCode, {
         filename: 'test.tsx',
         plugins: [['./swc-plugin-react-intl-auto-fs.wasm', {
           hashId: true,
           hashAlgorithm: 'murmur3'
         }]]
       });
       const pluginId = extractIdFromCode(pluginResult.code);

       // Extract with CLI
       const cliResult = execSync(
         './bin/react-intl-extract test.tsx --hash-id --hash-algorithm murmur3 --output -',
         { input: testCode, encoding: 'utf8' }
       );
       const cliId = JSON.parse(cliResult)[0].id;

       expect(pluginId).toBe(cliId);
     });

     it('should generate identical IDs with base64 hash', async () => {
       // Similar test for base64...
     });
   });
   ```

2. Создать `__tests__/cli.test.js` для тестирования CLI:
   ```javascript
   describe('CLI Tool', () => {
     it('should extract messages to JSON file', () => {
       // Test CLI output format
     });

     it('should support --extract-source-location', () => {
       // Test source location inclusion
     });
   });
   ```

**Влияние:**
- Гарантия консистентности между компонентами
- Раннее обнаружение регрессий
- Увеличение покрытия тестами

### 📊 ActionLog:
- `2026-02-08 18:48` План задачи создан

---

## [ ] HYBRID_EXTRACT-011: Create example projects (webpack, CLI-only, JS API)

### 📋 Metadata
- **status:** `todo`
- **depends:** `HYBRID_EXTRACT-010`
- **priority:** `P2`
- **files:** `examples/**/*`

### 📝 Details

Создать примеры проектов демонстрирующие различные сценарии использования.

**Требования:**
- **webpack-project**: Пример интеграции с webpack + SWC loader
- **cli-only**: Пример использования только CLI (без плагина)
- **jsapi-project**: Пример использования JS API в кастомном скрипте
- Каждый пример должен иметь README с инструкциями
- Примеры должны работать после `npm install && npm run build`

**Проблемные места:**
- Примеры должны использовать локальную версию пакета (npm link или file:)
- Размер репозитория увеличится
- Нужно поддерживать примеры в актуальном состоянии

**Изменения:**
1. Структура `examples/`:
   ```
   examples/
   ├── README.md
   ├── webpack-project/
   │   ├── README.md
   │   ├── webpack.config.js
   │   ├── src/
   │   │   ├── components/
   │   │   │   ├── App.tsx
   │   │   │   └── Button.tsx
   │   │   └── messages.ts
   │   └── package.json
   ├── cli-only/
   │   ├── README.md
   │   ├── src/
   │   │   └── messages.ts
   │   └── package.json
   └── jsapi-project/
       ├── README.md
       ├── extract.js
       └── package.json
   ```

2. Каждый пример должен включать:
   - `package.json` с зависимостями
   - Исходный код с React Intl сообщениями
   - README с инструкциями по запуску
   - Ожидаемый вывод (результат трансформации или извлечения)

**Влияние:**
- Упрощение onboarding для новых пользователей
- Демонстрация различных use cases
- Интеграционное тестирование через примеры

### 📊 ActionLog:
- `2026-02-08 18:48` План задачи created

---

## [ ] HYBRID_EXTRACT-012: Update documentation and README

### 📋 Metadata
- **status:** `todo`
- **depends:** `HYBRID_EXTRACT-011`
- **priority:** `P2`
- **files:** `README.md`, `docs/CLI.md`, `docs/JS_API.md`, `docs/ARCHITECTURE.md`

### 📝 Details

Обновить документацию, описав новые компоненты (CLI, JS API) и архитектуру решения.

**Требования:**
- Обновить основной README с описанием CLI и JS API
- Создать отдельную документацию для CLI (опции, примеры)
- Создать документацию для JS API (функции, TypeScript types)
- Создать документ с описанием архитектуры (workspace, shared core)
- Обновить CHANGELOG

**Проблемные места:**
- Много новой информации, нужна хорошая структура
- Примеры кода должны быть протестированы
- Нужно описать все опции CLI и JS API

**Изменения:**
1. Обновить `README.md`:
   - Добавить раздел "CLI Tool"
   - Добавить раздел "JS API"
   - Обновить раздел "Architecture"
   - Добавить сравнение с babel-plugin-react-intl

2. Создать `docs/CLI.md`:
   ```markdown
   # CLI Tool Documentation

   ## Installation
   ## Usage
   ## Options
   ## Examples
   ```

3. Создать `docs/JS_API.md`:
   ```markdown
   # JavaScript API

   ## extractMessages
   ## Options
   ## Examples
   ```

4. Создать `docs/ARCHITECTURE.md`:
   ```markdown
   # Architecture

   ## Workspace Structure
   ## Shared Core Library
   ## Component Interaction
   ## ID Generation Consistency
   ```

**Влияние:**
- Улучшение UX для разработчиков
- Снижение количества вопросов и issues
- Лучшее понимание архитектуры

### 📊 ActionLog:
- `2026-02-08 18:48` План задачи создан

---

## ✅ Критерии готовности EPIC

- [ ] Workspace структура Cargo создана и работает
- [ ] Shared Core Library содержит всю общую логику
- [ ] CLI Tool компилируется и проходит тесты
- [ ] JS API работает и имеет TypeScript definitions
- [ ] ID consistency тесты проходят (плагин и CLI генерируют одинаковые ID)
- [ ] Примеры проектов работают и протестированы
- [ ] Документация обновлена и актуальна
- [ ] CI/CD пайплайн обновлен для новых компонентов
- [ ] Пакет публикуется в npm без ошибок

---

## 📊 Сводка по задачам

| Task ID | Название | Приоритет | Зависимости | Статус |
|---------|----------|-----------|-------------|--------|
| HYBRID_EXTRACT-001 | Create Cargo workspace structure | P0 | - | ⏳ |
| HYBRID_EXTRACT-002 | Extract ID generation to shared core | P0 | 001 | ⏳ |
| HYBRID_EXTRACT-003 | Extract AST traversal to shared core | P0 | 002 | ⏳ |
| HYBRID_EXTRACT-004 | Create CLI tool crate | P0 | 003 | ⏳ |
| HYBRID_EXTRACT-005 | CLI argument parsing and globbing | P1 | 004 | ⏳ |
| HYBRID_EXTRACT-006 | JSON output format | P1 | 005 | ⏳ |
| HYBRID_EXTRACT-007 | Source location extraction | P1 | 006 | ⏳ |
| HYBRID_EXTRACT-008 | Create JS API with napi-rs | P1 | 003 | ⏳ |
| HYBRID_EXTRACT-009 | Update package.json with CLI and JS API | P1 | 004, 008 | ⏳ |
| HYBRID_EXTRACT-010 | Integration tests for ID consistency | P2 | 007, 009 | ⏳ |
| HYBRID_EXTRACT-011 | Create example projects | P2 | 010 | ⏳ |
| HYBRID_EXTRACT-012 | Update documentation | P2 | 011 | ⏳ |

---

## ✅ EPIC план создан

**Файл:** `.ai/plans/HYBRID_EXTRACT.hybrid-message-extraction-plan.md`

**Дата создания:** 2026-02-08 18:48

**Следующие шаги:**
- Используйте `tasks` для просмотра всех задач
- Используйте `task HYBRID_EXTRACT-001` для начала работы с первой задачей
- Рекомендуется начать с задач приоритета P0 без зависимостей (HYBRID_EXTRACT-001)
