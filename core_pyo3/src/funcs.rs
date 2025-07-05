use core_rs::{funcs, traits::ReadableCell};
use pyo3::prelude::*;

use crate::structs::readable::cell::WrapperCell;

#[inline]
#[pyfunction]
pub(crate) fn find_cell_by_coords(
    py: Python<'_>,
    row: u32,
    col: u16,
    cells: Vec<WrapperCell>,
) -> PyResult<Option<WrapperCell>> {
    py.allow_threads(|| {
        let cells = cells.iter().map(|c| &c.0).collect();

        Ok(funcs::find_cell_by_coords(row, col, cells)?.map(|c| WrapperCell(c.clone())))
    })
}

#[inline]
#[pyfunction]
pub(crate) fn find_value_by_coords(
    py: Python<'_>,
    row: u32,
    col: u16,
    cells: Vec<WrapperCell>,
) -> PyResult<Option<String>> {
    py.allow_threads(|| {
        let cells = cells.iter().map(|c| &c.0).collect();

        Ok(funcs::find_cell_by_coords(row, col, cells)?.map(|c| {
            let guard = c.read();
            guard.get_value()
        }))
    })
}

#[inline]
#[pyfunction]
pub(crate) fn find_cell_by_regex(
    py: Python<'_>,
    regex: String,
    cells: Vec<WrapperCell>,
) -> PyResult<Option<WrapperCell>> {
    py.allow_threads(|| {
        let cells = cells.iter().map(|c| &c.0).collect();

        Ok(funcs::find_cell_by_regex(regex, cells)?.map(|c| WrapperCell(c.clone())))
    })
}

#[inline]
#[pyfunction]
pub(crate) fn find_cell_by_str(
    py: Python<'_>,
    value: String,
    cells: Vec<WrapperCell>,
) -> PyResult<Option<WrapperCell>> {
    py.allow_threads(|| {
        let cells = cells.iter().map(|c| &c.0).collect();

        Ok(funcs::find_cell_by_str(value, cells)?.map(|c| WrapperCell(c.clone())))
    })
}

#[inline]
#[pyfunction]
pub(crate) fn find_cell_by_letter(
    py: Python<'_>,
    letter: String,
    cells: Vec<WrapperCell>,
) -> PyResult<Option<WrapperCell>> {
    py.allow_threads(|| {
        let cells = cells.iter().map(|c| &c.0).collect();

        Ok(funcs::find_cell_by_letter(letter, cells)?.map(|c| WrapperCell(c.clone())))
    })
}

#[inline]
#[pyfunction]
pub(crate) fn find_cells_by_regex(
    py: Python<'_>,
    regex: String,
    cells: Vec<WrapperCell>,
) -> PyResult<Vec<WrapperCell>> {
    py.allow_threads(|| {
        let cells = cells.iter().map(|c| &c.0).collect();

        Ok(funcs::find_cells_by_regex(regex, cells)?
            .into_iter()
            .map(|c| WrapperCell(c.clone()))
            .collect())
    })
}

#[inline]
#[pyfunction]
pub(crate) fn find_cells_by_str(
    py: Python<'_>,
    value: String,
    cells: Vec<WrapperCell>,
) -> PyResult<Vec<WrapperCell>> {
    py.allow_threads(|| {
        let cells = cells.iter().map(|c| &c.0).collect();

        Ok(funcs::find_cells_by_str(value, cells)?
            .into_iter()
            .map(|c| WrapperCell(c.clone()))
            .collect())
    })
}

#[inline]
#[pyfunction]
pub(crate) fn find_cells_for_rows_by_regex(
    py: Python<'_>,
    regex: String,
    col_stop: u16,
    cells: Vec<WrapperCell>,
) -> PyResult<Vec<WrapperCell>> {
    py.allow_threads(|| {
        let cells = cells.iter().map(|c| &c.0).collect();

        Ok(funcs::find_cells_for_rows_by_regex(regex, col_stop, cells)?
            .into_iter()
            .map(|c| WrapperCell(c.clone()))
            .collect())
    })
}

#[inline]
#[pyfunction]
pub(crate) fn find_cells_for_cols_by_regex(
    py: Python<'_>,
    regex: String,
    row_stop: u32,
    cells: Vec<WrapperCell>,
) -> PyResult<Vec<WrapperCell>> {
    py.allow_threads(|| {
        let cells = cells.iter().map(|c| &c.0).collect();

        Ok(funcs::find_cells_for_cols_by_regex(regex, row_stop, cells)?
            .into_iter()
            .map(|c| WrapperCell(c.clone()))
            .collect())
    })
}

#[inline]
#[pyfunction]
pub(crate) fn find_cells_multi_regex(
    py: Python<'_>,
    before_regex: String,
    after_regex: String,
    cells: Vec<WrapperCell>,
) -> PyResult<Vec<WrapperCell>> {
    py.allow_threads(|| {
        let cells = cells.iter().map(|c| &c.0).collect();

        Ok(
            funcs::find_cells_multi_regex(before_regex, after_regex, cells)?
                .into_iter()
                .map(|c| WrapperCell(c.clone()))
                .collect(),
        )
    })
}

#[inline]
#[pyfunction]
pub(crate) fn find_cells_between_regex(
    py: Python<'_>,
    before_regex: String,
    after_regex: String,
    cells: Vec<WrapperCell>,
) -> PyResult<Vec<WrapperCell>> {
    py.allow_threads(|| {
        let cells = cells.iter().map(|c| &c.0).collect();

        Ok(
            funcs::find_cells_between_regex(before_regex, after_regex, cells)?
                .into_iter()
                .map(|c| WrapperCell(c.clone()))
                .collect(),
        )
    })
}

#[inline]
#[pyfunction]
pub(crate) fn find_cells_range_rows(
    py: Python<'_>,
    start_row: u32,
    end_row: u32,
    cells: Vec<WrapperCell>,
) -> PyResult<Vec<WrapperCell>> {
    py.allow_threads(|| {
        let cells = cells.iter().map(|c| &c.0).collect();

        Ok(funcs::find_cells_range_rows(start_row, end_row, cells)?
            .into_iter()
            .map(|c| WrapperCell(c.clone()))
            .collect())
    })
}

#[inline]
#[pyfunction]
pub(crate) fn find_cells_range_cols(
    py: Python<'_>,
    start_col: u16,
    end_col: u16,
    cells: Vec<WrapperCell>,
) -> PyResult<Vec<WrapperCell>> {
    py.allow_threads(|| {
        let cells = cells.iter().map(|c| &c.0).collect();

        Ok(funcs::find_cells_range_cols(start_col, end_col, cells)?
            .into_iter()
            .map(|c| WrapperCell(c.clone()))
            .collect())
    })
}

#[inline]
#[pyfunction]
pub(crate) fn find_values_by_col_rows(
    py: Python<'_>,
    col: u16,
    rows: Vec<u32>,
    cells: Vec<WrapperCell>,
) -> PyResult<Vec<String>> {
    py.allow_threads(|| {
        let cells = cells.iter().map(|c| &c.0).collect();

        Ok(funcs::find_values_by_col_rows(col, rows, cells)?)
    })
}

#[inline]
#[pyfunction]
pub(crate) fn find_values_by_row_cols(
    py: Python<'_>,
    row: u32,
    cols: Vec<u16>,
    cells: Vec<WrapperCell>,
) -> PyResult<Vec<String>> {
    py.allow_threads(|| {
        let cells = cells.iter().map(|c| &c.0).collect();

        Ok(funcs::find_values_by_row_cols(row, cols, cells)?)
    })
}
