from report_core import HelperSheet, column_number_to_letter
from uuid import uuid4


if __name__ == "__main__":
    sheet = {
        "id": 1,
        "name": "Тестовый лист",
        "max_row": 50,
        "max_column": 10,
        "index": 1,
        "main": False,
        "cells": [],
    }

    for row in range(1, 51):
        for col in range(1, 11):
            cell = f"{column_number_to_letter(col)}{row}"

            sheet["cells"].append(
                {
                    "id": uuid4(),
                    "row": row,
                    "column": col,
                    "cell": cell,
                    "value": f"Привет: {cell}. Строка: {row}",
                    "formula": None,
                    "data_type": "s",
                    "number_format": "Generate",
                    "cell_type": None,
                    "el_type": None,
                    "is_merge": False,
                    "start_column": None,
                    "end_column": None,
                    "start_row": None,
                    "end_row": None,
                    "style_id": None,
                },
            )

    h = HelperSheet(sheets=[sheet])
    sheet = h.find_sheet_by_name("Тестовый лист")
    print(sheet)

    if sheet:
        for c in sheet.cells:
            print("Cell:", c.cell, "Value:", c.value)
