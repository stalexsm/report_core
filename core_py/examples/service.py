from report_core import Service
import time


class S10406(Service):
    def summary_0(self, sheets, **kwargs):
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

        print(sheet)

        cell = sheet.find_cell_by_letter("E5")
        if cell:
            print(cell.value)
            print(cell.formula)
            print(cell.style)
            print(cell.letter)
            print(cell.hidden_value)

        fmt = self._fmt_0(**kwargs)
        print(fmt)

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
    result = s.to_json()

    print(result)

    print(f"Выполнено за: {time.time() - start_time:.3f} сек.")


if __name__ == "__main__":
    main()
