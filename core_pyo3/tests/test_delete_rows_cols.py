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
