import time
from uuid import UUID, uuid4

from report_core import Service
from report_core.readable import create_finder, find_cell_by_letter


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
        data_type: str = "s",
    ):
        self.id = uuid4()
        self.row = row
        self.column = col
        self.value = value
        self.data_type = data_type


class Sheet:
    id: int = 1
    name: str = "Тестовый лист"
    sheet_state: str = "visible"
    merge_cells: list[list[int]] | None = None
    cells: list[Cell] = []

    def __init__(self, name: str, cells: list[Cell]):
        self.name = name
        self.cells = cells


class S10406(Service):
    def summary_0(self, sheets, **kwargs):
        f = create_finder(sheets)
        print("create_finder", f)

        sheet = f.find_sheet_by_name("Тестовый лист")
        print(sheet)

        if sheet:
            for c in sheet.cells:
                print(c)

            cell = sheet.find_cell_by_letter("A1")
            print("Find", cell)

            cell = sheet.find_cell_by_coords(1, 1)
            print("Find Cell By Coords", cell)

            value = sheet.find_value_by_coords(1, 1)
            print("Find Value By Coords", value)

            cell = find_cell_by_letter("A1", sheet.cells)
            print("Find With F", cell)

            cells = sheet.find_values_by_col_rows(1, [1, 2])
            print("Find Values By Col Rows", cells)

            cells = sheet.find_values_by_row_cols(1, [1, 2])
            print("Find Values By Row Cols", cells)

            print("Merge Cells", sheet.merge_cells)

            cell = sheet.find_cell_by_coords(2, 2)
            if cell:
                print("Value None", cell.value)
            cell = sheet.find_cell_by_coords(3, 3)
            if cell:
                print("Value str", cell.value)
            cell = sheet.find_cell_by_coords(4, 4)
            if cell:
                print("Value int", cell.value)

            cell = sheet.find_cell_by_str("099")
            if cell:
                print("Value Find By Str", cell.value)

            cells = sheet.find_cells_by_str("099")
            if cell:
                print("Value Find By Str", cells)

        return "Summary"

    def _fmt_0(self, **kwargs):
        for s in self._sheets:
            print("Fmt --> ", s)

        return "Fmt"


def main():
    cells = []
    for row in range(1, 11):
        for col in range(1, 11):
            if row == 2 and col == 2:
                cell = Cell(
                    row,
                    col,
                    value=None,
                )
            elif row == 3 and col == 3:
                cell = Cell(
                    row,
                    col,
                    value="099",
                )

            elif row == 4 and col == 4:
                cell = Cell(
                    row,
                    col,
                    value="099",
                    data_type="n",
                )
            else:
                cell = Cell(
                    row,
                    col,
                    value=f"Привет: {row} {col}",
                )

            cells.append(cell)

    sheet = Sheet(name="Тестовый лист", cells=cells)
    sheet.merge_cells = [[1, 2, 1, 2]]

    start_time = time.time()
    s = S10406("uow")
    s.summary_0([sheet])

    print(f"Выполнено за: {time.time() - start_time:.3f} сек.")


if __name__ == "__main__":
    main()
