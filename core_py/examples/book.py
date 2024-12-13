from report_core import XLSXBook
import time


def main():
    book = XLSXBook()
    sheet = book.add_sheet("Sheet1")
    print(sheet)

    start_time = time.time()

    for r in range(1, 11):
        for c in range(1, 11):
            cell = sheet.cell(r, c, f"Dynamic value: r{r}:c{c}")

            if r == 5 and c == 5:
                cell.value = "100"
                cell.formula = "SUM(A1:A5)"
                cell.style = "A"

    print(sheet)

    cell = sheet.find_cell_by_letter("E5")
    print(cell.value)
    print(cell.formula)
    print(cell.style)

    print(book.to_json())

    print(f"Выполнено за: {time.time() - start_time:.3f} сек.")


if __name__ == "__main__":
    main()
