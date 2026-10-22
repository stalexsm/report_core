use std::sync::Arc;

use core_rs::structs::book::Book;

use parking_lot::RwLock;
use pyo3::{
    exceptions::PyNotImplementedError,
    prelude::*,
    types::{PyDict, PyList, PyString},
};

use super::sheet::WrapperSheet;

/// Сервис
///
/// Базовый класс для сервисов, строящих отчёты из данных БД.
#[pyclass(from_py_object)]
#[pyo3(module = "report_core", name = "Service", subclass)]
#[derive(Debug, Clone)]
pub struct WrapperService {
    _conn_db: Py<PyAny>,
    inner: Arc<RwLock<Book>>,
}

#[pymethods]
impl WrapperService {
    pub fn __repr__(slf: &Bound<'_, Self>) -> PyResult<String> {
        let class_name: Bound<'_, PyString> = slf.get_type().qualname()?;

        let slf = slf.borrow();
        let slf = slf.inner.read();

        Ok(format!(
            "{}, sheets: {}",
            class_name,
            slf.get_sheet_collection().len(),
        ))
    }

    /// Подключение к базе данных
    #[getter]
    pub fn _conn_db(&self) -> PyResult<Py<PyAny>> {
        Ok(self._conn_db.clone())
    }

    /// Данный метод для реализации генерации данных сервиса
    /// ----------------------------------------------------
    ///
    /// Arguments:
    /// ---------
    ///     sheets: list
    ///         Список листов
    ///     kwargs: Any
    ///         Дополнительные параметры
    ///
    /// Returns:
    /// --------
    ///     Any
    #[pyo3(signature = (sheets, /, **kwargs))]
    #[allow(unused)]
    pub fn summary_0(
        &self,
        sheets: &Bound<'_, PyList>,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<&Bound<'_, PyAny>> {
        Err(PyNotImplementedError::new_err(
            "Method 'summary_0' is not implemented",
        ))
    }

    /// Данный метод для форматирования отчета сервиса.
    /// ----------------------------------------------
    ///
    /// Arguments:
    /// ---------
    ///     kwargs: Any
    ///         Дополнительные параметры
    ///
    /// Returns:
    /// --------
    ///     Any
    #[pyo3(signature = (**kwargs))]
    #[allow(unused)]
    pub fn _fmt_0(&self, kwargs: Option<&Bound<'_, PyDict>>) -> PyResult<&Bound<'_, PyAny>> {
        Err(PyNotImplementedError::new_err(
            "Method 'fmt_0' is not implemented",
        ))
    }

    /// Инициализация парсера
    /// ---------------------
    ///
    /// Arguments:
    /// ---------
    ///     conn_db: Any
    ///         conn_db для работы с базой данных
    #[new]
    fn new(conn_db: Py<PyAny>) -> PyResult<Self> {
        let inner = Arc::new(RwLock::new(Book::new()));

        Ok(Self {
            _conn_db: conn_db,
            inner,
        })
    }

    /// Получение списка листов
    /// ---
    ///
    /// Returns:
    /// ---
    ///     list[Sheet]
    #[getter]
    #[pyo3(name = "_sheets")]
    pub fn sheets(&self) -> PyResult<Vec<WrapperSheet>> {
        let slf = self.inner.read();
        let sheets = slf
            .get_sheet_collection()
            .iter()
            .map(|s| WrapperSheet(Arc::clone(s)))
            .collect();

        Ok(sheets)
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
    #[pyo3(name = "_add_sheet", signature = (name, sheet_state="visible"))]
    pub fn add_sheet(&self, name: &str, sheet_state: &str) -> PyResult<WrapperSheet> {
        let sheet = self.inner.write().add_sheet(name, sheet_state);
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
    #[pyo3(name = "_copy_sheet")]
    pub fn copy_sheet(&self, sheet: WrapperSheet) -> PyResult<WrapperSheet> {
        let sheet = self.inner.write().copy_sheet(sheet.0);

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
    #[pyo3(name = "_get_sheet_index")]
    pub fn get_sheet_index(&self, idx: i32) -> PyResult<Option<WrapperSheet>> {
        if let Some(sheet) = self.inner.read().get_sheet_index(idx) {
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
    #[pyo3(name = "_get_sheet_name")]
    pub fn get_sheet_name(&self, name: String) -> PyResult<Option<WrapperSheet>> {
        if let Some(sheet) = self.inner.read().get_sheet_name(&name) {
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
        Ok(self.inner.read().to_json()?)
    }

    /// Преобразование книги в dict
    /// ---
    ///
    /// Returns:
    /// ---
    ///     dict[str, Any]
    pub fn to_dict(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let s = self.inner.read().to_json()?;
        let py_module_json = py.import("json")?;
        let py_fn_loads = py_module_json.getattr("loads")?;
        let py_dict = py_fn_loads.call1((s,))?;

        Ok(py_dict.into())
    }
}
