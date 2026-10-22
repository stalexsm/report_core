use std::sync::Arc;

use core_rs::structs::book::Book;

use parking_lot::RwLock;
use pyo3::prelude::*;

use super::sheet::WrapperSheet;

/// Книга Excel: контейнер листов, из которых собирается отчёт.
#[pyclass(from_py_object)]
#[pyo3(module = "report_core", name = "Book")]
#[derive(Debug, Clone)]
pub struct WrapperBook(pub(crate) Arc<RwLock<Book>>);

#[pymethods]
impl WrapperBook {
    pub fn __repr__(slf: &Bound<'_, Self>) -> PyResult<String> {
        let slf = slf.borrow();
        let slf = slf.0.read();

        Ok(format!(
            "Book, sheets: {}",
            slf.get_sheet_collection().len(),
        ))
    }

    /// Получение списка листов
    /// ---
    ///
    /// Returns:
    /// ---
    ///     list[Sheet]
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

    /// Инициализация Книги
    /// ----
    ///
    /// Example:
    /// ---
    /// ```python
    /// book = Book()
    /// ```
    #[new]
    pub fn new() -> Self {
        Self(Arc::new(RwLock::new(Book::new())))
    }

    /// Добавление листа в книгу
    /// ---
    ///
    /// Arguments:
    /// ---
    ///     name: str
    ///         Имя листа
    ///     sheet_state: str
    ///         Состояние и видимость листа
    ///
    /// Returns:
    /// ---
    ///     Sheet
    #[pyo3(signature = (name, sheet_state="visible"))]
    pub fn add_sheet(&self, name: &str, sheet_state: &str) -> PyResult<WrapperSheet> {
        let sheet = self.0.write().add_sheet(name, sheet_state);

        Ok(WrapperSheet(sheet))
    }

    /// Копирование листа в книгу
    /// ---
    ///
    /// Arguments:
    /// ---
    ///     sheet: Sheet
    ///         Лист
    ///
    /// Returns:
    /// ---
    ///     Sheet
    pub fn copy_sheet(&self, sheet: WrapperSheet) -> PyResult<WrapperSheet> {
        let sheet = self.0.write().copy_sheet(sheet.0);

        Ok(WrapperSheet(sheet))
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
    ///     Sheet | None
    pub fn get_sheet_index(&self, idx: i32) -> PyResult<Option<WrapperSheet>> {
        if let Some(sheet) = self.0.read().get_sheet_index(idx) {
            Ok(Some(WrapperSheet(Arc::clone(sheet))))
        } else {
            Ok(None)
        }
    }

    /// Получение листа по названию
    /// ---
    ///
    /// Arguments:
    /// ---
    ///     name: str
    ///         Название листа
    ///
    /// Returns:
    /// ---
    ///     Sheet | None
    pub fn get_sheet_name(&self, name: String) -> PyResult<Option<WrapperSheet>> {
        if let Some(sheet) = self.0.read().get_sheet_name(&name) {
            Ok(Some(WrapperSheet(Arc::clone(sheet))))
        } else {
            Ok(None)
        }
    }

    /// Преобразование книги в json
    /// ---
    ///
    /// Returns:
    /// ---
    ///     str
    pub fn to_json(&self) -> PyResult<String> {
        Ok(self.0.read().to_json()?)
    }

    /// Преобразование книги в dict
    /// ---
    ///
    /// Returns:
    /// ---
    ///     dict[str, Any]
    pub fn to_dict(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let s = self.0.read().to_json()?;
        let py_module_json = py.import("json")?;
        let py_dict = py_module_json.getattr("loads")?.call1((s,))?;

        Ok(py_dict.into())
    }
}

impl Default for WrapperBook {
    fn default() -> Self {
        Self::new()
    }
}
