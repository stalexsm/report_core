"""Класс Service: базовый класс сервисов отчётов.

Service наследуется в Python: хранит conn_db, ведёт книгу листов
(подчёркнутые методы-обёртки) и отдаёт заготовки summary_0/_fmt_0,
пока не переопределённые наследником.
"""

import json

import pytest
from report_core import Service


class StubService(Service):
    """Минимальный наследник без переопределений."""


class ReadyService(Service):
    """Наследник с переопределёнными шагами генерации и форматирования."""

    def summary_0(self, sheets, /, **kwargs):
        """Свои данные вместо заготовки."""
        return "данные"

    def _fmt_0(self, **kwargs):
        """Своё форматирование вместо заготовки."""
        return "формат"


@pytest.fixture
def conn_db():
    """Произвольный объект-подключение."""
    return object()


def test_conn_db_keeps_reference(conn_db) -> None:
    """conn_db сохраняется и отдаётся как есть."""
    service = StubService(conn_db)

    assert service._conn_db is conn_db


def test_repr_uses_subclass_name(conn_db) -> None:
    """repr называет класс наследника и число листов."""
    assert repr(StubService(conn_db)) == "StubService, sheets: 0"


def test_add_sheet_and_sheets(conn_db) -> None:
    """_add_sheet добавляет лист, _sheets его отдаёт."""
    service = StubService(conn_db)
    added = service._add_sheet("лист1")
    added.cell(1, 1, "значение")

    sheets = service._sheets
    assert [s.name for s in sheets] == ["лист1"]
    assert sheets[0].find_value_by_coords(1, 1) == "значение"


def test_add_sheet_state(conn_db) -> None:
    """Состояние листа берётся из аргумента."""
    service = StubService(conn_db)
    sheet = service._add_sheet("скрытый", "hidden")

    assert sheet.sheet_state == "hidden"


def test_copy_sheet(conn_db) -> None:
    """_copy_sheet добавляет копию листа с новым именем в книгу сервиса."""
    service = StubService(conn_db)
    original = service._add_sheet("лист1")
    original.cell(1, 1, "значение")
    copy = service._copy_sheet(original)

    assert copy.name == "Sheet 2"
    assert copy.find_value_by_coords(1, 1) == "значение"
    assert len(service._sheets) == 2


def test_get_sheet_index_and_name(conn_db) -> None:
    """Поиск листа по индексу и имени, промах даёт None."""
    service = StubService(conn_db)
    service._add_sheet("лист1")

    by_index = service._get_sheet_index(0)
    by_name = service._get_sheet_name("лист1")

    assert by_index is not None and by_name is not None
    assert by_index.name == "лист1"
    assert by_name.name == "лист1"
    assert service._get_sheet_index(9) is None
    assert service._get_sheet_name("нет") is None


def test_to_json_and_to_dict(conn_db) -> None:
    """Сериализация книги сервиса: json повторяет dict."""
    service = StubService(conn_db)
    service._add_sheet("лист1")

    assert json.loads(service.to_json()) == service.to_dict()


def test_summary_0_not_implemented(conn_db) -> None:
    """Заготовка summary_0 требует переопределения."""
    with pytest.raises(NotImplementedError, match="summary_0"):
        StubService(conn_db).summary_0([])


def test_fmt_0_not_implemented(conn_db) -> None:
    """Заготовка _fmt_0 требует переопределения."""
    with pytest.raises(NotImplementedError, match="fmt_0"):
        StubService(conn_db)._fmt_0()


def test_overridden_methods_are_callable(conn_db) -> None:
    """Переопределённые шаги наследника работают."""
    service = ReadyService(conn_db)

    assert service.summary_0([]) == "данные"
    assert service._fmt_0() == "формат"
