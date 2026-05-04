## [unreleased]

### 🚀 Features

- Обновление pyo3 до 0.28. Проделана миграция.

### 💼 Other

- Обновление зависимостей.
- Обновление зависимости pyo3 0.28.1 -> 0.28.2.

## [0.12.3] - 2026-01-21

### 🐛 Bug Fixes

- Для поиска по параметрам и возвращении ячеек, вернул сортировку.

## [0.12.2] - 2026-01-16

### 🐛 Bug Fixes

- Правки обращения к значению ячейки при поиске.

### 🚜 Refactor

- Заменил клонирование на Arc::clone

## [0.12.1] - 2026-01-15

### 🐛 Bug Fixes

- Убрал избыточный Box для CellValue.

### 🚜 Refactor

- Изменил клонирование на Arc::clone
- Заменил clone на Arc::clone

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

### 🚜 Refactor

- Добавление проверки метода *_regex в examples/finder.py.
- Убрал клонирование ячеек при сериализации.
- Внес правки по рекомендауиям clippy.
- Правки названия метода.
- Ушел от unwrap при преобразовании объектов python в rust.
- Оптимизация получения максимальных значений по ячейкам.

## [0.11.1] - 2025-10-07

### 🐛 Bug Fixes

- Убираем поддержку windows x32.

## [0.11.0] - 2025-10-07

### 🐛 Bug Fixes

- Rust version

### 💼 Other

- Обновление зависимостей.

## [0.10.0] - 2025-07-11

### 🚀 Features

- Переработка проекта с оптимизациями.

### 🐛 Bug Fixes

- Worlkflows, правки в указании директорий для сботки.
- Downgrade pyo3-build-config to v0.25.0
- Disable auto-initialize feature in pyo3
- Package name and description in pyproject.toml
- Data_type по умолчанию General -> s

## [0.9.4] - 2025-07-04

### 🐛 Bug Fixes

- Разрешать значения ячеек любого типа Python

## [0.9.3] - 2025-07-04

### 🐛 Bug Fixes

- Разрешил Python dict или class для атрибута cells

## [0.9.2] - 2025-07-03

### 🐛 Bug Fixes

- Remove skip_serializing_if attribute from MergeCells range

## [0.9.1] - 2025-06-30

### 🐛 Bug Fixes

- Set default data type for empty cells to "s"

## [0.9.0] - 2025-06-30

### 🚀 Features

- Добавил возможность при создании листа передавать sheet_state.

### 🐛 Bug Fixes

- Set "General" data type for empty cells

### 💼 Other

- Upgrade package dependencies in Cargo.lock

## [0.8.1] - 2025-06-24

### 🐛 Bug Fixes

- Добавьте поддержку состояния листа и элементы управления видимостью

## [0.8.0] - 2025-06-09

### 🚀 Features

- Add create_finder in readable module

### 🐛 Bug Fixes

- Переименуйте параметры границ диапазона для обеспечения согласованности

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

### 🚜 Refactor

- Rename uow property to conn_db in Service class

## [0.7.1] - 2025-02-20

### 🐛 Bug Fixes

- Правки поиска по строке. Чистим пробелы. Добавление новых методов в pyi

## [0.7.0] - 2025-02-20

### 🚀 Features

- Добавление новых методов (find_cell_by_str, find_cells_by_str) поиска ячеек по строке.

### 🚜 Refactor

- Импорты

## [0.6.2] - 2025-02-18

### 🐛 Bug Fixes

- Правки обработки значения в ячейки из Python в Rust, когда значение число, но имеет тип 's'

## [0.6.1] - 2025-02-18

### 🐛 Bug Fixes

- Правки конвертации данных из python в rust. Cell.value может приходить null

### 💼 Other

- Обновление зависимостей

## [0.6.0] - 2025-01-30

### 🚀 Features

- Добавление констант в py слой ширины колонки и высоты строки
- Для всех методов и функций поиска освободил от GIL привязки
- Добавление новых значений для высоты и ширины ячеек по умолчанию
- Добавление нового метода (find_value_coords) для структур

### 💼 Other

- Обновление зависимостей

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

- Добавил crate serde_with
- Обновление

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

