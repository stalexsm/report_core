use std::sync::Arc;

use core_rs::structs::book::Book;

use parking_lot::RwLock;
use pyo3::prelude::*;

use super::sheet::WrapperSheet;

#[pyclass]
#[pyo3(module = "report_core", name = "Book")]
#[derive(Debug, Clone)]
pub struct WrapperBook(pub(crate) Arc<RwLock<Book>>);

#[pymethods]
impl WrapperBook {
    pub fn __repr__(slf: &Bound<'_, Self>) -> PyResult<String> {
        Python::with_gil(|_py| {
            let slf = slf.borrow();
            let slf = slf.0.read();

            Ok(format!(
                "Book, sheets: {}",
                slf.get_sheet_collection().len(),
            ))
        })
    }

    #[getter]
    pub fn sheets(&self) -> PyResult<Vec<WrapperSheet>> {
        Python::with_gil(|_py| {
            let slf = self.0.read();
            let sheets = slf
                .get_sheet_collection()
                .to_vec()
                .iter()
                .map(|s| WrapperSheet(s.clone()))
                .collect();

            Ok(sheets)
        })
    }

    #[new]
    pub fn new() -> Self {
        Self(Arc::new(RwLock::new(Book::new())))
    }

    pub fn add_sheet(&self, name: String) -> PyResult<WrapperSheet> {
        Python::with_gil(|_py| {
            let sheet = self.0.write().add_sheet(&name);
            Ok(WrapperSheet(sheet))
        })
    }

    pub fn get_sheet_index(&self, idx: i32) -> PyResult<Option<WrapperSheet>> {
        Python::with_gil(|_py| {
            if let Some(sheet) = self.0.read().get_sheet_index(idx) {
                Ok(Some(WrapperSheet(sheet.clone())))
            } else {
                Ok(None)
            }
        })
    }

    pub fn get_sheet_name(&self, name: String) -> PyResult<Option<WrapperSheet>> {
        Python::with_gil(|_py| {
            if let Some(sheet) = self.0.read().get_sheet_name(&name) {
                Ok(Some(WrapperSheet(sheet.clone())))
            } else {
                Ok(None)
            }
        })
    }

    pub fn to_json(&self) -> PyResult<String> {
        Python::with_gil(|_py| {
            let res = self.0.read().to_json();
            match res {
                Ok(s) => Ok(s),
                Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    e.to_string(),
                )),
            }
        })
    }

    pub fn to_dict(&self) -> PyResult<Py<PyAny>> {
        Python::with_gil(|py| {
            let res = self.0.read().to_json();
            match res {
                Ok(s) => {
                    let py_module_json = py.import("json")?;
                    let py_dict = py_module_json.getattr("loads")?.call1((s,))?;

                    Ok(py_dict.into())
                }
                Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    e.to_string(),
                )),
            }
        })
    }
}
