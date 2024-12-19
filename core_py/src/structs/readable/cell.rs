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

macro_rules! extract_from_py {
    ($obj:expr, $attr:ident, $type:ty) => {{
        let value: $type = if $obj.is_instance_of::<PyDict>() {
            $obj.get_item(stringify!($attr)).unwrap().extract().unwrap()
        } else {
            $obj.getattr(stringify!($attr)).unwrap().extract().unwrap()
        };

        value
    }};
}

#[pyclass]
#[pyo3(module = "readable", name = "ReadableCell")]
#[derive(Debug, Clone)]
pub struct WrapperCell(pub(crate) Arc<RwLock<Cell>>);

impl From<&Bound<'_, PyAny>> for WrapperCell {
    fn from(obj: &Bound<'_, PyAny>) -> Self {
        Python::with_gil(|_py| {
            let row = extract_from_py!(obj, row, u32);
            let column = extract_from_py!(obj, column, u16);
            let value = extract_from_py!(obj, value, String);
            let formula = extract_from_py!(obj, formula, Option<String>);
            let data_type = extract_from_py!(obj, data_type, String);

            let cell = Cell::extract(
                Coordinate::new(row, column),
                Some(value),
                formula,
                &data_type,
            );

            Self(Arc::new(RwLock::new(cell)))
        })
    }
}

#[pymethods]
impl WrapperCell {
    pub fn __repr__(slf: &Bound<'_, Self>) -> PyResult<String> {
        Python::with_gil(|_py| {
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
        Python::with_gil(|_py| {
            let slf = self.0.read();

            Ok(slf.get_coordinate().row)
        })
    }

    #[getter]
    pub fn column(&self) -> PyResult<u16> {
        Python::with_gil(|_py| {
            let slf = self.0.read();

            Ok(slf.get_coordinate().column)
        })
    }

    #[getter]
    pub fn letter(&self) -> PyResult<String> {
        Python::with_gil(|_py| {
            let slf = self.0.read();

            Ok(slf.get_letter())
        })
    }

    #[getter]
    pub fn get_value(&self) -> PyResult<String> {
        Python::with_gil(|_py| {
            let slf = self.0.read();

            Ok(slf.get_value())
        })
    }

    #[getter]
    pub fn get_formula(&self) -> PyResult<Option<String>> {
        Python::with_gil(|_py| {
            let slf = self.0.read();

            Ok(slf.get_formula())
        })
    }

    #[getter]
    pub fn get_style(&self) -> PyResult<Option<String>> {
        Python::with_gil(|_py| {
            let slf = self.0.read();
            let style = slf.get_style().map(|s| s.get_id());

            Ok(style)
        })
    }

    #[getter]
    pub fn get_data_type(&self) -> PyResult<String> {
        Python::with_gil(|_py| {
            let slf = self.0.read();

            Ok(slf.get_data_type())
        })
    }

    #[getter]
    pub fn is_formula(&self) -> PyResult<bool> {
        Python::with_gil(|_py| {
            let slf = self.0.read();

            Ok(slf.is_formula())
        })
    }

    #[getter]
    pub fn is_value_bool(&self) -> PyResult<bool> {
        Python::with_gil(|_py| {
            let slf = self.0.read();

            Ok(slf.is_value_bool())
        })
    }

    #[getter]
    pub fn is_value_numeric(&self) -> PyResult<bool> {
        Python::with_gil(|_py| {
            let slf = self.0.read();

            Ok(slf.is_value_numeric())
        })
    }

    #[getter]
    pub fn is_value_integer(&self) -> PyResult<bool> {
        Python::with_gil(|_py| {
            let slf = self.0.read();

            Ok(slf.is_value_integer())
        })
    }

    #[getter]
    pub fn is_value_datetime(&self) -> PyResult<bool> {
        Python::with_gil(|_py| {
            let slf = self.0.read();

            Ok(slf.is_value_datetime())
        })
    }

    #[getter]
    pub fn is_value_empty(&self) -> PyResult<bool> {
        Python::with_gil(|_py| {
            let slf = self.0.read();

            Ok(slf.is_value_empty())
        })
    }
}
