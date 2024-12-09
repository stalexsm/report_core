import time
from uuid import UUID, uuid4

from report_core import HelperSheet, column_number_to_letter


# @dataclass
class SheetCell:
    id: UUID
    row: int
    column: int
    cell: str
    value: str | None = None
    data_type: str = "s"
    formula: str | None = None
    number_format: str = "Generate"
    cell_type: str | None = None
    el_type: str | None = None
    is_merge: bool = False
    start_column: int | None = None
    end_column: int | None = None
    start_row: int | None = None
    end_row: int | None = None
    style_id: str | None = None

    def __init__(
        self,
        row: int,
        col: int,
        cell: str,
        value: str | None = None,
    ):
        self.id = uuid4()
        self.row = row
        self.column = col
        self.value = value
        self.cell = cell


# @dataclass
class Sheet:
    id: int = 1
    name: str = "Тестовый лист"
    max_row: int = 50
    max_column: int = 10
    index: int = 1
    main: bool = False
    cells: list[SheetCell] = []

    def __init__(self, name: str, cells: list[SheetCell]):
        self.name = name
        self.cells = cells


if __name__ == "__main__":
    start_time = time.time()

    cells = []
    for row in range(1, 51):
        for col in range(1, 11):
            cell = f"{column_number_to_letter(col)}{row}"

            cells.append(
                SheetCell(
                    row,
                    col,
                    cell=cell,
                    value=f"Привет: {cell}. Строка: {row}",
                ),
            )

    sheet = Sheet(name="Тестовый лист", cells=cells)

    h = HelperSheet(sheets=[sheet])
    print(h)

    sheet = h.find_sheet_by_name("Тестовый лист")
    print(sheet)

    if sheet:
        for c in sheet.cells:
            print("Cell:", c.cell, "Value:", c.value)

    print(f"Выполнено за: {time.time() - start_time:.3f} сек.")
