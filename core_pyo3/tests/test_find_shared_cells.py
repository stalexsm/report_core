"""Модульные find_* отдают собственные ячейки тем же объектом.

Регрессия к перф-бэклогу #28 (П-3): до фикса каждый вызов конвертировал
весь список заново по 6×getattr на ячейку, а найденная ячейка была
копией — другим объектом с теми же полями. После фикса нативные
Cell/ReadableCell распознаются downcast'ом и отдаются тем же Arc;
чужие duck-typed объекты по-прежнему извлекаются по атрибутам.
"""

import pytest
from report_core import Book, readable


@pytest.fixture
def native_cells():
    """Нативные ячейки readable-листа: прямоугольник 2x2."""
    book = Book()
    sheet = book.add_sheet("лист")
    for row in range(1, 3):
        for col in range(1, 3):
            sheet.cell(row, col, f"v{row}{col}")

    finder = readable.create_finder([sheet])
    found = finder.find_sheet_by_name("лист")
    assert found is not None

    return found.cells


def test_find_returns_same_object(native_cells) -> None:
    """Найденная нативная ячейка делит состояние с листом, а не копия."""
    book = Book()
    sheet = book.add_sheet("лист")
    sheet.cell(1, 1, "старое")

    finder = readable.create_finder([sheet])
    found_sheet = finder.find_sheet_by_name("лист")
    assert found_sheet is not None

    cells = found_sheet.cells

    # После снятия списка ячейка на листе меняется: список делит с листом
    # одни и те же данные (Arc), поэтому поиск по списку видит новое значение.
    # До фикса список хранил снимки-копии и поиск отдавал «старое».
    sheet.cell(1, 1, "новое")

    found = readable.find_cell_by_str("новое", cells)

    assert found is not None
    assert (found.row, found.column) == (1, 1)
    assert readable.find_cell_by_str("старое", cells) is None


def test_find_cells_by_str_returns_same_objects(native_cells) -> None:
    """Списковый поиск находит нативные ячейки списка."""
    found = readable.find_cells_by_str("v21", native_cells)

    assert [(cell.row, cell.column) for cell in found] == [(2, 1)]


def test_duck_typed_cells_still_extracted() -> None:
    """Чужие объекты (dict) извлекаются по атрибутам, как и раньше."""
    cells = [
        {
            "row": 1,
            "column": 1,
            "value": "a",
            "formula": None,
            "data_type": "s",
            "style_id": None,
        },
        {
            "row": 2,
            "column": 1,
            "value": "b",
            "formula": None,
            "data_type": "s",
            "style_id": None,
        },
    ]

    found = readable.find_cell_by_str("b", cells)

    assert found is not None
    assert (found.row, found.column) == (2, 1)
    assert found.value == "b"


def test_book_cells_downcast_share_state() -> None:
    """Ячейки книги (report_core.Cell) распознаются downcast'ом.

    Список снимается с листа, затем ячейка на листе меняется: поиск по
    списку видит новое значение — ячейки делят состояние с листом, а не
    являются снимками-копиями.
    """
    book = Book()
    sheet = book.add_sheet("лист")
    sheet.cell(1, 1, "старое")
    sheet.cell(2, 1, "b")

    cells = sheet.cells

    sheet.cell(1, 1, "новое")

    found = readable.find_cell_by_str("новое", cells)

    assert found is not None
    assert (found.row, found.column) == (1, 1)
    assert readable.find_cell_by_str("старое", cells) is None
    assert readable.find_cells_by_str("b", cells)
