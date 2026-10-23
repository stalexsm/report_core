"""find_cell_by_letter: невалидная буква ячейки — ValueError, а не паника.

Регрессия к issue #5: незаякоренный regex + unwrap в Coordinate::from(&str)
падали pyo3.PanicException (не ловится `except Exception`) на входах вида
"1A", "", "HELLO", "A99999999999999999".
"""

import uuid

import pytest
import report_core
from report_core import readable


class DuckCell:
    """Минимальный duck-typed ReadableCell для создания ReadableSheet."""

    id = uuid.uuid4()
    row = 1
    column = 1
    value = "x"
    data_type = "s"
    formula = None
    style_id = ""


class DuckSheet:
    """Минимальный duck-typed лист для ReadableSheet."""

    name = "t"
    sheet_state = "visible"
    merge_cells = None

    def __init__(self) -> None:
        self.cells = [DuckCell()]


INVALID_LETTERS = ["1A", "", "HELLO", "A99999999999999999"]


def test_sheet_find_cell_by_letter_invalid() -> None:
    """Sheet.find_cell_by_letter: невалидная буква — ValueError."""
    sheet = report_core.Book().add_sheet("t")

    for letter in INVALID_LETTERS:
        with pytest.raises(ValueError, match=f"invalid cell letter: {letter}"):
            sheet.find_cell_by_letter(letter)


def readable_sheet():
    """ReadableSheet через create_finder из duck-typed листа."""
    finder = readable.create_finder([DuckSheet()])

    return finder.find_sheet_by_name("t")


def test_readable_sheet_find_cell_by_letter_invalid() -> None:
    """ReadableSheet.find_cell_by_letter: невалидная буква — ValueError."""
    sheet = readable_sheet()

    for letter in INVALID_LETTERS:
        with pytest.raises(ValueError, match=f"invalid cell letter: {letter}"):
            sheet.find_cell_by_letter(letter)


def test_readable_find_cell_by_letter_invalid() -> None:
    """readable.find_cell_by_letter: невалидная буква — ValueError."""
    cells = readable_sheet().cells

    for letter in INVALID_LETTERS:
        with pytest.raises(ValueError, match=f"invalid cell letter: {letter}"):
            readable.find_cell_by_letter(letter, cells)


def test_valid_letter_still_works() -> None:
    """Валидные буквы не сломаны: отсутствующая ячейка — None."""
    sheet = report_core.Book().add_sheet("t")

    assert sheet.find_cell_by_letter("A1") is None
    assert sheet.find_cell_by_letter("$B$2") is None
