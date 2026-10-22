#!/usr/bin/env bash
# Регенерация .pyi-стабов Python-пакета из собранного Rust-расширения.
#
# Как работает: фича pyo3 `experimental-inspect` шьёт метаданные API в бинарь,
# `pyo3-introspection` извлекает их и строит стабы. Руками стабы не правим —
# только перегенерацией.
#
# Генерация идёт под самую старую поддерживаемую версию Python
# (requires-python = 3.12): от этого зависят условные импорты в стабах
# (например, `disjoint_base` берётся из typing_extensions, а не typing).
#
# Требования:
#   - pyo3-introspection той же версии, что pyo3 в Cargo.lock:
#       cargo install pyo3-introspection --locked --version <версия-pyo3>
#   - интерпретатор python3.12 (переопределяется переменной PYO3_STUBS_PYTHON)
#   - ruff (нормализует порядок импортов в сгенерированных стабах)
set -euo pipefail
cd "$(dirname "$0")/.."

# Версия генератора должна совпадать с версией pyo3: формат метаданных
# меняется вместе с релизами pyo3
PYO3_VERSION=$(awk -F'"' '/^name = "pyo3"$/{getline; print $2; exit}' Cargo.lock)
INTERPRETER=${PYO3_STUBS_PYTHON:-python3.12}

if ! command -v pyo3-introspection >/dev/null 2>&1; then
  echo "pyo3-introspection не найден. Установите:" >&2
  echo "  cargo install pyo3-introspection --locked --version ${PYO3_VERSION}" >&2
  exit 1
fi

# Сборка в dev-профиле: release-профиль проекта включает strip=true,
# который вырезает метаданные интроспекции из бинаря
PYO3_PYTHON="$(command -v "${INTERPRETER}")" cargo build -p core_pyo3 --features extension-module

LIB="target/debug/lib_report_core.dylib"
if [[ "$(uname -s)" == "Linux" ]]; then
  LIB="target/debug/lib_report_core.so"
fi

OUT=$(mktemp -d)
trap 'rm -rf "${OUT}"' EXIT
pyo3-introspection "${LIB}" _report_core "${OUT}"

# Раскладка по пакету: стаб модуля _report_core + стаб сабмодуля readable
mkdir -p core_pyo3/python/report_core/readable
cp "${OUT}/__init__.pyi" core_pyo3/python/report_core/_report_core.pyi
cp "${OUT}/readable.pyi" core_pyo3/python/report_core/readable/_readable.pyi

# Генератор эмитит импорты не в том порядке, который требует ruff (I001)
ruff check --fix core_pyo3/python/report_core >/dev/null

echo "Стабы обновлены:"
git status --short core_pyo3/python/report_core
