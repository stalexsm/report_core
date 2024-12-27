# __report-core__
[![CI](https://github.com/stalexsm/report_core/actions/workflows/CI.yml/badge.svg?event=push)](https://github.com/stalexsm/report_core/actions/workflows/CI.yml)
[![pypi](https://img.shields.io/pypi/v/report_core.svg)](https://pypi.python.org/pypi/report-core)
[![Supported Python versions](https://img.shields.io/pypi/pyversions/report-core.svg)](https://pypi.python.org/pypi/report-core/)

`report-core` - это вспомогательная Python-библиотека для построения отчетов в формате Excel (XLSX) для проекта, предоставляющая удобные инструменты для манипуляции данными..

## Установка

```poetry add report-core``` OR
```pip install report-core```

## Основные компоненты

- `Service`: Базовый класс для создания сервисов обработки данных и создания отчетов.
- `Book`: Представляет книгу отчета Excel.
- `Sheet`: Представляет лист Excel для создания новых листов отчета.
- `Cell`: Представляет ячейку в листе Excel для создания новых ячеек отчета.
- `Comment`: Представляет комментарий в листе Excel для ячеек.
- `readable`: Модуль только для чтения листов и ячеек Excel.
  - `Finder`: Помощник для работы с несколькими листами и ячейками.
  - `ReadableSheet`: Представляет лист Excel для чтения существующих листов отчета.
  - `ReadableCell`: Представляет ячейку в листе Excel для чтения существующих ячеек отчета.
  - `find_cell_by_coords`: Поиск ячейки в листе Excel по координатам.
  - `find_value_by_coords`: Поиск значения в листе Excel по координатам.
  - `find_cell_by_regex`: Поиск ячейки в листе Excel по регулярному выражению
  - `find_cell_by_letter`: Поиск ячейки в листе Excel по букве
  - `find_cells_by_regex`: Поиск ячеек в листе Excel по регулярному выражению
  - `find_cells_for_rows_by_regex`: Поиск ячеек в листе Excel по регулярному выражению до определеной колонки
  - `find_cells_for_cols_by_regex`: Поиск ячеек в листе Excel по регулярному выражению до определеной строки
  - `find_cells_multi_regex`: Поиск ячеек в листе Excel по нескольким регулярным выражениям
  - `find_cells_between_regex`: Поиск ячеек в листе Excel между двумя регулярными выражениями
  - `find_cells_range_rows`: Поиск ячеек в листе Excel по диапазону строк
  - `find_cells_range_cols`: Поиск ячеек в листе Excel по диапазону столбцов
  - `find_values_by_col_rows`: Поиск значений ячеек в листе Excel по диапазону строк и столбца
  - `find_values_by_row_cols`: Поиск значений ячеек в листе Excel по диапазону столбцов и строки
- `column_number_to_letter`: Функция для преобразования колонки с row в букву (1 -> A).
- `get_letter_coordinate`: Функция для получения координаты ячейки в стиле A1.

## Возможности

- Поиск листов по имени или шаблону
- Поиск ячеек по значению, регулярному выражению или адресу
- Манипуляция данными ячеек (установка значений, формул, форматов)
- Получение информации о ячейках (тип данных, числовой формат, стиль)
- Работа с датами и временем
- Создание пользовательских сервисов и форматтеров для создания отчетов xlsx.

## Пример использования

```python
from report_core import Service
import time


class S10406(Service):
    def summary_0(self, sheets, **kwargs):
        sheet = self._add_sheet("Sheet1")

        print(sheet)

        for r in range(1, 6):
            for c in range(1, 6):
                cell = sheet.cell(r, c, f"Dynamic value: r{r}:c{c}")

                if r == 5 and c == 5:
                    cell.value = "100"
                    cell.formula = "SUM(A1:A5)"
                    cell.style = "Style A"

        print(sheet)

        cell = sheet.find_cell_by_letter("E5")
        if cell:
            print(cell.value)
            print(cell.formula)
            print(cell.style)
            print(cell.letter)

        fmt = self._fmt_0(**kwargs)
        print(fmt)

        return "Summary"

    def _fmt_0(self, **kwargs):
        for s in self._sheets:
            print("Fmt --> ", s)

        return "Fmt"


def main():
    s = S10406("uow")
    s.summary_0([])

    print(s)
    result = s.to_json()

    print(result)

if __name__ == "__main__":
    main()
```
