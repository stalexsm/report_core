from report_core import XLSXBook
import time


def main():
    book = XLSXBook()
    sheet = book.add_sheet("Sheet1")
    print(sheet)

    start_time = time.time()

    for r in range(1, 11):
        for c in range(1, 11):
            sheet.cell(r, c, f"Dynamic value: r{r}:c{c}")

    print(sheet)

    print(book.to_json())

    print(f"Выполнено за: {time.time() - start_time:.3f} сек.")


if __name__ == "__main__":
    main()
