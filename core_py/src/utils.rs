use core_rs::datatype::CellRawValue;

use pyo3::{prelude::*, BoundObject, PyObject, Python};

/// Вспомогательная функция для преобразования CellRawValue в PyObject
pub(crate) fn raw_value_to_py(py: Python<'_>, value: &CellRawValue) -> PyResult<PyObject> {
    Ok(match value {
        CellRawValue::Empty => py.None(),
        CellRawValue::String(s) => s.into_pyobject(py).unwrap().into_any().unbind(),
        CellRawValue::Integer(i) => i.into_pyobject(py).unwrap().into_any().unbind(),
        CellRawValue::Numeric(n) => n.into_pyobject(py).unwrap().into_any().unbind(),
        CellRawValue::Bool(b) => b.into_pyobject(py).unwrap().into_any().unbind(),
        CellRawValue::Datetime(d) => d.into_pyobject(py).unwrap().into_any().unbind(),
    })
}
