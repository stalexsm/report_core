"""Тесты copy_sheet: копия не разделяет ячейки и комментарии с оригиналом.

Воспроизведение issue #4: поверхностное клонирование Sheet разделяло
`Arc<RwLock<Cell>>` / `Arc<RwLock<Comment>>` — мутация копии молча меняла
оригинал.
"""

import pytest
import report_core


@pytest.fixture
def book():
    return report_core.Book()


def test_copy_sheet_cells_independent(book):
    """Мутация ячейки копии не меняет ячейку оригинала."""
    sheet1 = book.add_sheet("A")
    sheet1.cell(1, 1, "orig")

    sheet2 = book.copy_sheet(sheet1)
    sheet2.cell(1, 1, "changed")

    assert sheet1.find_value_by_coords(1, 1) == "orig"
    assert sheet2.find_value_by_coords(1, 1) == "changed"


def test_copy_sheet_delete_rows_independent(book):
    """delete_rows на копии не двигает ячейки оригинала."""
    sheet1 = book.add_sheet("A")
    sheet1.cell(1, 1, "r1")
    sheet1.cell(2, 1, "r2")

    sheet2 = book.copy_sheet(sheet1)
    sheet2.delete_rows(1, 1)

    assert sheet1.find_value_by_coords(1, 1) == "r1"
    assert sheet1.find_value_by_coords(2, 1) == "r2"
    # Копия сдвинулась: строка 2 стала строкой 1
    assert sheet2.find_value_by_coords(1, 1) == "r2"


def test_copy_sheet_delete_cols_independent(book):
    """delete_cols на копии не двигает ячейки оригинала."""
    sheet1 = book.add_sheet("A")
    sheet1.cell(1, 1, "c1")
    sheet1.cell(1, 2, "c2")

    sheet2 = book.copy_sheet(sheet1)
    sheet2.delete_cols(1, 1)

    assert sheet1.find_value_by_coords(1, 1) == "c1"
    assert sheet1.find_value_by_coords(1, 2) == "c2"
    # Копия сдвинулась: колонка 2 стала колонкой 1
    assert sheet2.find_value_by_coords(1, 1) == "c2"


def test_copy_sheet_comments_independent(book):
    """Мутация комментария копии не меняет комментарий оригинала."""
    sheet1 = book.add_sheet("A")
    sheet1.add_comment(1, 1, "orig", "A.C")

    sheet2 = book.copy_sheet(sheet1)
    sheet2.comments[0].text = "changed"

    assert sheet1.comments[0].text == "orig"
    assert sheet2.comments[0].text == "changed"
