"""Валидация домена Excel в delete_rows/delete_cols (issue #8).

До фикса `idx + amount` считалось в узких типах (u16/u32): в release —
wrap, коллизии ключей карты и молчаливая потеря ячеек; в debug — паника
внутри `drain()`. Теперь невалидный диапазон — `ValueError`.
"""

import pytest
import report_core

# Границы листа Excel (те же константы, что в ядре)
MAX_ROW = 1_048_576
MAX_COL = 16_384


@pytest.fixture
def sheet():
    book = report_core.Book()
    return book.add_sheet("A")


def test_delete_cols_out_of_bounds_valueerror(sheet):
    """Диапазон удаления за границами листа → ValueError, а не тихая порча."""
    sheet.cell(1, 1, "A1")

    # 60000 + 10000 = 70000 > u16::MAX — раньше переполнялось
    with pytest.raises(ValueError):
        sheet.delete_cols(60000, 10000)

    # Ошибочный вызов ничего не удаляет
    assert sheet.get_value_cell(1, 1) == "A1"


def test_delete_rows_out_of_bounds_valueerror(sheet):
    """Диапазон удаления строк за границами листа → ValueError."""
    sheet.cell(1, 1, "A1")

    with pytest.raises(ValueError):
        sheet.delete_rows(MAX_ROW, 2)

    assert sheet.get_value_cell(1, 1) == "A1"


def test_delete_zero_idx_or_amount_valueerror(sheet):
    """Нулевые idx/amount вне домена Excel → ValueError."""
    with pytest.raises(ValueError):
        sheet.delete_cols(0, 1)

    with pytest.raises(ValueError):
        sheet.delete_cols(1, 0)

    with pytest.raises(ValueError):
        sheet.delete_rows(0, 1)

    with pytest.raises(ValueError):
        sheet.delete_rows(1, 0)


def test_delete_cols_boundary_valid(sheet):
    """Граничные валидные вызовы проходят, содержимое корректно."""
    # Последняя колонка XFD = MAX_COL удаляется
    sheet.cell(1, MAX_COL, "XFD1")
    sheet.delete_cols(MAX_COL, 1)
    assert sheet.cells == []

    # Все колонки одним вызовом
    sheet.cell(1, 1, "A1")
    sheet.cell(1, MAX_COL, "XFD1")
    sheet.delete_cols(1, MAX_COL)
    assert sheet.cells == []


def test_delete_rows_boundary_valid(sheet):
    """Граничные валидные вызовы для строк проходят."""
    # Последняя строка MAX_ROW удаляется
    sheet.cell(MAX_ROW, 1, "нижняя")
    sheet.delete_rows(MAX_ROW, 1)
    assert sheet.cells == []

    # Все строки одним вызовом
    sheet.cell(1, 1, "A1")
    sheet.cell(MAX_ROW, 1, "нижняя")
    sheet.delete_rows(1, MAX_ROW)
    assert sheet.cells == []


def test_delete_cols_shifts_content(sheet):
    """После валидного удаления колонки сдвигаются со значениями."""
    for r in (1, 2):
        for c in range(1, 6):
            sheet.cell(r, c, f"V{r}:{c}")

    # Удаляем колонки 2–3: 4→2, 5→3, колонка 1 не двигается
    sheet.delete_cols(2, 2)

    assert len(sheet.cells) == 6
    assert sheet.get_value_cell(1, 1) == "V1:1"
    assert sheet.get_value_cell(2, 2) == "V2:4"
    assert sheet.get_value_cell(1, 3) == "V1:5"


def test_delete_rows_shifts_content(sheet):
    """После валидного удаления строки сдвигаются со значениями."""
    for r in range(1, 6):
        for c in (1, 2):
            sheet.cell(r, c, f"V{r}:{c}")

    # Удаляем строки 2–3: 4→2, 5→3, строка 1 не двигается
    sheet.delete_rows(2, 2)

    assert len(sheet.cells) == 6
    assert sheet.get_value_cell(1, 1) == "V1:1"
    assert sheet.get_value_cell(2, 2) == "V4:2"
    assert sheet.get_value_cell(3, 1) == "V5:1"


def test_delete_rows_shifts_merge_cells(sheet):
    """Merge-диапазоны сдвигаются вместе со строками.

    До фикса delete_rows делегировал только в cells: объединения оставались
    в старой системе координат, и итоговый XLSX собирался с несогласованными
    объединениями.
    """
    sheet.add_merge_cells(1, 1, 1, 2)  # выше удаления
    sheet.add_merge_cells(7, 8, 1, 2)  # ниже удаления

    sheet.delete_rows(2, 4)

    assert sorted(sheet.merge_cells) == [(1, 1, 1, 2), (3, 4, 1, 2)]


def test_delete_rows_truncates_intersecting_merge_cells(sheet):
    """Merge-диапазон, чей хвост попал в удаление, усекается."""
    sheet.add_merge_cells(1, 2, 4, 5)  # строка 2 в удалении

    sheet.delete_rows(2, 4)

    assert sheet.merge_cells == [(1, 1, 4, 5)]


def test_delete_cols_shifts_and_truncates_merge_cells(sheet):
    """Merge-диапазоны сдвигаются влево и усекаются при удалении колонок."""
    sheet.add_merge_cells(1, 2, 1, 3)  # колонки 2–3 в удалении
    sheet.add_merge_cells(1, 2, 9, 10)  # правее удаления

    sheet.delete_cols(2, 4)

    assert sorted(sheet.merge_cells) == [(1, 2, 1, 1), (1, 2, 5, 6)]


def test_delete_rows_shifts_and_drops_comments(sheet):
    """Комментарии сдвигаются, а комментарии удалённых ячеек удаляются
   ."""
    sheet.add_comment(1, 1, "верх", "A")
    sheet.add_comment(3, 2, "удаляемый", "B")
    sheet.add_comment(7, 3, "нижний", "C")

    sheet.delete_rows(2, 4)

    coords = sorted(c.coordinate for c in sheet.comments)
    assert coords == [(1, 1), (3, 3)]


def test_delete_cols_shifts_and_drops_comments(sheet):
    """Комментарии сдвигаются влево при удалении колонок."""
    sheet.add_comment(1, 1, "левый", "A")
    sheet.add_comment(2, 3, "удаляемый", "B")
    sheet.add_comment(3, 7, "правый", "C")

    sheet.delete_cols(2, 4)

    coords = sorted(c.coordinate for c in sheet.comments)
    assert coords == [(1, 1), (3, 3)]


def test_delete_rows_shifts_row_dimensions(sheet):
    """Ключи размеров строк сдвигаются вместе со строками."""
    sheet.set_height_row(1, 10.0)
    sheet.set_height_row(3, 30.0)
    sheet.set_height_row(7, 70.0)

    sheet.delete_rows(2, 4)

    assert sheet.get_height_by_row(1) == 10.0  # выше — на месте
    assert sheet.get_height_by_row(3) == 70.0  # 7 → 3


def test_delete_cols_shifts_column_dimensions(sheet):
    """Ключи размеров колонок сдвигаются вместе с колонками."""
    sheet.set_width_column(1, 10.0)
    sheet.set_width_column(3, 30.0)
    sheet.set_width_column(7, 70.0)

    sheet.delete_cols(2, 4)

    assert sheet.get_width_by_column(1) == 10.0  # левее — на месте
    assert sheet.get_width_by_column(3) == 70.0  # 7 → 3
