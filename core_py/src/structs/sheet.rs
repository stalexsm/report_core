use std::sync::Arc;

use core_rs::structs::{
    range::{MergedRange, Range},
    sheet::Sheet,
};
use parking_lot::RwLock;
use pyo3::prelude::*;

#[pyclass]
#[pyo3(module = "report_core", name = "XLSXSheet")]
#[derive(Debug, Clone)]
pub struct WrapperSheet(pub(crate) Arc<RwLock<Sheet>>);

#[pymethods]
impl WrapperSheet {
    pub fn __repr__(slf: &Bound<'_, Self>) -> PyResult<String> {
        Python::with_gil(|_py| {
            let slf = slf.borrow();
            let slf = slf.0.read();
            Ok(format!(
                "XLSXSheet ({}) cells: {}",
                slf.name,
                slf.get_cell_collection().len()
            ))
        })
    }

    #[getter]
    pub fn name(&self) -> PyResult<String> {
        Python::with_gil(|_py| {
            let slf = self.0.read();
            Ok(slf.name.clone())
        })
    }

    #[setter]
    pub fn set_name(&self, name: String) -> PyResult<()> {
        Python::with_gil(|_py| {
            let mut slf = self.0.write();
            slf.name = name;

            Ok(())
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

    pub fn add_merge_cells(
        &mut self,
        start_row: u32,
        end_row: u32,
        start_col: u16,
        end_col: u16,
    ) -> PyResult<()> {
        Python::with_gil(|_py| {
            let range = Range::new(start_row, end_row, start_col, end_col);

            self.0.write().add_merge_range(range);

            Ok(())
        })
    }
}
