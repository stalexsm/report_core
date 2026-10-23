"""Освобождение GIL в листовых find_* и to_json/to_dict.

Регрессия к issue #25: методы find_* у WrapperSheet/ReadableSheet и
to_json/to_dict у Book/Service выполняли полный скан листа, не отпуская
GIL, — Python-потоки хоста простаивали всё время скана. Модульные функции
readable.* уже отпускают GIL; листовые методы обязаны вести себя так же.

Детектор: фоновый поток повторяет метод до истечения мерного окна,
главный поток в цикле считает витки Event.wait — каждый виток требует
GIL. Виток срабатывает раз в WAKEUP_STEP, но для возврата из wait нужен
GIL: пока сканы идут без GIL, витки накапливаются всё время окна; если
GIL удержан недетачнутым вызовом, главный поток спит в wait до его
конца, и витков — единицы-десятки.

Тайминг критичен: один вызов должен занимать не меньше нескольких
WAKEUP_STEP, иначе главный поток получает GIL в паузах между вызовами,
и режимы неразличимы. Лист ~100 000 ячеек — минимальный, при котором
самое короткое замеряемое действие (сортировка перед range-сканом)
длится миллисекунды; больше не нужно — скан линеен. find_values_by_*
растягивается числом координат LOOKUPS.

У сериализации свой порог: в to_dict после детачнутой serde-фазы идёт
json.loads под GIL, поэтому даже в норме витков набирался меньше,
чем у чистых сканов; порог ниже, но всё ещё на порядок выше, чем даёт
удержанный GIL.

O(1)-поиски (find_cell_by_letter, find_cell_by_coords,
find_value_by_coords) лист не сканируют и детектором не проверяются —
py.detach им добавляется для единообразия.
"""

import threading
import time

import pytest
import report_core
from report_core import readable

# Лист ~100 000 ячеек с меткой в дальнем углу: минимальный размер, при
# котором каждый замеряемый вызов длится не меньше миллисекунды
ROWS = 500
COLS = 200

FILLER = "x"
NEEDLE = "needle"
NOMATCH = "zzz"

# Мерное окно и шаг витков главного потока. При отпущенном GIL виток
# срабатывает каждые WAKEUP_STEP (~1-1.5 тыс. за окно); при удержанном
# главный спит в wait до конца недетачнутого вызова (для вызова в C
# миллисекунд — 0.1 / (0.05 + C/2) тыс. витков). Порог — посередине с
# запасом в обе стороны.
MEASURE_SECONDS = 0.1
WAKEUP_STEP = 0.00005
MIN_WAKEUPS = 400

# Порог сериализации: json.loads в to_dict идёт под GIL, поэтому норма
# ниже (~250-320 витков); удержанный GIL даёт единицы
MIN_WAKEUPS_JSON = 150

# Пробы на измерение: планировочный сбой (главному потоку разово досталось
# меньше витков) — единичен, удержанный GIL даёт мало витков в каждой
# пробе, поэтому берём максимум; при наборе порога дальше не идём
PROBES = 3

# Таймауты жизненного цикла пробного потока
STARTED_TIMEOUT_SECONDS = 30
JOIN_TIMEOUT_SECONDS = 300

# Объём запроса для find_values_by_*: по числу ячеек листа — чтобы выборка
# значений длилась миллисекунды, а не микросекунды
LOOKUPS = ROWS * COLS


def gil_wakeups_during(action, min_wakeups=MIN_WAKEUPS):
    """Витки ожидания GIL в главном потоке, пока фоновый поток гоняет action()."""
    best = 0

    for _ in range(PROBES):
        started = threading.Event()
        done = threading.Event()
        wakeups = 0

        def worker(action=action, started=started, done=done):
            started.set()
            deadline = time.monotonic() + MEASURE_SECONDS
            while time.monotonic() < deadline:
                action()
            done.set()

        thread = threading.Thread(target=worker, name="gil-probe", daemon=True)
        thread.start()
        assert started.wait(timeout=STARTED_TIMEOUT_SECONDS)

        while not done.wait(timeout=WAKEUP_STEP):
            wakeups += 1

        thread.join(timeout=JOIN_TIMEOUT_SECONDS)
        best = max(best, wakeups)

        if best >= min_wakeups:
            break

    return best


def fill_sheet(sheet, rows=ROWS, cols=COLS):
    """Заполнить лист рутиной и поставить метку в дальний угол."""
    cell = sheet.cell
    for r in range(1, rows + 1):
        for c in range(1, cols + 1):
            cell(r, c, FILLER)
    sheet.cell(rows, cols - 1, NEEDLE)


@pytest.fixture(scope="module")
def big_book():
    """Книга с листом на ~100 000 ячеек и меткой в дальнем углу."""
    book = report_core.Book()
    fill_sheet(book.add_sheet("t"))

    return book


@pytest.fixture(scope="module")
def big_sheet(big_book):
    return big_book.sheets[0]


class DuckCell:
    """Минимальный duck-typed ReadableCell для создания ReadableSheet."""

    __slots__ = ("column", "row", "value")

    data_type = "s"
    formula = None
    style_id = ""

    def __init__(self, row, column, value) -> None:
        self.row = row
        self.column = column
        self.value = value


class DuckSheet:
    """Минимальный duck-typed лист для ReadableSheet."""

    name = "t"
    sheet_state = "visible"
    merge_cells = None

    def __init__(self, cells) -> None:
        self.cells = cells


@pytest.fixture(scope="module")
def readable_sheet():
    """ReadableSheet на ~100 000 ячеек с меткой в дальнем углу."""
    cells = [
        DuckCell(r, c, FILLER) for r in range(1, ROWS + 1) for c in range(1, COLS + 1)
    ]
    cells.append(DuckCell(ROWS, COLS - 1, NEEDLE))
    finder = readable.create_finder([DuckSheet(cells)])

    return finder.find_sheet_by_name("t")


@pytest.fixture(scope="module")
def service():
    """Сервис с книгой на ~100 000 ячеек и меткой в дальнем углу."""
    svc = report_core.Service(None)
    fill_sheet(svc._add_sheet("t"))

    return svc


# Полные сканы WrapperSheet: узкое окно диапазона не уменьшает стоимость —
# перед сканом сортируется вся коллекция листа
SHEET_ACTIONS = {
    "find_cell_by_regex": lambda sheet: sheet.find_cell_by_regex(NEEDLE),
    "find_cell_by_str": lambda sheet: sheet.find_cell_by_str(NEEDLE),
    "find_cells_by_regex": lambda sheet: sheet.find_cells_by_regex(NOMATCH),
    "find_cells_by_str": lambda sheet: sheet.find_cells_by_str(NOMATCH),
    "find_cells_for_rows_by_regex": lambda sheet: sheet.find_cells_for_rows_by_regex(
        NOMATCH, COLS
    ),
    "find_cells_for_cols_by_regex": lambda sheet: sheet.find_cells_for_cols_by_regex(
        NOMATCH, ROWS
    ),
    "find_cells_multi_regex": lambda sheet: sheet.find_cells_multi_regex(NOMATCH, NOMATCH),
    "find_cells_between_regex": lambda sheet: sheet.find_cells_between_regex(
        NOMATCH, NOMATCH
    ),
    "find_cells_range_rows": lambda sheet: sheet.find_cells_range_rows(1, 2),
    "find_cells_range_cols": lambda sheet: sheet.find_cells_range_cols(1, 2),
}

# Полные сканы ReadableSheet; find_values_by_* сканирует по списку координат
READABLE_ACTIONS = {
    "find_cell_by_regex": lambda sheet: sheet.find_cell_by_regex(NEEDLE),
    "find_cell_by_str": lambda sheet: sheet.find_cell_by_str(NEEDLE),
    "find_cells_by_regex": lambda sheet: sheet.find_cells_by_regex(NOMATCH),
    "find_cells_by_str": lambda sheet: sheet.find_cells_by_str(NOMATCH),
    "find_cells_for_rows_by_regex": lambda sheet: sheet.find_cells_for_rows_by_regex(
        NOMATCH, COLS
    ),
    "find_cells_for_cols_by_regex": lambda sheet: sheet.find_cells_for_cols_by_regex(
        NOMATCH, ROWS
    ),
    "find_cells_multi_regex": lambda sheet: sheet.find_cells_multi_regex(NOMATCH, NOMATCH),
    "find_cells_between_regex": lambda sheet: sheet.find_cells_between_regex(
        NOMATCH, NOMATCH
    ),
    "find_cells_range_rows": lambda sheet: sheet.find_cells_range_rows(1, 2),
    "find_cells_range_cols": lambda sheet: sheet.find_cells_range_cols(1, 2),
    "find_values_by_col_rows": lambda sheet: sheet.find_values_by_col_rows(
        COLS - 1, [r % ROWS + 1 for r in range(LOOKUPS)]
    ),
    "find_values_by_row_cols": lambda sheet: sheet.find_values_by_row_cols(
        ROWS, [c % COLS + 1 for c in range(LOOKUPS)]
    ),
}

# Сериализация: serde-обход всей книги
BOOK_JSON_ACTIONS = {
    "to_json": lambda book: book.to_json(),
    "to_dict": lambda book: book.to_dict(),
}

SERVICE_JSON_ACTIONS = {
    "to_json": lambda svc: svc.to_json(),
    "to_dict": lambda svc: svc.to_dict(),
}


@pytest.mark.parametrize("name", sorted(SHEET_ACTIONS))
def test_sheet_find_releases_gil(big_sheet, name):
    """find_* у WrapperSheet: скан листа идёт с отпущенным GIL."""
    wakeups = gil_wakeups_during(lambda: SHEET_ACTIONS[name](big_sheet))

    assert wakeups >= MIN_WAKEUPS, (
        f"{name}: GIL удержан весь скан ({wakeups} витков вместо >= {MIN_WAKEUPS})"
    )


@pytest.mark.parametrize("name", sorted(READABLE_ACTIONS))
def test_readable_sheet_find_releases_gil(readable_sheet, name):
    """find_* у ReadableSheet: скан листа идёт с отпущенным GIL."""
    wakeups = gil_wakeups_during(lambda: READABLE_ACTIONS[name](readable_sheet))

    assert wakeups >= MIN_WAKEUPS, (
        f"{name}: GIL удержан весь скан ({wakeups} витков вместо >= {MIN_WAKEUPS})"
    )


@pytest.mark.parametrize("name", sorted(BOOK_JSON_ACTIONS))
def test_book_to_json_releases_gil(big_book, name):
    """to_json/to_dict у Book: обход книги идёт с отпущенным GIL."""
    wakeups = gil_wakeups_during(
        lambda: BOOK_JSON_ACTIONS[name](big_book), min_wakeups=MIN_WAKEUPS_JSON
    )

    assert wakeups >= MIN_WAKEUPS_JSON, (
        f"{name}: GIL удержан весь обход ({wakeups} витков вместо >= {MIN_WAKEUPS_JSON})"
    )


@pytest.mark.parametrize("name", sorted(SERVICE_JSON_ACTIONS))
def test_service_to_json_releases_gil(service, name):
    """to_json/to_dict у Service: обход книги идёт с отпущенным GIL."""
    wakeups = gil_wakeups_during(
        lambda: SERVICE_JSON_ACTIONS[name](service), min_wakeups=MIN_WAKEUPS_JSON
    )

    assert wakeups >= MIN_WAKEUPS_JSON, (
        f"{name}: GIL удержан весь обход ({wakeups} витков вместо >= {MIN_WAKEUPS_JSON})"
    )
