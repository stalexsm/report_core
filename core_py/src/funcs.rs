use core_rs::{funcs, traits::ReadableCell};
use pyo3::{exceptions::PyRuntimeError, prelude::*};

use crate::structs::readable::cell::WrapperCell;

#[inline]
#[pyfunction]
pub(crate) fn find_cell_by_coords(
    row: u32,
    col: u16,
    cells: Vec<WrapperCell>,
) -> PyResult<Option<WrapperCell>> {
    Python::with_gil(|_py| {
        let cells = cells.iter().map(|c| &c.0).collect();

        match funcs::find_cell_by_coords(row, col, cells) {
            Ok(cell) => {
                if let Some(cell) = cell {
                    Ok(Some(WrapperCell(cell.clone())))
                } else {
                    Ok(None)
                }
            }
            Err(e) => Err(PyRuntimeError::new_err(format!("{}", e))),
        }
    })
}

#[inline]
#[pyfunction]
pub(crate) fn find_value_by_coords(
    row: u32,
    col: u16,
    cells: Vec<WrapperCell>,
) -> PyResult<Option<String>> {
    Python::with_gil(|_py| {
        let cells = cells.iter().map(|c| &c.0).collect();

        match funcs::find_cell_by_coords(row, col, cells) {
            Ok(cell) => {
                if let Some(cell) = cell {
                    let guard = cell.read();
                    Ok(Some(guard.get_value()))
                } else {
                    Ok(None)
                }
            }
            Err(e) => Err(PyRuntimeError::new_err(format!("{}", e))),
        }
    })
}

#[inline]
#[pyfunction]
pub(crate) fn find_cell_by_regex(
    regex: String,
    cells: Vec<WrapperCell>,
) -> PyResult<Option<WrapperCell>> {
    Python::with_gil(|_py| {
        let cells = cells.iter().map(|c| &c.0).collect();

        match funcs::find_cell_by_regex(regex, cells) {
            Ok(cell) => {
                if let Some(cell) = cell {
                    Ok(Some(WrapperCell(cell.clone())))
                } else {
                    Ok(None)
                }
            }
            Err(e) => Err(PyRuntimeError::new_err(format!("{}", e))),
        }
    })
}

#[inline]
#[pyfunction]
pub(crate) fn find_cell_by_letter(
    letter: String,
    cells: Vec<WrapperCell>,
) -> PyResult<Option<WrapperCell>> {
    Python::with_gil(|_py| {
        let cells = cells.iter().map(|c| &c.0).collect();

        match funcs::find_cell_by_letter(letter, cells) {
            Ok(cell) => {
                if let Some(cell) = cell {
                    Ok(Some(WrapperCell(cell.clone())))
                } else {
                    Ok(None)
                }
            }
            Err(e) => Err(PyRuntimeError::new_err(format!("{}", e))),
        }
    })
}

#[inline]
#[pyfunction]
pub(crate) fn find_cells_by_regex(
    regex: String,
    cells: Vec<WrapperCell>,
) -> PyResult<Vec<WrapperCell>> {
    Python::with_gil(|_py| {
        let cells = cells.iter().map(|c| &c.0).collect();

        match funcs::find_cells_by_regex(regex, cells) {
            Ok(cells) => Ok(cells.into_iter().map(|c| WrapperCell(c.clone())).collect()),
            Err(e) => Err(PyRuntimeError::new_err(format!("{}", e))),
        }
    })
}

#[inline]
#[pyfunction]
pub(crate) fn find_cells_for_rows_by_regex(
    regex: String,
    col_stop: u16,
    cells: Vec<WrapperCell>,
) -> PyResult<Vec<WrapperCell>> {
    Python::with_gil(|_py| {
        let cells = cells.iter().map(|c| &c.0).collect();

        match funcs::find_cells_for_rows_by_regex(regex, col_stop, cells) {
            Ok(cells) => Ok(cells.into_iter().map(|c| WrapperCell(c.clone())).collect()),
            Err(e) => Err(PyRuntimeError::new_err(format!("{}", e))),
        }
    })
}

#[inline]
#[pyfunction]
pub(crate) fn find_cells_for_cols_by_regex(
    regex: String,
    row_stop: u32,
    cells: Vec<WrapperCell>,
) -> PyResult<Vec<WrapperCell>> {
    Python::with_gil(|_py| {
        let cells = cells.iter().map(|c| &c.0).collect();

        match funcs::find_cells_for_cols_by_regex(regex, row_stop, cells) {
            Ok(cells) => Ok(cells.into_iter().map(|c| WrapperCell(c.clone())).collect()),
            Err(e) => Err(PyRuntimeError::new_err(format!("{}", e))),
        }
    })
}

#[inline]
#[pyfunction]
pub(crate) fn find_cells_multi_regex(
    before_regex: String,
    after_regex: String,
    cells: Vec<WrapperCell>,
) -> PyResult<Vec<WrapperCell>> {
    Python::with_gil(|_py| {
        let cells = cells.iter().map(|c| &c.0).collect();

        match funcs::find_cells_multi_regex(before_regex, after_regex, cells) {
            Ok(cells) => Ok(cells.into_iter().map(|c| WrapperCell(c.clone())).collect()),
            Err(e) => Err(PyRuntimeError::new_err(format!("{}", e))),
        }
    })
}

#[inline]
#[pyfunction]
pub(crate) fn find_cells_between_regex(
    before_regex: String,
    after_regex: String,
    cells: Vec<WrapperCell>,
) -> PyResult<Vec<WrapperCell>> {
    Python::with_gil(|_py| {
        let cells = cells.iter().map(|c| &c.0).collect();

        match funcs::find_cells_between_regex(before_regex, after_regex, cells) {
            Ok(cells) => Ok(cells.into_iter().map(|c| WrapperCell(c.clone())).collect()),
            Err(e) => Err(PyRuntimeError::new_err(format!("{}", e))),
        }
    })
}

#[inline]
#[pyfunction]
pub(crate) fn find_cells_range_rows(
    start_row: u32,
    end_row: u32,
    cells: Vec<WrapperCell>,
) -> PyResult<Vec<WrapperCell>> {
    Python::with_gil(|_py| {
        let cells = cells.iter().map(|c| &c.0).collect();

        match funcs::find_cells_range_rows(start_row, end_row, cells) {
            Ok(cells) => Ok(cells.into_iter().map(|c| WrapperCell(c.clone())).collect()),
            Err(e) => Err(PyRuntimeError::new_err(format!("{}", e))),
        }
    })
}

#[inline]
#[pyfunction]
pub(crate) fn find_cells_range_cols(
    start_col: u16,
    end_col: u16,
    cells: Vec<WrapperCell>,
) -> PyResult<Vec<WrapperCell>> {
    Python::with_gil(|_py| {
        let cells = cells.iter().map(|c| &c.0).collect();

        match funcs::find_cells_range_cols(start_col, end_col, cells) {
            Ok(cells) => Ok(cells.into_iter().map(|c| WrapperCell(c.clone())).collect()),
            Err(e) => Err(PyRuntimeError::new_err(format!("{}", e))),
        }
    })
}

#[inline]
#[pyfunction]
pub(crate) fn find_values_by_col_rows(
    col: u16,
    rows: Vec<u32>,
    cells: Vec<WrapperCell>,
) -> PyResult<Vec<String>> {
    Python::with_gil(|_py| {
        let cells = cells.iter().map(|c| &c.0).collect();

        match funcs::find_values_by_col_rows(col, rows, cells) {
            Ok(cells) => Ok(cells),
            Err(e) => Err(PyRuntimeError::new_err(format!("{}", e))),
        }
    })
}

#[inline]
#[pyfunction]
pub(crate) fn find_values_by_row_cols(
    row: u32,
    cols: Vec<u16>,
    cells: Vec<WrapperCell>,
) -> PyResult<Vec<String>> {
    Python::with_gil(|_py| {
        let cells = cells.iter().map(|c| &c.0).collect();

        match funcs::find_values_by_row_cols(row, cols, cells) {
            Ok(cells) => Ok(cells),
            Err(e) => Err(PyRuntimeError::new_err(format!("{}", e))),
        }
    })
}
