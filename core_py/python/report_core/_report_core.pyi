from abc import ABC, abstractmethod
from datetime import datetime
from typing import Any, Sequence, Self, final, Literal

class XLSXBook:
    sheets: Sequence[XLSXSheet]

    def __init__(self) -> None:
        """
        Инициализация Книги
        --------------------

        Example:
        --------
        .. code-block:: python3

            book = XLSXBook()
        """
        ...

    def add_sheet(
        self,
        name: str,
        rows: int | None = None,
        cols: int | None = None,
    ) -> XLSXSheet:
        """
        Добавление листа в книгу
        ------------------------

        Arguments:
        ---------
            name: str
                Имя листа
            rows: int | None
                Количество строк
            cols: int | None Количество колонок

        Returns:
        --------
            XLSXSheet
        """

    def get_sheet_index(self, idx: int) -> XLSXSheet | None:
        """
        Получение листа по индексу
        --------------------------

        Arguments:
        ---------
            idx: int
                Индекс листа

        Returns:
        --------
            XLSXSheet | None

        """

    def get_sheet_name(self, name: str) -> XLSXSheet | None:
        """
        Получение листа по названию
        ---------------------------

        Arguments:
        ---------
            name: str
                Имя листа

        Returns:
        --------
            XLSXSheet | None
        """

    def to_json(self) -> str:
        """
        Преобразование книги в json
        ---------------------------

        Returns:
        --------
            str
        """

    def to_dict(self) -> dict[str, Any]:
        """
        Преобразование книги в dict
        ---------------------------

        Returns:
        --------
            Dict[str, Any]
        """

class XLSXSheet:
    """Тип данных листа с которыми работает парсер."""

    name: str
    max_row: int
    max_column: int
    index: int
    cells: Sequence[XLSXSheetCell]

    def __init__(
        self,
        name: str,
        index: int,
        rows: int = 50,
        cols: int = 30,
    ) -> None:
        """
        Инициализация
        -------------

        Arguments:
        ---------
            name: str
                Имя листа
            index: int
                Индекс листа
            rows: int
                Количество строк
            cols: int
                Количество колоно
        """

    @final
    def find_cell_by_cell(self, cell: str) -> XLSXSheetCell | None:
        """
        Функция для получения ячейки по cell (A1).
        ------------------------------------------

        Arguments:
        ---------
            cell: str
                Ячейка в формате A1

        Returns:
        --------
            XLSXSheetCell | None
        """

    @final
    def find_cell_by_coords(self, row: int, col: int) -> XLSXSheetCell | None:
        """
        Функция для ячейки по координатам.
        ---------------------------------

        Arguments:
        ---------
            row: int
                Номер строки
            col: int
                Номер колонки

        Returns:
        --------
            XLSXSheetCell | None

        """

    @final
    def write_cell(self, row: int, col: int, value: str) -> XLSXSheetCell:
        """
        Добавление ячейки в лист с заданным значением
        ------------------------

        Arguments:
        ---------
            row: int
                Номер строки
            col: int
                Номер колонки
            value: str
                Значение ячейки

        Returns:
        --------
            XLSXSheetCell

        """

    @final
    def delete_cols(self, idx: int, cols: int) -> None:
        """
        Метод удаления колонок
        ----------------------

        Arguments:
        ---------
            idx: int
                Номер колонки
            cols: int
                Количество колонок
        """

    @final
    def delete_rows(self, idx: int, rows: int) -> None:
        """
        Метод удаления строк
        --------------------

        Arguments:
        ---------
            idx: int
                Номер строки
            rows: int
                Количество строк

        """

    @final
    def set_merged_cells(
        self,
        start_row: int,
        end_row: int,
        start_column: int,
        end_column: int,
    ) -> None:
        """
        Метод для добавления данных по объединению ячеек.
        -------------------------------------------------

        Arguments:
        ---------
            start_row: int
                Номер начальной строки
            end_row: int
                Номер конечной строки
            start_column: int
                Номер начальной колонки
            end_column: int
                Номер конечной колонки

        """

    @final
    def generate_empty_cells(self) -> None:
        """Метод для генерации пустых ячеек."""

    @final
    def iter_cells(
        self,
        min_row: int | None = None,
        max_row: int | None = None,
        min_col: int | None = None,
        max_col: int | None = None,
    ) -> Sequence[XLSXSheetCell]:
        """
        Получить список всех ячеек в заданном диапазоне.
        ------------------------------------------------

        Arguments:
        ---------
            min_row: int | None
                Номер начальной строки
            max_row: int | None
                Номер конечной строки
            min_col: int | None
                Номер начальной колонки
            max_col: int | None
                Номер конечной колонки

        Returns:
        --------
            Sequence[XLSXSheetCell]

        """

class XLSXSheetCell:
    """Тип данных ячеек листа с которыми работает парсер."""

    row: int
    column: int
    cell: str
    value: Any | None
    formula: str | None
    data_type: str
    number_format: str
    is_merge: bool
    start_column: int | None
    end_column: int | None
    start_row: int | None
    end_row: int | None
    style_id: str | None
    hidden_value: str | None
    comment: str | None

    def __init__(
        self,
        sheet: XLSXSheet,
        row: int,
        column: int,
        value: str | None,
    ) -> None:
        """
        Инициализация
        -------------

        Arguments:
        ---------
            sheet: XLSXSheet
                Лист в который находится ячейка
            row: int
                Номер строки
            column: int
                Номер колонки
            value: str | None
                Значение ячейки

        """

    @final
    def set_value(self, value: str) -> None:
        """
        Метод для добавления значения ячейки.
        ------------------------------------

        Arguments:
        ---------
            value: str
                Значение ячейки
        """

    @final
    def set_hidden_value(self, value: str) -> None:
        """
        Метод для добавления скрытого значения ячейки.
        ---------------------------------------------

        Arguments:
        ---------
            value: str
                Значение ячейки

        """

    @final
    def set_comment(self, value: str) -> None:
        """
        Метод для добавления комментария ячейки.
        ----------------------------------------

        Arguments:
        ---------
            value: str
                Комментарий ячейки

        """

    @final
    def set_value_number(self, value: float) -> None:
        """
        Метод для добавления значения ячейки Numbers.
        ---------------------------------------------

        Arguments:
        ---------
            value: float
                Значение ячейки

        """

    @final
    def set_value_bool(self, value: bool) -> None:
        """
        Метод для добавления значения ячейки Bool.
        ------------------------------------------
        Arguments:
        ---------
            value: bool
                Значение ячейки
        """

    @final
    def set_value_str(self, value: str) -> None:
        """
        Метод для добавления значения ячейки String.
        --------------------------------------------

        Arguments:
        ---------
            value: str
                Значение ячейки

        """

    @final
    def set_value_datetime(self, value: datetime) -> None:
        """
        Метод для добавления значения ячейки Datetime.
        ----------------------------------------------
        Arguments:
        ---------
            value: datetime
                Значение ячейки

        """

    @final
    def set_formula(self, value: str) -> None:
        """
        Метод для добавления формулы ячейки String.
        -------------------------------------------

        Arguments:
        ---------
            value: str
                Значение ячейки

        """

    @final
    def set_data_type(self, value: Literal["s", "n", "d", "b"]) -> None:
        """
        Метод для добавления значения ячейки data_type.
        ----------------------------------------------
        Arguments:
        ---------
            value: Literal["s", "n", "d", "b"]
                Тип ячейки

        """

    @final
    def set_number_format(self, value: str) -> None:
        """
        Метод для добавления значения ячейки number_format.
        --------------------------------------------------
        Arguments:
        ---------
            value: str
                Формат ячейки

        """

    @final
    @property
    def is_formula(self) -> bool:
        """
        Метод для получения флага, ячейка с формулой или нет.
        ----------------------------------------------------

        Arguments:
        ---------
            value: str
                Формат ячейки

        """

    @final
    @property
    def is_value_bool(self) -> bool:
        """
        Проверить, является ли значение ячейки boolean
        ----------------------------------------------

        Returns:
        --------
            bool
        """

    @final
    @property
    def is_value_numeric(self) -> bool:
        """
        Проверить, является ли значение ячейки numeric
        ----------------------------------------------

        Returns:
        --------
            bool

        """

    @final
    @property
    def is_value_datetime(self) -> bool:
        """
        Проверить, является ли значение ячейки datetime
        ----------------------------------------------

        Returns:
        --------
            bool

        """

    @final
    @property
    def is_value_empty(self) -> bool:
        """
        Проверить, является ли значение ячейки empty
        ----------------------------------------------

        Returns:
        --------
            bool

        """

    @final
    def set_style_id(self, value: str) -> None:
        """
        Метод для добавления стиля к ячейки
        ----------------------------------------------

        Returns:
        --------
            bool
        """

class XLSXSheetRead:
    """Тип данных листа с которыми работает парсер."""

    name: str
    max_row: int
    max_column: int
    index: int
    cells: Sequence[XLSXSheetCellRead]

    @final
    def find_cell_by_pattern_regex(self, pattern: str) -> XLSXSheetCellRead | None:
        """
        Функция для поиска ячейки при помощи регулярных выражений.
        ----------------------------------------------------------
        Arguments:
        ---------
            pattern: str
                Паттерн для поиска
        Returns:
        --------
            XLSXSheetCellRead | None

        """

    @final
    def find_cells_by_pattern_regex(self, pattern: str) -> Sequence[XLSXSheetCellRead]:
        """
        Функция для поиска ячеек при помощи регулярных выражений.
        ---------------------------------------------------------

        Arguments:
        ---------
            pattern: str
                Паттерн для поиска
        Returns:
        --------
            Sequence[XLSXSheetCellRead]

        """

    @final
    def find_cells_for_rows_pattern_regex(
        self, pattern: str, column_stop: int | None = None
    ) -> Sequence[XLSXSheetCellRead]:
        """
        Функция поиска ячейеек колонок для строк которые соответствуют патерну.
        -----------------------------------------------------------------------

        Arguments:
        ---------
            pattern: str
                Паттерн для поиска
            column_stop: int | None
                Индекс колонки для остановки поиска
        Returns:
        --------
            Sequence[XLSXSheetCellRead]

        """

    @final
    def find_cells_for_cols_pattern_regex(
        self, pattern: str, row_stop: int | None = None
    ) -> Sequence[XLSXSheetCellRead]:
        """
        Функция поиска ячейеек строк для колонок которые соответствуют патерну.
        -----------------------------------------------------------------------

        Arguments:
        ---------
            pattern: str
                Паттерн для поиска
            row_stop: int | None
                Индекс строки для остановки поиска

        Returns:
        --------
            Sequence[XLSXSheetCellRead]

        """

    @final
    def find_cells_multi_pattern_regex(
        self,
        pattern_1: str,
        pattern_2: str,
    ) -> Sequence[XLSXSheetCellRead]:
        """
        Функция для поиска ячеек при помощи регулярных выражений по двум паттернам.
        --------------------------------------------------------------------------

        Arguments:
        ---------
            pattern_1: str
                Паттерн для поиска
            pattern_2: str
                Паттерн для поиска
        Returns:
        --------
            Sequence[XLSXSheetCellRead]

        """

    @final
    def find_cell_by_cell(self, cell: str) -> XLSXSheetCellRead | None:
        """
        Функция для получения ячейки по cell (A1).
        ------------------------------------------

        Arguments:
        ---------
            cell: str
                Ячейка в формате A1
        Returns:
        --------
            XLSXSheetCellRead | None

        """

    @final
    def find_cell_by_coords(self, row: int, col: int) -> XLSXSheetCellRead | None:
        """
        Функция для ячейки по координатам.
        ---------------------------------

        Arguments:
        ---------
            row: int
                Номер строки
            col: int
                Номер колонки
        Returns:
        --------
            XLSXSheetCellRead | None

        """

    @final
    def find_cells_between_patterns(
        self,
        pattern_after: str,
        pattern_before: str,
    ) -> Sequence[XLSXSheetCellRead]:
        """
        Метод ищет ячейки между двумя патернами.
        ----------------------------------------

        Arguments:
        ---------
            pattern_after: str
                Паттерн для начала поиска
            pattern_before: str
                Паттерн для окончания поиска
        Returns:
        --------
            Sequence[XLSXSheetCellRead]

        """

    @final
    def find_cells_by_range_rows(
        self,
        start_row: int,
        end_row: int,
    ) -> Sequence[XLSXSheetCellRead]:
        """
        Возвращаем все ячейки, которые находятся в диапазоне строк
        ----------------------------------------------------------

        Arguments:
        ---------
            start_row: int
                Номер начальной строки
            end_row: int
                Номер конечной строки

        Returns:
        --------
            Sequence[XLSXSheetCellRead]

        """

    @final
    def find_cells_by_range_cols(
        self,
        start_col: int,
        end_col: int,
    ) -> Sequence[XLSXSheetCellRead]:
        """
        Возвращаем все ячейки, которые находятся в диапазоне колонок
        ------------------------------------------------------------

        Arguments:
        ---------
            start_col: int
                Номер начальной колонки
            end_col: int
                Номер конечной колонки

        Returns:
        --------
            Sequence[XLSXSheetCellRead]

        """

    @final
    def iter_cells(
        self,
        min_row: int | None = None,
        max_row: int | None = None,
        min_col: int | None = None,
        max_col: int | None = None,
    ) -> Sequence[XLSXSheetCellRead]:
        """
        Получить список всех ячеек в заданном диапазоне.
        -----------------------------------------------

        Arguments:
        ---------
            min_row: int | None
                Номер начальной строки
            max_row: int | None
                Номер конечной строки
            min_col: int | None
                Номер начальной колонки
            max_col: int | None
                Номер конечной колонки

        Returns:
        --------
            Sequence[XLSXSheetCellRead]

        """

class XLSXSheetCellRead:
    """Тип данных ячеек листа с которыми работает парсер."""

    row: int
    column: int
    cell: str
    value: Any | None
    formula: str | None
    data_type: str
    number_format: str
    is_merge: bool
    start_column: int | None
    end_column: int | None
    start_row: int | None
    end_row: int | None
    style_id: str | None
    hidden_value: str | None
    comment: str | None

    @final
    @property
    def is_formula(self) -> bool:
        """
        Метод для получения флага, ячейка с формулой или нет.
        ----------------------------------------------------

        Returns:
        --------
            bool
        """

    @final
    @property
    def is_value_bool(self) -> bool:
        """
        Проверить, является ли значение ячейки boolean
        ----------------------------------------------------

        Returns:
        --------
            bool
        """

    @final
    @property
    def is_value_numeric(self) -> bool:
        """
        Проверить, является ли значение ячейки numeric
        ----------------------------------------------------

        Returns:
        --------
            bool
        """

    @final
    @property
    def is_value_datetime(self) -> bool:
        """
        Проверить, является ли значение ячейки datetime
        ----------------------------------------------------

        Returns:
        --------
            bool
        """

    @final
    @property
    def is_value_empty(self) -> bool:
        """
        Проверить, является ли значение ячейки empty
        ----------------------------------------------------

        Returns:
        --------
            bool
        """

class HelperCell:
    """Утилита по работе со списком ячеек."""

    @final
    @staticmethod
    def find_cell_by_pattern_regex(
        pattern: str, cells: Sequence[XLSXSheetCellRead]
    ) -> XLSXSheetCellRead | None:
        """
        Функция для поиска ячейки при помощи регулярных выражений.
        ----------------------------------------------------------

        Arguments:
        ---------
            pattern: str
                Паттерн для поиска
            cells: Sequence[XLSXSheetCellRead]
                Список ячеек

        Returns:
        --------
            XLSXSheetCellRead | None

        """

    @final
    @staticmethod
    def find_cells_by_pattern_regex(
        pattern: str, cells: Sequence[XLSXSheetCellRead]
    ) -> Sequence[XLSXSheetCellRead]:
        """
        Функция для поиска ячеек при помощи регулярных выражений.
        ---------------------------------------------------------

        Arguments:
        ---------
            pattern: str
                Паттерн для поиска
            cells: Sequence[XLSXSheetCellRead]
                Список ячеек

        Returns:
        --------
            Sequence[XLSXSheetCellRead]

        """

    @final
    @staticmethod
    def find_cells_for_rows_pattern_regex(
        pattern: str, cells: Sequence[XLSXSheetCellRead], colunm_stop: int | None = None
    ) -> Sequence[XLSXSheetCellRead]:
        """
        Функция поиска ячейеек колонок для строк которые соответствуют патерну.
        -----------------------------------------------------------------------

        Arguments:
        ---------
            pattern: str
                Паттерн для поиска
            cells: Sequence[XLSXSheetCellRead]
                Список ячеек
            colunm_stop: int | None
                Индекс колонки для остановки поиска

        Returns:
        --------
            Sequence[XLSXSheetCellRead]

        """

    @final
    @staticmethod
    def find_cells_for_cols_pattern_regex(
        pattern: str, cells: Sequence[XLSXSheetCellRead], row_stop: int | None = None
    ) -> Sequence[XLSXSheetCellRead]:
        """
        Функция поиска ячеек строк для колонок которые соответствуют патерну.
        -----------------------------------------------------------------------

        Arguments:
        ---------
            pattern: str
                Паттерн для поиска
            cells: Sequence[XLSXSheetCellRead]
                Список ячеек
            row_stop: int | None
                Индекс строки для остановки поиска

        Returns:
        --------
            Sequence[XLSXSheetCellRead]

        """

    @final
    @staticmethod
    def find_cells_multi_pattern_regex(
        pattern_1: str, pattern_2: str, cells: Sequence[XLSXSheetCellRead]
    ) -> Sequence[XLSXSheetCellRead]:
        """
        Функция для поиска ячеек при помощи регулярных выражений по двум паттернам.
        --------------------------------------------------------------------------

        Arguments:
        ---------
            pattern_1: str
                Паттерн для поиска
            pattern_2: str
                Паттерн для поиска
            cells: Sequence[XLSXSheetCellRead]
                Список ячеек


        Returns:
        --------
            Sequence[XLSXSheetCellRead]

        """

    @final
    @staticmethod
    def find_cell_by_cell(
        cell: str, cells: Sequence[XLSXSheetCellRead]
    ) -> XLSXSheetCellRead | None:
        """
        Функция для получения ячейки по cell (A1).
        ------------------------------------------

        Arguments:
        ---------
            cell: str
                Ячейка в формате A1
            cells: Sequence[XLSXSheetCellRead]
                Список ячеек

        Returns:
        --------
            XLSXSheetCellRead | None

        """

    @final
    @staticmethod
    def find_cell_by_coords(
        row: int, col: int, cells: Sequence[XLSXSheetCellRead]
    ) -> XLSXSheetCellRead | None:
        """
        Функция для ячейки по координатам.
        ---------------------------------

        Arguments:
        ---------
            row: int
                Номер строки
            col: int
                Номер колонки
            cells: Sequence[XLSXSheetCellRead]
                Список ячеек

        Returns:
        --------
            XLSXSheetCellRead | None
        """

    @final
    @staticmethod
    def find_cells_between_patterns(
        pattern_after: str, pattern_before: str, cells: Sequence[XLSXSheetCellRead]
    ) -> Sequence[XLSXSheetCellRead]:
        """
        Метод ищет ячейки между двумя патернами.
        ----------------------------------------

        Arguments:
        ---------
            pattern_after: str
                Паттерн для поиска после
            pattern_before: str
                Паттерн для поиска до
            cells: Sequence[XLSXSheetCellRead]
                Список ячеек

        Returns:
        --------
            Sequence[XLSXSheetCellRead]

        """

    @final
    @staticmethod
    def iter_cells(
        min_row: int | None,
        max_row: int | None,
        min_col: int | None,
        max_col: int | None,
        cells: Sequence[XLSXSheetCellRead],
    ) -> Sequence[XLSXSheetCellRead]:
        """
        Получить список всех ячеек в заданном диапазоне.
        -----------------------------------------------

        Arguments:
        ---------
            min_row: int | None
                Номер начальной строки
            max_row: int | None
                Номер конечной строки
            min_col: int | None
                Номер начальной колонки
            max_col: int | None
                Номер конечной колонки
            cells: Sequence[XLSXSheetCellRead]
                Список ячеек

        Returns:
        --------
            Sequence[XLSXSheetCellRead]

        """

    @final
    @staticmethod
    def find_cells_by_range_rows(
        start_row: int,
        end_row: int,
        cells: Sequence[XLSXSheetCellRead],
    ) -> Sequence[XLSXSheetCellRead]:
        """
        Возвращаем все ячейки, которые находятся в диапазоне строк.
        ----------------------------------------------------------

        Arguments:
        ---------
            start_row: int
                Номер начальной строки
            end_row: int
                Номер конечной строки
            cells: Sequence[XLSXSheetCellRead]
                Список ячеек

        Returns:
        --------
            Sequence[XLSXSheetCellRead]

        """

    @final
    @staticmethod
    def find_cells_by_range_cols(
        start_col: int,
        end_col: int,
        cells: Sequence[XLSXSheetCellRead],
    ) -> Sequence[XLSXSheetCellRead]:
        """
        Возвращаем все ячейки, которые находятся в диапазоне колонок.
        -------------------------------------------------------------

        Arguments:
        ---------
            start_col: int
                Номер начальной колонки
            end_col: int
                Номер конечной колонки
            cells: Sequence[XLSXSheetCellRead]
                Список ячеек

        Returns:
        --------
            Sequence[XLSXSheetCellRead]
        """

class HelperSheet:
    """Парсер"""

    def __init__(self: Self, sheets: Sequence[Any]) -> None:
        """
        Инициализация парсера
        ----------------------

        Arguments:
        ---------
            sheets: Sequence[Any]
                Список листов
        """

    @property
    def sheets(self: Self) -> Sequence[XLSXSheetRead]:
        """
        Данный метод позволяет получить список листов в парсере
        -------------------------------------------------------

        Returns:
        --------
            Sequence[XLSXSheetRead]
        """

    @final
    def find_sheet_by_name(self: Self, name: str) -> XLSXSheetRead | None:
        """
        Данный метод позволяет сделать поиск по названию листа
        ------------------------------------------------------

        Arguments:
        ---------
            name: str
                Имя листа

        Returns:
        --------
            XLSXSheetRead | None
        """

    @final
    def find_sheet_by_pattern(self, pattern: str) -> XLSXSheetRead | None:
        """
        Данный метод позволяет сделать поиск листа по шаблону regex
        -----------------------------------------------------------

        Arguments:
        ---------
            pattern: str
                Паттерн для поиска

        Returns:
        --------
            XLSXSheetRead | None

        """

    @final
    def find_sheet_by_index(self, idx: int) -> XLSXSheetRead | None:
        """
        Данный метод позволяет сделать поиск по индексу листа
        ------------------------------------------------------

        Arguments:
        ---------
            idx: int
                Индекс листа

        Returns:
        --------
            XLSXSheetRead | None
        """

    @final
    def get_sheets_without_names(
        self, name_list: Sequence[str]
    ) -> Sequence[XLSXSheetRead]:
        """
        Метод для получения списка листов, исключая передаваесый список.
        ---------------------------------------------------------------

        Arguments:
        ---------
            name_list: Sequence[str]
                Список имен листов

        Returns:
        --------
            Sequence[XLSXSheetRead]
        """

    @final
    def get_sheets_with_names(
        self, name_list: Sequence[str]
    ) -> Sequence[XLSXSheetRead]:
        """
        Метод для получения списка листов, передаваесых названий в параметрах.
        ---------------------------------------------------------------------

        Arguments:
        ---------
            name_list: Sequence[str]
                Список имен листов

        Returns:
        --------
            Sequence[XLSXSheetRead]
        """

class Service(ABC):
    """Сервис"""

    uow: Any

    def __init__(self: Self, uow: Any) -> None:
        """
        Инициализация парсера
        ---------------------

        Arguments:
        ---------
            uow: Any
                UoW для работы с базой данных
        """
        ...

    @abstractmethod
    def summary_0(self: Self, sheets: Sequence[Any], /, **kwargs: Any) -> Any:
        """
        Данный метод для реализации генерации данных сервиса
        ----------------------------------------------------

        Arguments:
        ---------
            sheets: Sequence[Any]
                Список листов
            kwargs: Any
                Дополнительные параметры

        Returns:
        --------
            Any
        """

    def summary_1(self: Self, sheets: Sequence[Any], /, **kwargs: Any) -> Any:
        """
        Данный метод для реализации генерации данных сервиса
        ----------------------------------------------------

        Arguments:
        ---------
            sheets: Sequence[Any]
                Список листов
            kwargs: Any
                Дополнительные параметры

        Returns:
        --------
            Any
        """

    def summary_2(self: Self, sheets: Sequence[Any], /, **kwargs: Any) -> Any:
        """
        Данный метод для реализации генерации данных сервиса
        ----------------------------------------------------

        Arguments:
        ---------
            sheets: Sequence[Any]
                Список листов
            kwargs: Any
                Дополнительные параметры

        Returns:
        --------
            Any
        """

    def summary_3(self: Self, sheets: Sequence[Any], /, **kwargs: Any) -> Any:
        """
        Данный метод для реализации генерации данных сервиса
        ----------------------------------------------------

        Arguments:
        ---------
            sheets: Sequence[Any]
                Список листов
            kwargs: Any
                Дополнительные параметры

        Returns:
        --------
            Any
        """

    def summary_4(self: Self, sheets: Sequence[Any], /, **kwargs: Any) -> Any:
        """
        Данный метод для реализации генерации данных сервиса
        ----------------------------------------------------

        Arguments:
        ---------
            sheets: Sequence[Any]
                Список листов
            kwargs: Any
                Дополнительные параметры

        Returns:
        --------
            Any
        """

    def summary_5(self: Self, sheets: Sequence[Any], /, **kwargs: Any) -> Any:
        """
        Данный метод для реализации генерации данных сервиса
        ----------------------------------------------------

        Arguments:
        ---------
            sheets: Sequence[Any]
                Список листов
            kwargs: Any
                Дополнительные параметры

        Returns:
        --------
            Any
        """

    def summary_6(self: Self, sheets: Sequence[Any], /, **kwargs: Any) -> Any:
        """
        Данный метод для реализации генерации данных сервиса
        ----------------------------------------------------

        Arguments:
        ---------
            sheets: Sequence[Any]
                Список листов
            kwargs: Any
                Дополнительные параметры

        Returns:
        --------
            Any
        """

    def summary_7(self: Self, sheets: Sequence[Any], /, **kwargs: Any) -> Any:
        """
        Данный метод для реализации генерации данных сервиса
        ----------------------------------------------------

        Arguments:
        ---------
            sheets: Sequence[Any]
                Список листов
            kwargs: Any
                Дополнительные параметры

        Returns:
        --------
            Any
        """

    def summary_8(self: Self, sheets: Sequence[Any], /, **kwargs: Any) -> Any:
        """
        Данный метод для реализации генерации данных сервиса
        ----------------------------------------------------

        Arguments:
        ---------
            sheets: Sequence[Any]
                Список листов
            kwargs: Any
                Дополнительные параметры

        Returns:
        --------
            Any
        """

    @abstractmethod
    def fmt_0(self: Self, book: XLSXBook, /, **kwargs: Any) -> Any:
        """
        Данный метод для форматирования отчета сервиса.
        ----------------------------------------------

        Arguments:
        ---------
            book: XLSXBook
                Книга для форматирования
            kwargs: Any
                Дополнительные параметры

        Returns:
        --------
            Any
        """

    def fmt_1(self: Self, book: XLSXBook, /, **kwargs: Any) -> Any:
        """
        Данный метод для форматирования отчета сервиса.
        ----------------------------------------------

        Arguments:
        ---------
            book: XLSXBook
                Книга для форматирования
            kwargs: Any
                Дополнительные параметры

        Returns:
        --------
            Any
        """
        ...

    def fmt_2(self: Self, book: XLSXBook, /, **kwargs: Any) -> Any:
        """
        Данный метод для форматирования отчета сервиса.
        ----------------------------------------------

        Arguments:
        ---------
            book: XLSXBook
                Книга для форматирования
            kwargs: Any
                Дополнительные параметры

        Returns:
        --------
            Any
        """

    def fmt_3(self: Self, book: XLSXBook, /, **kwargs: Any) -> Any:
        """
        Данный метод для форматирования отчета сервиса.
        ----------------------------------------------

        Arguments:
        ---------
            book: XLSXBook
                Книга для форматирования
            kwargs: Any
                Дополнительные параметры

        Returns:
        --------
            Any
        """

    def fmt_4(self: Self, book: XLSXBook, /, **kwargs: Any) -> Any:
        """
        Данный метод для форматирования отчета сервиса.
        ----------------------------------------------

        Arguments:
        ---------
            book: XLSXBook
                Книга для форматирования
            kwargs: Any
                Дополнительные параметры

        Returns:
        --------
            Any
        """

    def fmt_5(self: Self, book: XLSXBook, /, **kwargs: Any) -> Any:
        """
        Данный метод для форматирования отчета сервиса.
        ----------------------------------------------

        Arguments:
        ---------
            book: XLSXBook
                Книга для форматирования
            kwargs: Any
                Дополнительные параметры

        Returns:
        --------
            Any
        """

    def fmt_6(self: Self, book: XLSXBook, /, **kwargs: Any) -> Any:
        """
        Данный метод для форматирования отчета сервиса.
        ----------------------------------------------

        Arguments:
        ---------
            book: XLSXBook
                Книга для форматирования
            kwargs: Any
                Дополнительные параметры

        Returns:
        --------
            Any
        """

    def fmt_7(self: Self, book: XLSXBook, /, **kwargs: Any) -> Any:
        """
        Данный метод для форматирования отчета сервиса.
        ----------------------------------------------

        Arguments:
        ---------
            book: XLSXBook
                Книга для форматирования
            kwargs: Any
                Дополнительные параметры

        Returns:
        --------
            Any
        """

    def fmt_8(self: Self, book: XLSXBook, /, **kwargs: Any) -> Any:
        """
        Данный метод для форматирования отчета сервиса.
        ----------------------------------------------

        Arguments:
        ---------
            book: XLSXBook
                Книга для форматирования
            kwargs: Any
                Дополнительные параметры

        Returns:
        --------
            Any
        """

def column_number_to_letter(column: int) -> str:
    """
    Функция для преобразования номера колонки в букву
    -------------------------------------------------

    Arguments:
    ---------
        column: int
            Номер колонки

    Returns:
    --------
        str
    """

def get_letter_coordinate(row: int, column: int) -> str:
    """
    Функция для получения координаты ячейки в стиле A1
    -------------------------------------------------

    Arguments:
    ---------
        row: int
            Номер строки
        column: int
            Номер колонки

    Returns:
    --------
        str

    """

def version() -> str:
    """
    Для получения версии
    ---------------------

    Returns:
    --------
        str

    """

__all__ = (
    "XLSXBook",
    "XLSXSheet",
    "XLSXSheetCell",
    "XLSXSheetRead",
    "XLSXSheetCellRead",
    "HelperSheet",
    "HelperCell",
    "Service",
    "column_number_to_letter",
    "get_letter_coordinate",
    "version",
)
