"""Класс Book: контейнер листов.

Систематическое покрытие публичного API: добавление и поиск листов,
копирование, сериализация в json/dict, repr.
"""

import json

import pytest
from report_core import Book


@pytest.fixture
def book() -> Book:
    """Книга с одним листом «отчёт» и ячейкой A1."""
    book = Book()
    sheet = book.add_sheet("отчёт")
    sheet.cell(1, 1, "значение")

    return book


def test_new_book_is_empty() -> None:
    """Новая книга не содержит листов."""
    assert len(Book().sheets) == 0


def test_repr_shows_sheet_count(book: Book) -> None:
    """repr называет тип и число листов."""
    assert repr(book) == "Book, sheets: 1"


def test_add_sheet_defaults_to_visible(book: Book) -> None:
    """Лист добавляется видимым по умолчанию."""
    sheet = Book().add_sheet("новый")

    assert sheet.sheet_state == "visible"


def test_add_sheet_keeps_state(book: Book) -> None:
    """Состояние листа берётся из аргумента."""
    sheet = book.add_sheet("скрытый", "hidden")

    assert sheet.sheet_state == "hidden"


def test_sheets_lists_added_sheets(book: Book) -> None:
    """sheets отдаёт листы в порядке добавления."""
    book.add_sheet("второй")

    assert [s.name for s in book.sheets] == ["отчёт", "второй"]


def test_sheets_shares_sheet_with_book(book: Book) -> None:
    """Лист из sheets — тот же объект, а не копия: мутации видны."""
    sheet = book.sheets[0]
    sheet.cell(9, 9, "дополнение")

    stored = book.get_sheet_name("отчёт")

    assert stored is not None
    assert stored.find_value_by_coords(9, 9) == "дополнение"


def test_get_sheet_index_returns_sheet(book: Book) -> None:
    """Лист по индексу находится."""
    sheet = book.get_sheet_index(0)

    assert sheet is not None
    assert sheet.name == "отчёт"


def test_get_sheet_index_out_of_bounds(book: Book) -> None:
    """Несуществующий индекс даёт None, а не ошибку."""
    assert book.get_sheet_index(5) is None


def test_get_sheet_name_found(book: Book) -> None:
    """Лист по имени находится."""
    sheet = book.get_sheet_name("отчёт")

    assert sheet is not None
    assert sheet.find_value_by_coords(1, 1) == "значение"


def test_get_sheet_name_missing(book: Book) -> None:
    """Незнакомое имя даёт None."""
    assert book.get_sheet_name("нет такого") is None


def test_copy_sheet_appends_copy(book: Book) -> None:
    """Копия листа попадает в книгу отдельным листом с новым именем."""
    original = book.sheets[0]
    copy = book.copy_sheet(original)

    assert copy.name == "Sheet 2"
    assert len(book.sheets) == 2
    assert copy.find_value_by_coords(1, 1) == "значение"


def test_to_dict_contains_sheet_data(book: Book) -> None:
    """to_dict отдаёт структуру с именем, состоянием и ячейками."""
    data = book.to_dict()

    sheet = data["sheets"][0]
    assert sheet["name"] == "отчёт"
    assert sheet["sheet_state"] == "visible"
    assert sheet["cells"] == [
        {"row": 1, "column": 1, "value": "значение", "data_type": "s"}
    ]


def test_to_json_matches_to_dict(book: Book) -> None:
    """to_json — то же представление, что to_dict, в формате JSON."""
    assert json.loads(book.to_json()) == book.to_dict()
