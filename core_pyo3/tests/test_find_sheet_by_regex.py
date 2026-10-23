"""find_sheet_by_regex: невалидный regex — ValueError, а не паника.

Регрессия к issue #6: `Regex::new(pattern).unwrap()` на невалидном шаблоне
(например, "(") падал pyo3.PanicException (не ловится `except Exception`).

Семантика поиска — без учёта регистра, как у клеточных find_*_by_regex.
"""

import uuid

import pytest
from report_core import readable


class DuckCell:
    """Минимальный duck-typed ReadableCell для создания ReadableSheet."""

    id = uuid.uuid4()
    row = 1
    column = 1
    value = "x"
    data_type = "s"
    formula = None
    style_id = ""


class DuckSheet:
    """Минимальный duck-typed лист для ReadableSheet."""

    name = "t"
    sheet_state = "visible"
    merge_cells = None

    def __init__(self) -> None:
        self.cells = [DuckCell()]


def make_finder() -> readable.Finder:
    """Finder по одному duck-typed листу с именем "t"."""
    return readable.create_finder([DuckSheet()])


INVALID_PATTERNS = ["(", ")", "[", "*", "+", "?", "(?P<"]


def test_find_sheet_by_regex_invalid() -> None:
    """Finder.find_sheet_by_regex: невалидный шаблон — ValueError."""
    finder = make_finder()

    for pattern in INVALID_PATTERNS:
        with pytest.raises(ValueError):
            finder.find_sheet_by_regex(pattern)


def test_find_sheet_by_regex_no_match() -> None:
    """Finder.find_sheet_by_regex: валидный шаблон без совпадений — None."""
    finder = make_finder()

    assert finder.find_sheet_by_regex("no-such-sheet-\\d+") is None


def test_find_sheet_by_regex_match() -> None:
    """Finder.find_sheet_by_regex: валидный шаблон с совпадением — лист."""
    finder = make_finder()

    sheet = finder.find_sheet_by_regex("^t$")

    assert sheet is not None
    assert sheet.name == "t"


def test_find_sheet_by_regex_case_insensitive() -> None:
    """Finder.find_sheet_by_regex: поиск без учёта регистра (как клеточные)."""
    finder = make_finder()

    sheet = finder.find_sheet_by_regex("^T$")

    assert sheet is not None
    assert sheet.name == "t"
