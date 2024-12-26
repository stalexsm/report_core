from report_core import Service
from report_core.readable import Finder, find_cell_by_letter
import time

from uuid import UUID, uuid4


class Cell:
    id: UUID
    row: int
    column: int
    value: str | None = None
    data_type: str = "s"
    formula: str | None = None

    def __init__(
        self,
        row: int,
        col: int,
        value: str | None = None,
    ):
        self.id = uuid4()
        self.row = row
        self.column = col
        self.value = value


class Sheet:
    id: int = 1
    name: str = "Тестовый лист"
    sheet_state: str = "visible"
    merge_cells: list[list[int, int, int, int]] | None = None
    cells: list[Cell] = []

    def __init__(self, name: str, cells: list[Cell]):
        self.name = name
        self.cells = cells


class S10406(Service):
    def summary_0(self, sheets, **kwargs):
        f = Finder(sheets)

        sheet = f.find_sheet_by_name("Тестовый лист")
        print(sheet)

        if sheet:
            for c in sheet.cells:
                print(c)

            cell = sheet.find_cell_by_letter("A1")
            print("Find", cell)

            cell = sheet.find_cell_by_coords(1, 1)
            print("Find By Coords", cell)

            cell = find_cell_by_letter("A1", sheet.cells)
            print("Find With F", cell)

            cells = sheet.find_values_by_col_rows(1, [1, 2])
            print("Find Values By Col Rows", cells)

            cells = sheet.find_values_by_row_cols(1, [1, 2])
            print("Find Values By Row Cols", cells)

            print("Merge Cells", sheet.merge_cells)

        return "Summary"

    def _fmt_0(self, **kwargs):
        for s in self._sheets:
            print("Fmt --> ", s)

        return "Fmt"


def main():
    cells = []
    for row in range(1, 11):
        for col in range(1, 11):
            cells.append(
                Cell(
                    row,
                    col,
                    value=f"Привет: {row} {col}",
                ),
            )

    sheet = Sheet(name="Тестовый лист", cells=cells)
    sheet.merge_cells = [[1, 2, 1, 2]]

    start_time = time.time()
    s = S10406("uow")
    s.summary_0([sheet])

    print(f"Выполнено за: {time.time() - start_time:.3f} сек.")


if __name__ == "__main__":
    main()
