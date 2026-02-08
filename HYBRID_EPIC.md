# HYBRID_EPIC: Гибридное решение для извлечения сообщений React Intl

## 🎯 Цель

Создать гибридное решение для извлечения сообщений React Intl, состоящее из:
1. **SWC Plugin** - трансформация кода (добавление ID)
2. **CLI Tool** - извлечение сообщений в JSON файлы
3. **JS API** - программный доступ к функционалу

**Ключевое требование**: Переиспользование Rust кода между плагином и CLI для гарантии генерации **идентичных ID**.

---

## 🏗️ Архитектура решения

Компоненты:
- Shared Rust Core (lib crate):
  - Functions:
    - ID Generation
    - AST Traversal
    - Message Extraction
    - Path Resolution
  - Dependencies: -
- JS API:
  - Functions:
    - Message Extraction in js files
  - Dependencies:
    - Shared Rust Core (lib crate)
- SWC Plugin (WASM)
  - Functions:
    - Code transform
  - Dependencies:
    - Shared Rust Core (lib crate)
- CLI Tool:
  - Functions:
    - Message extraction and saving to files
    - Run via npm/npx
  - Dependencies:
    - JS API

---

## 📦 Компоненты системы

### 1. SWC Plugin (`swc-plugin-react-intl-auto-fs`)

**Назначение**: Трансформация исходного кода - добавление ID к сообщениям

**Что делает**:
- Обрабатывает `defineMessages()`, `formatMessage()`, `<FormattedMessage>`
- Генерирует ID сообщений согласно настройкам (hash_id, hash_algorithm)
- Добавляет ID в AST (Abstract Syntax Tree)
- Выводит обработанный код

**Что НЕ делает**:
- Не записывает файлы (WASM ограничения)
- Не сохраняет состояние между файлами

**Пример использования**:
```javascript
// webpack.config.js
module.exports = {
  module: {
    rules: [{
      test: /\.(ts|tsx|js|jsx)$/,
      use: {
        loader: 'swc-loader',
        options: {
          jsc: {
            experimental: {
              plugins: [
                ['swc-plugin-react-intl-auto-fs', {
                  hashId: true,
                  hashAlgorithm: 'base64'
                }]
              ]
            }
          }
        }
      }
    }]
  }
}
```

**Исходный код**:
```typescript
import { defineMessages } from 'react-intl';

const messages = defineMessages({
  hello: 'Hello World',
  greeting: {
    defaultMessage: 'Welcome!',
    description: 'A greeting'
  }
});
```

**Результат трансформации**:
```typescript
import { defineMessages } from 'react-intl';

const messages = defineMessages({
  hello: {
    id: 'aGVsbG8=',           // base64 хэш
    defaultMessage: 'Hello World'
  },
  greeting: {
    id: 'Z3JlZXRpbmc=',       // base64 хэш
    defaultMessage: 'Welcome!',
    description: 'A greeting'
  }
});
```

---


### 3. JS API

**Назначение**: Программный доступ к функционалу из Node.js

**Варианты использования**:
- Кастомные скрипты сборки
- Интеграция с CI/CD
- Тестирование

**API**:
```javascript
const { extractMessages } = require('swc-plugin-react-intl-auto-fs');

// Вариант 1: Извлечь сообщения из кода
const messages = await extractMessages({
  code: 'const messages = defineMessages({ hello: "World" })',
  filename: 'test.tsx',
  options: {
    hashId: true,
    hashAlgorithm: 'base64'
  }
});

console.log(messages);
// [
//   { id: 'aGVsbG8=', defaultMessage: 'World' }
// ]
```

---


### 3. CLI Tool (`swc-plugin-react-intl-auto-fs extract`)

**Назначение**: Извлечение сообщений в JSON файлы

**Что делает**:
- Сканирует исходные файлы (поддерживает glob паттерны)
- Применяет ту же логику генерации ID, что и плагин
- Сохраняет сообщения в JSON файлы
- Поддерживает различные форматы вывода

**Преимущества**:
- Полный доступ к файловой системе
- Может обрабатывать множество файлов
- Не зависит от инструментов сборки (webpack, vite, etc.)

**Установка**:
```bash
npm install -D swc-plugin-react-intl-auto-fs
```

**Использование**:
Run:
```bash
# Базовое использование
npx swc-plugin-react-intl-auto-fs extract 'src/**/*.{ts,tsx}' --output .react-intl/messages.json
```
Output:
```json
// .react-intl/messages.json
// выделенные описания сообщений содержат все преданные и сгенерированные параметры, а не
// только те, что указанны в примере
[{"id": "generated_id", "defaultMessage": "Hello"}]
```

Run:
```sh
# С указанием формата
npx swc-plugin-react-intl-auto-fs extract 'src/**/*.{ts,tsx}' --output .react-intl/messages.json --format json
```
Output:
```json
// .react-intl/messages.json
[{"id": "generated_id", "defaultMessage": "Hello"}]
```

Run:
```sh
# С указание пути до исходника
# добавляет пле file - относительный путь от корня проекта или cwd до 
# исходного файла, содержащего описание сообщение
npx swc-plugin-react-intl-auto-fs extract 'src/**/*.{ts,tsx}' --output .react-intl/messages.json --extract-source-location
```
Output:
```json
// .react-intl/messages.json
[{"id": "generated_id", "defaultMessage": "Hello", "file": "src/components/App.tsx"}]
```

Run:
```sh
# С хэшированием ID
npx swc-plugin-react-intl-auto-fs extract 'src/**/*.{ts,tsx}' --output .react-intl/messages.json --hash-id --hash-algorithm base64
```
Output:
```json
// .react-intl/messages.json
[{"id": "generated_id_hashed", "defaultMessage": "Hello"}]
```

Run:
```sh
# С указанием output как директории
# Выделяет по отдельному файлу на каждый исходник с описанием сообщений
npx swc-plugin-react-intl-auto-fs extract 'src/**/*.{ts,tsx}' --output .react-intl
```
Output:
```json
// .react-intl/components/App.json
[{"id": "generated_id", "defaultMessage": "Hello"}]
// .react-intl/components/Button.json
[{"id": "generated_id", "defaultMessage": "World"}]
```


**Структура выходных файлов**:
```
.react-intl/
├── components/
│   ├── App.json
│   └── Button.json
├── pages/
│   └── Home.json
└── messages.json          # агрегированный файл (опционально)
```

**Пример содержимого App.json**:
```json
[
  {
    "id": "aGVsbG8=",
    "defaultMessage": "Hello World",
    "file": "src/components/App.tsx"
  },
  {
    "id": "Z3JlZXRpbmc=",
    "defaultMessage": "Welcome!",
    "file": "src/components/App.tsx"
  }
]
```

---

## 🔧 Переиспользование Rust кода

### Проблема
Нужно гарантировать, что плагин и CLI генерируют **идентичные ID** для одинаковых сообщений.

### Решение
Создать shared Rust library crate, используемую обоими компонентами.

### Структура проекта
```
swc-plugin-react-intl-auto-fs/
├── Cargo.toml                    # Workspace definition
├── package.json
├── src/                          # SWC Plugin (WASM target)
│   ├── lib.rs
│   ├── types.rs
│   ├── utils.rs
│   └── visitors.rs
├── crates/
│   └── react-intl-core/         # Shared library
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── id_generator.rs   # Генерация ID
│           ├── message_extractor.rs
│           ├── path_utils.rs     # Работа с путями
│           └── ast_utils.rs      # Утилиты для AST
├── cli/                          # CLI Tool
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
└── index.js                      # JS API
```

### Shared Core Library

**crates/react-intl-core/src/lib.rs**:
```rust
//! Core library for React Intl message extraction and ID generation
//! Used by both SWC plugin and CLI tool

pub mod id_generator;
pub mod message_extractor;
pub mod path_utils;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub default_message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ExtractionOptions {
    pub hash_id: bool,
    pub hash_algorithm: String,  // "murmur3", "base64"
    pub include_source_location: bool,
    pub separator: String,
}

impl Default for ExtractionOptions {
    fn default() -> Self {
        Self {
            hash_id: false,
            hash_algorithm: "murmur3".to_string(),
            include_source_location: false,
            separator: ".".to_string(),
        }
    }
}

/// Генерирует ID для сообщения
pub fn generate_message_id(
    prefix: &str,
    default_message: &str,
    options: &ExtractionOptions,
) -> String {
    use id_generator::*;
    
    let base_id = if options.hash_id {
        match options.hash_algorithm.as_str() {
            "base64" => hash_base64(prefix),
            _ => hash_murmur3(prefix),
        }
    } else {
        prefix.to_string()
    };
    
    base_id
}
```

**crates/react-intl-core/src/id_generator.rs**:
```rust
use murmur3::murmur3_32;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use std::io::Cursor;

/// murmur3 hash с seed=0 (совместимо с babel-plugin-react-intl)
pub fn hash_murmur3(input: &str) -> String {
    let mut cursor = Cursor::new(input);
    let hash = murmur3_32(&mut cursor, 0).unwrap_or(0);
    hash.to_string()
}

/// base64 encoding
pub fn hash_base64(input: &str) -> String {
    BASE64.encode(input.as_bytes())
}

/// Создает суффикс из defaultMessage
pub fn create_suffix(default_message: &str) -> String {
    hash_murmur3(default_message)
}
```

### Гарантия идентичности

**Пример**:
```typescript
// Исходный код (одинаковый для плагина и CLI)
const messages = defineMessages({
  hello: 'Hello World'
});
```

**Плагин генерирует**:
```typescript
// С hash_id: true, hashAlgorithm: 'base64'
{
  id: 'Y29tcG9uZW50cy5oZWxsbw==',  // base64('components.hello')
  defaultMessage: 'Hello World'
}
```

**CLI извлекает**:
```json
[
  {
    "id": "Y29tcG9uZW50cy5oZWxsbw==",
    "defaultMessage": "Hello World",
    "file": "src/components/App.tsx"
  }
]
```

**✅ ID идентичны!**

---

## 📝 Варианты использования

### Сценарий 1: Webpack + SWC Loader (Только трансформация)

**Цель**: Быстрая сборка с добавлением ID

```javascript
// webpack.config.js
module.exports = {
  module: {
    rules: [{
      test: /\.(ts|tsx)$/,
      use: {
        loader: 'swc-loader',
        options: {
          jsc: {
            parser: { syntax: 'typescript', tsx: true },
            experimental: {
              plugins: [
                ['swc-plugin-react-intl-auto-fs', {
                  hashId: true,
                  hashAlgorithm: 'murmur3'
                }]
              ]
            }
          }
        }
      }
    }]
  }
}
```

**Результат**: Код трансформирован, ID добавлены в бандл

---

### Сценарий 2: CLI для извлечения сообщений

**Цель**: Получить JSON с сообщениями для переводчиков

```bash
# Установка
npm install -D swc-plugin-react-intl-auto-fs

# Извлечение
npx swc-plugin-react-intl-auto-fs extract 'src/**/*.{ts,tsx}' --output .react-intl --extract-source-location

# Результат
ls -la .react-intl/
# components/App.json
# components/Button.json
# messages.json (агрегированный)
```

---

### Сценарий 3: Совместное использование Webpack + CLI

**Цель**: Полная интеграция в процесс разработки

```json
// package.json
{
  "scripts": {
    "build": "npm run extract && webpack --mode=production",
    "extract": "npx swc-plugin-react-intl-auto-fs extract 'src/**/*.{ts,tsx}' --output .react-intl"
  }
}
```

```javascript
// webpack.config.js
module.exports = {
  module: {
    rules: [{
      test: /\.(ts|tsx)$/,
      use: {
        loader: 'swc-loader',
        options: {
          jsc: {
            experimental: {
              plugins: [
                ['swc-plugin-react-intl-auto-fs', {
                  hashId: true,  // Должно совпадать с CLI!
                  hashAlgorithm: 'murmur3'
                }]
              ]
            }
          }
        }
      }
    }]
  }
}
```

**Важно**: Настройки hash_id и hashAlgorithm должны быть идентичны в плагине и CLI!

---

### Сценарий 4: Программное использование (JS API)

**Цель**: Кастомная сборка или тестирование

```javascript
// scripts/extract.js
const { extractMessages } = require('swc-plugin-react-intl-auto-fs');
const fs = require('fs');
const glob = require('glob');

async function extract() {
  const files = glob.sync('src/**/*.{ts,tsx}');
  const allMessages = [];
  
  for (const file of files) {
    const code = fs.readFileSync(file, 'utf-8');
    const messages = await extractMessages({
      code,
      filename: file,
      options: {
        hashId: true,
        hashAlgorithm: 'base64'
      }
    });
    allMessages.push(...messages);
  }
  
  fs.writeFileSync(
    'messages.json',
    JSON.stringify(allMessages, null, 2)
  );
}

extract();
```

---

## 🧪 Требования к тестированию

### 1. Автоматизированные тесты

#### Rust тесты (обязательно)

```bash
# Установка зависимостей
cargo add --dev tempfile
cargo add --dev serial_test

# Запуск тестов
cargo test
```

**Что тестировать**:
- Генерация ID (murmur3, base64)
- Извлечение сообщений из разных паттернов
- Работа с путями
- Сериализация в JSON

**Пример теста**:
```rust
#[test]
fn test_id_generation_consistency() {
    let options = ExtractionOptions {
        hash_id: true,
        hash_algorithm: "base64".to_string(),
        ..Default::default()
    };
    
    // Плагин и CLI должны генерировать одинаковый ID
    let id1 = generate_message_id("components.hello", "Hello", &options);
    let id2 = generate_message_id("components.hello", "Hello", &options);
    
    assert_eq!(id1, id2);
    assert_eq!(id1, "Y29tcG9uZW50cy5oZWxsbw==");
}
```

#### Jest тесты (обязательно)

```bash
# Установка
npm install

# Запуск
npm run build
cargo test
npm test
```

**Что тестировать**:
- Интеграция плагина с SWC
- CLI команды
- JS API
- Генерация одинаковых ID в плагине и CLI

**Пример интеграционного теста**:
```javascript
const { execSync } = require('child_process');

describe('Plugin and CLI consistency', () => {
  it('should generate identical IDs', () => {
    const code = `const messages = defineMessages({ hello: 'World' })`;
    
    // Плагин
    const pluginResult = runPlugin(code, { hashId: true, hashAlgorithm: 'base64' });
    
    // CLI
    const cliResult = runCLI('test.tsx', { hashId: true, hashAlgorithm: 'base64' });
    
    expect(pluginResult.id).toBe(cliResult.id);
    expect(pluginResult.id).toBe('Y29tcG9uZW50cy5oZWxsbw==');
  });
});
```

### 2. Ручное тестирование через примеры

**Требование**: В папке `examples/` должны быть рабочие примеры проектов

#### Структура examples/

**Примечание:** package.json только корневой, все зависимости для тестирования (webpack etc) cтавятся как dev зависимости в корень

```
examples/
├── package.json               # package.json только корневой
├── webpack-project/           # Пример с webpack
│   ├── webpack.config.js
│   ├── src/
│   │   ├── components/
│   │   │   ├── App.tsx
│   │   │   └── Button.tsx
│   │   └── messages.ts
│   └── .swcrc
├── jsapi-project/             # Пример с JS API
│   └── index.js
└── cli-only/                  # Только CLI
    └── src/

```

#### Инструкция по проверке

**Шаг 1**: Установка
```bash
cd examples/webpack-project
npm install
```

**Шаг 2**: Сборка (webpack + плагин)
```bash
npm run build
# Проверить: ID добавлены в dist/
```

**Шаг 3**: Извлечение (CLI)
```bash
npm run extract
# Проверить: созданы файлы в .react-intl/
```

**Шаг 4**: Сравнение ID
```bash
# ID в бандле webpack должны совпадать с ID в .react-intl/
cat dist/main.js | grep "id:"
cat .react-intl/components/App.json | grep "id"
```

---

## 📥 Установка зависимостей

### Для разработки

```bash
# JavaScript зависимости
npm install

# Rust зависимости (общие)
cargo add serde --features derive
cargo add serde_json
cargo add murmur3
cargo add base64

# Для плагина (WASM)
cargo add swc_core --features ecma_plugin_transform,ecma_ast,ecma_visit,ecma_utils

# Для CLI
# (обычные Rust crates, без WASM-специфичных)

# Dev зависимости
npm install -D @swc/core@^1.15.0
npm install -D jest
npm install -D typescript

cargo add --dev tempfile
cargo add --dev serial_test
```

### Для пользователей

```bash
# Локальная установка для проекта
npm install -D swc-plugin-react-intl-auto-fs
```

---

## 🚀 Порядок сборки и тестирования

```bash
# Шаг 1: Сборка WASM плагина
npm run build

# Шаг 2: Запуск Rust тестов
cargo test

# Шаг 3: Запуск Jest тестов
npm test

# Полный цикл
npm run build && cargo test && npm test
```

---

## 📚 Дополнительные замечания

### Ограничения архитектуры

1. **WASM плагин не может писать файлы**: Это архитектурное ограничение SWC/WASM
2. **Плагин и CLI работают независимо**: Нет автоматической синхронизации
3. **Настройки должны совпадать**: hash_id и hashAlgorithm должны быть одинаковы в плагине и CLI

### Рекомендации по использованию

1. **Для разработки**: Используйте webpack/vite с плагином (быстрая пересборка)
2. **Для извлечения сообщений**: Используйте CLI (в `package.json` scripts)
3. **Для CI/CD**: CLI в автоматическом режиме
4. **Для кастомных сценариев**: JS API

---

## ✅ Критерии готовности

- [ ] SWC Plugin компилируется и проходит все тесты
- [ ] Shared Core Library протестирована
- [ ] JS API документирована и протестирована
- [ ] CLI Tool собирается и работает корректно
- [ ] Примеры в `examples` работают
- [ ] Документация написана
- [ ] Плагин и CLI генерируют идентичные ID
