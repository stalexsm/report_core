use chrono::NaiveDateTime;
use parking_lot::RwLock;
use pyo3::prelude::*;
use std::sync::Arc;

use core_rs::{
    structs::cell::Cell,
    traits::{ReadableCell, WriteableCell},
};

#[pyclass]
#[pyo3(module = "report_core", name = "Cell")]
#[derive(Debug, Clone)]
pub struct WrapperCell(pub(crate) Arc<RwLock<Cell>>);

#[pymethods]
impl WrapperCell {
    pub fn __repr__(slf: &Bound<'_, Self>) -> PyResult<String> {
        Python::attach(|_py| {
            let slf = slf.borrow();
            let slf = slf.0.read();

            Ok(format!("Cell, Coords: {:?}", slf.get_coordinate()))
        })
    }

    #[getter]
    pub fn row(&self) -> PyResult<u32> {
        Python::attach(|_py| {
            let slf = self.0.read();

            Ok(slf.get_coordinate().row)
        })
    }

    #[getter]
    pub fn column(&self) -> PyResult<u16> {
        Python::attach(|_py| {
            let slf = self.0.read();

            Ok(slf.get_coordinate().column)
        })
    }

    #[getter]
    pub fn letter(&self) -> PyResult<String> {
        Python::attach(|_py| {
            let slf = self.0.read();

            Ok(slf.get_letter())
        })
    }

    #[getter]
    pub fn get_value(&self) -> PyResult<String> {
        Python::attach(|_py| {
            let slf = self.0.read();

            Ok(slf.get_value())
        })
    }

    #[getter]
    pub fn get_formula(&self) -> PyResult<Option<String>> {
        Python::attach(|_py| {
            let slf = self.0.read();

            Ok(slf.get_formula())
        })
    }

    #[getter]
    pub fn get_style(&self) -> PyResult<Option<String>> {
        Python::attach(|_py| {
            let slf = self.0.read();
            let style = slf.get_style().map(|s| s.get_id());

            Ok(style)
        })
    }

    #[getter]
    pub fn get_data_type(&self) -> PyResult<String> {
        Python::attach(|_py| {
            let slf = self.0.read();

            Ok(slf.get_data_type())
        })
    }

    #[getter]
    pub fn get_hidden_value(&self) -> PyResult<Option<String>> {
        Python::attach(|_py| {
            let slf = self.0.read();

            Ok(slf.get_hidden_value())
        })
    }

    #[getter]
    pub fn is_formula(&self) -> PyResult<bool> {
        Python::attach(|_py| {
            let slf = self.0.read();

            Ok(slf.is_formula())
        })
    }

    #[getter]
    pub fn is_value_bool(&self) -> PyResult<bool> {
        Python::attach(|_py| {
            let slf = self.0.read();

            Ok(slf.is_value_bool())
        })
    }

    #[getter]
    pub fn is_value_numeric(&self) -> PyResult<bool> {
        Python::attach(|_py| {
            let slf = self.0.read();

            Ok(slf.is_value_numeric())
        })
    }

    #[getter]
    pub fn is_value_integer(&self) -> PyResult<bool> {
        Python::attach(|_py| {
            let slf = self.0.read();

            Ok(slf.is_value_integer())
        })
    }

    #[getter]
    pub fn is_value_datetime(&self) -> PyResult<bool> {
        Python::attach(|_py| {
            let slf = self.0.read();

            Ok(slf.is_value_datetime())
        })
    }

    #[getter]
    pub fn is_value_empty(&self) -> PyResult<bool> {
        Python::attach(|_py| {
            let slf = self.0.read();

            Ok(slf.is_value_empty())
        })
    }

    #[setter]
    pub fn set_value(&self, value: &str) -> PyResult<()> {
        Python::attach(|_py| {
            let mut slf = self.0.write();
            slf.set_value(value);

            Ok(())
        })
    }

    #[setter]
    pub fn set_formula(&self, value: &str) -> PyResult<()> {
        Python::attach(|_py| {
            let mut slf = self.0.write();
            slf.set_formula(value);

            Ok(())
        })
    }

    #[setter]
    pub fn set_style(&self, value: &str) -> PyResult<()> {
        Python::attach(|_py| {
            let mut slf = self.0.write();
            slf.set_style(value);

            Ok(())
        })
    }

    #[setter]
    pub fn set_hidden_value(&self, value: &str) -> PyResult<()> {
        Python::attach(|_py| {
            let mut slf = self.0.write();
            slf.set_hidden_value(value);

            Ok(())
        })
    }

    pub fn set_value_number(&self, value: f64) -> PyResult<()> {
        Python::attach(|_py| {
            let mut slf = self.0.write();
            slf.set_value_number(value);

            Ok(())
        })
    }

    pub fn set_value_integer(&self, value: i32) -> PyResult<()> {
        Python::attach(|_py| {
            let mut slf = self.0.write();
            slf.set_value_integer(value);

            Ok(())
        })
    }

    pub fn set_value_bool(&self, value: bool) -> PyResult<()> {
        Python::attach(|_py| {
            let mut slf = self.0.write();
            slf.set_value_bool(value);

            Ok(())
        })
    }

    pub fn set_value_datetime(&self, value: NaiveDateTime) -> PyResult<()> {
        Python::attach(|_py| {
            let mut slf = self.0.write();
            slf.set_value_datetime(value);

            Ok(())
        })
    }
}
