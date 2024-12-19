use core_rs::writer::{sheet::XLSXSheet, DEFAULT_COL, DEFAULT_ROW};
use parking_lot::Mutex;
use pyo3::{exceptions::PyRuntimeError, prelude::*};
use std::sync::Arc;

use crate::utils::raw_value_to_py;

use super::cell::WrapperXLSXSheetCell;

#[pyclass]
#[pyo3(module = "report_core", name = "XLSXSheet")]
#[derive(Debug, Clone)]
pub struct WrapperXLSXSheet(pub(crate) Arc<Mutex<XLSXSheet>>);

#[pymethods]
impl WrapperXLSXSheet {
    pub fn __repr__(slf: &Bound<'_, Self>) -> PyResult<String> {
        Python::with_gil(|_py| {
            let slf = slf.borrow();
            let slf_lock = slf.0.lock();
            Ok(format!(
                "XLSXSheet ({}) cells: {}",
                slf_lock.name,
                slf_lock.cells().count()
            ))
        })
    }

    #[new]
    #[pyo3(signature=(name, index, rows=DEFAULT_ROW, cols=DEFAULT_COL))]
    pub fn new(name: String, index: i32, rows: u32, cols: u16) -> PyResult<Self> {
        Python::with_gil(|_py| {
            let sheet = XLSXSheet::new(name, index, rows, cols);

            Ok(Self(sheet))
        })
    }

    #[getter]
    pub fn name(&self) -> PyResult<String> {
        Python::with_gil(|_py| Ok(self.0.lock().name.clone()))
    }

    #[getter]
    pub fn max_row(&self) -> PyResult<u32> {
        Python::with_gil(|_py| Ok(self.0.lock().max_row))
    }

    #[getter]
    pub fn max_column(&self) -> PyResult<u16> {
        Python::with_gil(|_py| Ok(self.0.lock().max_column))
    }

    #[getter]
    pub fn index(&self) -> PyResult<i32> {
        Python::with_gil(|_py| Ok(self.0.lock().index))
    }

    #[getter]
    pub fn cells(&self) -> PyResult<Vec<WrapperXLSXSheetCell>> {
        Python::with_gil(|_py| {
            let sheet = self.0.lock();
            let cells = sheet
                .cells()
                .map(|c| WrapperXLSXSheetCell(Arc::clone(c)))
                .collect();

            Ok(cells)
        })
    }

    pub fn write_cell(
        &mut self,
        row: u32,
        col: u16,
        value: String,
    ) -> PyResult<WrapperXLSXSheetCell> {
        Python::with_gil(|_py| {
            self.0
                .lock()
                .write_cell(row, col, &value)
                .map(|cell| WrapperXLSXSheetCell(Arc::clone(&cell)))
                .map_err(|e| PyRuntimeError::new_err(format!("Failed to write cell: {}", e)))
        })
    }

    pub fn delete_cols(&mut self, idx: u16, cols: u16) -> PyResult<()> {
        Python::with_gil(|_py| {
            self.0
                .lock()
                .delete_cols(idx, cols)
                .map_err(|e| PyRuntimeError::new_err(format!("Failed to delete cols: {}", e)))
        })
    }

    pub fn delete_rows(&mut self, idx: u32, rows: u32) -> PyResult<()> {
        Python::with_gil(|_py| {
            self.0
                .lock()
                .delete_rows(idx, rows)
                .map_err(|e| PyRuntimeError::new_err(format!("Failed to delete rows: {}", e)))
        })
    }

    pub fn set_merged_cells(
        &mut self,
        start_row: u32,
        end_row: u32,
        start_column: u16,
        end_column: u16,
    ) -> PyResult<()> {
        Python::with_gil(|_py| {
            self.0
                .lock()
                .set_merged_cells(start_row, end_row, start_column, end_column)
                .map_err(|e| PyRuntimeError::new_err(format!("Failed Merged cells{}", e)))
        })
    }

    pub fn generate_empty_cells(&mut self) -> PyResult<()> {
        Python::with_gil(|_py| {
            self.0
                .lock()
                .generate_empty_cells()
                .map_err(|e| PyRuntimeError::new_err(format!("Failed generate empty cells: {}", e)))
        })
    }

    #[pyo3(signature=(min_row=None, max_row=None, min_col=None, max_col=None))]
    pub fn iter_cells(
        &self,
        min_row: Option<u32>,
        max_row: Option<u32>,
        min_col: Option<u16>,
        max_col: Option<u16>,
    ) -> PyResult<Vec<WrapperXLSXSheetCell>> {
        Python::with_gil(|_py| {
            self.0
                .lock()
                .iter_cells(min_row, max_row, min_col, max_col)
                .map(|cells| {
                    cells
                        .into_iter()
                        .map(|c| WrapperXLSXSheetCell(Arc::clone(c)))
                        .collect()
                })
                .map_err(|e| PyRuntimeError::new_err(format!("{}", e)))
        })
    }

    /// Поиск ячейки по буквенной координате A1 (cell)
    pub fn find_cell_by_cell(&self, cell: &str) -> PyResult<Option<WrapperXLSXSheetCell>> {
        Python::with_gil(|_py| {
            self.0
                .lock()
                .find_cell_by_cell(cell)
                .map(|cell| cell.map(WrapperXLSXSheetCell))
                .map_err(|e| PyRuntimeError::new_err(format!("{}", e)))
        })
    }

    /// Поиск ячейки по координате
    pub fn find_cell_by_coords(
        &self,
        row: u32,
        col: u16,
    ) -> PyResult<Option<WrapperXLSXSheetCell>> {
        Python::with_gil(|_py| {
            self.0
                .lock()
                .find_cell_by_coords(row, col)
                .map(|cell| cell.map(WrapperXLSXSheetCell))
                .map_err(|e| PyRuntimeError::new_err(format!("{}", e)))
        })
    }

    /// Поиск значенияячейки по координате
    pub fn find_value_by_coords(&self, row: u32, col: u16) -> PyResult<PyObject> {
        Python::with_gil(|py| match self.0.lock().find_value_by_coords(row, col) {
            Ok(Some(value)) => {
                let value = raw_value_to_py(py, &value.raw_value)?;
                Ok(value)
            }
            Ok(None) => Ok(py.None()),
            Err(e) => Err(PyRuntimeError::new_err(format!("{}", e))),
        })
    }
}
