"""Import-smoke тесты: убеждаются, что собранный wheel пригоден к использованию.

Запускаются в CI (джоба test) против wheel, установленного через pip.
"""

import report_core


def test_import_report_core():
    """Пакет импортируется и публичное API на месте."""
    assert "Book" in report_core.__all__
    assert "Sheet" in report_core.__all__
    assert "Service" in report_core.__all__


def test_version_is_exposed():
    """Версия пакета доступна и непустая."""
    assert report_core.version()
    assert report_core.__version__


def test_core_types_available():
    """Основные типы ядра доступны из корневого пространства имён."""
    assert report_core.Book
    assert report_core.Sheet
    assert report_core.Service
