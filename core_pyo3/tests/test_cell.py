"""Класс Cell: геттеры, сеттеры и флаги типа значения.

Систематическое покрытие публичного API ячейки листа: координаты,
значения всех поддерживаемых типов, формулы, стили, скрытое значение.
"""

import datetime

import pytest
from report_core import Book


@pytest.fixture
def sheet():
    """Лист с одной ячейкой A1 («текст»)."""
    book = Book()
    return book.add_sheet("лист")


def test_coordinates(sheet) -> None:
    """Координаты ячейки читаются как строка, колонка и буква."""
    cell = sheet.cell(3, 2, "x")

    assert cell.row == 3
    assert cell.column == 2
    assert cell.letter == "B3"


def test_letter_columns_beyond_z(sheet) -> None:
    """Буквенная координата считается с учётом двухбуквенных колонок."""
    assert sheet.cell(1, 26, "x").letter == "Z1"
    assert sheet.cell(1, 27, "x").letter == "AA1"


def test_text_value_and_data_type(sheet) -> None:
    """Текстовая ячейка: значение и тип «s»."""
    cell = sheet.cell(1, 1, "текст")

    assert cell.value == "текст"
    assert cell.data_type == "s"


def test_repr(sheet) -> None:
    """repr называет тип и координаты."""
    assert (
        repr(sheet.cell(2, 5, "x")) == "Cell, Coords: Coordinate { row: 2, column: 5 }"
    )


def test_optional_fields_default_to_none(sheet) -> None:
    """Формула, стиль и скрытое значение пусты у новой ячейки."""
    cell = sheet.cell(1, 1, "x")

    assert cell.formula is None
    assert cell.style is None
    assert cell.hidden_value is None
    assert not cell.is_formula


def test_set_text_fields(sheet) -> None:
    """Сеттеры значения, формулы, стиля и скрытого значения."""
    cell = sheet.cell(1, 1, "x")
    cell.value = "новое"
    cell.formula = "=SUM(A1)"
    cell.style = "st1"
    cell.hidden_value = "скрытое"

    assert cell.value == "новое"
    assert cell.formula == "=SUM(A1)"
    assert cell.is_formula
    assert cell.style == "st1"
    assert cell.hidden_value == "скрытое"


def test_set_value_number(sheet) -> None:
    """Число: тип «n», флаг numeric без integer."""
    cell = sheet.cell(1, 1)
    cell.set_value_number(3.5)

    assert cell.value == "3.5"
    assert cell.data_type == "n"
    assert cell.is_value_numeric
    assert not cell.is_value_integer


def test_set_value_integer(sheet) -> None:
    """Целое число: тип «n», флаг integer, но не numeric."""
    cell = sheet.cell(1, 1)
    cell.set_value_integer(7)

    assert cell.value == "7"
    assert cell.data_type == "n"
    assert cell.is_value_integer
    assert not cell.is_value_numeric


def test_set_value_bool(sheet) -> None:
    """Логическое значение: тип «b» и флаг bool."""
    cell = sheet.cell(1, 1)
    cell.set_value_bool(True)

    assert cell.value == "true"
    assert cell.data_type == "b"
    assert cell.is_value_bool


def test_set_value_datetime(sheet) -> None:
    """Дата-время: тип «d» и флаг datetime."""
    cell = sheet.cell(1, 1)
    cell.set_value_datetime(datetime.datetime(2024, 1, 2, 3, 4, 5))

    assert cell.value == "2024-01-02 03:04:05"
    assert cell.data_type == "d"
    assert cell.is_value_datetime


def test_value_flags_for_text(sheet) -> None:
    """Текстовое значение не отвечает ни одному типному флагу."""
    cell = sheet.cell(1, 1, "текст")

    assert not cell.is_value_numeric
    assert not cell.is_value_integer
    assert not cell.is_value_bool
    assert not cell.is_value_datetime
    assert not cell.is_value_empty
