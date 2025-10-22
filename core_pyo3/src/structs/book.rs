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
        Python::attach(|_py| {
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
        Python::attach(|_py| {
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

    #[pyo3(signature = (name, sheet_state="visible"))]
    pub fn add_sheet(&self, name: &str, sheet_state: &str) -> PyResult<WrapperSheet> {
        Python::attach(|_py| {
            let sheet = self.0.write().add_sheet(name, sheet_state);

            Ok(WrapperSheet(sheet))
        })
    }

    pub fn copy_sheet(&self, sheet: WrapperSheet) -> PyResult<WrapperSheet> {
        Python::attach(|_py| {
            let sheet = self.0.write().copy_sheet(sheet.0);

            Ok(WrapperSheet(sheet))
        })
    }

    pub fn get_sheet_index(&self, idx: i32) -> PyResult<Option<WrapperSheet>> {
        Python::attach(|_py| {
            if let Some(sheet) = self.0.read().get_sheet_index(idx) {
                Ok(Some(WrapperSheet(sheet.clone())))
            } else {
                Ok(None)
            }
        })
    }

    pub fn get_sheet_name(&self, name: String) -> PyResult<Option<WrapperSheet>> {
        Python::attach(|_py| {
            if let Some(sheet) = self.0.read().get_sheet_name(&name) {
                Ok(Some(WrapperSheet(sheet.clone())))
            } else {
                Ok(None)
            }
        })
    }

    pub fn to_json(&self) -> PyResult<String> {
        Python::attach(|_py| Ok(self.0.read().to_json()?))
    }

    pub fn to_dict(&self) -> PyResult<Py<PyAny>> {
        Python::attach(|py| {
            let s = self.0.read().to_json()?;
            let py_module_json = py.import("json")?;
            let py_dict = py_module_json.getattr("loads")?.call1((s,))?;

            Ok(py_dict.into())
        })
    }
}

impl Default for WrapperBook {
    fn default() -> Self {
        Self::new()
    }
}
