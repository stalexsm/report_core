use std::sync::Arc;

use core_rs::{funcs, traits::ReadableCell};
use pyo3::{prelude::*, types::PyList};

use crate::structs::readable::cell::WrapperCell;

/// вспомогательная функция для преобразования PyList во WrapperCell
fn extract_pylist(cells: &Bound<'_, PyList>) -> PyResult<Vec<WrapperCell>> {
    let mut wrapper_cells = Vec::with_capacity(cells.len());
    for item in cells.iter() {
        if let Ok(cell) = item.extract::<WrapperCell>() {
            wrapper_cells.push(cell);
        } else {
            wrapper_cells.push(WrapperCell::try_from(&item)?);
        }
    }

    Ok(wrapper_cells)
}

/// Поиск ячейки по координатам в списке ячеек
/// ---
///
/// Arguments:
/// ---
///     row: int
///         Строка ячейки
///     col: int
///         Колонка ячейки
///     cells: list[ReadableCell]
///         Список ячеек
///
/// Returns:
/// ---
///     ReadableCell | None
#[inline]
#[pyfunction]
#[pyo3(signature = (row, col, cells: "list[ReadableCell]"))]
pub(crate) fn find_cell_by_coords(
    py: Python<'_>,
    row: u32,
    col: u16,
    cells: &Bound<'_, PyList>,
) -> PyResult<Option<WrapperCell>> {
    let cells = extract_pylist(cells)?;

    py.detach(|| {
        let cells = cells.iter().map(|c| &c.0).collect();
        Ok(funcs::find_cell_by_coords(row, col, cells)?.map(|c| WrapperCell(Arc::clone(c))))
    })
}

/// Поиск значения ячейки по координатам в списке ячеек
/// ---
///
/// Arguments:
/// ---
///     row: int
///         Строка ячейки
///     col: int
///         Колонка ячейки
///     cells: list[ReadableCell]
///         Список ячеек
///
/// Returns:
/// ---
///     str | None
#[inline]
#[pyfunction]
#[pyo3(signature = (row, col, cells: "list[ReadableCell]"))]
pub(crate) fn find_value_by_coords(
    py: Python<'_>,
    row: u32,
    col: u16,
    cells: &Bound<'_, PyList>,
) -> PyResult<Option<String>> {
    let cells = extract_pylist(cells)?;

    py.detach(|| {
        let cells = cells.iter().map(|c| &c.0).collect();
        Ok(funcs::find_cell_by_coords(row, col, cells)?.map(|c| {
            let guard = c.read();
            guard.get_value()
        }))
    })
}

/// Поиск ячейки по регулярному выражению в списке ячеек
/// ---
///
/// Arguments:
/// ---
///     regex: str
///         Шаблон (регулярное выражение)
///     cells: list[ReadableCell]
///         Список ячеек
///
/// Returns:
/// ---
///     ReadableCell | None
#[inline]
#[pyfunction]
#[pyo3(signature = (regex, cells: "list[ReadableCell]"))]
pub(crate) fn find_cell_by_regex(
    py: Python<'_>,
    regex: String,
    cells: &Bound<'_, PyList>,
) -> PyResult<Option<WrapperCell>> {
    let cells = extract_pylist(cells)?;

    py.detach(|| {
        let cells = cells.iter().map(|c| &c.0).collect();
        Ok(funcs::find_cell_by_regex(regex, cells)?.map(|c| WrapperCell(Arc::clone(c))))
    })
}

/// Поиск ячейки по строковому значению в списке ячеек
/// ---
///
/// Arguments:
/// ---
///     value: str
///         Строка (значение)
///     cells: list[ReadableCell]
///         Список ячеек
///
/// Returns:
/// ---
///     ReadableCell | None
#[inline]
#[pyfunction]
#[pyo3(signature = (value, cells: "list[ReadableCell]"))]
pub(crate) fn find_cell_by_str(
    py: Python<'_>,
    value: String,
    cells: &Bound<'_, PyList>,
) -> PyResult<Option<WrapperCell>> {
    let cells = extract_pylist(cells)?;

    py.detach(|| {
        let cells = cells.iter().map(|c| &c.0).collect();
        Ok(funcs::find_cell_by_str(value, cells)?.map(|c| WrapperCell(Arc::clone(c))))
    })
}

/// Поиск ячейки по буквенной координате (A1) в списке ячеек
/// ---
///
/// Arguments:
/// ---
///     letter: str
///         Координата (буквенная) A1
///     cells: list[ReadableCell]
///         Список ячеек
///
/// Returns:
/// ---
///     ReadableCell | None
#[inline]
#[pyfunction]
#[pyo3(signature = (letter, cells: "list[ReadableCell]"))]
pub(crate) fn find_cell_by_letter(
    py: Python<'_>,
    letter: String,
    cells: &Bound<'_, PyList>,
) -> PyResult<Option<WrapperCell>> {
    let cells = extract_pylist(cells)?;

    py.detach(|| {
        let cells = cells.iter().map(|c| &c.0).collect();
        Ok(funcs::find_cell_by_letter(letter, cells)?.map(|c| WrapperCell(Arc::clone(c))))
    })
}

/// Поиск ячеек по регулярному выражению в списке ячеек
/// ---
///
/// Arguments:
/// ---
///     regex: str
///         Шаблон (регулярное выражение)
///     cells: list[ReadableCell]
///         Список ячеек
///
/// Returns:
/// ---
///     list[ReadableCell]
#[inline]
#[pyfunction]
#[pyo3(signature = (regex, cells: "list[ReadableCell]"))]
pub(crate) fn find_cells_by_regex(
    py: Python<'_>,
    regex: String,
    cells: &Bound<'_, PyList>,
) -> PyResult<Vec<WrapperCell>> {
    let cells = extract_pylist(cells)?;

    py.detach(|| {
        let cells = cells.iter().map(|c| &c.0).collect();
        Ok(funcs::find_cells_by_regex(regex, cells)?
            .into_iter()
            .map(|c| WrapperCell(Arc::clone(c)))
            .collect())
    })
}

/// Поиск ячеек по строковому значению в списке ячеек
/// ---
///
/// Arguments:
/// ---
///     value: str
///         Строка (значение)
///     cells: list[ReadableCell]
///         Список ячеек
///
/// Returns:
/// ---
///     list[ReadableCell]
#[inline]
#[pyfunction]
#[pyo3(signature = (value, cells: "list[ReadableCell]"))]
pub(crate) fn find_cells_by_str(
    py: Python<'_>,
    value: String,
    cells: &Bound<'_, PyList>,
) -> PyResult<Vec<WrapperCell>> {
    let cells = extract_pylist(cells)?;

    py.detach(|| {
        let cells = cells.iter().map(|c| &c.0).collect();
        Ok(funcs::find_cells_by_str(value, cells)?
            .into_iter()
            .map(|c| WrapperCell(Arc::clone(c)))
            .collect())
    })
}

/// Поиск ячеек по регулярному выражению до заданной колонки
/// ---
///
/// Arguments:
/// ---
///     regex: str
///         Шаблон (регулярное выражение)
///     col_stop: int
///         Значение колонки до которой забирать ячейки
///     cells: list[ReadableCell]
///         Список ячеек
///
/// Returns:
/// ---
///     list[ReadableCell]
#[inline]
#[pyfunction]
#[pyo3(signature = (regex, col_stop, cells: "list[ReadableCell]"))]
pub(crate) fn find_cells_for_rows_by_regex(
    py: Python<'_>,
    regex: String,
    col_stop: u16,
    cells: &Bound<'_, PyList>,
) -> PyResult<Vec<WrapperCell>> {
    let cells = extract_pylist(cells)?;

    py.detach(|| {
        let cells = cells.iter().map(|c| &c.0).collect();
        Ok(funcs::find_cells_for_rows_by_regex(regex, col_stop, cells)?
            .into_iter()
            .map(|c| WrapperCell(Arc::clone(c)))
            .collect())
    })
}

/// Поиск ячеек по регулярному выражению до заданной строки
/// ---
///
/// Arguments:
/// ---
///     regex: str
///         Шаблон (регулярное выражение)
///     row_stop: int
///         Значение строки до которой забирать ячейки
///     cells: list[ReadableCell]
///         Список ячеек
///
/// Returns:
/// ---
///     list[ReadableCell]
#[inline]
#[pyfunction]
#[pyo3(signature = (regex, row_stop, cells: "list[ReadableCell]"))]
pub(crate) fn find_cells_for_cols_by_regex(
    py: Python<'_>,
    regex: String,
    row_stop: u32,
    cells: &Bound<'_, PyList>,
) -> PyResult<Vec<WrapperCell>> {
    let cells = extract_pylist(cells)?;

    py.detach(|| {
        let cells = cells.iter().map(|c| &c.0).collect();
        Ok(funcs::find_cells_for_cols_by_regex(regex, row_stop, cells)?
            .into_iter()
            .map(|c| WrapperCell(Arc::clone(c)))
            .collect())
    })
}

/// Поиск ячеек по двум регулярным выражениям в списке ячеек
/// ---
///
/// Arguments:
/// ---
///     before_regex: str
///         Первый шаблон (регулярное выражение)
///     after_regex: str
///         Второй шаблон (регулярное выражение)
///     cells: list[ReadableCell]
///         Список ячеек
///
/// Returns:
/// ---
///     list[ReadableCell]
#[inline]
#[pyfunction]
#[pyo3(signature = (before_regex, after_regex, cells: "list[ReadableCell]"))]
pub(crate) fn find_cells_multi_regex(
    py: Python<'_>,
    before_regex: String,
    after_regex: String,
    cells: &Bound<'_, PyList>,
) -> PyResult<Vec<WrapperCell>> {
    let cells = extract_pylist(cells)?;

    py.detach(|| {
        let cells = cells.iter().map(|c| &c.0).collect();
        Ok(
            funcs::find_cells_multi_regex(before_regex, after_regex, cells)?
                .into_iter()
                .map(|c| WrapperCell(Arc::clone(c)))
                .collect(),
        )
    })
}

/// Поиск ячеек от первого регулярного выражения до второго
/// ---
///
/// Arguments:
/// ---
///     before_regex: str
///         Первый шаблон (регулярное выражение)
///     after_regex: str
///         Второй шаблон (регулярное выражение)
///     cells: list[ReadableCell]
///         Список ячеек
///
/// Returns:
/// ---
///     list[ReadableCell]
#[inline]
#[pyfunction]
#[pyo3(signature = (before_regex, after_regex, cells: "list[ReadableCell]"))]
pub(crate) fn find_cells_between_regex(
    py: Python<'_>,
    before_regex: String,
    after_regex: String,
    cells: &Bound<'_, PyList>,
) -> PyResult<Vec<WrapperCell>> {
    let cells = extract_pylist(cells)?;

    py.detach(|| {
        let cells = cells.iter().map(|c| &c.0).collect();
        Ok(
            funcs::find_cells_between_regex(before_regex, after_regex, cells)?
                .into_iter()
                .map(|c| WrapperCell(Arc::clone(c)))
                .collect(),
        )
    })
}

/// Поиск ячеек в диапазоне строк
/// ---
///
/// Arguments:
/// ---
///     start_row: int
///         Стартовая строка
///     end_row: int
///         Финишная строка
///     cells: list[ReadableCell]
///         Список ячеек
///
/// Returns:
/// ---
///     list[ReadableCell]
#[inline]
#[pyfunction]
#[pyo3(signature = (start_row, end_row, cells: "list[ReadableCell]"))]
pub(crate) fn find_cells_range_rows(
    py: Python<'_>,
    start_row: u32,
    end_row: u32,
    cells: &Bound<'_, PyList>,
) -> PyResult<Vec<WrapperCell>> {
    let cells = extract_pylist(cells)?;

    py.detach(|| {
        let cells = cells.iter().map(|c| &c.0).collect();
        Ok(funcs::find_cells_range_rows(start_row, end_row, cells)?
            .into_iter()
            .map(|c| WrapperCell(Arc::clone(c)))
            .collect())
    })
}

/// Поиск ячеек в диапазоне колонок
/// ---
///
/// Arguments:
/// ---
///     start_col: int
///         Стартовая колонка
///     end_col: int
///         Финишная колонка
///     cells: list[ReadableCell]
///         Список ячеек
///
/// Returns:
/// ---
///     list[ReadableCell]
#[inline]
#[pyfunction]
#[pyo3(signature = (start_col, end_col, cells: "list[ReadableCell]"))]
pub(crate) fn find_cells_range_cols(
    py: Python<'_>,
    start_col: u16,
    end_col: u16,
    cells: &Bound<'_, PyList>,
) -> PyResult<Vec<WrapperCell>> {
    let cells = extract_pylist(cells)?;

    py.detach(|| {
        let cells = cells.iter().map(|c| &c.0).collect();
        Ok(funcs::find_cells_range_cols(start_col, end_col, cells)?
            .into_iter()
            .map(|c| WrapperCell(Arc::clone(c)))
            .collect())
    })
}

/// Поиск значений ячеек по колонке и списку строк
/// ---
///
/// Arguments:
/// ---
///     col: int
///         Колонка
///     rows: list[int]
///         Список строк
///     cells: list[ReadableCell]
///         Список ячеек
///
/// Returns:
/// ---
///     list[str]
#[inline]
#[pyfunction]
#[pyo3(signature = (col, rows, cells: "list[ReadableCell]"))]
pub(crate) fn find_values_by_col_rows(
    py: Python<'_>,
    col: u16,
    rows: Vec<u32>,
    cells: &Bound<'_, PyList>,
) -> PyResult<Vec<String>> {
    let cells = extract_pylist(cells)?;

    py.detach(|| {
        let cells = cells.iter().map(|c| &c.0).collect();
        Ok(funcs::find_values_by_col_rows(col, rows, cells)?)
    })
}

/// Поиск значений ячеек по строке и списку колонок
/// ---
///
/// Arguments:
/// ---
///     row: int
///         Строка
///     cols: list[int]
///         Список колонок
///     cells: list[ReadableCell]
///         Список ячеек
///
/// Returns:
/// ---
///     list[str]
#[inline]
#[pyfunction]
#[pyo3(signature = (row, cols, cells: "list[ReadableCell]"))]
pub(crate) fn find_values_by_row_cols(
    py: Python<'_>,
    row: u32,
    cols: Vec<u16>,
    cells: &Bound<'_, PyList>,
) -> PyResult<Vec<String>> {
    let cells = extract_pylist(cells)?;

    py.detach(|| {
        let cells = cells.iter().map(|c| &c.0).collect();
        Ok(funcs::find_values_by_row_cols(row, cols, cells)?)
    })
}
