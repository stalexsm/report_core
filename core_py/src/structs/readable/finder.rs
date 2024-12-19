use std::sync::Arc;

use core_rs::structs::{finder::Finder, sheet::Sheet};

use parking_lot::RwLock;
use pyo3::{
    prelude::*,
    types::{PyList, PyString},
};

use super::sheet::WrapperSheet;

#[pyclass]
#[pyo3(module = "report_core", name = "Finder")]
#[derive(Debug, Clone)]
pub struct WrapperFinder(pub(crate) Arc<RwLock<Finder<Sheet>>>);

#[pymethods]
impl WrapperFinder {
    pub fn __repr__(slf: &Bound<'_, Self>) -> PyResult<String> {
        Python::with_gil(|_py| {
            let class_name: Bound<'_, PyString> = slf.get_type().qualname()?;

            let slf = slf.borrow();
            let slf = slf.0.read();

            Ok(format!(
                "{}, sheets: {}",
                class_name,
                slf.get_sheet_collection().len(),
            ))
        })
    }

    #[new]
    pub fn new(sheets: &Bound<'_, PyList>) -> PyResult<Self> {
        Python::with_gil(|_py| {
            let sheets: Vec<Sheet> = sheets
                .iter()
                .map(|s| WrapperSheet::from(&s).0.read().clone())
                .collect();

            Ok(Self(Arc::new(RwLock::new(Finder::new(sheets)))))
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

    pub fn get_sheet_index(&self, idx: i32) -> PyResult<Option<WrapperSheet>> {
        Python::with_gil(|_py| {
            if let Some(sheet) = self.0.read().get_sheet_index(idx) {
                Ok(Some(WrapperSheet(sheet.clone())))
            } else {
                Ok(None)
            }
        })
    }

    pub fn get_sheets_without_names(&self, name_list: Vec<String>) -> PyResult<Vec<WrapperSheet>> {
        Python::with_gil(|_py| {
            let name_list = name_list.iter().map(|n| n.as_str()).collect();

            let sheets = self
                .0
                .read()
                .get_sheets_without_names(name_list)
                .iter()
                .map(|s| {
                    let s = *s;
                    WrapperSheet(s.clone())
                })
                .collect();

            Ok(sheets)
        })
    }

    pub fn get_sheets_with_names(&self, name_list: Vec<String>) -> PyResult<Vec<WrapperSheet>> {
        Python::with_gil(|_py| {
            let name_list = name_list.iter().map(|n| n.as_str()).collect();

            let sheets = self
                .0
                .read()
                .get_sheets_with_names(name_list)
                .iter()
                .map(|s| {
                    let s = *s;
                    WrapperSheet(s.clone())
                })
                .collect();

            Ok(sheets)
        })
    }

    pub fn find_sheet_by_name(&self, name: String) -> PyResult<Option<WrapperSheet>> {
        Python::with_gil(|_py| {
            if let Some(sheet) = self.0.read().find_sheet_by_name(&name) {
                Ok(Some(WrapperSheet(sheet.clone())))
            } else {
                Ok(None)
            }
        })
    }

    pub fn find_sheet_by_regex(&self, pattern: String) -> PyResult<Option<WrapperSheet>> {
        Python::with_gil(|_py| {
            if let Some(sheet) = self.0.read().find_sheet_by_regex(&pattern) {
                Ok(Some(WrapperSheet(sheet.clone())))
            } else {
                Ok(None)
            }
        })
    }
}
