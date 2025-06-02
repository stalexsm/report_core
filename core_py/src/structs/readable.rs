use std::sync::Arc;

use core_rs::structs::{finder::Finder, sheet::Sheet};
use parking_lot::RwLock;
use pyo3::{prelude::*, types::PyList};

use crate::structs::readable::{finder::WrapperFinder, sheet::WrapperSheet};

pub mod cell;
pub mod finder;
pub mod sheet;

#[inline]
#[pyfunction]
pub(crate) fn create_finder(py: Python, sheets: &Bound<'_, PyList>) -> PyResult<WrapperFinder> {
    let sheets: Vec<Sheet> = sheets
        .iter()
        .map(|s| WrapperSheet::from(&s).0.read().clone())
        .collect();

    py.allow_threads(|| Ok(WrapperFinder(Arc::new(RwLock::new(Finder::new(sheets))))))
}
