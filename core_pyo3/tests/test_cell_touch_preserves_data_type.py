"""cell(r, c) без значения не сбрасывает data_type существующей ячейки.

Регрессия к issue #19: повторный вызов sheet.cell(r, c) (touch, без value)
выполнял set_data_type("s") для существующей неформульной ячейки — числовая
ячейка получала data_type="s" при value="3.14", и внешний писатель записал бы
число как текст. Паттерн санкционирован examples/service.py.
"""

import report_core


def test_cell_touch_preserves_data_type() -> None:
    """Повторный cell(r, c) без значения сохраняет data_type и value."""
    sheet = report_core.Book().add_sheet("t")

    sheet.cell(1, 1, "3.14")
    cell = sheet.cell(1, 1)

    assert cell.data_type == "n"
    assert cell.value == "3.14"


def test_cell_touch_creates_empty_string_typed_cell() -> None:
    """Touch по новой координате создаёт пустую ячейку с data_type "s"."""
    sheet = report_core.Book().add_sheet("t")

    cell = sheet.cell(1, 1)

    assert cell.data_type == "s"
    assert cell.value == ""
