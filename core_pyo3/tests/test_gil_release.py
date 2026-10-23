"""Освобождение GIL в листовых find_* и to_json/to_dict.

Регрессия к issue #25: методы find_* у WrapperSheet/ReadableSheet и
to_json/to_dict у Book/Service выполняли полный скан листа, не отпуская
GIL, — Python-потоки хоста простаивали всё время скана. Модульные функции
readable.* уже отпускают GIL; листовые методы обязаны вести себя так же.

Детектор: фоновый поток выполняет метод на листе ~1 000 000 ячеек, главный
поток в цикле считает миллисекундные витки Event.wait — каждый виток
требует GIL. Пока скан идёт без GIL, витки накапливаются всё его время;
если GIL удержан, главный поток получает управление один раз в самом
конце (единицы). Лёгкие в release-профиле сканы повторяются, чтобы
набрать достаточно витков; порог между режимами — десятки витков.

Продовые листы доходят до ~10 000 000 ячеек, но свойство «GIL отпущен»
от размера не зависит (скан линеен), поэтому тест гоняет миллион.

O(1)-поиски (find_cell_by_letter, find_cell_by_coords,
find_value_by_coords) лист не сканируют и детектором не проверяются —
py.detach им добавляется для единообразия.
"""

import threading

import pytest
import report_core
from report_core import readable

# ~1 000 000 ячеек на лист, как в больших продовых отчётах (там бывает
# и ~10 000 000, но свойство «GIL отпущен» от размера не зависит — скан
# линеен)
ROWS = 2000
COLS = 500

FILLER = "x"
NEEDLE = "needle"
NOMATCH = "zzz"

# Скан без GIL даёт по витку на миллисекунду, удержанный — единицы за
# весь прогон; порог посередине с запасом в обе стороны
MIN_WAKEUPS = 15

# Повторы для лёгких сканов: в release-профиле миллион ячеек сканируется
# за миллисекунды, поэтому вызов повторяется, пока не наберётся сигнал
REPEATS_SCAN = 30
REPEATS_VALUES = 10

# Объём запроса для find_values_by_*: 2 000 000 координат, чтобы выборка
# значений длилась заметно дольше порога детектора
LOOKUPS = 2_000_000


def gil_wakeups_during(action):
    """Витки ожидания GIL, успевшие в главном потоке за время action()."""
    started = threading.Event()
    done = threading.Event()
    wakeups = 0

    def worker():
        started.set()
        action()
        done.set()

    thread = threading.Thread(target=worker, name="gil-probe", daemon=True)
    thread.start()
    assert started.wait(timeout=30)

    while not done.wait(timeout=0.001):
        wakeups += 1

    thread.join(timeout=300)
    return wakeups


def fill_sheet(sheet):
    """Заполнить лист рутиной и поставить метку в дальний угол."""
    cell = sheet.cell
    for r in range(1, ROWS + 1):
        for c in range(1, COLS + 1):
            cell(r, c, FILLER)
    sheet.cell(ROWS, COLS - 1, NEEDLE)


@pytest.fixture(scope="module")
def big_book():
    """Книга с листом на ~1 000 000 ячеек и меткой в дальнем углу."""
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
    """ReadableSheet на ~1 000 000 ячеек с меткой в дальнем углу."""
    cells = [
        DuckCell(r, c, FILLER) for r in range(1, ROWS + 1) for c in range(1, COLS + 1)
    ]
    cells.append(DuckCell(ROWS, COLS - 1, NEEDLE))
    finder = readable.create_finder([DuckSheet(cells)])

    return finder.find_sheet_by_name("t")


@pytest.fixture(scope="module")
def service():
    """Сервис с книгой на ~1 000 000 ячеек и меткой в дальнем углу."""
    svc = report_core.Service(None)
    fill_sheet(svc._add_sheet("t"))

    return svc


# Полные сканы WrapperSheet: узкое окно диапазона не уменьшает стоимость —
# перед сканом сортируется вся коллекция листа. Каждая запись — (вызов,
# повторы): лёгкие в release-профиле сканы копят сигнал детектора.
SHEET_ACTIONS = {
    "find_cell_by_regex": (lambda sheet: sheet.find_cell_by_regex(NEEDLE), REPEATS_SCAN),
    "find_cell_by_str": (lambda sheet: sheet.find_cell_by_str(NEEDLE), REPEATS_SCAN),
    "find_cells_by_regex": (lambda sheet: sheet.find_cells_by_regex(NOMATCH), REPEATS_SCAN),
    "find_cells_by_str": (lambda sheet: sheet.find_cells_by_str(NOMATCH), REPEATS_SCAN),
    "find_cells_for_rows_by_regex": (
        lambda sheet: sheet.find_cells_for_rows_by_regex(NOMATCH, COLS),
        REPEATS_SCAN,
    ),
    "find_cells_for_cols_by_regex": (
        lambda sheet: sheet.find_cells_for_cols_by_regex(NOMATCH, ROWS),
        REPEATS_SCAN,
    ),
    "find_cells_multi_regex": (
        lambda sheet: sheet.find_cells_multi_regex(NOMATCH, NOMATCH),
        REPEATS_SCAN,
    ),
    "find_cells_between_regex": (
        lambda sheet: sheet.find_cells_between_regex(NOMATCH, NOMATCH),
        REPEATS_SCAN,
    ),
    "find_cells_range_rows": (lambda sheet: sheet.find_cells_range_rows(1, 2), REPEATS_SCAN),
    "find_cells_range_cols": (lambda sheet: sheet.find_cells_range_cols(1, 2), REPEATS_SCAN),
}

# Полные сканы ReadableSheet; find_values_by_* сканирует по списку координат
READABLE_ACTIONS = {
    "find_cell_by_regex": (lambda sheet: sheet.find_cell_by_regex(NEEDLE), REPEATS_SCAN),
    "find_cell_by_str": (lambda sheet: sheet.find_cell_by_str(NEEDLE), REPEATS_SCAN),
    "find_cells_by_regex": (lambda sheet: sheet.find_cells_by_regex(NOMATCH), REPEATS_SCAN),
    "find_cells_by_str": (lambda sheet: sheet.find_cells_by_str(NOMATCH), REPEATS_SCAN),
    "find_cells_for_rows_by_regex": (
        lambda sheet: sheet.find_cells_for_rows_by_regex(NOMATCH, COLS),
        REPEATS_SCAN,
    ),
    "find_cells_for_cols_by_regex": (
        lambda sheet: sheet.find_cells_for_cols_by_regex(NOMATCH, ROWS),
        REPEATS_SCAN,
    ),
    "find_cells_multi_regex": (
        lambda sheet: sheet.find_cells_multi_regex(NOMATCH, NOMATCH),
        REPEATS_SCAN,
    ),
    "find_cells_between_regex": (
        lambda sheet: sheet.find_cells_between_regex(NOMATCH, NOMATCH),
        REPEATS_SCAN,
    ),
    "find_cells_range_rows": (lambda sheet: sheet.find_cells_range_rows(1, 2), REPEATS_SCAN),
    "find_cells_range_cols": (lambda sheet: sheet.find_cells_range_cols(1, 2), REPEATS_SCAN),
    "find_values_by_col_rows": (
        lambda sheet: sheet.find_values_by_col_rows(
            COLS - 1, [r % ROWS + 1 for r in range(LOOKUPS)]
        ),
        REPEATS_VALUES,
    ),
    "find_values_by_row_cols": (
        lambda sheet: sheet.find_values_by_row_cols(
            ROWS, [c % COLS + 1 for c in range(LOOKUPS)]
        ),
        REPEATS_VALUES,
    ),
}

# Сериализация: тяжёлая вычислительная часть — serde-обход всей книги,
# одного прохода достаточно
BOOK_JSON_ACTIONS = {
    "to_json": (lambda book: book.to_json(), 1),
    "to_dict": (lambda book: book.to_dict(), 1),
}

SERVICE_JSON_ACTIONS = {
    "to_json": (lambda svc: svc.to_json(), 1),
    "to_dict": (lambda svc: svc.to_dict(), 1),
}


def repeated_action(actions, name, target):
    """Замыкание: repeats-кратный вызов метода над target."""
    action, repeats = actions[name]

    return lambda: [action(target) for _ in range(repeats)]


@pytest.mark.parametrize("name", sorted(SHEET_ACTIONS))
def test_sheet_find_releases_gil(big_sheet, name):
    """find_* у WrapperSheet: скан листа идёт с отпущенным GIL."""
    wakeups = gil_wakeups_during(repeated_action(SHEET_ACTIONS, name, big_sheet))

    assert wakeups >= MIN_WAKEUPS, (
        f"{name}: GIL удержан весь скан ({wakeups} витков вместо >= {MIN_WAKEUPS})"
    )


@pytest.mark.parametrize("name", sorted(READABLE_ACTIONS))
def test_readable_sheet_find_releases_gil(readable_sheet, name):
    """find_* у ReadableSheet: скан листа идёт с отпущенным GIL."""
    wakeups = gil_wakeups_during(repeated_action(READABLE_ACTIONS, name, readable_sheet))

    assert wakeups >= MIN_WAKEUPS, (
        f"{name}: GIL удержан весь скан ({wakeups} витков вместо >= {MIN_WAKEUPS})"
    )


@pytest.mark.parametrize("name", sorted(BOOK_JSON_ACTIONS))
def test_book_to_json_releases_gil(big_book, name):
    """to_json/to_dict у Book: обход книги идёт с отпущенным GIL."""
    wakeups = gil_wakeups_during(repeated_action(BOOK_JSON_ACTIONS, name, big_book))

    assert wakeups >= MIN_WAKEUPS, (
        f"{name}: GIL удержан весь обход ({wakeups} витков вместо >= {MIN_WAKEUPS})"
    )


@pytest.mark.parametrize("name", sorted(SERVICE_JSON_ACTIONS))
def test_service_to_json_releases_gil(service, name):
    """to_json/to_dict у Service: обход книги идёт с отпущенным GIL."""
    wakeups = gil_wakeups_during(repeated_action(SERVICE_JSON_ACTIONS, name, service))

    assert wakeups >= MIN_WAKEUPS, (
        f"{name}: GIL удержан весь обход ({wakeups} витков вместо >= {MIN_WAKEUPS})"
    )
