"""Readable-классы: Finder, ReadableSheet, ReadableCell.

Finder ищет по списку листов (собственные или duck-typed),
ReadableSheet читает структуру готового листа, ReadableCell —
значение ячейки и флаги типа.
"""

import uuid

import pytest
from report_core import Book, readable


def make_book_sheet():
    """Собственный лист книги: ячейки A1 «искомое» и B2 «другое»."""
    book = Book()
    sheet = book.add_sheet("данные")
    sheet.cell(1, 1, "искомое")
    sheet.cell(2, 2, "другое")

    return sheet


def duck_cell(row, column, value="x", data_type="s"):
    """Duck-typed ячейка для readable-слоя."""

    class Cell:
        id = uuid.uuid4()
        formula = None
        style_id = ""

        def __init__(self) -> None:
            self.row = row
            self.column = column
            self.value = value
            self.data_type = data_type

    return Cell()


def duck_sheet(name, cells):
    """Duck-typed лист для readable-слоя."""

    class Sheet:
        sheet_state = "visible"
        merge_cells = None

        def __init__(self) -> None:
            self.name = name
            self.cells = cells

    return Sheet()


@pytest.fixture
def finder():
    """Finder по собственному листу книги."""
    return readable.Finder([make_book_sheet()])


@pytest.fixture
def readable_sheet(finder):
    """Лист «данные» в readable-представлении."""
    sheet = finder.find_sheet_by_name("данные")
    assert sheet is not None

    return sheet


def test_finder_sheets_accessor(finder) -> None:
    """sheets отдаёт листы, по которым ищет поисковик."""
    assert [s.name for s in finder.sheets] == ["данные"]


def test_finder_get_sheet_index(finder) -> None:
    """Лист по индексу находится, промах — None."""
    sheet = finder.get_sheet_index(0)

    assert sheet is not None
    assert sheet.name == "данные"
    assert finder.get_sheet_index(9) is None


def test_finder_get_sheets_with_names(finder) -> None:
    """get_sheets_with_names оставляет только перечисленные листы."""
    assert [s.name for s in finder.get_sheets_with_names(["данные"])] == ["данные"]
    assert finder.get_sheets_with_names(["нет"]) == []


def test_finder_get_sheets_without_names(finder) -> None:
    """get_sheets_without_names убирает перечисленные листы."""
    assert finder.get_sheets_without_names(["данные"]) == []
    assert [s.name for s in finder.get_sheets_without_names(["нет"])] == ["данные"]


def test_finder_find_sheet_by_name(finder) -> None:
    """Поиск листа по имени: найден и промах."""
    assert finder.find_sheet_by_name("данные") is not None
    assert finder.find_sheet_by_name("нет") is None


def test_finder_find_sheet_by_regex(finder) -> None:
    """Поиск листа по шаблону без учёта регистра."""
    sheet = finder.find_sheet_by_regex("ДАН")

    assert sheet is not None
    assert sheet.name == "данные"


def test_finder_find_sheet_by_regex_invalid(finder) -> None:
    """Невалидный шаблон — ValueError."""
    with pytest.raises(ValueError):
        finder.find_sheet_by_regex("(")


def test_finder_repr(finder) -> None:
    """repr называет тип и число листов."""
    assert repr(finder) == "Finder, sheets: 1"


def test_readable_sheet_structure(readable_sheet) -> None:
    """Имя, состояние, размеры и repr readable-листа."""
    assert readable_sheet.name == "данные"
    assert readable_sheet.sheet_state == "visible"
    assert readable_sheet.max_row == 2
    assert readable_sheet.max_column == 2
    assert repr(readable_sheet) == "ReadableSheet (данные) cells: 2"


def test_readable_sheet_cells_sorted(readable_sheet) -> None:
    """Ячейки идут упорядоченно по координатам."""
    assert [(c.row, c.column) for c in readable_sheet.cells] == [(1, 1), (2, 2)]


def test_readable_sheet_find_value_by_coords(readable_sheet) -> None:
    """Значение ячейки по координатам."""
    assert readable_sheet.find_value_by_coords(1, 1) == "искомое"


def test_readable_sheet_get_cells_by_range(readable_sheet) -> None:
    """Выборка по диапазону: полный и урезанный."""
    assert len(readable_sheet.get_cells_by_range()) == 2

    cells = readable_sheet.get_cells_by_range(1, 1, 1, 1)

    assert [(c.row, c.column) for c in cells] == [(1, 1)]


def test_readable_sheet_merge_cells_from_duck() -> None:
    """merge-кортежи duck-листа читаются как диапазоны."""
    sheet = duck_sheet("m", [duck_cell(1, 1)])
    sheet.merge_cells = [(1, 2, 1, 2)]

    finder = readable.Finder([sheet])
    found = finder.find_sheet_by_name("m")

    assert found is not None
    assert found.merge_cells == [(1, 2, 1, 2)]


def test_readable_cell_fields(readable_sheet) -> None:
    """Геттеры readable-ячейки: координаты, буква, значение, тип."""
    cell = readable_sheet.find_cell_by_str("искомое")

    assert cell is not None
    assert (cell.row, cell.column) == (1, 1)
    assert cell.letter == "A1"
    assert cell.value == "искомое"
    assert cell.data_type == "s"
    assert cell.formula is None
    assert cell.style is None
    assert not cell.is_formula


def test_readable_cell_text_value_has_no_flags(readable_sheet) -> None:
    """Текстовая ячейка не отвечает ни одному типовому флагу."""
    cell = readable_sheet.find_cell_by_str("искомое")

    assert cell is not None
    assert not cell.is_value_numeric
    assert not cell.is_value_integer
    assert not cell.is_value_bool
    assert not cell.is_value_datetime
    assert not cell.is_value_empty


def test_readable_cell_repr(readable_sheet) -> None:
    """repr называет тип и координаты."""
    cell = readable_sheet.find_cell_by_coords(1, 1)

    assert repr(cell) == "ReadableCell, Coords: Coordinate { row: 1, column: 1 }"


@pytest.mark.parametrize(
    ("data_type", "value", "flag"),
    [
        ("n", "3.5", "is_value_numeric"),
        ("n", "7", "is_value_integer"),
        ("b", "true", "is_value_bool"),
        ("d", "2024-01-02 03:04:05", "is_value_datetime"),
        (None, None, "is_value_empty"),
    ],
)
def test_readable_cell_value_flags(data_type, value, flag) -> None:
    """Флаг типа значения отвечает фактическому значению duck-ячейки."""
    sheet = duck_sheet("t", [duck_cell(1, 1, value, data_type)])

    finder = readable.Finder([sheet])
    found = finder.find_sheet_by_name("t")
    assert found is not None

    cell = found.find_cell_by_coords(1, 1)
    assert cell is not None

    assert getattr(cell, flag)
