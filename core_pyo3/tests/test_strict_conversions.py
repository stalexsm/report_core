"""Строгие конверсии Python→Rust: мусорные координаты — ValueError, а не тихий мусор.

Регрессия к issue #21: `as`-приведения в PyValue::as_u32/as_u16 и разбор
merge_cells молча превращали мусор в валидные координаты: row=-1 → 4294967295,
col=65536 → 0, плоский список merge_cells → 4 «диапазона» — без единой ошибки
на стороне Python.
"""

import uuid

import pytest
from report_core import readable


def duck_cell(row=1, column=1):
    """Минимальная duck-typed ячейка с заданными координатами."""

    class Cell:
        id = uuid.uuid4()
        value = "x"
        data_type = "s"
        formula = None
        style_id = ""

        def __init__(self) -> None:
            self.row = row
            self.column = column

    return Cell()


def duck_sheet(cells, merge_cells=None):
    """Минимальный duck-typed лист для create_finder."""

    class Sheet:
        name = "t"
        sheet_state = "visible"

        def __init__(self) -> None:
            self.cells = cells
            self.merge_cells = merge_cells

    return Sheet()


BAD_COORDS = [
    pytest.param(-1, id="negative"),
    pytest.param(2**32, id="row-overflow"),
    pytest.param(1.5, id="fractional-float"),
    pytest.param(True, id="bool"),
    pytest.param("abc", id="garbage-string"),
    pytest.param(None, id="none"),
]

BAD_COLUMNS = [
    pytest.param(-1, id="negative"),
    pytest.param(2**16, id="column-overflow"),
    pytest.param(1.5, id="fractional-float"),
    pytest.param(True, id="bool"),
    pytest.param("abc", id="garbage-string"),
    pytest.param(None, id="none"),
]


@pytest.mark.parametrize("row", BAD_COORDS)
def test_cell_row_garbage_is_value_error(row) -> None:
    """Мусорный row ячейки — ValueError при создании ReadableSheet."""
    with pytest.raises(ValueError):
        readable.create_finder([duck_sheet([duck_cell(row=row)])])


@pytest.mark.parametrize("column", BAD_COLUMNS)
def test_cell_column_garbage_is_value_error(column) -> None:
    """Мусорный column ячейки — ValueError при создании ReadableSheet."""
    with pytest.raises(ValueError):
        readable.create_finder([duck_sheet([duck_cell(column=column)])])


@pytest.mark.parametrize(
    ("row", "column"),
    [
        pytest.param("5", 1, id="string-row"),
        pytest.param(2, "3", id="string-column"),
        pytest.param(1.0, 2.0, id="integral-float"),
        pytest.param(4294967295, 65535, id="upper-bound"),
    ],
)
def test_cell_valid_coords_accepted(row, column) -> None:
    """Валидные координаты (в том числе числовые строки) читаются как есть."""
    sheet = readable.create_finder([duck_sheet([duck_cell(row=row, column=column)])])
    sheet = sheet.find_sheet_by_name("t")
    cell = sheet.cells[0]

    assert (cell.row, cell.column) == (int(row), int(column))


BAD_MERGE_CELLS = [
    pytest.param([1, 2, 3, 4], id="flat-list"),
    pytest.param([[1, 2, 3]], id="short-range"),
    pytest.param([[1, 2, 3, 4, 5]], id="long-range"),
    pytest.param([[1, 2, 3, -4]], id="negative-coordinate"),
    pytest.param([[1, 2, 3, 65536]], id="column-overflow"),
    pytest.param([[1, 2, 3, "x"]], id="garbage-coordinate"),
    pytest.param([[1]], id="nested-scalar"),
    pytest.param([{"start_row": 1}], id="dict-range"),
    pytest.param("1:2", id="not-a-list"),
]


@pytest.mark.parametrize("merge_cells", BAD_MERGE_CELLS)
def test_merge_cells_garbage_is_value_error(merge_cells) -> None:
    """Мусорный merge_cells — ValueError, а не тихие «диапазоны»."""
    with pytest.raises(ValueError):
        readable.create_finder([duck_sheet([], merge_cells=merge_cells)])


@pytest.mark.parametrize(
    ("merge_cells", "expected"),
    [
        pytest.param(None, [], id="none-means-no-merges"),
        pytest.param([[1, 2, 1, 2]], [(1, 2, 1, 2)], id="single-range"),
        pytest.param(
            [[1, 2, 1, 2], [4, 8, 3, 6]],
            [(1, 2, 1, 2), (4, 8, 3, 6)],
            id="several-ranges",
        ),
        pytest.param([["1", "2", "1", "2"]], [(1, 2, 1, 2)], id="string-coords"),
    ],
)
def test_merge_cells_valid(merge_cells, expected) -> None:
    """Валидный merge_cells (4-элементные списки) читается без искажений."""
    sheet = readable.create_finder([duck_sheet([], merge_cells=merge_cells)])
    sheet = sheet.find_sheet_by_name("t")

    assert sheet.merge_cells == expected
