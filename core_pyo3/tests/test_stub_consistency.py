"""Сверка публичного API расширения с .pyi-стабами.

Защита от дрейфа стабов: имена членов рантайма (`dir()` бинарного
модуля `_report_core` и сабмодуля `readable`) сравниваются с
объявлениями в сгенерированных стабах в обе стороны. Расхождение
значит, что стабы не перегенерированы после изменения публичного
API: `./scripts/generate-stubs.sh`.

Тест гоняется в CI (quality.yml, джоба test) против установленного
wheel и стабов из репозитория; вне репозитория пропускается.
"""

import ast
import importlib
import types
from pathlib import Path

import pytest

STUBS_DIR = Path(__file__).resolve().parents[1] / "python" / "report_core"
MAIN_STUB = STUBS_DIR / "_report_core.pyi"
READABLE_STUB = STUBS_DIR / "readable" / "_readable.pyi"

pytestmark = pytest.mark.skipif(
    not MAIN_STUB.is_file() or not READABLE_STUB.is_file(),
    reason="стабы не найдены рядом с тестами: запустите ./scripts/generate-stubs.sh",
)


def parse_stub(
    path: Path,
) -> tuple[dict[str, set[str]], set[str], set[str], set[str]]:
    """Разбор стаба: члены классов, функции, константы и импорты модуля."""
    tree = ast.parse(path.read_text(encoding="utf-8"))

    classes: dict[str, set[str]] = {}
    functions: set[str] = set()
    constants: set[str] = set()
    imports: set[str] = set()

    for node in tree.body:
        match node:
            case ast.ClassDef():
                members = {
                    item.name
                    for item in node.body
                    if isinstance(item, (ast.FunctionDef, ast.AsyncFunctionDef))
                    # Dunder-протокол (__new__, __init__) в сверке имён не
                    # участвует: он есть у любого pyclass и от генератора
                    # не зависит.
                    and not item.name.startswith("_")
                }
                classes[node.name] = members
            case ast.FunctionDef() | ast.AsyncFunctionDef():
                functions.add(node.name)
            case ast.AnnAssign() | ast.Assign():
                targets = (
                    [node.target] if isinstance(node, ast.AnnAssign) else node.targets
                )
                # __all__ сверяется отдельным тестом, константой не считается.
                constants.update(
                    t.id
                    for t in targets
                    if isinstance(t, ast.Name) and t.id != "__all__"
                )
            case ast.ImportFrom():
                imports.update(alias.name for alias in node.names)
            case ast.Import():
                imports.update(alias.name for alias in node.names)

    return classes, functions, constants, imports


def public_names(obj) -> set[str]:
    """Публичные имена членов объекта: dir() без подчёркивания."""
    return {name for name in dir(obj) if not name.startswith("_")}


def assert_same_sets(runtime: set[str], stub: set[str], what: str) -> None:
    """Обе стороны сверки равны, иначе — отчёт с подсказкой."""
    missing_in_stub = sorted(runtime - stub)
    missing_in_runtime = sorted(stub - runtime)

    assert not missing_in_stub and not missing_in_runtime, (
        f"Стабы разошлись с рантаймом ({what}):\n"
        f"  нет в стабе: {missing_in_stub}\n"
        f"  нет в рантайме: {missing_in_runtime}\n"
        f"Перегенерируйте стабы: ./scripts/generate-stubs.sh"
    )


def module_surface(
    module,
) -> tuple[dict[str, set[str]], set[str], set[str], set[str]]:
    """Поверхность API модуля: классы, функции, константы, сабмодули.

    Члены берутся из `vars()`, а не `dir()`: dir() модуля-расширения
    дополняется typing-именами из стаба, которых в рантайме нет.
    """
    classes: dict[str, set[str]] = {}
    functions: set[str] = set()
    constants: set[str] = set()
    submodules: set[str] = set()

    for name, attr in vars(module).items():
        if name.startswith("_"):
            continue
        if isinstance(attr, types.ModuleType):
            submodules.add(name)
        elif isinstance(attr, type):
            classes[name] = public_names(attr)
        elif callable(attr):
            functions.add(name)
        else:
            constants.add(name)

    return classes, functions, constants, submodules


def test_module_surface_matches_stub() -> None:
    """Члены модуля _report_core совпадают со стабом в обе стороны."""
    runtime = importlib.import_module("report_core._report_core")
    stub_classes, stub_functions, stub_constants, stub_imports = parse_stub(MAIN_STUB)

    rt_classes, rt_functions, rt_constants, rt_submodules = module_surface(runtime)

    assert_same_sets(set(rt_classes), set(stub_classes), "классы модуля")
    for name, members in rt_classes.items():
        assert_same_sets(members, stub_classes[name], f"члены класса {name}")

    assert_same_sets(rt_functions, stub_functions, "функции модуля")
    # __version__ — константа с подчёркиванием: в dir() есть, в сверку
    # публичных имён не попала, сверяется явно.
    assert_same_sets(
        rt_constants | {"__version__"},
        stub_constants,
        "константы модуля",
    )
    # Сабмодули сверяются в одну сторону: стаб сверх того импортирует
    # typing-хелперы, которых в рантайме быть не должно.
    assert rt_submodules <= set(stub_imports), (
        f"Сабмодули не объявлены в стабе: {sorted(rt_submodules - set(stub_imports))}"
    )


def test_readable_module_surface_matches_stub() -> None:
    """Члены сабмодуля readable совпадают со стабом в обе стороны."""
    runtime = importlib.import_module("report_core.readable")
    stub_classes, stub_functions, stub_constants, stub_imports = parse_stub(
        READABLE_STUB
    )

    rt_classes, rt_functions, rt_constants, rt_submodules = module_surface(runtime)

    assert_same_sets(set(rt_classes), set(stub_classes), "классы сабмодуля")
    for name, members in rt_classes.items():
        assert_same_sets(members, stub_classes[name], f"члены класса {name}")

    assert_same_sets(rt_functions, stub_functions, "функции сабмодуля")
    assert_same_sets(rt_constants, stub_constants, "константы сабмодуля")
    # Сабмодули сверяются в одну сторону: стаб сверх того импортирует
    # typing-хелперы, которых в рантайме быть не должно.
    assert rt_submodules <= set(stub_imports), (
        f"Сабмодули не объявлены в стабе: {sorted(rt_submodules - set(stub_imports))}"
    )


def test_stub_all_matches_runtime() -> None:
    """__all__ стаба перечисляет ровно публичную поверхность рантайма."""
    runtime = importlib.import_module("report_core._report_core")
    tree = ast.parse(MAIN_STUB.read_text(encoding="utf-8"))

    stub_all: set[str] = set()
    for node in tree.body:
        match node:
            case ast.Assign(targets=[ast.Name(id="__all__")]):
                stub_all = set(ast.literal_eval(node.value))

    assert stub_all, "__all__ не найден в стабе _report_core"
    assert_same_sets(stub_all, public_names(runtime) | {"__version__"}, "__all__ стаба")


def test_package_exports_covered_by_stub() -> None:
    """Ручные реэкспорты пакета объявлены в стабе расширения."""
    package = importlib.import_module("report_core")
    readable_package = importlib.import_module("report_core.readable")

    stub_classes, stub_functions, stub_constants, _ = parse_stub(MAIN_STUB)
    main_surface = set(stub_classes) | stub_functions | stub_constants

    assert set(package.__all__) - {"readable"} <= main_surface, (
        "report_core.__all__ содержит имена, которых нет в _report_core.pyi"
    )

    readable_classes, readable_functions, readable_constants, _ = parse_stub(
        READABLE_STUB
    )
    readable_surface = set(readable_classes) | readable_functions | readable_constants

    assert set(readable_package.__all__) <= readable_surface, (
        "readable.__all__ содержит имена, которых нет в _readable.pyi"
    )
