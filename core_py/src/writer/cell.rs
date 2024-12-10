use crate::utils::raw_value_to_py;

use chrono::NaiveDateTime;
use core_rs::writer::cell::XLSXSheetCell;
use parking_lot::Mutex;
use pyo3::{exceptions::PyRuntimeError, prelude::*};
use std::sync::Arc;

#[pyclass]
#[pyo3(module = "report_core", name = "XLSXSheetCell")]
#[derive(Debug, Clone)]
pub struct WrapperXLSXSheetCell(pub(crate) Arc<Mutex<XLSXSheetCell>>);

#[pymethods]
impl WrapperXLSXSheetCell {
    pub fn __repr__(slf: &Bound<'_, Self>) -> PyResult<String> {
        Python::with_gil(|_py| {
            let slf = slf.borrow();
            let slf_lock = slf.0.lock();
            Ok(format!(
                "XLSXSheetCell [{}]: (row: {} col: {})",
                slf_lock.cell, slf_lock.row, slf_lock.column,
            ))
        })
    }

    #[new]
    #[pyo3(signature=(row, col, value=None))]
    pub fn new(row: u32, col: u16, value: Option<String>) -> PyResult<Self> {
        Python::with_gil(|_py| {
            let cell = XLSXSheetCell::new(row, col, value);

            Ok(Self(cell))
        })
    }

    #[getter]
    pub fn row(&self) -> PyResult<u32> {
        Python::with_gil(|_py| Ok(self.0.lock().row))
    }

    #[getter]
    pub fn column(&self) -> PyResult<u16> {
        Python::with_gil(|_py| Ok(self.0.lock().column))
    }

    #[getter]
    pub fn cell(&self) -> PyResult<String> {
        Python::with_gil(|_py| Ok(self.0.lock().cell.clone()))
    }

    /// Getter для получения значения из ячейки
    #[getter]
    pub fn value(&self) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            let value = raw_value_to_py(py, &self.0.lock().value.raw_value)?;
            Ok(value)
        })
    }

    #[getter]
    pub fn formula(&self) -> PyResult<Option<String>> {
        Python::with_gil(|_py| Ok(self.0.lock().formula.clone()))
    }

    #[getter]
    pub fn data_type(&self) -> PyResult<String> {
        Python::with_gil(|_py| Ok(self.0.lock().data_type.clone()))
    }

    #[getter]
    pub fn number_format(&self) -> PyResult<String> {
        Python::with_gil(|_py| Ok(self.0.lock().number_format.clone()))
    }

    #[getter]
    pub fn is_merge(&self) -> PyResult<bool> {
        Python::with_gil(|_py| Ok(self.0.lock().is_merge))
    }

    #[getter]
    pub fn start_row(&self) -> PyResult<Option<u32>> {
        Python::with_gil(|_py| Ok(self.0.lock().start_row))
    }

    #[getter]
    pub fn end_row(&self) -> PyResult<Option<u32>> {
        Python::with_gil(|_py| Ok(self.0.lock().end_row))
    }

    #[getter]
    pub fn start_column(&self) -> PyResult<Option<u16>> {
        Python::with_gil(|_py| Ok(self.0.lock().start_column))
    }

    #[getter]
    pub fn end_column(&self) -> PyResult<Option<u16>> {
        Python::with_gil(|_py| Ok(self.0.lock().end_column))
    }

    #[getter]
    pub fn style_id(&self) -> PyResult<Option<String>> {
        Python::with_gil(|_py| Ok(self.0.lock().style_id.clone()))
    }

    #[getter]
    pub fn hidden_value(&self) -> PyResult<Option<String>> {
        Python::with_gil(|_py| Ok(self.0.lock().hidden_value.clone()))
    }

    #[getter]
    pub fn comment(&self) -> PyResult<Option<String>> {
        Python::with_gil(|_py| Ok(self.0.lock().comment.clone()))
    }

    /// Метод для получения значения ячейки.
    pub fn set_value(&mut self, value: String) -> PyResult<()> {
        Python::with_gil(|_py| {
            self.0
                .lock()
                .set_value(value)
                .map_err(|e| PyRuntimeError::new_err(format!("{}", e)))
        })
    }

    /// Метод для добавления тех значения ячейки
    pub fn set_hidden_value(&mut self, value: String) -> PyResult<()> {
        Python::with_gil(|_py| {
            self.0
                .lock()
                .set_hidden_value(value)
                .map_err(|e| PyRuntimeError::new_err(format!("{}", e)))
        })
    }

    /// Метод для добавления комментария к ячейки
    pub fn set_comment(&mut self, value: String) -> PyResult<()> {
        Python::with_gil(|_py| {
            self.0
                .lock()
                .set_comment(value)
                .map_err(|e| PyRuntimeError::new_err(format!("{}", e)))
        })
    }

    /// Метод для получения значения ячейки Numbers.
    pub fn set_value_number(&mut self, value: f64) -> PyResult<()> {
        Python::with_gil(|_py| {
            self.0
                .lock()
                .set_value_number(value)
                .map_err(|e| PyRuntimeError::new_err(format!("{}", e)))
        })
    }

    /// Метод для получения значения ячейки Numbers.
    pub fn set_value_integer(&mut self, value: i32) -> PyResult<()> {
        Python::with_gil(|_py| {
            self.0
                .lock()
                .set_value_integer(value)
                .map_err(|e| PyRuntimeError::new_err(format!("{}", e)))
        })
    }

    /// Метод для получения значения ячейки Bool.
    pub fn set_value_bool(&mut self, value: bool) -> PyResult<()> {
        Python::with_gil(|_py| {
            self.0
                .lock()
                .set_value_bool(value)
                .map_err(|e| PyRuntimeError::new_err(format!("{}", e)))
        })
    }

    /// Метод для получения значения ячейки String.
    pub fn set_value_str(&mut self, value: String) -> PyResult<()> {
        Python::with_gil(|_py| {
            self.0
                .lock()
                .set_value_str(value)
                .map_err(|e| PyRuntimeError::new_err(format!("{}", e)))
        })
    }

    /// Метод для получения значения ячейки Datetime.
    pub fn set_value_datetime(&mut self, value: NaiveDateTime) -> PyResult<()> {
        Python::with_gil(|_py| {
            self.0
                .lock()
                .set_value_datetime(value)
                .map_err(|e| PyRuntimeError::new_err(format!("{}", e)))
        })
    }

    /// Метод для получения значения ячейки String.
    pub fn set_formula(&mut self, value: String) -> PyResult<()> {
        Python::with_gil(|_py| {
            self.0
                .lock()
                .set_formula(value)
                .map_err(|e| PyRuntimeError::new_err(format!("{}", e)))
        })
    }

    /// Метод для получения значения ячейки data_type.
    pub fn set_data_type(&mut self, value: String) -> PyResult<()> {
        Python::with_gil(|_py| {
            self.0
                .lock()
                .set_data_type(value)
                .map_err(|e| PyRuntimeError::new_err(format!("{}", e)))
        })
    }

    /// Метод для получения значения ячейки number_format.
    pub fn set_number_format(&mut self, value: String) -> PyResult<()> {
        Python::with_gil(|_py| {
            self.0
                .lock()
                .set_number_format(value)
                .map_err(|e| PyRuntimeError::new_err(format!("{}", e)))
        })
    }

    /// Метод для получения флага, ячейка с формулой или нет.
    #[getter]
    pub fn is_formula(&self) -> PyResult<bool> {
        Python::with_gil(|_py| Ok(self.0.lock().is_formula()))
    }

    /// Проверить, является ли значение ячейки boolean
    #[getter]
    pub fn is_value_bool(&self) -> PyResult<bool> {
        Python::with_gil(|_py| Ok(self.0.lock().is_value_bool()))
    }

    /// Проверить, является ли значение ячейки numeric
    #[getter]
    pub fn is_value_numeric(&self) -> PyResult<bool> {
        Python::with_gil(|_py| Ok(self.0.lock().is_value_numeric()))
    }

    /// Проверить, является ли значение ячейки datetime
    #[getter]
    pub fn is_value_datetime(&self) -> PyResult<bool> {
        Python::with_gil(|_py| Ok(self.0.lock().is_value_datetime()))
    }

    /// Проверить, является ли значение ячейки empty
    #[getter]
    pub fn is_value_empty(&self) -> PyResult<bool> {
        Python::with_gil(|_py| Ok(self.0.lock().is_value_empty()))
    }

    /// Метод для добавления стиля к ячейки
    pub fn set_style_id(&mut self, value: String) -> PyResult<()> {
        Python::with_gil(|_py| {
            self.0
                .lock()
                .set_style_id(value)
                .map_err(|e| PyRuntimeError::new_err(format!("{}", e)))
        })
    }
}
