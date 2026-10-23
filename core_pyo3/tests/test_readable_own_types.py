"""Readable-слой читает собственные типы библиотеки.

Регрессия к issue #22: `create_finder([book.sheets[0]])` падал на
собственных Sheet/Cell библиотеки — у Cell нет атрибута `style_id`
(только `style`), а кортежи `(1, 2, 1, 2)` из `Sheet.merge_cells`
не распознавались как merge-диапазоны.
"""

import pytest
from report_core import Book, readable


def sheet_with_cells() -> object:
    """Собственный лист библиотеки: две ячейки и объединение A1:B2."""
    book = Book()
    sheet = book.add_sheet("data", "visible")
    sheet.cell(1, 1, "a")
    sheet.cell(2, 2, "b")
    sheet.add_merge_cells(1, 2, 1, 2)

    return sheet


def test_own_sheet_merge_tuples_roundtrip() -> None:
    """Merge-кортежи собственного Sheet читаются как диапазоны."""
    finder = readable.create_finder([sheet_with_cells()])

    sheet = finder.find_sheet_by_name("data")

    assert sheet is not None
    assert sheet.merge_cells == [(1, 2, 1, 2)]


def test_own_cell_style_fallback() -> None:
    """Стиль собственной Cell читается из `style`, а не только `style_id`."""
    book = Book()
    sheet = book.add_sheet("data", "visible")
    styled = sheet.cell(1, 1, "a")
    styled.style = "mystyle"
    sheet.cell(2, 2, "b")

    finder = readable.create_finder([sheet])

    readable_sheet = finder.find_sheet_by_name("data")
    assert readable_sheet is not None

    readable_styled = readable_sheet.find_cell_by_coords(1, 1)
    assert readable_styled is not None
    assert readable_styled.style == "mystyle"
    assert readable_styled.value == "a"

    readable_plain = readable_sheet.find_cell_by_coords(2, 2)
    assert readable_plain is not None
    assert readable_plain.style is None


def test_duck_cell_style_id_still_works() -> None:
    """Duck-typed ячейка со `style_id` читается как раньше."""

    class Cell:
        row = 1
        column = 1
        value = "x"
        data_type = "s"
        formula = None
        style_id = "duck-style"

    class Sheet:
        name = "duck"
        sheet_state = "visible"

        def __init__(self) -> None:
            self.cells = [Cell()]
            self.merge_cells = None

    finder = readable.create_finder([Sheet()])

    sheet = finder.find_sheet_by_name("duck")
    assert sheet is not None

    cell = sheet.find_cell_by_coords(1, 1)
    assert cell is not None
    assert cell.style == "duck-style"


@pytest.mark.parametrize("bad_range", [(1, 2), (1, 2, 3), (1, 2, 3, 4, 5), ()])
def test_merge_tuple_wrong_length_is_value_error(bad_range) -> None:
    """Кортеж не длины 4 — ValueError, а не тихий диапазон (0, 0, 0, 0)."""

    class Sheet:
        name = "t"
        sheet_state = "visible"

        def __init__(self) -> None:
            self.cells = []
            self.merge_cells = [bad_range]

    with pytest.raises(ValueError, match="merge_cells"):
        readable.create_finder([Sheet()])
