use std::sync::Arc;

use core_rs::structs::finder::Finder;
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
    let sheets = sheet::shared_sheets(sheets)?;

    py.detach(|| Ok(WrapperFinder(Arc::new(RwLock::new(Finder::new(sheets))))))
}
