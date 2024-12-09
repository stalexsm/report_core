use core_rs::writer::book::XLSXBook;
use parking_lot::Mutex;
use pyo3::prelude::*;
use std::{collections::HashMap, sync::Arc};

use super::sheet::WrapperXLSXSheet;

#[pyclass]
#[pyo3(module = "report_core", name = "XLSXBook")]
#[derive(Debug, Clone)]
pub struct WrapperXLSXBook(pub(crate) Arc<Mutex<XLSXBook>>);

#[pymethods]
impl WrapperXLSXBook {
    pub fn __repr__(slf: &Bound<'_, Self>) -> PyResult<String> {
        Python::with_gil(|_py| {
            let slf = slf.borrow();
            let slf_lock = slf.0.lock();

            Ok(format!("XLSXBook, sheets: {}", slf_lock.sheets.len(),))
        })
    }

    #[new]
    pub fn new() -> Self {
        Python::with_gil(|_py| {
            let book = XLSXBook::new();

            Self(book)
        })
    }

    #[getter]
    pub fn sheets(&self) -> PyResult<Vec<WrapperXLSXSheet>> {
        Python::with_gil(|_py| {
            let book = self.0.lock();
            let sheets = book
                .sheets
                .iter()
                .map(|s| WrapperXLSXSheet(Arc::clone(s)))
                .collect();

            Ok(sheets)
        })
    }

    #[pyo3(signature = (name, rows=None, cols=None))]
    pub fn add_sheet(
        &mut self,
        name: String,
        rows: Option<u32>,
        cols: Option<u16>,
    ) -> WrapperXLSXSheet {
        Python::with_gil(|_py| WrapperXLSXSheet(self.0.lock().add_sheet(name, rows, cols)))
    }

    pub fn get_sheet_index(&self, idx: i32) -> Option<WrapperXLSXSheet> {
        Python::with_gil(|_py| {
            self.0
                .lock()
                .get_sheet_index(idx)
                .map(|s| WrapperXLSXSheet(Arc::clone(&s)))
        })
    }

    pub fn get_sheet_name(&self, name: String) -> Option<WrapperXLSXSheet> {
        Python::with_gil(|_py| {
            self.0
                .lock()
                .get_sheet_name(name)
                .map(|s| WrapperXLSXSheet(Arc::clone(&s)))
        })
    }

    pub fn to_json(&self) -> PyResult<String> {
        Python::with_gil(|_py| {
            let res = self.0.lock().to_json();
            match res {
                Ok(s) => Ok(s),
                Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    e.to_string(),
                )),
            }
        })
    }

    pub fn to_dict(&self) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            if let Ok(res) = self.0.lock().to_hashmap() {
                let hash_map: HashMap<String, String> =
                    res.into_iter().map(|(k, v)| (k, v.to_string())).collect();

                Ok(hash_map.into_pyobject(py).unwrap().into_any().unbind())
            } else {
                Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    "Failed to convert to dict",
                ))
            }
        })
    }
}
