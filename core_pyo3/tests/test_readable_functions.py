"""Функции сабмодуля readable: поиск по списку ячеек.

Каждая функция-дубликат метода листа работает со списком ячеек;
здесь фиксируются happy-path всех функций и ошибки невалидных
аргументов.
"""

import pytest
from report_core import Book, readable

# Прямоугольник значений 2x2: строка i, колонка j -> значение "v{i}{j}".
FILLED_ROWS = 2
FILLED_COLUMNS = 2


@pytest.fixture
def cells():
    """Список ячеек readable-листа с прямоугольником значений."""
    book = Book()
    sheet = book.add_sheet("лист")
    for row in range(1, FILLED_ROWS + 1):
        for col in range(1, FILLED_COLUMNS + 1):
            sheet.cell(row, col, f"v{row}{col}")

    finder = readable.create_finder([sheet])
    found = finder.find_sheet_by_name("лист")
    assert found is not None

    return found.cells


def test_find_cell_by_coords(cells) -> None:
    """Ячейка по координатам: найдена и промах."""
    found = readable.find_cell_by_coords(1, 2, cells)

    assert found is not None
    assert found.value == "v12"
    assert readable.find_cell_by_coords(9, 9, cells) is None


def test_find_value_by_coords(cells) -> None:
    """Значение по координатам: найдено и промах."""
    assert readable.find_value_by_coords(2, 1, cells) == "v21"
    assert readable.find_value_by_coords(9, 9, cells) is None


def test_find_cell_by_regex(cells) -> None:
    """Первая ячейка по шаблону."""
    found = readable.find_cell_by_regex(r"v2\d", cells)

    assert found is not None
    assert (found.row, found.column) == (2, 1)


def test_find_cell_by_regex_invalid(cells) -> None:
    """Невалидный шаблон доходит до движка regex и отдаёт RuntimeError."""
    with pytest.raises(RuntimeError):
        readable.find_cell_by_regex("(", cells)


def test_find_cell_by_str(cells) -> None:
    """Первая ячейка по точной строке."""
    found = readable.find_cell_by_str("v11", cells)

    assert found is not None
    assert (found.row, found.column) == (1, 1)


def test_find_cell_by_letter(cells) -> None:
    """Ячейка по буквенной координате."""
    found = readable.find_cell_by_letter("B2", cells)

    assert found is not None
    assert found.value == "v22"


def test_find_cell_by_letter_invalid(cells) -> None:
    """Невалидная буква — ValueError."""
    with pytest.raises(ValueError):
        readable.find_cell_by_letter("1A", cells)


def test_find_cells_by_regex(cells) -> None:
    """Все ячейки по шаблону."""
    found = readable.find_cells_by_regex(r"v\d1", cells)

    assert [(c.row, c.column) for c in found] == [(1, 1), (2, 1)]


def test_find_cells_by_str(cells) -> None:
    """Все ячейки по точной строке: пустой результат без совпадений."""
    assert readable.find_cells_by_str("нет", cells) == []
    assert [c.value for c in readable.find_cells_by_str("v11", cells)] == ["v11"]


def test_find_cells_for_rows_by_regex(cells) -> None:
    """Ячейки по шаблону в пределах колонок: col_stop отсекает хвост строки."""
    found = readable.find_cells_for_rows_by_regex(r"v\d\d", 1, cells)

    assert [(c.row, c.column) for c in found] == [(1, 1), (2, 1)]


def test_find_cells_for_cols_by_regex(cells) -> None:
    """Ячейки по шаблону в пределах строк: row_stop отсекает низ колонки."""
    found = readable.find_cells_for_cols_by_regex(r"v\d\d", 1, cells)

    assert [(c.row, c.column) for c in found] == [(1, 1), (1, 2)]


def test_find_cells_multi_regex(cells) -> None:
    """Чередующийся выбор: первая «до», затем первая «после»."""
    found = readable.find_cells_multi_regex(r"v11", r"v21", cells)

    assert [(c.row, c.column) for c in found] == [(1, 1), (2, 1)]


def test_find_cells_between_regex(cells) -> None:
    """Все ячейки строк от границы «до» до границы «после»."""
    found = readable.find_cells_between_regex(r"v11", r"v21", cells)

    assert [(c.row, c.column) for c in found] == [(1, 1), (1, 2), (2, 1), (2, 2)]


def test_find_cells_range_rows(cells) -> None:
    """Все ячейки в диапазоне строк, границы включительно."""
    found = readable.find_cells_range_rows(2, FILLED_ROWS, cells)

    assert [(c.row, c.column) for c in found] == [(2, 1), (2, 2)]


def test_find_cells_range_cols(cells) -> None:
    """Все ячейки в диапазоне колонок, границы включительно."""
    found = readable.find_cells_range_cols(2, FILLED_COLUMNS, cells)

    assert [(c.row, c.column) for c in found] == [(1, 2), (2, 2)]


def test_find_values_by_col_rows(cells) -> None:
    """Значения колонки по списку строк: порядок запроса, промах — пустая строка."""
    assert readable.find_values_by_col_rows(1, [2, 1, 9], cells) == ["v21", "v11", ""]


def test_find_values_by_row_cols(cells) -> None:
    """Значения строки по списку колонок: порядок запроса, промах — пустая строка."""
    assert readable.find_values_by_row_cols(1, [2, 1, 9], cells) == ["v12", "v11", ""]


def test_create_finder_and_find(cells) -> None:
    """create_finder строит поисковик по списку ячеек-листов."""
    book = Book()
    sheet = book.add_sheet("другой")
    sheet.cell(1, 1, "значение")

    finder = readable.create_finder([sheet])

    found = finder.find_sheet_by_name("другой")
    assert found is not None
    assert found.get_value_cell(1, 1) == "значение"
