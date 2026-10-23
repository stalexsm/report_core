use std::sync::Arc;

use core_rs::structs::{finder::Finder, sheet::Sheet};

use parking_lot::RwLock;
use pyo3::{
    prelude::*,
    types::{PyList, PyString},
};

use super::sheet::{WrapperSheet, shared_sheet};

/// Поисковик по листам готового XLSX-файла (только чтение).
#[pyclass(from_py_object)]
#[pyo3(module = "report_core.readable", name = "Finder")]
#[derive(Debug, Clone)]
pub struct WrapperFinder(pub(crate) Arc<RwLock<Finder<Sheet>>>);

#[pymethods]
impl WrapperFinder {
    pub fn __repr__(slf: &Bound<'_, Self>) -> PyResult<String> {
        let class_name: Bound<'_, PyString> = slf.get_type().qualname()?;

        let slf = slf.borrow();
        let slf = slf.0.read();

        Ok(format!(
            "{}, sheets: {}",
            class_name,
            slf.get_sheet_collection().len(),
        ))
    }

    /// Инициализация поисковика по списку листов (dict, ReadableSheet или
    /// лист книги). Собственные листы библиотеки разделяются без
    /// копирования — поисковик видит их последующие мутации.
    #[new]
    pub fn new(sheets: &Bound<'_, PyList>) -> PyResult<Self> {
        let sheets: PyResult<Vec<Arc<RwLock<Sheet>>>> =
            sheets.iter().map(|s| shared_sheet(&s)).collect();

        Ok(Self(Arc::new(RwLock::new(Finder::new(sheets?)))))
    }

    /// Списки листов, по которым ищет поисковик
    #[getter]
    pub fn sheets(&self) -> PyResult<Vec<WrapperSheet>> {
        let slf = self.0.read();
        let sheets = slf
            .get_sheet_collection()
            .iter()
            .map(|s| WrapperSheet(Arc::clone(s)))
            .collect();

        Ok(sheets)
    }

    /// Получение листа по индексу
    /// ---
    ///
    /// Arguments:
    /// ---
    ///     idx: int
    ///         Индекс листа
    ///
    /// Returns:
    /// ---
    ///     ReadableSheet | None
    pub fn get_sheet_index(&self, idx: i32) -> PyResult<Option<WrapperSheet>> {
        if let Some(sheet) = self.0.read().get_sheet_index(idx) {
            Ok(Some(WrapperSheet(Arc::clone(sheet))))
        } else {
            Ok(None)
        }
    }

    /// Получение листов, которых нет в списке имён
    /// ---
    ///
    /// Arguments:
    /// ---
    ///     name_list: list[str]
    ///         Список имён
    ///
    /// Returns:
    /// ---
    ///     list[ReadableSheet]
    pub fn get_sheets_without_names(&self, name_list: Vec<String>) -> PyResult<Vec<WrapperSheet>> {
        let name_list = name_list.iter().map(|n| n.as_str()).collect();

        let sheets = self
            .0
            .read()
            .get_sheets_without_names(name_list)
            .iter()
            .map(|s| {
                let s = *s;
                WrapperSheet(Arc::clone(s))
            })
            .collect();

        Ok(sheets)
    }

    /// Получение листов из списка имён
    /// ---
    ///
    /// Arguments:
    /// ---
    ///     name_list: list[str]
    ///         Список имён
    ///
    /// Returns:
    /// ---
    ///     list[ReadableSheet]
    pub fn get_sheets_with_names(&self, name_list: Vec<String>) -> PyResult<Vec<WrapperSheet>> {
        let name_list = name_list.iter().map(|n| n.as_str()).collect();

        let sheets = self
            .0
            .read()
            .get_sheets_with_names(name_list)
            .iter()
            .map(|s| {
                let s = *s;
                WrapperSheet(Arc::clone(s))
            })
            .collect();

        Ok(sheets)
    }

    /// Поиск листа по имени
    /// ---
    ///
    /// Arguments:
    /// ---
    ///     name: str
    ///         Имя листа
    ///
    /// Returns:
    /// ---
    ///     ReadableSheet | None
    pub fn find_sheet_by_name(&self, name: String) -> PyResult<Option<WrapperSheet>> {
        if let Some(sheet) = self.0.read().find_sheet_by_name(&name) {
            Ok(Some(WrapperSheet(Arc::clone(sheet))))
        } else {
            Ok(None)
        }
    }

    /// Поиск листа по регулярному выражению (без учёта регистра — как клеточные
    /// find_*_by_regex)
    /// ---
    ///
    /// Arguments:
    /// ---
    ///     pattern: str
    ///         Шаблон (регулярное выражение)
    ///
    /// Returns:
    /// ---
    ///     ReadableSheet | None
    ///
    /// Raises:
    /// ---
    ///     ValueError
    ///         Если pattern — невалидное регулярное выражение (например, "(")
    pub fn find_sheet_by_regex(&self, pattern: String) -> PyResult<Option<WrapperSheet>> {
        let slf = self.0.read();
        let sheet = slf
            .find_sheet_by_regex(&pattern)
            .map_err(crate::user_value_error)?;

        Ok(sheet.map(|s| WrapperSheet(Arc::clone(s))))
    }
}
