## [0.13.0] - 2026-10-23

### ⚠️ BREAKING CHANGES

- Устранён техдолг и security-низкие бэклога ревью 2026-10-22 (#29) (#51)

### 🚀 Features

- Обновление pyo3 до 0.28. Проделана миграция.
- Генерация .pyi-стабов из Rust через experimental-inspect. (#35)

### 🐛 Bug Fixes

- Для copy_sheet добавил глубокое клонирование ячеек и комментариев. (#32)
- Для find_cell_by_letter — ValueError вместо паники на невалидной букве. (#36)
- Для find_sheet_by_regex — ValueError вместо паники на невалидном regex. (#37)
- Детерминизм выдачи find_cell_by_* и find_values_by_*. (#38)
- Добавил валидацию домена Excel для delete_rows/delete_cols. (#39)
- Для touch cell(r, c) — сохранение data_type существующей ячейки. (#40)
- Для delete_rows/delete_cols — сдвиг merge_cells, комментариев и размеров строк/колонок. (#41)
- Для readable-стабов — ValueError вместо тихих as-приведений мусорных координат (#42)
- Для readable-слоя — чтение собственных типов библиотеки (style, кортежи merge_cells). (#43)
- Для set_value — строки nan/inf остаются строками, а не числами. (#44)
- Для extract_py_value_auto — лимит глубины вложенности списков, ValueError вместо SIGSEGV. (#45)
- Для листовых find_*/to_json/to_dict — py.detach, полный скан листа не держит GIL. (#25) (#46)
- Для create_finder и Finder — листы разделяются по Arc без глубокого копирования. (#26) (#47)

### 💼 Other

- Обновление зависимостей.
- Обновление зависимости pyo3 0.28.1 -> 0.28.2.
- Обновить зависимости и файлы блокировок. pyo3 0.28.3 -> 0.29.0

### 🚜 Refactor

- [**breaking**] Устранён техдолг и security-низкие бэклога ревью 2026-10-22 (#29) (#51)

### 📚 Documentation

- Инструкции для агентов, обновление зависимостей и правки ruff (#30)
- Переписан README: добавлено описание Rust-ядра, рабочие примеры и состав пакета. (#50)

### ⚡ Performance

- Устранён перф-бэклог ревью 2026-10-22: кэш сортированных ключей, O(1)-поиски, прямой to_dict. (#49)

### 🧪 Testing

- Систематическое pytest-покрытие core_pyo3 и CI-сверка .pyi-стабов с API (#48)

### ⚙️ Miscellaneous Tasks

- *(cliff)* Доработка по обработке коммитов.
- *(ci)* Обновление версий actions.
- Добавить лицензию MIT
- Добавить джобы качества (fmt, msrv, clippy, test, deny, skylos, semgrep). (#31)
- Добавил проверки ruff и ty в гейты и quality-воркфлоу. (#33)

## [0.12.3] - 2026-01-21

### 🐛 Bug Fixes

- Для поиска по параметрам и возвращении ячеек, вернул сортировку.

### 💼 Other

- V0.12.3

## [0.12.2] - 2026-01-16

### 🐛 Bug Fixes

- Правки обращения к значению ячейки при поиске.

### 💼 Other

- V0.12.2

### 🚜 Refactor

- Заменил клонирование на Arc::clone

## [0.12.1] - 2026-01-15

### 🐛 Bug Fixes

- Убрал избыточный Box для CellValue.

### 💼 Other

- V0.12.1

### 🚜 Refactor

- Изменил клонирование на Arc::clone
- Заменил clone на Arc::clone
- Refactor: Оптимизация методов по поиску ячеек. Так же удаление колонок и
строк.

## [0.12.0] - 2026-01-15

### 🚀 Features

- Обновление pyo3 0.26 -> 0.27. Сделана миграция по изменениям.
- Обновление rust c 1.90 -> 1.92.

### 🐛 Bug Fixes

- Исправление вызова метода для find_cell_by_str.
- Убрал escape для регулярных выражений при поиске по ячейкам.
- Правки работы RWLock и regex. Минимизация lock time.
- Правки в pyi для python _conn_db -> conn_db в методе __init__.
- Правки поиск по regex регистронезависимый.

### 💼 Other

- Обновление зависимостей.
- V0.12.0

### 🚜 Refactor

- Добавление проверки метода *_regex в examples/finder.py.
- Убрал клонирование ячеек при сериализации.
- Внес правки по рекомендауиям clippy.
- Правки названия метода.
- Ушел от unwrap при преобразовании объектов python в rust.
- Оптимизация получения максимальных значений по ячейкам.

### ⚙️ Miscellaneous Tasks

- Ci: Обновление workflow для macos. Удаление macos-13, так как github
больше не поддердивает.

## [0.11.1] - 2025-10-07

### 🐛 Bug Fixes

- Убираем поддержку windows x32.

### 💼 Other

- V0.11.1

## [0.11.0] - 2025-10-07

### 🚀 Features

- Feat: Обновление pyo3 c 0.25.* до 0.26.0. Проведена миграция по переходу
на новую версию.
- Feat: Добавление обработки стиля переданного через python объект, при
конвертации.

### 🐛 Bug Fixes

- Rust version

### 💼 Other

- Обновление зависимостей.
- V0.11.0

## [0.10.0] - 2025-07-11

### 🚀 Features

- Переработка проекта с оптимизациями.
- Feat: Изменил сигнатуры функций Python, чтобы они принимали ссылки на
PyList, и добавил вспомогательную функцию для правильного извлечения
объектов WrapperCell из элементов списка Python.
- Feat: Добваление в структуры dimensions новые аттрибуты level и
collapsed.

### 🐛 Bug Fixes

- Worlkflows, правки в указании директорий для сботки.
- Downgrade pyo3-build-config to v0.25.0
- Disable auto-initialize feature in pyo3
- Package name and description in pyproject.toml
- Fix: Добавил проверку формулы перед установкой типа данных по умолчанию
в string
- Data_type по умолчанию General -> s

### 💼 Other

- V0.10.0

## [0.9.4] - 2025-07-04

### 🐛 Bug Fixes

- Разрешать значения ячеек любого типа Python

### 💼 Other

- V0.9.4

## [0.9.3] - 2025-07-04

### 🐛 Bug Fixes

- Разрешил Python dict или class для атрибута cells

### 💼 Other

- V0.9.3

## [0.9.2] - 2025-07-03

### 🐛 Bug Fixes

- Remove skip_serializing_if attribute from MergeCells range

### 💼 Other

- V0.9.2

## [0.9.1] - 2025-06-30

### 🐛 Bug Fixes

- Set default data type for empty cells to "s"

### 💼 Other

- V0.9.1

## [0.9.0] - 2025-06-30

### 🚀 Features

- Добавил возможность при создании листа передавать sheet_state.

### 🐛 Bug Fixes

- Set "General" data type for empty cells

### 💼 Other

- Upgrade package dependencies in Cargo.lock
- V0.9.0

## [0.8.1] - 2025-06-24

### 🐛 Bug Fixes

- Добавьте поддержку состояния листа и элементы управления видимостью

### 💼 Other

- V0.8.1

## [0.8.0] - 2025-06-09

### 🚀 Features

- Add create_finder in readable module

### 🐛 Bug Fixes

- Переименуйте параметры границ диапазона для обеспечения согласованности

### 💼 Other

- V0.8.0

## [0.7.2] - 2025-06-02

### 🐛 Bug Fixes

- Fix service type hints and simplify method returns
- Export Service and Comment types
- Add @final decorators to Book and Service methods
- Add @final decorator to all Finder and Sheet methods

### 💼 Other

- Обновление зависимостей
- Update dependencies and build configuration
- Upgrade PyO3 to v0.25.0
- Update parking_lot and related dependencies
- V0.7.2

### 🚜 Refactor

- Rename uow property to conn_db in Service class

## [0.7.1] - 2025-02-20

### 🐛 Bug Fixes

- Правки поиска по строке. Чистим пробелы. Добавление новых методов в pyi

### 💼 Other

- V0.7.1

## [0.7.0] - 2025-02-20

### 🚀 Features

- Добавление новых методов (find_cell_by_str, find_cells_by_str) поиска ячеек по строке.

### 💼 Other

- V0.7.0

### 🚜 Refactor

- Импорты

## [0.6.2] - 2025-02-18

### 🐛 Bug Fixes

- Правки обработки значения в ячейки из Python в Rust, когда значение число, но имеет тип 's'

### 💼 Other

- V0.6.2

## [0.6.1] - 2025-02-18

### 🐛 Bug Fixes

- Правки конвертации данных из python в rust. Cell.value может приходить null

### 💼 Other

- Обновление зависимостей
- V0.6.1

## [0.6.0] - 2025-01-30

### 🚀 Features

- Добавление констант в py слой ширины колонки и высоты строки
- Для всех методов и функций поиска освободил от GIL привязки
- Добавление новых значений для высоты и ширины ячеек по умолчанию
- Добавление нового метода (find_value_coords) для структур

### 💼 Other

- Обновление зависимостей
- V0.6.0

### 🚜 Refactor

- В to_dict разделил получение модуля, метода и вызов метода, обновление зависимостей

### 📚 Documentation

- Правки в CHANCELOG.md

## [0.5.1] - 2025-01-13

### 🚀 Features

- Добавление нового метода для сервиса. to_dict
- Добавление метода to_dict для Book
- Добавление новых методов. Поиск значений строк в колонке и наоборот
- Добавление нового метода для сервиса и книги по копированию листа
- Подготовка для реализации функционала ширины/высоты и скрытия колонок и строк
- Добавление настроек ширины и высоты ячеек для листа
- Добавление комментариев к ячейкам

### 🐛 Bug Fixes

- Правим ключи в Style для JSON и в самой схеме
- Правки merge_cells в readable.ReadableSheet. Разрешил при конвертировании из py в rust передавать merge_cells -> None
- Правки метода в sheet.cell для получения и обработки передаваемого значения для ячейки
- Внес изменения для сериализации в sheet для атрибута merge_cells
- Внес правки в получение данных из python для merge_cells
- Правки в python example тип list для merge_cells
- Убрал из py слоя row_row_dimensions и column_dimensions
- Добавдение новых tagret для компиляции версии под Linux, Mac, Windows и Musllinux

### 💼 Other

- Обновление зависимостей
- V0.5.0
- Поднял resolver
- Resolver вернул обратно
- Настройки

### 🚜 Refactor

- Удаление всех не нужных файлов и папок
- Добавление #[inline] к методам
- Исправления по clippy
- Внес немного правки в тест
- Изменение CHANGELOG.md

### 📚 Documentation

- Обновление README.md
- Обновление badge CI
- Правки в README.md

### ⚙️ Miscellaneous Tasks

- Внес правки в CI.yml
- Правки CI.yml, для правильного получения cargo metadata
- Правки в CI.yml
- Правки CI.yml
- Внес изменения в Ci.yml
- Жксперимент с CI.yml
- Эксперимент Ci.yml
- Внесение правок в CI.yml
- Улучшение Ci.yml
- Правки
- Правки в CI.yml для правильной сборки
- Добавление Linux aarch64
- Добавление Linux armv7 x86
- Добавление musllinux
- Добавление Linux ppc64le s390x && windows x86 x64

## [0.4.2] - 2024-12-19

### 🚀 Features

- Сделал сортировку с помощью rayon
- Переезд c std:sync:Mutex -> parking_lot::Mutex
- Обернул Cell::value в Box
- Все методы is_... стали для python @property
- Добавление нового метода, поиск значения по координатам

### 🐛 Bug Fixes

- Удалил лишние файлы
- Удалил лишнее копирование листов. За счет этого увеличилась скорость
- Вернул описание методов для python
- Оптимизация метода по генерации недостоющих ячеек
- Вернул потерявшие методы для Helper и для Write XLSXSheet find_value_by_coords
- Правки в описании метода и добавления потерявшего метода
- Добавление утененного метода для py связвнного с hidden_value
- Правки импорта в py

### 💼 Other

- Инициализация проекта
- Добавил crate serde_with
- V0.2.0
- V0.3.0
- V0.3.1
- Обновление
-  Переработка структур в проекте
- Добавление методов
- Добавление методов для мерджей
- Добавил тесты
- Правки метода between
- Сделал в слое rust данные по Arc RwLock
- Работа над листом и методами
- Добавил sheet_state
- Добавил методы для листа
- Добавил методы для Cell
- Добавил правильную серилизацию книги
- Описание pyi и правки для json
- Refactoring
- Добавил max_row, max_column
- Добавление тестов для Cell
- Добавление метода letter для Cell
- Добавление Service
- Добавил  __version__
- Добавление traits
- Разделение для Finder только листы и ячейки readable
- Исправление Finder
- Добавление нового модуля в py
- Преобразование типов из Python в Rust
- Сделал extract Python -> Rust
- Добавление funcs
- Небольшие изменения
- Правки в funcs
- Перенес логики find_* в одно место
- Добавлены новые методы
- Добавление функций в readable модуль
- Правки в README.md
- V0.3.2
- Merge branch 'main' into New
- Merge pull request #1 from stalexsm/New

Переработка внутренностей.
- Bump 0.3.2 -> 0.4.0
- V0.4.1
- V0.4.2

### 🚜 Refactor

- Проверка диапазона ячеек
- Вынес логику обработки python типов в utils

### 📚 Documentation

- Внес правки в описание README.md
- *(README.md)* Внес правки
- *(README.md)* Внес правки, добавил badges
- *(README.md)* Внес правки в path badge
- *(README.md)* Правки в бейджи и настройка
- *(README.md)* Правки
- *(README.md)* Правки в badge для github
- Правки снова для badges
- Правки для badges
- Правки README.md
- README.md

