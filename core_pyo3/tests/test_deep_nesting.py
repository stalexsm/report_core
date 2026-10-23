"""Readable-слой не падает от глубоко вложенных списков.

Регрессия к issue #24: список глубины ~10⁵ в читаемом атрибуте
(`value`, `merge_cells`) уводил нативную рекурсию extract_py_value_auto
в переполнение стека — процесс погибал от SIGSEGV, который нельзя
перехватить. Ожидаемое поведение — ValueError.
"""

import pytest
from report_core import readable

# Глубина, на которой неограниченная рекурсия гарантированно
# переполняет нативный стек.
CRASH_DEPTH = 100_000


def deep_nested() -> list:
    """Список, вложенный вглубь на CRASH_DEPTH уровней (без рекурсии)."""
    nested: list = []
    for _ in range(CRASH_DEPTH):
        nested = [nested]
    return nested


class DuckCell:
    """Duck-typed ячейка: row, column, value, formula, data_type, style_id."""

    def __init__(self, value: object) -> None:
        self.row = 1
        self.column = 1
        self.value = value
        self.formula = None
        self.data_type = "n"
        self.style_id = None


class DuckSheet:
    """Duck-typed лист: name, sheet_state, merge_cells, cells."""

    def __init__(self, merge_cells: object) -> None:
        self.name = "data"
        self.sheet_state = "visible"
        self.merge_cells = merge_cells
        self.cells = []


def test_deep_nested_value_raises_value_error() -> None:
    """Глубоко вложенный список в value — ValueError, а не падение процесса."""
    sheet = DuckSheet(None)
    sheet.cells = [DuckCell(deep_nested())]

    with pytest.raises(ValueError):
        readable.create_finder([sheet])


def test_deep_nested_merge_cells_raises_value_error() -> None:
    """Глубоко вложенный список в merge_cells — ValueError, а не падение."""
    sheet = DuckSheet(deep_nested())

    with pytest.raises(ValueError):
        readable.create_finder([sheet])
