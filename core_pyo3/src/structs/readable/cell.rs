use std::sync::Arc;

use core_rs::{
    structs::{cell::Cell, coordinate::Coordinate},
    traits::ReadableCell,
};
use parking_lot::RwLock;
use pyo3::{
    prelude::*,
    types::{PyBool, PyDict, PyFloat, PyInt, PyString},
};

// Универсальный enum для всех Python типов
#[derive(Debug, Clone)]
pub enum PyValue {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    None,
}

impl PyValue {
    // Методы для конвертации в конкретные типы
    pub fn as_string(&self) -> Option<String> {
        match self {
            PyValue::String(s) => Some(s.clone()),
            PyValue::Int(i) => Some(i.to_string()),
            PyValue::Float(f) => Some(f.to_string()),
            PyValue::Bool(b) => Some(b.to_string()),
            PyValue::None => None,
        }
    }

    pub fn as_u32(&self) -> u32 {
        match self {
            PyValue::Int(i) => *i as u32,
            PyValue::Float(f) => *f as u32,
            PyValue::String(s) => s.parse().unwrap_or(0),
            PyValue::Bool(b) => {
                if *b {
                    1
                } else {
                    0
                }
            }
            PyValue::None => 0,
        }
    }

    pub fn as_u16(&self) -> u16 {
        match self {
            PyValue::Int(i) => *i as u16,
            PyValue::Float(f) => *f as u16,
            PyValue::String(s) => s.parse().unwrap_or(0),
            PyValue::Bool(b) => {
                if *b {
                    1
                } else {
                    0
                }
            }
            PyValue::None => 0,
        }
    }

    pub fn as_string_direct(&self) -> String {
        match self {
            PyValue::String(s) => s.to_string(),
            PyValue::Int(i) => i.to_string(),
            PyValue::Float(f) => f.to_string(),
            PyValue::Bool(b) => b.to_string(),
            PyValue::None => String::new(),
        }
    }
}

// Функция для автоматического извлечения Python значения
fn extract_py_value_auto(py_value: &Bound<'_, PyAny>) -> PyValue {
    if py_value.is_none() {
        PyValue::None
    } else if py_value.is_instance_of::<PyBool>() {
        // Проверяем bool перед int, так как bool является подклассом int в Python
        PyValue::Bool(py_value.extract::<bool>().unwrap())
    } else if py_value.is_instance_of::<PyInt>() {
        PyValue::Int(py_value.extract::<i64>().unwrap())
    } else if py_value.is_instance_of::<PyFloat>() {
        PyValue::Float(py_value.extract::<f64>().unwrap())
    } else if py_value.is_instance_of::<PyString>() {
        PyValue::String(py_value.extract::<String>().unwrap())
    } else {
        // Для любого другого типа пытаемся преобразовать в строку
        PyValue::String(py_value.str().unwrap().to_string())
    }
}

// Один универсальный макрос для извлечения любого атрибута
macro_rules! py_extract {
    ($obj:expr, $attr:ident) => {{
        let py_value = if $obj.is_instance_of::<PyDict>() {
            $obj.get_item(stringify!($attr)).unwrap()
        } else {
            $obj.getattr(stringify!($attr)).unwrap()
        };

        extract_py_value_auto(&py_value)
    }};
}

#[pyclass]
#[pyo3(module = "report_core.readable", name = "ReadableCell")]
#[derive(Debug, Clone)]
pub struct WrapperCell(pub(crate) Arc<RwLock<Cell>>);

impl From<&Bound<'_, PyAny>> for WrapperCell {
    fn from(obj: &Bound<'_, PyAny>) -> Self {
        Python::with_gil(|_py| {
            let row = py_extract!(obj, row).as_u32();
            let column = py_extract!(obj, column).as_u16();
            let value = py_extract!(obj, value).as_string();
            let formula = py_extract!(obj, formula).as_string();
            let data_type = py_extract!(obj, data_type).as_string_direct();

            let cell = Cell::extract(Coordinate::new(row, column), value, formula, &data_type);

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
