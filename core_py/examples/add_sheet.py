import time

from report_core import XLSXBook

if __name__ == "__main__":
    start_time = time.time()

    book = XLSXBook()

    sheet = book.add_sheet("A", 100, 100)
    print("Sheet", sheet)

    print("Write Cell 150x150")
    cell = sheet.write_cell(150, 150, "Жопа")

    sheet.generate_empty_cells()

    cell.set_formula("SUM(A1:A10)")
    cell.set_style_id("Style: Percent")

    print("Write Cell 2x2")
    cell = sheet.write_cell(2, 2, "100")

    print(
        "Find Cell 2x2 value:",
        (cell.value, type(cell.value)) if cell else cell,
    )

    print("Write Cell 3x3")
    cell = sheet.write_cell(3, 3, "300.0")

    print(
        "Find Cell 3x3 value:",
        (cell.value, type(cell.value)) if cell else cell,
    )

    for cell in sheet.cells:
        if cell.row == 99 and cell.column == 99:
            cell.set_value("Yop! Жопа")
            cell.set_style_id("Style Yop! Жопа")

        if cell.row == 100 and cell.column == 100:
            cell.set_formula("=A1")

    cell = sheet.find_cell_by_coords(100, 100)
    print(
        "Find Cell 100x100 value:",
        cell.value if cell else cell,
        "formula:",
        cell.formula if cell else cell,
        "StyleID:",
        cell.style_id if cell else cell,
        "IF formula:",
        cell.is_formula() if cell else cell,
        "Data type:",
        cell.data_type if cell else cell,
    )

    cell = sheet.find_cell_by_coords(99, 99)
    print(
        "Find Cell 99x99 value:",
        cell.value if cell else cell,
        "StyleID:",
        cell.style_id if cell else cell,
    )

    cell = sheet.find_cell_by_coords(120, 120)
    print("Find Cell 120x120", cell)

    print(f"Выполнено за: {time.time() - start_time:.3f} сек.")
