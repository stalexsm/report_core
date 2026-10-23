use std::sync::Arc;

use core_rs::structs::{finder::Finder, sheet::Sheet};
use parking_lot::RwLock;
use pyo3::{prelude::*, types::PyList};

use crate::structs::readable::finder::WrapperFinder;

pub mod cell;
pub mod finder;
pub mod sheet;

/// Создание Finder по списку листов (dict, ReadableSheet или лист книги).
/// Собственные листы библиотеки разделяются без копирования — Finder видит
/// их последующие мутации.
/// ---
///
/// Arguments:
/// ---
///     sheets: list
///         Список листов
///
/// Returns:
/// ---
///     Finder
#[inline]
#[pyfunction]
pub(crate) fn create_finder(py: Python, sheets: &Bound<'_, PyList>) -> PyResult<WrapperFinder> {
    let sheets: PyResult<Vec<Arc<RwLock<Sheet>>>> =
        sheets.iter().map(|s| sheet::shared_sheet(&s)).collect();

    py.detach(|| Ok(WrapperFinder(Arc::new(RwLock::new(Finder::new(sheets?))))))
}
