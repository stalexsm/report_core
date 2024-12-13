from report_core import XLSXBook


def main():
    book = XLSXBook()
    sheet = book.add_sheet("Sheet1")
    print(sheet)

    sheet.name = "A"

    sheet = book.get_sheet_index(0)
    print(sheet)

    print(book)

    for sheet_ in book.sheets:
        sheet_.name = "B"

    print(sheet)

    sheet.add_merge_cells(1, 2, 1, 2)
    print(sheet.merge_cells)

    print(book.to_json())


if __name__ == "__main__":
    main()
