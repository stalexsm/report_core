import sys

from . import _report_core
from ._report_core import (
    DEFAULT_COLUMN_WIDTH,
    DEFAULT_ROW_HEIGHT,
    Book,
    Service,
    Sheet,
    __version__,
    column_number_to_letter,
    get_letter_coordinate,
    version,
)

# Сабмодуль readable реализован в Rust внутри _report_core: публикуем его
# как атрибут пакета и в sys.modules, чтобы import report_core.readable
# отдавал rust-модуль, а не питоновскую обёртку ниже по дереву.
readable = _report_core.readable

sys.modules[f"{__name__}.readable"] = readable

__all__ = (
    "DEFAULT_COLUMN_WIDTH",
    "DEFAULT_ROW_HEIGHT",
    "Book",
    "Service",
    "Sheet",
    "__version__",
    "column_number_to_letter",
    "get_letter_coordinate",
    "readable",
    "version",
)
