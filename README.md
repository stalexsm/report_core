# report-core

[![CI](https://github.com/stalexsm/report_core/actions/workflows/CI.yml/badge.svg?event=push)](https://github.com/stalexsm/report_core/actions/workflows/CI.yml)
[![PyPI](https://img.shields.io/pypi/v/report_core.svg)](https://pypi.python.org/pypi/report-core)
[![Supported Python versions](https://img.shields.io/pypi/pyversions/report-core.svg)](https://pypi.python.org/pypi/report-core/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/stalexsm/report_core/blob/main/LICENSE)

**report-core** — Python-библиотека с ядром на [Rust](crates/core_rs) ([PyO3](core_pyo3)) для построения XLSX-отчётов.

Отчёт описывается как модель в памяти: **книга → листы → ячейки, комментарии, объединения, стили**. Модель быстро наполняется данными, по ней выполняется поиск, а результат сериализуется в `dict`/`JSON` — итоговый `.xlsx`-файл рендерит приложение любым движком (xlsxwriter, openpyxl и т.д.). Такой подход удобен, когда отчёт собирается по частям в разных задачах, хранится в БД или передаётся между сервисами.

Библиотека создана как вспомогательное ядро для проекта **gpns-boriy**, где используется для генерации и разбора табличных отчётов.

## Возможности

- **Ядро на Rust** — быстрое наполнение, поиск и сериализация; тяжёлые операции (полный скан листа, `to_dict`/`to_json`) не удерживают GIL.
- **Модель отчёта**: `Book`, `Sheet`, `Cell`, `Comment`; объединения ячеек, размеры и скрытость строк/колонок, именованные стили.
- **Типизированные значения ячеек**: строки, числа, `bool`, `datetime`, формулы, скрытые значения; проверка типов (`is_value_numeric`, `is_value_datetime`, ...).
- **Поиск**: по строке, регулярному выражению, координатам и букве (A1), по диапазонам строк/колонок, между двумя шаблонами.
- **`Service`** — базовый класс для сервисов, строящих отчёты из данных БД: наследник реализует `summary_0` (генерация) и `_fmt_0` (форматирование).
- **[`readable`](#чтение-готового-отчёта)** — read-only сабмодуль для поиска по готовым листам без риска их изменить.
- **Сериализация**: `to_dict()` и `to_json()` на основе serde.
- **Полная типизация**: `py.typed` + `.pyi`-стабы, генерируемые из Rust-кода.

## Установка

```bash
pip install report-core
```

Python 3.12+ (CPython, PyPy). Готовые колёса: manylinux и musllinux (x86_64, x86, aarch64, armv7, s390x, ppc64le), macOS (Intel и Apple Silicon), Windows x64.

## Быстрый старт

```python
from report_core import Book

book = Book()
sheet = book.add_sheet("Пример")

sheet.cell(1, 1, "Привет, мир!")
cell = sheet.cell(2, 1)
cell.set_value_integer(42)
cell.style = "accent"          # имя стиля; словарь стилей — на стороне рендерера

data = book.to_dict()          # dict: {"sheets": [{"name": ..., "cells": [...], ...}]}
# payload = book.to_json()     # тот же отчёт одной JSON-строкой
```

## Пример: сервис отчёта

Сервисы наследуются от `Service`, который хранит книгу и подключение к БД (`conn_db`):

```python
from report_core import Service
from report_core.readable import Finder


class DrillingReport(Service):
    """Сервис строит лист отчёта «Смета» из данных БД."""

    def summary_0(self, sheets, **kwargs):
        sheet = self._add_sheet("Смета")

        # Шапка: cell() — get-or-create по (строка, колонка)
        sheet.cell(1, 1, "Наименование работ")
        sheet.cell(1, 2, "Ед. изм.")
        sheet.cell(1, 3, "Количество")
        sheet.cell(1, 4, "Сумма")

        # Данные: типизированные значения, стиль
        for i, (work, unit, qty, price) in enumerate(kwargs["rows"], start=2):
            sheet.cell(i, 1, work)
            sheet.cell(i, 2, unit)
            sheet.cell(i, 3).set_value_integer(qty)
            total = sheet.cell(i, 4)
            total.set_value_number(qty * price)
            total.style = "money"

        # Итоговая строка: формула и комментарий
        last = sheet.max_row + 1
        grand = sheet.cell(last, 4)
        grand.formula = f"SUM(D2:D{last - 1})"
        sheet.add_comment(last, 4, "Автосумма по смете", "report-core")

        return sheet.name


rows = [("Бурение", "м", 120, 1500.0), ("Цементирование", "м", 80, 900.0)]
report = DrillingReport(conn_db=None)  # здесь подключение/сессия БД, если она нужна сервису
report.summary_0([], rows=rows)

data = report.to_dict()  # dict — для сохранения в БД, очереди или HTTP
```

## Чтение готового отчёта

Сабмодуль `report_core.readable` даёт read-only доступ к листам: принимает собственные листы книги, их `dict`-представление или уже собранные `ReadableSheet`. Полезен, когда один отчёт читают несколько обработчиков — найти шапку, колонки, диапазоны данных:

```python
from report_core.readable import Finder

finder = Finder(report._sheets)          # собственные листы сервиса
sheet = finder.find_sheet_by_name("Смета")

cement = sheet.find_cell_by_regex(r"^Цементирование")
print(cement.letter, cement.value)               # A3 Цементирование

grand = sheet.find_cell_by_coords(4, 4)
print(grand.formula)                             # SUM(D2:D3)

# Вся строка данных:
print([(c.letter, c.value) for c in sheet.find_cells_range_rows(2, 2)])
# [('A2', 'Бурение'), ('B2', 'м'), ('C2', '120'), ('D2', '180000')]
```

Помимо методов `ReadableSheet`, в модуле есть одноимённые функции поиска
(`find_cell_by_regex`, `find_cells_between_regex`, `find_values_by_col_rows`, ...) и
фабрика `create_finder` — псевдоним `Finder`.

## Состав пакета

| Компонент | Назначение |
|---|---|
| `Book` | Книга: контейнер листов отчёта; `add_sheet`, `copy_sheet`, `to_dict`, `to_json` |
| `Service` | Базовый класс сервиса отчёта: `summary_0`, `_fmt_0`, `_add_sheet`, доступ к `conn_db` |
| `Sheet` | Лист: ячейки, комментарии, объединения, размеры, `delete_rows`/`delete_cols`, поиск |
| `Cell` | Ячейка: `value`, `formula`, `style`, `letter`, типизированные `set_value_*` |
| `Comment` | Комментарий к ячейке: текст, автор, координата |
| `readable.Finder` | Поиск листов по имени/regex/индексу в списке листов |
| `readable.ReadableSheet` / `ReadableCell` | Read-only доступ к листу и ячейкам |
| `column_number_to_letter`, `get_letter_coordinate` | Преобразование координат (1 → `A`, `(2, 4)` → `D2`) |
| `DEFAULT_COLUMN_WIDTH`, `DEFAULT_ROW_HEIGHT` | Стандартные размеры Excel (~140 px и ~20 px) |
| `version()`, `__version__` | Версия ядра |

Границы модели соответствуют Excel: 1 048 576 строк × 16 384 колонки, выход за границы и нулевые координаты проверяются с понятными исключениями.

## Разработка

```bash
uv sync                          # venv + editable-сборка report_core через maturin
cargo test --workspace           # Rust-тесты
pytest core_pyo3/tests           # смок-тесты Python-пакета
./scripts/generate-stubs.sh      # регенерация .pyi-стабов после изменений публичного API
```

Стабы генерируются из Rust-кода (`pyo3 experimental-inspect` + `pyo3-introspection`) — вручную их не редактировать. Публичная документация методов живёт в Rust `///`-докстрингах: именно оттуда она попадает в IDE.

## Лицензия

[MIT](LICENSE)
