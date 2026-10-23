"""Детерминизм выдачи find_cell_by_* и find_values_by_*.

Регрессия к issue #7: одиночные find_cell_by_regex/str/letter и
find_values_by_col_rows/row_cols обходили несортированный ahash::HashMap —
выбор ячейки и порядок значений менялись между запусками процесса, а
позиционный zip значений find_values_by_* с заголовками молча искажал отчёт.
"""

import uuid

import pytest
import report_core
from report_core import readable


@pytest.fixture
def sheet():
    """Лист с тремя одинаковыми значениями в разных координатах."""
    book = report_core.Book()
    sheet = book.add_sheet("t")

    sheet.cell(2, 3, "dup")
    sheet.cell(1, 2, "dup")
    sheet.cell(3, 1, "dup")

    return sheet


def test_find_cell_by_regex_deterministic(sheet):
    """find_cell_by_regex: повторные вызовы дают одну ячейку — с меньшими координатами."""
    results = [(cell.row, cell.column) for _ in range(50) if (cell := sheet.find_cell_by_regex("dup"))]

    assert results == [(1, 2)] * len(results)
    assert len(results) == 50


def test_find_cell_by_str_deterministic(sheet):
    """find_cell_by_str: повторные вызовы дают одну ячейку — с меньшими координатами."""
    results = [(cell.row, cell.column) for _ in range(50) if (cell := sheet.find_cell_by_str("dup"))]

    assert results == [(1, 2)] * len(results)
    assert len(results) == 50


def test_find_cell_by_letter_deterministic(sheet):
    """find_cell_by_letter: повторные вызовы дают идентичный результат."""
    results = [(cell.row, cell.column) for _ in range(50) if (cell := sheet.find_cell_by_letter("B1"))]

    assert results == [(1, 2)] * len(results)
    assert len(results) == 50


class DuckCell:
    """Минимальный duck-typed ReadableCell для создания ReadableSheet."""

    data_type = "s"
    formula = None
    style_id = ""

    def __init__(self, row, column, value) -> None:
        self.id = uuid.uuid4()
        self.row = row
        self.column = column
        self.value = value


class DuckSheet:
    """Минимальный duck-typed лист для ReadableSheet."""

    name = "t"
    sheet_state = "visible"
    merge_cells = None

    def __init__(self, cells) -> None:
        self.cells = cells


@pytest.fixture
def readable_sheet():
    """ReadableSheet: колонка 2 заполнена строками 1-5, строка 1 — колонками 1-5."""
    cells = [DuckCell(r, 2, f"col2 r{r}") for r in range(1, 6)]
    cells += [DuckCell(1, c, f"row1 c{c}") for c in (1, 3, 4, 5)]
    finder = readable.create_finder([DuckSheet(cells)])

    return finder.find_sheet_by_name("t")


def test_find_values_by_col_rows_request_order(readable_sheet):
    """find_values_by_col_rows: значения в порядке запрошенных строк, без пропусков."""
    expected = ["col2 r5", "col2 r1", "col2 r3"]
    for _ in range(50):
        assert readable_sheet.find_values_by_col_rows(2, [5, 1, 3]) == expected

    # Отсутствующая строка — пустая строка: длина равна длине запроса
    assert readable_sheet.find_values_by_col_rows(2, [2, 99]) == ["col2 r2", ""]


def test_find_values_by_row_cols_request_order(readable_sheet):
    """find_values_by_row_cols: значения в порядке запрошенных колонок, без пропусков."""
    expected = ["row1 c4", "row1 c1", "row1 c5"]
    for _ in range(50):
        assert readable_sheet.find_values_by_row_cols(1, [4, 1, 5]) == expected

    # Отсутствующая колонка — пустая строка: длина равна длине запроса
    assert readable_sheet.find_values_by_row_cols(1, [3, 99]) == ["row1 c3", ""]
