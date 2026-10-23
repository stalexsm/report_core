"""
Rust-ядро для построения отчётов на основе XLSX.
"""

from datetime import datetime
from typing import Any, Final, SupportsFloat, SupportsIndex, final

from typing_extensions import disjoint_base

from . import readable as readable

__all__ = ["DEFAULT_COLUMN_WIDTH", "DEFAULT_ROW_HEIGHT", "Book", "Cell", "Comment", "Service", "Sheet", "__version__", "column_number_to_letter", "get_letter_coordinate", "readable", "version"]

DEFAULT_COLUMN_WIDTH: Final[float]
"""
Ширина колонки по умолчанию (~140 пикселей).
"""

DEFAULT_ROW_HEIGHT: Final[float]
"""
Высота строки по умолчанию (~20 пикселей).
"""

__version__: Final[str]
"""
Версия ядра.
"""

@final
class Book:
    """
    Книга Excel: контейнер листов, из которых собирается отчёт.
    """
    def __new__(cls, /) -> Book:
        """
        Инициализация Книги
        ----

        Example:
        ---
        ```python
        book = Book()
        ```
        """
    def add_sheet(self, /, name: str, sheet_state: str = "visible") -> Sheet:
        """
        Добавление листа в книгу
        ---

        Arguments:
        ---
            name: str
                Имя листа
            sheet_state: str
                Состояние и видимость листа

        Returns:
        ---
            Sheet
        """
    def copy_sheet(self, /, sheet: Sheet) -> Sheet:
        """
        Копирование листа в книгу
        ---

        Arguments:
        ---
            sheet: Sheet
                Лист

        Returns:
        ---
            Sheet
        """
    def get_sheet_index(self, /, idx: SupportsIndex) -> Sheet | None:
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
    def get_sheet_name(self, /, name: str) -> Sheet | None:
        """
        Получение листа по названию
        ---

        Arguments:
        ---
            name: str
                Название листа

        Returns:
        ---
            Sheet | None
        """
    @property
    def sheets(self, /) -> list[Sheet]:
        """
        Получение списка листов
        ---

        Returns:
        ---
            list[Sheet]
        """
    def to_dict(self, /) -> Any:
        """
        Преобразование книги в dict
        ---

        Returns:
        ---
            dict[str, Any]
        """
    def to_json(self, /) -> str:
        """
        Преобразование книги в json
        ---

        Returns:
        ---
            str
        """

@final
class Cell:
    """
    Тип данных ячеек листа с которыми работает парсер.
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
    @formula.setter
    def formula(self, /, value: str) -> None:
        """
        Установка формулы ячейки
        """
    @property
    def hidden_value(self, /) -> str | None:
        """
        Скрытое значение ячейки
        """
    @hidden_value.setter
    def hidden_value(self, /, value: str) -> None:
        """
        Установка скрытого значения ячейки
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
    def set_value_bool(self, /, value: bool) -> None:
        """
        Метод для добавления значения ячейки Boolean.
        ---

        Arguments:
        ---------
            value: bool
                Значение ячейки
        """
    def set_value_datetime(self, /, value: datetime) -> None:
        """
        Метод для добавления значения ячейки Дата/Время.
        ---

        Arguments:
        ---------
            value: datetime
                Значение ячейки
        """
    def set_value_integer(self, /, value: SupportsIndex) -> None:
        """
        Метод для добавления значения ячейки Integer.
        ---

        Arguments:
        ---------
            value: int
                Значение ячейки
        """
    def set_value_number(self, /, value: SupportsFloat | SupportsIndex) -> None:
        """
        Метод для добавления значения ячейки Numbers.
        ---

        Arguments:
        ---------
            value: float
                Значение ячейки
        """
    @property
    def style(self, /) -> str | None:
        """
        Стиль ячейки
        """
    @style.setter
    def style(self, /, value: str) -> None:
        """
        Установка стиля ячейки
        """
    @property
    def value(self, /) -> str:
        """
        Значение ячейки
        """
    @value.setter
    def value(self, /, value: str) -> None:
        """
        Установка значения ячейки
        """

@final
class Comment:
    """
    Тип данных комментарий.
    """
    @property
    def author(self, /) -> str:
        """
        Автор комментария
        """
    @author.setter
    def author(self, /, val: str) -> None:
        """
        Установка автора комментария
        """
    @property
    def column(self, /) -> int:
        """
        Колонка комментария
        """
    @property
    def coordinate(self, /) -> tuple[int, int]:
        """
        Получение координаты комментария
        ---

        Returns:
        ---
            tuple[int, int]
        """
    @property
    def row(self, /) -> int:
        """
        Строка комментария
        """
    @property
    def text(self, /) -> str:
        """
        Текст комментария
        """
    @text.setter
    def text(self, /, val: str) -> None:
        """
        Установка текста комментария
        """

@disjoint_base
class Service:
    """
    Сервис

    Базовый класс для сервисов, строящих отчёты из данных БД.
    """
    def __new__(cls, /, conn_db: Any) -> Service:
        """
        Инициализация парсера
        ---------------------

        Arguments:
        ---------
            conn_db: Any
                conn_db для работы с базой данных
        """
    def _add_sheet(self, /, name: str, sheet_state: str = "visible") -> Sheet:
        """
        Добавление листа в книгу
        ---

        Arguments:
        ---
            name: str
                Имя листа
            sheet_state: str
                Состояние и видимость листа

        Returns:
        ---
            Sheet
        """
    @property
    def _conn_db(self, /) -> Any:
        """
        Подключение к базе данных
        """
    def _copy_sheet(self, /, sheet: Sheet) -> Sheet:
        """
        Копирование листа в книгу
        ---

        Arguments:
        ---
            sheet: Sheet
                Лист

        Returns:
        ---
            Sheet
        """
    def _fmt_0(self, /, **kwargs) -> Any:
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
    def _get_sheet_index(self, /, idx: SupportsIndex) -> Sheet | None:
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
    def _get_sheet_name(self, /, name: str) -> Sheet | None:
        """
        Получение листа по названию
        ---

        Arguments:
        ---
            name: str
                Название листа

        Returns:
        ---
            Sheet | None
        """
    @property
    def _sheets(self, /) -> list[Sheet]:
        """
        Получение списка листов
        ---

        Returns:
        ---
            list[Sheet]
        """
    def summary_0(self, sheets: list, /, **kwargs) -> Any:
        """
        Данный метод для реализации генерации данных сервиса
        ----------------------------------------------------

        Arguments:
        ---------
            sheets: list
                Список листов
            kwargs: Any
                Дополнительные параметры

        Returns:
        --------
            Any
        """
    def to_dict(self, /) -> Any:
        """
        Преобразование книги в dict
        ---

        Returns:
        ---
            dict[str, Any]
        """
    def to_json(self, /) -> str:
        """
        Преобразование книги в json
        ---

        Returns:
        ---
            str
        """

@final
class Sheet:
    """
    Тип данных листа с которыми работает парсер.
    """
    def add_comment(self, /, row: SupportsIndex, col: SupportsIndex, text: str, author: str) -> None:
        """
        Добавление комментария в ячейку
        ---

        Arguments:
        ---
            row: int
                Строка ячейки
            col: int
                Колонка ячейки
            text: str
                Текст комментария
            author: str
                Автор комментария
        """
    def add_merge_cells(self, /, start_row: SupportsIndex, end_row: SupportsIndex, start_col: SupportsIndex, end_col: SupportsIndex) -> None:
        """
        Функция для добавления объединённых ячеек на лист.
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
    def cell(self, /, row: SupportsIndex, col: SupportsIndex, value: str | None = None) -> Cell:
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
    @property
    def cells(self, /) -> list[Cell]:
        """
        Ячейки листа
        ---

        Returns:
        ---
            list[Cell]
        """
    @property
    def comments(self, /) -> list[Comment]:
        """
        Комментарии листа
        ---

        Returns:
        ---
            list[Comment]
        """
    def delete_cols(self, /, idx: SupportsIndex, amount: SupportsIndex) -> None:
        """
        Метод удаления колонок
        ---

        Arguments:
        ---
            idx: int
                Номер колонки
            amount: int
                Количество колонок
        """
    def delete_rows(self, /, idx: SupportsIndex, amount: SupportsIndex) -> None:
        """
        Метод удаления строк
        ---

        Arguments:
        ---
            idx: int
                Номер строки
            amount: int
                Количество строк
        """
    def find_cell_by_letter(self, /, letter: str) -> Cell | None:
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

        Raises:
        ---
            ValueError
                Если буква не координата ячейки (например, "1A") или за границами листа
        """
    def find_cell_by_regex(self, /, regex: str) -> Cell | None:
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
    def find_cell_by_str(self, /, value: str) -> Cell | None:
        """
        Функция для получения ячейки по строковому значению.
        ---

        Arguments:
        ---
            value: str
                Строка (значение)

        Returns:
        ---
            Cell | None
        """
    def find_cells_between_regex(self, /, before_regex: str, after_regex: str) -> list[Cell]:
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
            list[Cell]
        """
    def find_cells_by_regex(self, /, regex: str) -> list[Cell]:
        """
        Функция для получения ячеек по регулярному (шаблону) значению.
        ---

        Arguments:
        ---
            regex: str
                Шаблон (регулярное значение)

        Returns:
        ---
            list[Cell]
        """
    def find_cells_by_str(self, /, value: str) -> list[Cell]:
        """
        Функция для получения ячеек по строковому значению.
        ---

        Arguments:
        ---
            value: str
                Строка (значение)

        Returns:
        ---
            list[Cell]
        """
    def find_cells_for_cols_by_regex(self, /, regex: str, row_stop: SupportsIndex) -> list[Cell]:
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
            list[Cell]
        """
    def find_cells_for_rows_by_regex(self, /, regex: str, col_stop: SupportsIndex) -> list[Cell]:
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
            list[Cell]
        """
    def find_cells_multi_regex(self, /, before_regex: str, after_regex: str) -> list[Cell]:
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
            list[Cell]
        """
    def find_cells_range_cols(self, /, start_col: SupportsIndex, end_col: SupportsIndex) -> list[Cell]:
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
            list[Cell]
        """
    def find_cells_range_rows(self, /, start_row: SupportsIndex, end_row: SupportsIndex) -> list[Cell]:
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
            list[Cell]
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
    def get_cells_by_range(self, /, start_row: SupportsIndex | None = None, end_row: SupportsIndex | None = None, start_col: SupportsIndex | None = None, end_col: SupportsIndex | None = None) -> list[Cell]:
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
            list[Cell]
        """
    def get_height_by_row(self, /, row_num: SupportsIndex) -> float:
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
    def get_hidden_by_column(self, /, col_num: SupportsIndex) -> bool:
        """
        Метод для получения скрытости колонки
        ---

        Arguments:
        ---
            col_num: int
                Номер колонки

        Returns:
        ---
            bool
        """
    def get_hidden_by_row(self, /, row_num: SupportsIndex) -> bool:
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
    def get_width_by_column(self, /, col_num: SupportsIndex) -> float:
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
    @name.setter
    def name(self, /, name: str) -> None:
        """
        Установка имени листа
        """
    def set_height_row(self, /, row_num: SupportsIndex, val: SupportsFloat | SupportsIndex) -> None:
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
    def set_hidden_column(self, /, col_num: SupportsIndex, val: bool) -> None:
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
    def set_hidden_row(self, /, row_num: SupportsIndex, val: bool) -> None:
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
    def set_sheet_state(self, /, state: str) -> None:
        """
        Метод установки видимости листа
        ---

        Arguments:
        ---
            state: str
                Тип видимости: Literal["visible", "hidden"]

        Returns:
        ---
            None
        """
    def set_width_column(self, /, col_num: SupportsIndex, val: SupportsFloat | SupportsIndex) -> None:
        """
        Метод установки ширины колонки
        ---

        Arguments:
        ---
            col_num: int
                Номер колонки
            val: float
                Ширина колонки

        Returns:
        ---
            None
        """
    @property
    def sheet_state(self, /) -> str:
        """
        Состояние и видимость листа
        """

def column_number_to_letter(col: SupportsIndex) -> str:
    """
    Функция для преобразования номера колонки в букву
    ---

    Arguments:
    ---
        col: u16
            Номер колонки

    Returns:
    ---
        str
    """

def get_letter_coordinate(row: SupportsIndex, col: SupportsIndex) -> str:
    """
    Функция для получения координаты ячейки в стиле A1
    ---

    Arguments:
    ---
        row: u32
            Номер строки
        col: u16
            Номер колонки

    Returns:
    ---
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
