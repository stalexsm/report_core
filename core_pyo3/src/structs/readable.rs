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
    let sheets: Result<Vec<Sheet>, _> = sheets
        .iter()
        .map(|s| WrapperSheet::try_from(&s).map(|ws| ws.0.read().clone()))
        .collect();

    py.detach(|| Ok(WrapperFinder(Arc::new(RwLock::new(Finder::new(sheets?))))))
}
