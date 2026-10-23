"""Finder читает те же листы, а не их копии.

Регрессия к issue #26: create_finder и конструктор Finder глубоко
копировали каждый лист (`ws.0.read().clone()` + новый Arc) — создание
было O(суммы ячеек книги), данные хранились дважды, а мутации исходных
листов в копию не попадали. Собственные листы библиотеки разделяются
по Arc: мутация исходного листа видна через Finder.
"""

from report_core import Book, Sheet, readable
from report_core.readable import Finder


def make_book_sheet() -> Sheet:
    """Лист книги с одной ячейкой."""
    book = Book()
    sheet = book.add_sheet("data")
    sheet.cell(1, 1, "a")

    return sheet


def assert_mutation_visible(finder: Finder, sheet: Sheet) -> None:
    """Мутация исходного листа после создания Finder видна через поиск."""
    sheet.cell(1, 1, "b")

    found = finder.find_sheet_by_name("data")
    assert found is not None

    cell = found.find_cell_by_coords(1, 1)
    assert cell is not None
    assert cell.value == "b"


def test_create_finder_shares_book_sheet() -> None:
    """create_finder разделяет лист книги, а не копирует его."""
    sheet = make_book_sheet()

    assert_mutation_visible(readable.create_finder([sheet]), sheet)


def test_finder_constructor_shares_book_sheet() -> None:
    """Конструктор Finder разделяет лист книги, а не копирует его."""
    sheet = make_book_sheet()

    assert_mutation_visible(readable.Finder([sheet]), sheet)


def test_finder_sheets_accessor_same_sheet() -> None:
    """sheets у Finder — те же листы, что переданы: имя мутации видно."""
    sheet = make_book_sheet()
    finder = readable.create_finder([sheet])

    sheet.name = "renamed"

    assert finder.sheets[0].name == "renamed"
