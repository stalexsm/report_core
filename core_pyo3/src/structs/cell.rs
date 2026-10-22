use chrono::NaiveDateTime;
use parking_lot::RwLock;
use pyo3::prelude::*;
use std::sync::Arc;

use core_rs::{
    structs::cell::Cell,
    traits::{ReadableCell, WriteableCell},
};

/// Тип данных ячеек листа с которыми работает парсер.
#[pyclass(from_py_object)]
#[pyo3(module = "report_core", name = "Cell")]
#[derive(Debug, Clone)]
pub struct WrapperCell(pub(crate) Arc<RwLock<Cell>>);

#[pymethods]
impl WrapperCell {
    pub fn __repr__(slf: &Bound<'_, Self>) -> PyResult<String> {
        let slf = slf.borrow();
        let slf = slf.0.read();

        Ok(format!("Cell, Coords: {:?}", slf.get_coordinate()))
    }

    /// Строка ячейки
    #[getter]
    pub fn row(&self) -> PyResult<u32> {
        let slf = self.0.read();

        Ok(slf.get_coordinate().row)
    }

    /// Колонка ячейки
    #[getter]
    pub fn column(&self) -> PyResult<u16> {
        let slf = self.0.read();

        Ok(slf.get_coordinate().column)
    }

    /// Получение буквенной координаты
    /// ---
    ///
    /// Returns:
    /// ---
    ///     str
    #[getter]
    pub fn letter(&self) -> PyResult<String> {
        let slf = self.0.read();

        Ok(slf.get_letter())
    }

    /// Значение ячейки
    #[getter]
    pub fn get_value(&self) -> PyResult<String> {
        let slf = self.0.read();

        Ok(slf.get_value())
    }

    /// Формула ячейки
    #[getter]
    pub fn get_formula(&self) -> PyResult<Option<String>> {
        let slf = self.0.read();

        Ok(slf.get_formula())
    }

    /// Стиль ячейки
    #[getter]
    pub fn get_style(&self) -> PyResult<Option<String>> {
        let slf = self.0.read();
        let style = slf.get_style().map(|s| s.get_id());

        Ok(style)
    }

    /// Тип данных ячейки
    #[getter]
    pub fn get_data_type(&self) -> PyResult<String> {
        let slf = self.0.read();

        Ok(slf.get_data_type())
    }

    /// Скрытое значение ячейки
    #[getter]
    pub fn get_hidden_value(&self) -> PyResult<Option<String>> {
        let slf = self.0.read();

        Ok(slf.get_hidden_value())
    }

    /// Метод для получения флага, ячейка с формулой или нет.
    /// ---
    ///
    /// Returns:
    /// ---
    ///     bool
    #[getter]
    pub fn is_formula(&self) -> PyResult<bool> {
        let slf = self.0.read();

        Ok(slf.is_formula())
    }

    /// Проверить, является ли значение ячейки boolean
    /// ---
    ///
    /// Returns:
    /// ---
    ///     bool
    #[getter]
    pub fn is_value_bool(&self) -> PyResult<bool> {
        let slf = self.0.read();

        Ok(slf.is_value_bool())
    }

    /// Проверить, является ли значение ячейки numeric
    /// ---
    ///
    /// Returns:
    /// ---
    ///     bool
    #[getter]
    pub fn is_value_numeric(&self) -> PyResult<bool> {
        let slf = self.0.read();

        Ok(slf.is_value_numeric())
    }

    /// Проверить, является ли значение ячейки integer
    /// ---
    ///
    /// Returns:
    /// ---
    ///     bool
    #[getter]
    pub fn is_value_integer(&self) -> PyResult<bool> {
        let slf = self.0.read();

        Ok(slf.is_value_integer())
    }

    /// Проверить, является ли значение ячейки datetime
    /// ---
    ///
    /// Returns:
    /// ---
    ///     bool
    #[getter]
    pub fn is_value_datetime(&self) -> PyResult<bool> {
        let slf = self.0.read();

        Ok(slf.is_value_datetime())
    }

    /// Проверить, является ли значение ячейки empty
    /// ---
    ///
    /// Returns:
    /// ---
    ///     bool
    #[getter]
    pub fn is_value_empty(&self) -> PyResult<bool> {
        let slf = self.0.read();

        Ok(slf.is_value_empty())
    }

    /// Установка значения ячейки
    #[setter]
    pub fn set_value(&self, value: &str) -> PyResult<()> {
        let mut slf = self.0.write();
        slf.set_value(value);

        Ok(())
    }

    /// Установка формулы ячейки
    #[setter]
    pub fn set_formula(&self, value: &str) -> PyResult<()> {
        let mut slf = self.0.write();
        slf.set_formula(value);

        Ok(())
    }

    /// Установка стиля ячейки
    #[setter]
    pub fn set_style(&self, value: &str) -> PyResult<()> {
        let mut slf = self.0.write();
        slf.set_style(value);

        Ok(())
    }

    /// Установка скрытого значения ячейки
    #[setter]
    pub fn set_hidden_value(&self, value: &str) -> PyResult<()> {
        let mut slf = self.0.write();
        slf.set_hidden_value(value);

        Ok(())
    }

    /// Метод для добавления значения ячейки Numbers.
    /// ---
    ///
    /// Arguments:
    /// ---------
    ///     value: float
    ///         Значение ячейки
    pub fn set_value_number(&self, value: f64) -> PyResult<()> {
        let mut slf = self.0.write();
        slf.set_value_number(value);

        Ok(())
    }

    /// Метод для добавления значения ячейки Integer.
    /// ---
    ///
    /// Arguments:
    /// ---------
    ///     value: int
    ///         Значение ячейки
    pub fn set_value_integer(&self, value: i32) -> PyResult<()> {
        let mut slf = self.0.write();
        slf.set_value_integer(value);

        Ok(())
    }

    /// Метод для добавления значения ячейки Boolean.
    /// ---
    ///
    /// Arguments:
    /// ---------
    ///     value: bool
    ///         Значение ячейки
    pub fn set_value_bool(&self, value: bool) -> PyResult<()> {
        let mut slf = self.0.write();
        slf.set_value_bool(value);

        Ok(())
    }

    /// Метод для добавления значения ячейки Дата/Время.
    /// ---
    ///
    /// Arguments:
    /// ---------
    ///     value: datetime
    ///         Значение ячейки
    pub fn set_value_datetime(&self, value: NaiveDateTime) -> PyResult<()> {
        let mut slf = self.0.write();
        slf.set_value_datetime(value);

        Ok(())
    }
}
