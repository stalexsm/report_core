use std::sync::Arc;

use core_rs::{
    structs::{coordinate::Coordinate, range::MergedRange, sheet::Sheet},
    traits::ReadableSheet,
};
use parking_lot::RwLock;
use pyo3::{prelude::*, types::PyString};

use super::cell::WrapperCell;

#[pyclass]
#[pyo3(module = "readable", name = "ReadableSheet")]
#[derive(Debug, Clone)]
pub struct WrapperSheet(pub(crate) Arc<RwLock<Sheet>>);

#[pymethods]
impl WrapperSheet {
    pub fn __repr__(slf: &Bound<'_, Self>) -> PyResult<String> {
        Python::with_gil(|_py| {
            let class_name: Bound<'_, PyString> = slf.get_type().qualname()?;

            let slf = slf.borrow();
            let slf = slf.0.read();

            Ok(format!(
                "{} ({}) cells: {}",
                class_name,
                slf.get_name(),
                slf.get_cell_collection().len()
            ))
        })
    }

    #[getter]
    pub fn name(&self) -> PyResult<String> {
        Python::with_gil(|_py| {
            let slf = self.0.read();
            Ok(slf.get_name())
        })
    }

    #[getter]
    pub fn get_max_row(&self) -> PyResult<u32> {
        Python::with_gil(|_py| {
            let slf = self.0.read();

            Ok(slf.get_max_row())
        })
    }

    #[getter]
    pub fn get_max_column(&self) -> PyResult<u16> {
        Python::with_gil(|_py| {
            let slf = self.0.read();

            Ok(slf.get_max_column())
        })
    }

    #[getter]
    pub fn get_cells(&self) -> PyResult<Vec<WrapperCell>> {
        Python::with_gil(|_py| {
            let slf = self.0.read();

            let cells = slf
                .get_cell_collection_sorted()
                .into_iter()
                .map(|cell| WrapperCell(cell.clone()))
                .collect();

            Ok(cells)
        })
    }

    #[getter]
    pub fn get_merge_cells(&self) -> PyResult<Vec<MergedRange>> {
        Python::with_gil(|_py| {
            let merged_cells = self
                .0
                .read()
                .get_merge_cell_collection()
                .iter()
                .map(|range| range.into())
                .collect();

            Ok(merged_cells)
        })
    }

    pub fn get_value_cell(&self, row: u32, col: u16) -> PyResult<String> {
        Python::with_gil(|_py| {
            let slf = self.0.read();

            let coord = Coordinate::from((row, col));
            Ok(slf.get_cell_value(coord))
        })
    }

    #[pyo3(signature = (start_row=None, end_row=None, start_col=None, end_col=None))]
    pub fn get_cells_by_range(
        &self,
        start_row: Option<u32>,
        end_row: Option<u32>,
        start_col: Option<u16>,
        end_col: Option<u16>,
    ) -> PyResult<Vec<WrapperCell>> {
        Python::with_gil(|_py| {
            let slf = self.0.read();

            let cells = slf
                .get_cell_collection_by_range(start_row, end_row, start_col, end_col)
                .map(|cell| WrapperCell(cell.clone()))
                .collect::<Vec<_>>();

            Ok(cells)
        })
    }

    pub fn find_cell_by_regex(&self, regex: &str) -> PyResult<Option<WrapperCell>> {
        Python::with_gil(|_py| {
            let slf = self.0.read();

            match slf.find_cell_by_regex(regex) {
                Ok(cell) => Ok(cell.map(|c| WrapperCell(c.clone()))),
                Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(e.to_string())),
            }
        })
    }

    pub fn find_cell_by_letter(&self, letter: &str) -> PyResult<Option<WrapperCell>> {
        Python::with_gil(|_py| {
            let slf = self.0.read();

            match slf.find_cell_by_letter(letter) {
                Ok(cell) => Ok(cell.map(|c| WrapperCell(c.clone()))),
                Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(e.to_string())),
            }
        })
    }

    pub fn find_cells_by_regex(&self, regex: &str) -> PyResult<Vec<WrapperCell>> {
        Python::with_gil(|_py| {
            let slf = self.0.read();

            match slf.find_cells_by_regex(regex) {
                Ok(cells) => {
                    let cells = cells
                        .into_iter()
                        .map(|cell| WrapperCell(cell.clone()))
                        .collect();

                    Ok(cells)
                }
                Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(e.to_string())),
            }
        })
    }

    pub fn find_cells_for_rows_by_regex(
        &self,
        regex: &str,
        col_stop: u16,
    ) -> PyResult<Vec<WrapperCell>> {
        Python::with_gil(|_py| {
            let slf = self.0.read();

            match slf.find_cells_for_rows_by_regex(regex, col_stop) {
                Ok(cells) => {
                    let cells = cells
                        .into_iter()
                        .map(|cell| WrapperCell(cell.clone()))
                        .collect();

                    Ok(cells)
                }
                Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(e.to_string())),
            }
        })
    }

    pub fn find_cells_for_cols_by_regex(
        &self,
        regex: &str,
        row_stop: u32,
    ) -> PyResult<Vec<WrapperCell>> {
        Python::with_gil(|_py| {
            let slf = self.0.read();

            match slf.find_cells_for_cols_by_regex(regex, row_stop) {
                Ok(cells) => {
                    let cells = cells
                        .into_iter()
                        .map(|cell| WrapperCell(cell.clone()))
                        .collect();

                    Ok(cells)
                }
                Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(e.to_string())),
            }
        })
    }

    pub fn find_cells_multi_regex(
        &self,
        before_regex: &str,
        after_regex: &str,
    ) -> PyResult<Vec<WrapperCell>> {
        Python::with_gil(|_py| {
            let slf = self.0.read();

            match slf.find_cells_multi_regex(before_regex, after_regex) {
                Ok(cells) => {
                    let cells = cells
                        .into_iter()
                        .map(|cell| WrapperCell(cell.clone()))
                        .collect();

                    Ok(cells)
                }
                Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(e.to_string())),
            }
        })
    }

    pub fn find_cells_between_regex(
        &self,
        before_regex: &str,
        after_regex: &str,
    ) -> PyResult<Vec<WrapperCell>> {
        Python::with_gil(|_py| {
            let slf = self.0.read();

            match slf.find_cells_between_regex(before_regex, after_regex) {
                Ok(cells) => {
                    let cells = cells
                        .into_iter()
                        .map(|cell| WrapperCell(cell.clone()))
                        .collect();

                    Ok(cells)
                }
                Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(e.to_string())),
            }
        })
    }

    pub fn find_cells_range_rows(
        &self,
        start_row: u32,
        end_row: u32,
    ) -> PyResult<Vec<WrapperCell>> {
        Python::with_gil(|_py| {
            let slf = self.0.read();

            match slf.find_cells_range_rows(start_row, end_row) {
                Ok(cells) => {
                    let cells = cells
                        .into_iter()
                        .map(|cell| WrapperCell(cell.clone()))
                        .collect();

                    Ok(cells)
                }
                Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(e.to_string())),
            }
        })
    }

    pub fn find_cells_range_cols(
        &self,
        start_col: u16,
        end_col: u16,
    ) -> PyResult<Vec<WrapperCell>> {
        Python::with_gil(|_py| {
            let slf = self.0.read();

            match slf.find_cells_range_cols(start_col, end_col) {
                Ok(cells) => {
                    let cells = cells
                        .into_iter()
                        .map(|cell| WrapperCell(cell.clone()))
                        .collect();

                    Ok(cells)
                }
                Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(e.to_string())),
            }
        })
    }
}
