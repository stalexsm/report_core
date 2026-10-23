"""Класс Comment: автор, текст и координаты.

Комментарий создаётся на листе через add_comment, читается
через comments; сеттеры меняют автора и текст.
"""

import pytest
from report_core import Book


@pytest.fixture
def sheet_with_comment():
    """Лист с одним комментарием в A1."""
    book = Book()
    sheet = book.add_sheet("лист")
    sheet.add_comment(1, 1, "текст комментария", "автор")

    return sheet


def test_comment_added_to_sheet(sheet_with_comment) -> None:
    """Добавленный комментарий попадает в список comments."""
    comments = sheet_with_comment.comments

    assert len(comments) == 1


def test_comment_fields(sheet_with_comment) -> None:
    """Геттеры отдают автора, текст и координату."""
    comment = sheet_with_comment.comments[0]

    assert comment.author == "автор"
    assert comment.text == "текст комментария"
    assert comment.coordinate == (1, 1)
    assert comment.row == 1
    assert comment.column == 1


def test_comment_setters(sheet_with_comment) -> None:
    """Сеттеры меняют автора и текст."""
    comment = sheet_with_comment.comments[0]
    comment.author = "новый автор"
    comment.text = "новый текст"

    assert comment.author == "новый автор"
    assert comment.text == "новый текст"


def test_several_comments_keep_order() -> None:
    """Несколько комментариев идут в порядке добавления."""
    book = Book()
    sheet = book.add_sheet("лист")
    sheet.add_comment(2, 3, "второй", "b")
    sheet.add_comment(1, 1, "первый", "a")

    assert [c.coordinate for c in sheet.comments] == [(2, 3), (1, 1)]
