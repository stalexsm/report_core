import time

from report_core import DEFAULT_COLUMN_WIDTH, DEFAULT_ROW_HEIGHT, Service


class S10406(Service):
    def summary_0(self, sheets, **kwargs):
        print("DEFAULT_COLUMN_WIDTH", DEFAULT_COLUMN_WIDTH)
        print("DEFAULT_ROW_HEIGHT", DEFAULT_ROW_HEIGHT)

        sheet = self._add_sheet("Sheet1")

        print(sheet)

        for r in range(1, 6):
            for c in range(1, 6):
                cell = sheet.cell(r, c, f"Dynamic value: r{r}:c{c}")

                if r == 5 and c == 5:
                    cell.value = "100"
                    cell.formula = "SUM(A1:A5)"
                    cell.style = "Style A"
                    cell.hidden_value = "Hidden"

        # Добавляем слияние ячеек
        sheet.add_merge_cells(1, 2, 1, 2)

        sheet.set_height_row(1, 75.0)
        sheet.set_width_column(1, 100.0)

        print(sheet)

        sheet.add_comment(1, 1, "Комментарий к ячейке", "Автор")
        for comment in sheet.comments:
            print("Comment:", comment.coordinate)
            print("Comment:", comment.text)
            print("Comment:", comment.author)
            print("Comment:", comment.row)
            print("Comment:", comment.column)

        cell = sheet.find_cell_by_letter("E5")
        if cell:
            print(cell.value)
            print(cell.formula)
            print(cell.style)
            print(cell.letter)
            print(cell.hidden_value)

        fmt = self._fmt_0(**kwargs)
        print(fmt)

        sheet2 = self._copy_sheet(sheet)
        print(sheet2)

        return "Summary"

    def _fmt_0(self, **kwargs):
        for s in self._sheets:
            print("Fmt --> ", s)

        return "Fmt"


def main():
    start_time = time.time()

    s = S10406("uow")
    s.summary_0([])

    print(s)

    result = s.to_dict()
    print(result)

    print(f"Выполнено за: {time.time() - start_time:.3f} сек.")


if __name__ == "__main__":
    main()
