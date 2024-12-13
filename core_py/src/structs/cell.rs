use parking_lot::RwLock;
use pyo3::prelude::*;
use std::sync::Arc;

use core_rs::structs::cell::Cell;

#[pyclass]
#[pyo3(module = "report_core", name = "XLSXCell")]
#[derive(Debug, Clone)]
pub struct WrapperCell(pub(crate) Arc<RwLock<Cell>>);

#[pymethods]
impl WrapperCell {
    pub fn __repr__(slf: &Bound<'_, Self>) -> PyResult<String> {
        Python::with_gil(|_py| {
            let slf = slf.borrow();
            let slf = slf.0.read();

            Ok(format!("XLSXCell, Coords: {:?}", slf.get_coordinate()))
        })
    }
}
