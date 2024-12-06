## __report_core__
[![CI](https://github.com/stalexsm/report_core/actions/workflows/CI.yml/badge.svg)](https://github.com/stalexsm/report_core/actions/workflows/CI.yml)

---

`report_core` - это вспомогательная Python-библиотека для построения отчетов в формате Excel (XLSX) для проекта, предоставляющая удобные инструменты для манипуляции данными..

## Установка

```
poetry add report_core
```

## Основные компоненты

- `XLSXBook`: Представляет книгу отчета Excel.
- `XLSXSheet`: Представляет лист Excel для создания новых листов отчета.
- `XLSXSheetCell`: Представляет ячейку в листе Excel для создания новых ячеек отчета.
- `XLSXSheetRead`: Представляет лист Excel для чтения существующих листов отчета.
- `XLSXSheetCellRead`: Представляет ячейку в листе Excel для чтения существующих ячеек отчета.
- `Service`: Базовый класс для создания сервисов обработки данных и создания отчетов.
- `HelperSheet`: Помощник для работы с несколькими листами и ячейками.
- `HelperCell`: Помощник для работы с ячейками.
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
from report_core import  Service, HelperSheet, XLSXBook

class MyService(Service):
    def summary_0(self, sheets, /, **kwargs) -> XLSXBook:
        """Данный метод предназначен для формирования отчета"""

        book = XLSXBook()
        new_sheet = book.add_sheet("Отчет")

        h = HelperSheet(sheets)
        sheet = h.find_sheet_by_pattern("Страница с данными")

        if sheet:
            cell = sheet.find_cell_pattern_regex("Итого:")
            if cell:
                total = float(cell.value)
                print(f"Итоговая сумма: {total}")

        # Вызовем метод форматирования
        book = self.fmt_0(book, year=2024)

        return book


    def fmt_0(self, sheets, /, **kwargs):
        """Данный метод предназначен для форматирования отчета"""
        return sheets

# Использование
service = MyService(uow="my_unit_of_work")
sheets = [...]  # Ваши данные листов

processed_sheets = service.summary_0(sheets)
```
