# Генерация .pyi-стабов: pyo3 `experimental-inspect`

Исследование официального механизма генерации стабов PyO3 и его внедрение в `core_pyo3`.
Актуально для pyo3/pyo3-introspection 0.30.1, maturin 1.15.0.

## Как это устроено

1. Макросы `#[pyclass]`/`#[pymodule]`/`#[pyfunction]` при включённой фиче `experimental-inspect`
   шьют JSON-метаданные API в бинарь расширения (детерминированно с 0.29.0, [#6076](https://github.com/PyO3/pyo3/pull/6076)).
2. Крейт `pyo3-introspection` (первый-party, репозиторий [PyO3/pyo3](https://github.com/pyo3/pyo3),
   автор «PyO3 Project and Contributors», версии в такт pyo3) статически извлекает метаданные
   из бинаря (парсер goblin, без запуска Python) и строит `.pyi`.
3. `maturin` тоже умеет генерировать стабы, но тащит собственную (отстающую) копию генератора —
   см. «Инструменты».

До стабилизации фичи единственным рабочим вариантом был сторонний
[pyo3-stub-gen](https://github.com/Jij-Inc/pyo3-stub-gen) (Jij-Inc) — он упоминался
в гайде PyO3 как «лучшее решение»; официальный механизм его вытесняет.

## Что дал релиз 0.29.0 (июнь 2026)

Цитата из [release notes](https://github.com/PyO3/pyo3/releases/tag/v0.29.0): «New in this release
is a CLI in `pyo3-introspection` to generate type stubs along with the `experimental-inspect`
feature. Downstream, `maturin` has also gained support to generate type stubs…».

Ключевые изменения фичи ([CHANGELOG](https://github.com/PyO3/pyo3/blob/main/CHANGELOG.md), 0.29.0):

- докстринги из Rust `///` попадают в стабы [#5782](https://github.com/PyO3/pyo3/pull/5782);
- базовые классы [#5331](https://github.com/PyO3/pyo3/pull/5331), `@typing.final` [#5552](https://github.com/PyO3/pyo3/pull/5552);
- `TypeHint`/`PyStaticExpr` вместо `TypeInfo`; аннотации аргументов и возвратов
  для всех нативных типов [#5438](https://github.com/PyO3/pyo3/pull/5438),
  [#5634](https://github.com/PyO3/pyo3/pull/5634); `_typeshed.Incomplete` как дефолт
  вместо `typing.Any` [#5744](https://github.com/PyO3/pyo3/pull/5744);
- `INPUT_TYPE`/`OUTPUT_TYPE` для сторонних крейтов [#5770](https://github.com/PyO3/pyo3/pull/5770);
- относительные импорты в стабах [#6097](https://github.com/PyO3/pyo3/pull/6097);
- аннотации в `#[pyo3(signature = ...)]` разрешены без фичи (игнорируются) [#5999](https://github.com/PyO3/pyo3/pull/5999).

Позже добавлено (0.29.x–0.30.x): `__all__` для полных модулей
[#6242](https://github.com/PyO3/pyo3/pull/6242), реэкспорт сабмодулей в стабах родителя
[#6360](https://github.com/PyO3/pyo3/pull/6360), `@disjoint_base` (PEP 800)
[#6362](https://github.com/PyO3/pyo3/pull/6362), `SupportsIndex`/`SupportsFloat` на входах
[#6396](https://github.com/PyO3/pyo3/pull/6396), `SupportsGetItem` на `Vec<T>`-входах
[#6413](https://github.com/PyO3/pyo3/pull/6413).

## Инструменты генерации

| Инструмент | Версия генератора | Примечание |
|---|---|---|
| `pyo3-introspection` (cargo) | = версии pyo3 | рекомендуемый путь, полные стабы (`__all__`, реэкспорт сабмодулей) |
| `maturin generate-stubs` | зашита в maturin 1.15.0, старее pyo3 0.30.1 | без `__all__` и реэкспорта сабмодулей |
| `maturin build --generate-stubs` | тот же | на нашем layout падает (`No module named _report_core found`) |

## Что внедрено в проекте

- pyo3 0.30.1, фича `experimental-inspect` включена постоянно (+80 КБ к бинарю, ~3,4%).
- Модуль `_report_core` переведён на декларативный синтаксис (`#[pymodule] mod`,
  `#[pymodule_export]`) — обязательное условие интроспекции; сабмодуль `readable` —
  вложенный `#[pymodule] mod` (`submodule` для вложенных неявный, явно указывать нельзя).
- Регистрация `sys.modules["report_core.readable"]` перенесена из Rust в
  `report_core/__init__.py`: `#[pymodule_init]` с аргументом-модулем помечал бы модуль
  как `Incomplete` в стабах.
- Русские докстринги перенесены из рукописных `.pyi` в Rust `///` — единственный
  источник документации; стабы генерируются уже с ними (попутно исправлены опечатки:
  «Корирование», «видисмости», «объедененных», «ячeек» и др.).
- Точечные аннотации `#[pyo3(signature = (cells: "list[ReadableCell]"))]` в функциях
  `readable` — класс из того же модуля стаба, импорт не нужен.
- `scripts/generate-stubs.sh` — регенерация: debug-сборка под python 3.12 (старейшая
  поддерживаемая), `pyo3-introspection`, раскладка стабов по пакету, нормализация
  импортов ruff.
- Рукописные стабы заменены сгенерированными (`_report_core.pyi`, `readable/_readable.pyi`).
- Гейт «Стабы» в `docs/agents/git-workflow.md`, правило — в корневом `AGENTS.md`.

## Ограничения (проверено на практике)

- Интроспекция работает только с декларативным `#[pymodule] mod`; модуль-функция
  не поддерживается (докладано в гайде).
- `strip = true` в release-профиле вырезает метаданные — генерация только из
  debug-сборки (скрипт это учитывает).
- Сборка должна идти под целевой интерпретатор: от версии Python зависят условные
  импорты стабов (`disjoint_base`: `typing_extensions` до 3.15, `typing` с 3.15).
  Генерируем под 3.12 = нижняя граница `requires-python`.
- Строковые аннотации в `#[pyo3(signature = (...))]` эмитятся дословно в кавычках,
  и импорты для имён из них **не добавляются**: допустимы только builtins и имена,
  уже доступные в модуле стаба. Поэтому `"list[Any]"` в `readable` (нет импорта `Any`)
  и `"Literal[...]"` не используются; вместо Literal — описание допустимых значений
  в докстринге.
- Сабмодуль, объявленный вложенным `#[pymodule] mod`, получает `__name__`
  вида `_report_core.readable` (раньше был просто `readable`).

## Источники

- Гайд: Type stub generation and introspection — https://pyo3.rs/latest/type-stub
  (исходник в крейте pyo3: `guide/src/type-stub.md`)
- Фичи pyo3 — https://pyo3.rs/main/features (`experimental-inspect`)
- Релиз 0.29.0 — https://github.com/PyO3/pyo3/releases/tag/v0.29.0
- CHANGELOG pyo3 — https://github.com/PyO3/pyo3/blob/main/CHANGELOG.md
- maturin: `--generate-stubs` — https://www.maturin.rs/distribution.html
- Трекинг фичи — https://github.com/PyO3/pyo3/issues/5137
