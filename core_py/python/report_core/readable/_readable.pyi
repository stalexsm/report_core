from typing import Any, Sequence

class Finder:
    sheets: Sequence[ReadableSheet]

    def __repr__(self) -> str: ...
    def __init__(self, sheets: Sequence[Any]) -> None: ...
    def get_sheet_index(self, idx: int) -> ReadableSheet | None: ...
    def get_sheets_without_names(
        self, name_list: list[str]
    ) -> Sequence[ReadableSheet]: ...
    def get_sheets_with_names(
        self, name_list: list[str]
    ) -> Sequence[ReadableSheet]: ...
    def find_sheet_by_name(self, name: str) -> ReadableSheet | None: ...
    def find_sheet_by_regex(self, pattern: str) -> ReadableSheet | None: ...

class ReadableSheet:
    name: str
    cells: Sequence[ReadableCell]
    merge_cells: Sequence[tuple[int, int, int, int]]

    def __repr__(self) -> str: ...
    @property
    def max_row(self) -> int: ...
    @property
    def max_column(self) -> int: ...
    def get_value_cell(self, row: int, column: int) -> str: ...
    def get_cells_by_range(
        self,
        start_row: int | None = None,
        end_row: int | None = None,
        start_column: int | None = None,
        end_column: int | None = None,
    ) -> Sequence[ReadableCell]: ...
    def find_cell_by_regex(self, regex: str) -> ReadableCell | None: ...
    def find_cell_by_coords(self, row: int, col: int) -> ReadableCell | None: ...
    def find_cell_by_letter(self, letter: str) -> ReadableCell | None: ...
    def find_cells_by_regex(self, regex: str) -> Sequence[ReadableCell]: ...
    def find_cells_for_rows_by_regex(
        self, regex: str, col_stop: int
    ) -> Sequence[ReadableCell]: ...
    def find_cells_for_cols_by_regex(
        self, regex: str, row_stop: int
    ) -> Sequence[ReadableCell]: ...
    def find_cells_multi_regex(
        self, before_regex: str, after_regex: str
    ) -> Sequence[ReadableCell]: ...
    def find_cells_between_regex(
        self, before_regex: str, after_regex: str
    ) -> Sequence[ReadableCell]: ...
    def find_cells_range_rows(
        self, start_row: int, end_row: int
    ) -> Sequence[ReadableCell]: ...
    def find_cells_range_cols(
        self, start_col: int, end_col: int
    ) -> Sequence[ReadableCell]: ...
    def find_values_by_col_rows(
        self, col: int, rows: Sequence[int]
    ) -> Sequence[str]: ...
    def find_values_by_row_cols(
        self, row: int, cols: Sequence[int]
    ) -> Sequence[str]: ...
    def find_value_by_coords(self, row: int, col: int) -> str | None: ...

class ReadableCell:
    value: str
    formula: str | None = None
    style: str | None = None
    data_type: str | None = None

    def __repr__(self) -> str: ...
    @property
    def row(self) -> int: ...
    @property
    def column(self) -> int: ...
    @property
    def letter(self) -> str: ...
    @property
    def is_formula(self) -> bool: ...
    @property
    def is_value_bool(self) -> bool: ...
    @property
    def is_value_numeric(self) -> bool: ...
    @property
    def is_value_integer(self) -> bool: ...
    @property
    def is_value_datetime(self) -> bool: ...
    @property
    def is_value_empty(self) -> bool: ...

def find_cell_by_coords(
    row: int, col: int, cells: Sequence[ReadableCell]
) -> ReadableCell | None: ...
def find_value_by_coords(
    row: int, col: int, cells: Sequence[ReadableCell]
) -> str | None: ...
def find_cell_by_regex(
    regex: str, cells: Sequence[ReadableCell]
) -> ReadableCell | None: ...
def find_cell_by_letter(
    letter: str, cells: Sequence[ReadableCell]
) -> ReadableCell | None: ...
def find_cells_by_regex(
    regex: str, cells: Sequence[ReadableCell]
) -> Sequence[ReadableCell]: ...
def find_cells_for_rows_by_regex(
    regex: str, col_stop: int, cells: Sequence[ReadableCell]
) -> Sequence[ReadableCell]: ...
def find_cells_for_cols_by_regex(
    regex: str, row_stop: int, cells: Sequence[ReadableCell]
) -> Sequence[ReadableCell]: ...
def find_cells_multi_regex(
    before_regex: str, after_regex: str, cells: Sequence[ReadableCell]
) -> Sequence[ReadableCell]: ...
def find_cells_between_regex(
    before_regex: str, after_regex: str, cells: Sequence[ReadableCell]
) -> Sequence[ReadableCell]: ...
def find_cells_range_rows(
    start_row: int, end_row: int, cells: Sequence[ReadableCell]
) -> Sequence[ReadableCell]: ...
def find_cells_range_cols(
    start_col: int, end_col: int, cells: Sequence[ReadableCell]
) -> Sequence[ReadableCell]: ...
def find_values_by_col_rows(
    col: int, rows: Sequence[int], cells: Sequence[ReadableCell]
) -> Sequence[str]: ...
def find_values_by_row_cols(
    row: int, cols: Sequence[int], cells: Sequence[ReadableCell]
) -> Sequence[str]: ...
