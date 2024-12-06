from report_core import XLSXBook
import time


if __name__ == "__main__":
    start_time = time.time()

    book = XLSXBook()
    sheet = book.add_sheet("A", 5, 5)
    print("Sheet", sheet)

    sheet.write_cell(1, 1, "1000")

    book.get_sheet_name("A")
    print("Find Name Sheet", sheet)

    book.get_sheet_index(0)
    print("Find Idx Sheet", sheet)

    print(f"Выполнено за: {time.time() - start_time:.3f} сек.")
