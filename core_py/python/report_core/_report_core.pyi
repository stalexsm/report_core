from abc import ABC, abstractmethod
from datetime import datetime
from typing import Any, Sequence, final, Self

__all__ = (
    "__version__",
    "Book",
    "Sheet",
    "Cell",
    "column_number_to_letter",
    "get_letter_coordinate",
    "version",
)

__version__: str

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

class Service(ABC):
    """Сервис"""

    _uow: Any

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

    @abstractmethod
    def _fmt_0(self: Self, /, **kwargs: Any) -> Any:
        """
        Данный метод для форматирования отчета сервиса.
        ----------------------------------------------

        Arguments:
        ---------
            kwargs: Any
                Дополнительные параметры

        Returns:
        --------
            Any
        """

    @final
    def _add_sheet(self, name: str) -> Sheet:
        """
        Добавление листа в книгу
        ---

        Arguments:
        ---
            name: str
                Имя листа
        Returns:
        ---
            Sheet
        """

    @final
    def _copy_sheet(self, sheet: Sheet) -> Sheet:
        """
        Корирование листа в книгу
        ---

        Arguments:
        ---
            sheet: Sheet
                Лист
        Returns:
        ---
            Sheet
        """

    @final
    def _get_sheet_index(self, idx: int) -> Sheet | None:
        """
        Получение листа по индексу
        ---

        Arguments:
        ---
            idx: int
                Индекс листа

        Returns:
        ---
            Sheet | None

        """

    @final
    def _get_sheet_name(self, name: str) -> Sheet | None:
        """
        Получение листа по названию
        ---

        Arguments:
        ---
            name: str
                Имя листа

        Returns:
        ---
            Sheet | None
        """

    @property
    def _sheets(self) -> Sequence[Sheet]:
        """
        Получение списка листов
        ---

        Returns:
        ---
            Sequence[Sheet]

        """
    @final
    def to_json(self) -> str:
        """
        Преобразование книги в json
        ---

        Returns:
        ---
            str
        """

    @final
    def to_dict(self) -> dict[str, Any]:
        """
        Преобразование книги в dict
        ---

        Returns:
        ---
            dict[str, Any]
        """

class Book:
    sheets: Sequence[Sheet]

    def __init__(self) -> None:
        """
        Инициализация Книги
        ----

        Example:
        ---
        .. code-block:: python3

            book = Book()
        """
        ...

    def add_sheet(self, name: str) -> Sheet:
        """
        Добавление листа в книгу
        ---

        Arguments:
        ---
            name: str
                Имя листа
        Returns:
        ---
            Sheet
        """

    def copy_sheet(self, sheet: Sheet) -> Sheet:
        """
        Корирование листа в книгу
        ---

        Arguments:
        ---
            sheet: Sheet
                Лист
        Returns:
        ---
            Sheet
        """

    def get_sheet_index(self, idx: int) -> Sheet | None:
        """
        Получение листа по индексу
        ---

        Arguments:
        ---
            idx: int
                Индекс листа

        Returns:
        ---
            Sheet | None

        """

    def get_sheet_name(self, name: str) -> Sheet | None:
        """
        Получение листа по названию
        ---

        Arguments:
        ---
            name: str
                Имя листа

        Returns:
        ---
            Sheet | None
        """

    def to_json(self) -> str:
        """
        Преобразование книги в json
        ---

        Returns:
        ---
            str
        """

    @final
    def to_dict(self) -> dict[str, Any]:
        """
        Преобразование книги в dict
        ---

        Returns:
        ---
            dict[str, Any]
        """

class Sheet:
    """Тип данных листа с которыми работает парсер."""

    name: str
    merge_cells: Sequence[tuple[int, int, int, int]]
    cells: Sequence[Cell]
    sheet_state: str

    @final
    def add_merge_cells(
        self,
        start_row: int,
        end_row: int,
        start_col: int,
        end_col: int,
    ) -> None:
        """
        Функция для добавления объедененных ячеек на лист.
        ---

        Arguments:
        ---
            start_row: int
                Начальная строка
            end_row: int
                Конечная строка
            start_col: int
                Начальная колонка
            end_col: int
                Конечная колонка
        """

    @property
    def max_row(self) -> int:
        """
        Получение максимальной строки в книге
        ---

        Returns:
        ---
            int
        """

    @property
    def max_column(self) -> int:
        """
        Получение максимальной колонки в книге
        ---

        Returns:
        ---
            int
        """

    @final
    def cell(
        self,
        row: int,
        col: int,
        value: str | None = None,
    ) -> Cell:
        """
        Функция для получения/добавления ячейки.
        ---

        Arguments:
        ---
            row: int
                Строка ячейки
            col: int
                Колонка ячейки
            value: str | None
                Значение ячейки

        Returns:
        ---
            Cell
        """

    @final
    def get_value_cell(self, row: int, col: int) -> str:
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
    @final
    def set_height_row(self, row_num: int, val: float) -> None:
        """
        Метод установки высоты ячеек
        ---

        Arguments:
        ---
            row_num: int
                Номер строки
            val: float
                Высота ячейки

        Returns:
        ---
            None
        """

    @final
    def set_hidden_row(self, row_num: int, val: bool) -> None:
        """
        Метод установки скрытости строки
        ---

        Arguments:
        ---
            row_num: int
                Номер строки
            val: bool
                Скрытость строки

        Returns:
        ---
            None
        """

    @final
    def set_width_column(self, col_num: int, val: float) -> None:
        """
        Метод установки ширины колонки
        ---

        Arguments:
        ---
            col_num: int
                Номер колонки
            val: float
                ширина колонки

        Returns:
        ---
            None
        """

    @final
    def set_hidden_column(self, col_num: int, val: bool) -> None:
        """
        Метод установки скрытости колонки
        ---

        Arguments:
        ---
            col_num: int
                Номер колонки
            val: bool
                Скрытость колонки

        Returns:
        ---
            None
        """

    @final
    def delete_cols(self, idx: int, cols: int) -> None:
        """
        Метод удаления колонок
        ---

        Arguments:
        ---
            idx: int
                Номер колонки
            cols: int
                Количество колонок
        """

    @final
    def delete_rows(self, idx: int, rows: int) -> None:
        """
        Метод удаления строк
        ---

        Arguments:
        ---
            idx: int
                Номер строки
            rows: int
                Количество строк

        """

    @final
    def get_cells_by_range(
        self,
        min_row: int | None = None,
        max_row: int | None = None,
        min_col: int | None = None,
        max_col: int | None = None,
    ) -> Sequence[Cell]:
        """
        Получить список всех ячеек в заданном диапазоне.
        ---

        Arguments:
        ---
            min_row: int | None
                Номер начальной строки
            max_row: int | None
                Номер конечной строки
            min_col: int | None
                Номер начальной колонки
            max_col: int | None
                Номер конечной колонки

        Returns:
        ---
            Sequence[Cell]

        """

    @final
    def get_height_by_row(self, row_num: int) -> float:
        """
        Метод для получения высоты строки
        ---

        Arguments:
        ---
            row_num: int
                Номер строки

        Returns:
        ---
            float
        """

    @final
    def get_hidden_by_row(self, row_num: int) -> bool:
        """
        Метод для получения скрытости строки
        ---

        Arguments:
        ---
            row_num: int
                Номер строки

        Returns:
        ---
            bool
        """

    @final
    def get_width_by_column(self, col_num: int) -> float:
        """
        Метод для получения ширины колонки
        ---

        Arguments:
        ---
            col_num: int
                Номер колонки

        Returns:
        ---
            float
        """

    @final
    def get_hidden_by_column(self, col_num: int) -> bool:
        """
        Метод для получения скрытости колонки
        ---

        Arguments:
        ---
            col_num: int
                Номер колонки

        Returns:
        ---
            None
        """

    @final
    def find_cell_by_regex(self, regex: str) -> Cell | None:
        """
        Функция для получения ячейки по регулярному (шаблону) значению.
        ---

        Arguments:
        ---
            regex: str
                Шаблон (регулярное значение)

        Returns:
        ---
            Cell | None
        """

    @final
    def find_cell_by_letter(self, letter: str) -> Cell | None:
        """
        Функция для получения ячейки по буквенной координате (A1).
        ---

        Arguments:
        ---------
            letter: str
                Координата (буквенная) A1

        Returns:
        --------
            Cell | None
        """

    @final
    def find_cells_by_regex(self, regex: str) -> Sequence[Cell]:
        """
        Функция для получения ячеек по регулярному (шаблону) значению).
        ---

        Arguments:
        ---
            regex: str
                Шаблон (регулярное значение)

        Returns:
        ---
             Sequence[Cell]
        """

    @final
    def find_cells_for_rows_by_regex(self, regex: str, col_stop: int) -> Sequence[Cell]:
        """
        Функция для получения ячeек по регулярному (шаблону) значению) до определенной колонки.
        ---

        Arguments:
        ---
            regex: str
                Шаблон (регулярное значение)
            col_stop: int
                Значение колонки до которой забирать ячейки

        Returns:
        ---
             Sequence[Cell]
        """

    @final
    def find_cells_for_cols_by_regex(self, regex: str, row_stop: int) -> Sequence[Cell]:
        """
        Функция для получения ячeек по регулярному (шаблону) значению) до определенной строки.
        ---

        Arguments:
        ---
            regex: str
                Шаблон (регулярное значение)
            row_stop: int
                Значение строки до которой забирать ячейки

        Returns:
        ---
             Sequence[Cell]
        """

    @final
    def find_cells_multi_regex(
        self,
        before_regex: str,
        after_regex: str,
    ) -> Sequence[Cell]:
        """
        Функция для получения ячeек по регулярным (шаблонам) значениям).
        Находит ячейки в соостветствии шаблонов.
        ---

        Arguments:
        ---
            before_regex: str
                Первый шаблон (регулярное значение)
            after_regex: str
                Второй шаблон (регулярное значение)

        Returns:
        ---
             Sequence[Cell]
        """

    @final
    def find_cells_between_regex(
        self,
        before_regex: str,
        after_regex: str,
    ) -> Sequence[Cell]:
        """
        Функция для получения ячeек по регулярным (шаблонам) значениям).
        Находит ячейки от первого шаборна до второго.
        ---

        Arguments:
        ---
            before_regex: str
                Первый шаблон (регулярное значение)
            after_regex: str
                Второй шаблон (регулярное значение)

        Returns:
        ---
             Sequence[Cell]
        """

    @final
    def find_cells_range_rows(
        self,
        start_row: int,
        end_row: int,
    ) -> Sequence[Cell]:
        """
        Функция для получения ячeек в диапазоне строк.
        ---

        Arguments:
        ---
            start_row: int
                Cтартовая строка
            end_row: int
                Финишная строка

        Returns:
        ---
            Sequence[Cell]
        """

    @final
    def find_cells_range_cols(
        self,
        start_col: int,
        end_col: int,
    ) -> Sequence[Cell]:
        """
        Функция для получения ячeек в диапазоне колонок.
        ---

        Arguments:
        ---
            start_col: int
                Cтартовая колонка
            end_col: int
                Финишная колонка

        Returns:
        ---
            Sequence[Cell]
        """

class Cell:
    """Тип данных ячеек листа с которыми работает парсер."""

    row: int
    column: int
    value: Any | None
    formula: str | None
    data_type: str
    style: str | None
    hidden_value: str | None

    @property
    def letter(self) -> int:
        """
        Получение буквенной координаты
        ---

        Returns:
        ---
            str
        """

    @property
    def is_formula(self) -> bool:
        """
        Метод для получения флага, ячейка с формулой или нет.
        ---

        Returns:
        ---
            bool

        """

    @property
    def is_value_bool(self) -> bool:
        """
        Проверить, является ли значение ячейки boolean
        ---

        Returns:
        ---
            bool
        """

    @property
    def is_value_numeric(self) -> bool:
        """
        Проверить, является ли значение ячейки numeric
        ---

        Returns:
        ---
            bool

        """

    @property
    def is_value_datetime(self) -> bool:
        """
        Проверить, является ли значение ячейки datetime
        ---

        Returns:
        ---
            bool

        """

    @property
    def is_value_empty(self) -> bool:
        """
        Проверить, является ли значение ячейки empty
        ---

        Returns:
        ---
            bool

        """

    @final
    def set_value_number(self, value: float) -> None:
        """
        Метод для добавления значения ячейки Numbers.
        ---

        Arguments:
        ---------
            value: float
                Значение ячейки

        """

    @final
    def set_value_integer(self, value: int) -> None:
        """
        Метод для добавления значения ячейки Integer.
        ---

        Arguments:
        ---------
            value: int
                Значение ячейки

        """

    @final
    def set_value_bool(self, value: bool) -> None:
        """
        Метод для добавления значения ячейки Boolean.
        ---

        Arguments:
        ---------
            value: bool
                Значение ячейки

        """

    @final
    def set_value_datetime(self, value: datetime) -> None:
        """
        Метод для добавления значения ячейки Дата/Время.
        ---

        Arguments:
        ---------
            value: datetime
                Значение ячейки

        """
