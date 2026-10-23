"""
Сабмодуль для работы с готовыми XLSX-файлами (только чтение).
"""

from typing import SupportsIndex, final

from _typeshed import SupportsGetItem

__all__ = ["Finder", "ReadableCell", "ReadableSheet", "create_finder", "find_cell_by_coords", "find_cell_by_letter", "find_cell_by_regex", "find_cell_by_str", "find_cells_between_regex", "find_cells_by_regex", "find_cells_by_str", "find_cells_for_cols_by_regex", "find_cells_for_rows_by_regex", "find_cells_multi_regex", "find_cells_range_cols", "find_cells_range_rows", "find_value_by_coords", "find_values_by_col_rows", "find_values_by_row_cols"]

@final
class Finder:
    """
    Поисковик по листам готового XLSX-файла (только чтение).
    """
    def __new__(cls, /, sheets: list) -> Finder:
        """
        Инициализация поисковика по списку листов (dict или ReadableSheet).
        """
    def find_sheet_by_name(self, /, name: str) -> ReadableSheet | None:
        """
        Поиск листа по имени
        ---

        Arguments:
        ---
            name: str
                Имя листа

        Returns:
        ---
            ReadableSheet | None
        """
    def find_sheet_by_regex(self, /, pattern: str) -> ReadableSheet | None:
        """
        Поиск листа по регулярному выражению (без учёта регистра — как клеточные
        find_*_by_regex)
        ---

        Arguments:
        ---
            pattern: str
                Шаблон (регулярное выражение)

        Returns:
        ---
            ReadableSheet | None

        Raises:
        ---
            ValueError
                Если pattern — невалидное регулярное выражение (например, "(")
        """
    def get_sheet_index(self, /, idx: SupportsIndex) -> ReadableSheet | None:
        """
        Получение листа по индексу
        ---

        Arguments:
        ---
            idx: int
                Индекс листа

        Returns:
        ---
            ReadableSheet | None
        """
    def get_sheets_with_names(self, /, name_list: SupportsGetItem[int, str]) -> list[ReadableSheet]:
        """
        Получение листов из списка имён
        ---

        Arguments:
        ---
            name_list: list[str]
                Список имён

        Returns:
        ---
            list[ReadableSheet]
        """
    def get_sheets_without_names(self, /, name_list: SupportsGetItem[int, str]) -> list[ReadableSheet]:
        """
        Получение листов, которых нет в списке имён
        ---

        Arguments:
        ---
            name_list: list[str]
                Список имён

        Returns:
        ---
            list[ReadableSheet]
        """
    @property
    def sheets(self, /) -> list[ReadableSheet]:
        """
        Списки листов, по которым ищет поисковик
        """

@final
class ReadableCell:
    """
    Ячейка листа готового XLSX-файла (только чтение).
    """
    @property
    def column(self, /) -> int:
        """
        Колонка ячейки
        """
    @property
    def data_type(self, /) -> str:
        """
        Тип данных ячейки
        """
    @property
    def formula(self, /) -> str | None:
        """
        Формула ячейки
        """
    @property
    def is_formula(self, /) -> bool:
        """
        Метод для получения флага, ячейка с формулой или нет.
        ---

        Returns:
        ---
            bool
        """
    @property
    def is_value_bool(self, /) -> bool:
        """
        Проверить, является ли значение ячейки boolean
        ---

        Returns:
        ---
            bool
        """
    @property
    def is_value_datetime(self, /) -> bool:
        """
        Проверить, является ли значение ячейки datetime
        ---

        Returns:
        ---
            bool
        """
    @property
    def is_value_empty(self, /) -> bool:
        """
        Проверить, является ли значение ячейки empty
        ---

        Returns:
        ---
            bool
        """
    @property
    def is_value_integer(self, /) -> bool:
        """
        Проверить, является ли значение ячейки integer
        ---

        Returns:
        ---
            bool
        """
    @property
    def is_value_numeric(self, /) -> bool:
        """
        Проверить, является ли значение ячейки numeric
        ---

        Returns:
        ---
            bool
        """
    @property
    def letter(self, /) -> str:
        """
        Получение буквенной координаты
        ---

        Returns:
        ---
            str
        """
    @property
    def row(self, /) -> int:
        """
        Строка ячейки
        """
    @property
    def style(self, /) -> str | None:
        """
        Стиль ячейки
        """
    @property
    def value(self, /) -> str:
        """
        Значение ячейки
        """

@final
class ReadableSheet:
    """
    Лист готового XLSX-файла (только чтение).
    """
    @property
    def cells(self, /) -> list[ReadableCell]:
        """
        Ячейки листа
        ---

        Returns:
        ---
            list[ReadableCell]
        """
    def find_cell_by_coords(self, /, row: SupportsIndex, col: SupportsIndex) -> ReadableCell | None:
        """
        Функция для получения ячейки по координатам
        ---

        Arguments:
        ---
            row: int
                Строка ячейки
            col: int
                Колонка ячейки

        Returns:
        ---
            ReadableCell | None
        """
    def find_cell_by_letter(self, /, letter: str) -> ReadableCell | None:
        """
        Функция для получения ячейки по буквенной координате (A1).
        ---

        Arguments:
        ---
            letter: str
                Координата (буквенная) A1

        Returns:
        ---
            ReadableCell | None

        Raises:
        ---
            ValueError
                Если буква не координата ячейки (например, "1A") или за границами листа
        """
    def find_cell_by_regex(self, /, regex: str) -> ReadableCell | None:
        """
        Функция для получения ячейки по регулярному (шаблону) значению.
        ---

        Arguments:
        ---
            regex: str
                Шаблон (регулярное значение)

        Returns:
        ---
            ReadableCell | None
        """
    def find_cell_by_str(self, /, value: str) -> ReadableCell | None:
        """
        Функция для получения ячейки по строковому значению.
        ---

        Arguments:
        ---
            value: str
                Строка (значение)

        Returns:
        ---
            ReadableCell | None
        """
    def find_cells_between_regex(self, /, before_regex: str, after_regex: str) -> list[ReadableCell]:
        """
        Функция для получения ячеек по регулярным (шаблонам) значениям.
        Находит ячейки от первого шаблона до второго.
        ---

        Arguments:
        ---
            before_regex: str
                Первый шаблон (регулярное значение)
            after_regex: str
                Второй шаблон (регулярное значение)

        Returns:
        ---
            list[ReadableCell]
        """
    def find_cells_by_regex(self, /, regex: str) -> list[ReadableCell]:
        """
        Функция для получения ячеек по регулярному (шаблону) значению.
        ---

        Arguments:
        ---
            regex: str
                Шаблон (регулярное значение)

        Returns:
        ---
            list[ReadableCell]
        """
    def find_cells_by_str(self, /, value: str) -> list[ReadableCell]:
        """
        Функция для получения ячеек по строковому значению.
        ---

        Arguments:
        ---
            value: str
                Строка (значение)

        Returns:
        ---
            list[ReadableCell]
        """
    def find_cells_for_cols_by_regex(self, /, regex: str, row_stop: SupportsIndex) -> list[ReadableCell]:
        """
        Функция для получения ячеек по регулярному (шаблону) значению до определенной строки.
        ---

        Arguments:
        ---
            regex: str
                Шаблон (регулярное значение)
            row_stop: int
                Значение строки до которой забирать ячейки

        Returns:
        ---
            list[ReadableCell]
        """
    def find_cells_for_rows_by_regex(self, /, regex: str, col_stop: SupportsIndex) -> list[ReadableCell]:
        """
        Функция для получения ячеек по регулярному (шаблону) значению до определенной колонки.
        ---

        Arguments:
        ---
            regex: str
                Шаблон (регулярное значение)
            col_stop: int
                Значение колонки до которой забирать ячейки

        Returns:
        ---
            list[ReadableCell]
        """
    def find_cells_multi_regex(self, /, before_regex: str, after_regex: str) -> list[ReadableCell]:
        """
        Функция для получения ячеек по регулярным (шаблонам) значениям.
        Находит ячейки в соответствии шаблонов.
        ---

        Arguments:
        ---
            before_regex: str
                Первый шаблон (регулярное значение)
            after_regex: str
                Второй шаблон (регулярное значение)

        Returns:
        ---
            list[ReadableCell]
        """
    def find_cells_range_cols(self, /, start_col: SupportsIndex, end_col: SupportsIndex) -> list[ReadableCell]:
        """
        Функция для получения ячеек в диапазоне колонок.
        ---

        Arguments:
        ---
            start_col: int
                Стартовая колонка
            end_col: int
                Финишная колонка

        Returns:
        ---
            list[ReadableCell]
        """
    def find_cells_range_rows(self, /, start_row: SupportsIndex, end_row: SupportsIndex) -> list[ReadableCell]:
        """
        Функция для получения ячеек в диапазоне строк.
        ---

        Arguments:
        ---
            start_row: int
                Стартовая строка
            end_row: int
                Финишная строка

        Returns:
        ---
            list[ReadableCell]
        """
    def find_value_by_coords(self, /, row: SupportsIndex, col: SupportsIndex) -> str | None:
        """
        Функция для поиска значения ячейки по координатам
        ---

        Arguments:
        ---
            row: int
                Индекс строки
            col: int
                Индекс колонки

        Returns:
        ---
            str | None
        """
    def find_values_by_col_rows(self, /, col: SupportsIndex, rows: SupportsGetItem[int, SupportsIndex]) -> list[str]:
        """
        Функция для получения значений ячеек по колонке и списку строк.
        ---

        Значения идут в порядке запрошенных строк; отсутствующая ячейка даёт
        пустую строку — длина результата всегда равна длине запроса.

        Arguments:
        ---
            col: int
                Колонка
            rows: list[int]
                Список строк

        Returns:
        ---
            list[str]
        """
    def find_values_by_row_cols(self, /, row: SupportsIndex, cols: SupportsGetItem[int, SupportsIndex]) -> list[str]:
        """
        Функция для получения значений ячеек по строке и списку колонок.
        ---

        Значения идут в порядке запрошенных колонок; отсутствующая ячейка даёт
        пустую строку — длина результата всегда равна длине запроса.

        Arguments:
        ---
            row: int
                Строка
            cols: list[int]
                Список колонок

        Returns:
        ---
            list[str]
        """
    def get_cells_by_range(self, /, start_row: SupportsIndex | None = None, end_row: SupportsIndex | None = None, start_col: SupportsIndex | None = None, end_col: SupportsIndex | None = None) -> list[ReadableCell]:
        """
        Получить список всех ячеек в заданном диапазоне.
        ---

        Arguments:
        ---
            start_row: int | None
                Номер начальной строки
            end_row: int | None
                Номер конечной строки
            start_col: int | None
                Номер начальной колонки
            end_col: int | None
                Номер конечной колонки

        Returns:
        ---
            list[ReadableCell]
        """
    def get_value_cell(self, /, row: SupportsIndex, col: SupportsIndex) -> str:
        """
        Функция для получения значения ячейки.
        ---

        Arguments:
        ---
            row: int
                Строка ячейки
            col: int
                Колонка ячейки

        Returns:
        ---
            str
        """
    @property
    def max_column(self, /) -> int:
        """
        Получение максимальной колонки в книге
        ---

        Returns:
        ---
            int
        """
    @property
    def max_row(self, /) -> int:
        """
        Получение максимальной строки в книге
        ---

        Returns:
        ---
            int
        """
    @property
    def merge_cells(self, /) -> list[tuple[int, int, int, int]]:
        """
        Объединённые диапазоны ячеек
        ---

        Returns:
        ---
            list[tuple[int, int, int, int]]
        """
    @property
    def name(self, /) -> str:
        """
        Имя листа
        """
    @property
    def sheet_state(self, /) -> str:
        """
        Состояние и видимость листа
        """

def create_finder(sheets: list) -> Finder:
    """
    Создание Finder по списку листов (dict или ReadableSheet).
    ---

    Arguments:
    ---
        sheets: list
            Список листов

    Returns:
    ---
        Finder
    """

def find_cell_by_coords(row: SupportsIndex, col: SupportsIndex, cells: list[ReadableCell]) -> ReadableCell | None:
    """
    Поиск ячейки по координатам в списке ячеек
    ---

    Arguments:
    ---
        row: int
            Строка ячейки
        col: int
            Колонка ячейки
        cells: list[ReadableCell]
            Список ячеек

    Returns:
    ---
        ReadableCell | None
    """

def find_cell_by_letter(letter: str, cells: list[ReadableCell]) -> ReadableCell | None:
    """
    Поиск ячейки по буквенной координате (A1) в списке ячеек
    ---

    Arguments:
    ---
        letter: str
            Координата (буквенная) A1
        cells: list[ReadableCell]
            Список ячеек

    Returns:
    ---
        ReadableCell | None

    Raises:
    ---
        ValueError
            Если буква не координата ячейки (например, "1A") или за границами листа
    """

def find_cell_by_regex(regex: str, cells: list[ReadableCell]) -> ReadableCell | None:
    """
    Поиск ячейки по регулярному выражению в списке ячеек
    ---

    Arguments:
    ---
        regex: str
            Шаблон (регулярное выражение)
        cells: list[ReadableCell]
            Список ячеек

    Returns:
    ---
        ReadableCell | None
    """

def find_cell_by_str(value: str, cells: list[ReadableCell]) -> ReadableCell | None:
    """
    Поиск ячейки по строковому значению в списке ячеек
    ---

    Arguments:
    ---
        value: str
            Строка (значение)
        cells: list[ReadableCell]
            Список ячеек

    Returns:
    ---
        ReadableCell | None
    """

def find_cells_between_regex(before_regex: str, after_regex: str, cells: list[ReadableCell]) -> list[ReadableCell]:
    """
    Поиск ячеек от первого регулярного выражения до второго
    ---

    Arguments:
    ---
        before_regex: str
            Первый шаблон (регулярное выражение)
        after_regex: str
            Второй шаблон (регулярное выражение)
        cells: list[ReadableCell]
            Список ячеек

    Returns:
    ---
        list[ReadableCell]
    """

def find_cells_by_regex(regex: str, cells: list[ReadableCell]) -> list[ReadableCell]:
    """
    Поиск ячеек по регулярному выражению в списке ячеек
    ---

    Arguments:
    ---
        regex: str
            Шаблон (регулярное выражение)
        cells: list[ReadableCell]
            Список ячеек

    Returns:
    ---
        list[ReadableCell]
    """

def find_cells_by_str(value: str, cells: list[ReadableCell]) -> list[ReadableCell]:
    """
    Поиск ячеек по строковому значению в списке ячеек
    ---

    Arguments:
    ---
        value: str
            Строка (значение)
        cells: list[ReadableCell]
            Список ячеек

    Returns:
    ---
        list[ReadableCell]
    """

def find_cells_for_cols_by_regex(regex: str, row_stop: SupportsIndex, cells: list[ReadableCell]) -> list[ReadableCell]:
    """
    Поиск ячеек по регулярному выражению до заданной строки
    ---

    Arguments:
    ---
        regex: str
            Шаблон (регулярное выражение)
        row_stop: int
            Значение строки до которой забирать ячейки
        cells: list[ReadableCell]
            Список ячеек

    Returns:
    ---
        list[ReadableCell]
    """

def find_cells_for_rows_by_regex(regex: str, col_stop: SupportsIndex, cells: list[ReadableCell]) -> list[ReadableCell]:
    """
    Поиск ячеек по регулярному выражению до заданной колонки
    ---

    Arguments:
    ---
        regex: str
            Шаблон (регулярное выражение)
        col_stop: int
            Значение колонки до которой забирать ячейки
        cells: list[ReadableCell]
            Список ячеек

    Returns:
    ---
        list[ReadableCell]
    """

def find_cells_multi_regex(before_regex: str, after_regex: str, cells: list[ReadableCell]) -> list[ReadableCell]:
    """
    Поиск ячеек по двум регулярным выражениям в списке ячеек
    ---

    Arguments:
    ---
        before_regex: str
            Первый шаблон (регулярное выражение)
        after_regex: str
            Второй шаблон (регулярное выражение)
        cells: list[ReadableCell]
            Список ячеек

    Returns:
    ---
        list[ReadableCell]
    """

def find_cells_range_cols(start_col: SupportsIndex, end_col: SupportsIndex, cells: list[ReadableCell]) -> list[ReadableCell]:
    """
    Поиск ячеек в диапазоне колонок
    ---

    Arguments:
    ---
        start_col: int
            Стартовая колонка
        end_col: int
            Финишная колонка
        cells: list[ReadableCell]
            Список ячеек

    Returns:
    ---
        list[ReadableCell]
    """

def find_cells_range_rows(start_row: SupportsIndex, end_row: SupportsIndex, cells: list[ReadableCell]) -> list[ReadableCell]:
    """
    Поиск ячеек в диапазоне строк
    ---

    Arguments:
    ---
        start_row: int
            Стартовая строка
        end_row: int
            Финишная строка
        cells: list[ReadableCell]
            Список ячеек

    Returns:
    ---
        list[ReadableCell]
    """

def find_value_by_coords(row: SupportsIndex, col: SupportsIndex, cells: list[ReadableCell]) -> str | None:
    """
    Поиск значения ячейки по координатам в списке ячеек
    ---

    Arguments:
    ---
        row: int
            Строка ячейки
        col: int
            Колонка ячейки
        cells: list[ReadableCell]
            Список ячеек

    Returns:
    ---
        str | None
    """

def find_values_by_col_rows(col: SupportsIndex, rows: SupportsGetItem[int, SupportsIndex], cells: list[ReadableCell]) -> list[str]:
    """
    Поиск значений ячеек по колонке и списку строк
    ---

    Arguments:
    ---
        col: int
            Колонка
        rows: list[int]
            Список строк
        cells: list[ReadableCell]
            Список ячеек

    Returns:
    ---
        list[str]
    """

def find_values_by_row_cols(row: SupportsIndex, cols: SupportsGetItem[int, SupportsIndex], cells: list[ReadableCell]) -> list[str]:
    """
    Поиск значений ячеек по строке и списку колонок
    ---

    Arguments:
    ---
        row: int
            Строка
        cols: list[int]
            Список колонок
        cells: list[ReadableCell]
            Список ячеек

    Returns:
    ---
        list[str]
    """
