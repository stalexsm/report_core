"""Класс Sheet: структура листа.

Имя и состояние, размеры, get-or-create ячеек, объединения,
размеры и скрытость строк/колонок, выборки ячеек по диапазону,
поиск по значению и шаблону.
"""

import pytest
from report_core import Book

# Прямоугольник значений 3x3: строка i, колонка j -> значение "(i, j)".
FILLED_ROWS = 3
FILLED_COLUMNS = 3


@pytest.fixture
def sheet():
    """Лист с прямоугольником ячеек (1..3) x (1..3)."""
    book = Book()
    sheet = book.add_sheet("лист")
    for row in range(1, FILLED_ROWS + 1):
        for col in range(1, FILLED_COLUMNS + 1):
            sheet.cell(row, col, f"v{row}{col}")

    return sheet


def test_initial_name_and_state() -> None:
    """Новый лист наследует имя и состояние из add_sheet."""
    book = Book()
    sheet = book.add_sheet("данные", "hidden")

    assert sheet.name == "данные"
    assert sheet.sheet_state == "hidden"


def test_name_setter(sheet) -> None:
    """Имя листа переименовывается."""
    sheet.name = "переименованный"

    assert sheet.name == "переименованный"


def test_set_sheet_state(sheet) -> None:
    """Видимость листа переключается методом set_sheet_state."""
    sheet.set_sheet_state("hidden")

    assert sheet.sheet_state == "hidden"


def test_repr(sheet) -> None:
    """repr называет тип, имя и число ячеек."""
    assert repr(sheet) == "Sheet (лист) cells: 9"


def test_max_row_and_column(sheet) -> None:
    """Размеры листа — по максимальной заполненной ячейке."""
    assert sheet.max_row == FILLED_ROWS
    assert sheet.max_column == FILLED_COLUMNS


def test_empty_sheet_dimensions() -> None:
    """Пустой лист: нулевые размеры."""
    book = Book()
    sheet = book.add_sheet("пустой")

    assert sheet.max_row == 0
    assert sheet.max_column == 0


def test_cell_creates_and_reads(sheet) -> None:
    """cell с значением создаёт ячейку, find_value_by_coords читает её."""
    sheet.cell(5, 5, "новая")

    assert sheet.find_value_by_coords(5, 5) == "новая"


def test_cell_get_or_create_returns_same_cell(sheet) -> None:
    """Повторный cell без значения отдаёт ту же ячейку без изменений."""
    created = sheet.cell(1, 1, "оригинал")
    fetched = sheet.cell(1, 1)

    assert fetched.value == "оригинал"
    assert created.data_type == fetched.data_type


def test_cells_sorted_by_coords(sheet) -> None:
    """cells отдаёт ячейки упорядоченно: строка, затем колонка."""
    coords = [(c.row, c.column) for c in sheet.cells]

    assert coords == sorted(coords)
    assert len(coords) == FILLED_ROWS * FILLED_COLUMNS


def test_merge_cells_roundtrip(sheet) -> None:
    """Добавленное объединение читается кортежом (строки, колонки)."""
    sheet.add_merge_cells(1, 2, 1, 3)

    assert sheet.merge_cells == [(1, 2, 1, 3)]


def test_get_cells_by_range_full(sheet) -> None:
    """Без границ get_cells_by_range отдаёт все ячейки."""
    assert len(sheet.get_cells_by_range()) == FILLED_ROWS * FILLED_COLUMNS


def test_get_cells_by_range_subrange(sheet) -> None:
    """Границы фильтруют ячейки по строкам и колонкам."""
    cells = sheet.get_cells_by_range(2, FILLED_ROWS, 2, FILLED_COLUMNS)

    assert [(c.row, c.column) for c in cells] == [(2, 2), (2, 3), (3, 2), (3, 3)]


def test_get_cells_by_range_out_of_bounds(sheet) -> None:
    """Диапазон за пределами данных пуст."""
    assert sheet.get_cells_by_range(100, 200, 100, 200) == []


def test_default_dimensions(sheet) -> None:
    """До переопределения действуют ширина и высота по умолчанию."""
    assert sheet.get_height_by_row(1) == pytest.approx(20.0)
    assert sheet.get_width_by_column(1) == pytest.approx(22.5)
    assert not sheet.get_hidden_by_row(1)
    assert not sheet.get_hidden_by_column(1)


def test_set_dimensions_and_hidden(sheet) -> None:
    """Установленные размеры и скрытость читаются обратно."""
    sheet.set_height_row(1, 25.5)
    sheet.set_width_column(1, 100.0)
    sheet.set_hidden_row(2, True)
    sheet.set_hidden_column(2, True)

    assert sheet.get_height_by_row(1) == pytest.approx(25.5)
    assert sheet.get_width_by_column(1) == pytest.approx(100.0)
    assert sheet.get_hidden_by_row(2)
    assert sheet.get_hidden_by_column(2)


def test_find_cell_by_str(sheet) -> None:
    """Поиск по точной строке находит ячейку по координате."""
    found = sheet.find_cell_by_str("v23")

    assert found is not None
    assert (found.row, found.column) == (2, 3)


def test_find_cell_by_str_missing(sheet) -> None:
    """Ненайденное значение — None."""
    assert sheet.find_cell_by_str("нет") is None


def test_find_cell_by_regex(sheet) -> None:
    """Поиск по шаблону находит первую подходящую ячейку."""
    found = sheet.find_cell_by_regex(r"v2\d")

    assert found is not None
    assert (found.row, found.column) == (2, 1)


def test_find_cell_by_letter(sheet) -> None:
    """Поиск по буквенной координате отдаёт ячейку."""
    found = sheet.find_cell_by_letter("B2")

    assert found is not None
    assert found.value == "v22"


def test_find_cell_by_letter_invalid(sheet) -> None:
    """Буква не в формате координаты — ValueError."""
    with pytest.raises(ValueError, match="invalid cell letter"):
        sheet.find_cell_by_letter("1A")


def test_find_cells_by_str(sheet) -> None:
    """Поиск всех ячеек по точной строке."""
    sheet.cell(9, 9, "v11")

    found = sheet.find_cells_by_str("v11")

    assert [(c.row, c.column) for c in found] == [(1, 1), (9, 9)]


def test_find_cells_by_regex(sheet) -> None:
    """Поиск всех ячеек по шаблону."""
    found = sheet.find_cells_by_regex(r"v\d2")

    assert [(c.row, c.column) for c in found] == [(1, 2), (2, 2), (3, 2)]


def test_find_cells_for_rows_by_regex(sheet) -> None:
    """Ячейки по шаблону в пределах колонок: col_stop отсекает хвост строки."""
    found = sheet.find_cells_for_rows_by_regex(r"v\d\d", 2)

    assert [(c.row, c.column) for c in found] == [
        (1, 1),
        (1, 2),
        (2, 1),
        (2, 2),
        (3, 1),
        (3, 2),
    ]


def test_find_cells_for_cols_by_regex(sheet) -> None:
    """Ячейки по шаблону в пределах строк: row_stop отсекает низ колонки."""
    found = sheet.find_cells_for_cols_by_regex(r"v\d\d", 2)

    assert [(c.row, c.column) for c in found] == [
        (1, 1),
        (1, 2),
        (1, 3),
        (2, 1),
        (2, 2),
        (2, 3),
    ]


def test_find_cells_multi_regex(sheet) -> None:
    r"""Чередующийся выбор: первая под шаблон «до», затем первая под «после».

    Для сетки v11..v33: v11 матчится с шаблоном «до» (v1\d), v21 — с
    шаблоном «после» (v\d1), дальше пара начинается заново.
    """
    found = sheet.find_cells_multi_regex(r"v1\d", r"v\d1")

    assert [(c.row, c.column) for c in found] == [(1, 1), (2, 1)]


def test_find_cells_between_regex(sheet) -> None:
    """Все ячейки строк от границы «до» до границы «после».

    Границы — строки первых ячеек под шаблоны: v11 -> строка 1,
    v22 -> строка 2; в результат попадают все ячейки этих строк.
    """
    found = sheet.find_cells_between_regex(r"v11", r"v22")

    assert [(c.row, c.column) for c in found] == [
        (1, 1),
        (1, 2),
        (1, 3),
        (2, 1),
        (2, 2),
        (2, 3),
    ]


def test_find_cells_range_rows(sheet) -> None:
    """Все ячейки в диапазоне строк, границы включительно."""
    found = sheet.find_cells_range_rows(2, FILLED_ROWS)

    assert [(c.row, c.column) for c in found] == [
        (2, 1),
        (2, 2),
        (2, 3),
        (3, 1),
        (3, 2),
        (3, 3),
    ]


def test_find_cells_range_cols(sheet) -> None:
    """Все ячейки в диапазоне колонок, границы включительно."""
    found = sheet.find_cells_range_cols(2, FILLED_COLUMNS)

    assert [(c.row, c.column) for c in found] == [
        (1, 2),
        (1, 3),
        (2, 2),
        (2, 3),
        (3, 2),
        (3, 3),
    ]


def test_find_value_by_coords(sheet) -> None:
    """Значение по координатам: есть и нет."""
    assert sheet.find_value_by_coords(1, 1) == "v11"
    assert sheet.find_value_by_coords(100, 100) is None


def test_find_cell_by_coords(sheet) -> None:
    """Ячейка по координатам: найдена и промах."""
    found = sheet.find_cell_by_coords(2, 3)

    assert found is not None
    assert (found.row, found.column) == (2, 3)
    assert found.value == "v23"
    assert sheet.find_cell_by_coords(100, 100) is None


def test_find_values_by_col_rows(sheet) -> None:
    """Значения по колонке и списку строк — в порядке запроса, пропуски пустые."""
    assert sheet.find_values_by_col_rows(2, [1, 3]) == ["v12", "v32"]
    assert sheet.find_values_by_col_rows(2, [5, 1]) == ["", "v12"]


def test_find_values_by_row_cols(sheet) -> None:
    """Значения по строке и списку колонок — в порядке запроса, пропуски пустые."""
    assert sheet.find_values_by_row_cols(3, [1, 2]) == ["v31", "v32"]
    assert sheet.find_values_by_row_cols(3, [1, 99]) == ["v31", ""]


def test_cell_rejects_zero_coords() -> None:
    """Нулевые координаты вне домена Excel — ValueError, а не тихая ячейка."""
    book = Book()
    sheet = book.add_sheet("лист")

    with pytest.raises(ValueError, match="вне домена"):
        sheet.cell(0, 1, "x")

    with pytest.raises(ValueError, match="вне домена"):
        sheet.cell(1, 0, "x")

    assert sheet.max_row == 0
    assert sheet.max_column == 0


def test_add_merge_cells_rejects_inverted_range(sheet) -> None:
    """Перевёрнутый диапазон объединения — ValueError."""
    with pytest.raises(ValueError, match="больше конца"):
        sheet.add_merge_cells(3, 1, 1, 2)

    with pytest.raises(ValueError, match="вне домена"):
        sheet.add_merge_cells(1, 1_048_577, 1, 2)
