use core_rs::{datatype::CellValue, reader::cell::XLSXSheetCellRead};
use pyo3::{prelude::*, types::PyDict};

use crate::utils::raw_value_to_py;

macro_rules! extract_sheetcell {
    ($obj:expr, $($attr:ident),+) => {
        {
            let cell = if $obj.is_instance_of::<PyDict>() {
                XLSXSheetCellRead {
                    $($attr: $obj.get_item(stringify!($attr)).unwrap().extract().unwrap()),+,
                    ..Default::default()
                }
            } else {
                XLSXSheetCellRead {
                    $($attr: $obj.getattr(stringify!($attr)).unwrap().extract().unwrap()),+,
                    ..Default::default()
                }
            };

            WrapperXLSXSheetCellRead(cell)
        }
    };
}

#[pyclass]
#[pyo3(module = "report_core", name = "XLSXSheetCellRead")]
#[derive(Debug, Clone)]
pub struct WrapperXLSXSheetCellRead(pub(crate) XLSXSheetCellRead);

impl From<&Bound<'_, PyAny>> for WrapperXLSXSheetCellRead {
    fn from(obj: &Bound<'_, PyAny>) -> Self {
        Python::with_gil(|_py| {
            let mut s = extract_sheetcell!(
                obj,
                row,
                column,
                cell,
                // value,
                formula,
                data_type,
                number_format,
                is_merge,
                start_row,
                end_row,
                start_column,
                end_column,
                style_id
            );

            // Временное решение.
            let value = if obj.is_instance_of::<PyDict>() {
                obj.get_item("value")
                    .unwrap()
                    .extract::<String>()
                    .unwrap_or("".to_string())
            } else {
                obj.getattr("value")
                    .unwrap()
                    .extract::<String>()
                    .unwrap_or("".to_string())
            };
            // Хак
            s.0.value = Box::new(CellValue {
                raw_value: CellValue::quess_typed_value(&value),
            });

            s
        })
    }
}

#[pymethods]
impl WrapperXLSXSheetCellRead {
    pub fn __repr__(slf: &Bound<'_, Self>) -> PyResult<String> {
        Python::with_gil(|_py| {
            let slf = slf.borrow();

            Ok(format!(
                "XLSXSheetCellRead [{}]: (row: {} col: {})",
                slf.0.cell, slf.0.row, slf.0.column,
            ))
        })
    }

    #[getter]
    pub fn row(&self) -> PyResult<u32> {
        Python::with_gil(|_py| Ok(self.0.row))
    }

    #[getter]
    pub fn column(&self) -> PyResult<u16> {
        Python::with_gil(|_py| Ok(self.0.column))
    }

    #[getter]
    pub fn cell(&self) -> PyResult<String> {
        Python::with_gil(|_py| Ok(self.0.cell.clone()))
    }

    /// Getter для получения значения из ячейки
    #[getter]
    pub fn value(&self) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            let value = raw_value_to_py(py, &self.0.value.raw_value)?;
            Ok(value)
        })
    }

    #[getter]
    pub fn formula(&self) -> PyResult<Option<String>> {
        Python::with_gil(|_py| Ok(self.0.formula.clone()))
    }

    #[getter]
    pub fn data_type(&self) -> PyResult<String> {
        Python::with_gil(|_py| Ok(self.0.data_type.clone()))
    }

    #[getter]
    pub fn number_format(&self) -> PyResult<String> {
        Python::with_gil(|_py| Ok(self.0.number_format.clone()))
    }

    #[getter]
    pub fn is_merge(&self) -> PyResult<bool> {
        Python::with_gil(|_py| Ok(self.0.is_merge))
    }

    #[getter]
    pub fn start_row(&self) -> PyResult<Option<u32>> {
        Python::with_gil(|_py| Ok(self.0.start_row))
    }

    #[getter]
    pub fn end_row(&self) -> PyResult<Option<u32>> {
        Python::with_gil(|_py| Ok(self.0.end_row))
    }

    #[getter]
    pub fn start_column(&self) -> PyResult<Option<u16>> {
        Python::with_gil(|_py| Ok(self.0.start_column))
    }

    #[getter]
    pub fn end_column(&self) -> PyResult<Option<u16>> {
        Python::with_gil(|_py| Ok(self.0.end_column))
    }

    #[getter]
    pub fn style_id(&self) -> PyResult<Option<String>> {
        Python::with_gil(|_py| Ok(self.0.style_id.clone()))
    }

    #[getter]
    pub fn hidden_value(&self) -> PyResult<Option<String>> {
        Python::with_gil(|_py| Ok(self.0.hidden_value.clone()))
    }

    #[getter]
    pub fn comment(&self) -> PyResult<Option<String>> {
        Python::with_gil(|_py| Ok(self.0.comment.clone()))
    }

    /// Метод для получения флага, ячейка с формулой или нет.
    #[getter]
    pub fn is_formula(&self) -> PyResult<bool> {
        Python::with_gil(|_py| Ok(self.0.is_formula()))
    }

    /// Проверить, является ли значение ячейки boolean
    #[getter]
    pub fn is_value_bool(&self) -> PyResult<bool> {
        Python::with_gil(|_py| Ok(self.0.is_value_bool()))
    }

    /// Проверить, является ли значение ячейки numeric
    #[getter]
    pub fn is_value_numeric(&self) -> PyResult<bool> {
        Python::with_gil(|_py| Ok(self.0.is_value_numeric()))
    }

    /// Проверить, является ли значение ячейки numeric
    #[getter]
    pub fn is_value_integer(&self) -> PyResult<bool> {
        Python::with_gil(|_py| Ok(self.0.is_value_integer()))
    }

    /// Проверить, является ли значение ячейки datetime
    #[getter]
    pub fn is_value_datetime(&self) -> PyResult<bool> {
        Python::with_gil(|_py| Ok(self.0.is_value_datetime()))
    }

    /// Проверить, является ли значение ячейки empty
    #[getter]
    pub fn is_value_empty(&self) -> PyResult<bool> {
        Python::with_gil(|_py| Ok(self.0.is_value_empty()))
    }
}
