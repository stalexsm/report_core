use std::sync::Arc;

use core_rs::{
    structs::{cell::Cell, coordinate::Coordinate},
    traits::ReadableCell,
};
use parking_lot::RwLock;
use pyo3::{
    prelude::*,
    types::{PyDict, PyString},
};

use crate::py_extract;

#[pyclass]
#[pyo3(module = "report_core.readable", name = "ReadableCell")]
#[derive(Debug, Clone)]
pub struct WrapperCell(pub(crate) Arc<RwLock<Cell>>);

impl TryFrom<&Bound<'_, PyAny>> for WrapperCell {
    type Error = PyErr;

    fn try_from(obj: &Bound<'_, PyAny>) -> Result<Self, Self::Error> {
        Python::attach(|_py| {
            let row = py_extract!(obj, row)?.as_u32();
            let column = py_extract!(obj, column)?.as_u16();
            let value = py_extract!(obj, value)?.as_string();
            let formula = py_extract!(obj, formula)?.as_string();
            let data_type = py_extract!(obj, data_type)?.as_string_direct();
            let style_id = py_extract!(obj, style_id)?.as_string();

            let cell = Cell::extract(
                Coordinate::new(row, column),
                value,
                formula,
                &data_type,
                style_id,
            );

            Ok(Self(Arc::new(RwLock::new(cell))))
        })
    }
}

#[pymethods]
impl WrapperCell {
    pub fn __repr__(slf: &Bound<'_, Self>) -> PyResult<String> {
        Python::attach(|_py| {
            let class_name: Bound<'_, PyString> = slf.get_type().qualname()?;

            let slf = slf.borrow();
            let slf = slf.0.read();

            Ok(format!(
                "{}, Coords: {:?}",
                class_name,
                slf.get_coordinate()
            ))
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
}
