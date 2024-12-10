use core_rs::reader::sheet::XLSXSheetRead;
use pyo3::{
    exceptions::PyRuntimeError,
    prelude::*,
    types::{PyDict, PyList},
};

use crate::utils::raw_value_to_py;

use super::cell::WrapperXLSXSheetCellRead;

macro_rules! extract_sheet {
    ($obj:expr, $($attr:ident),+) => {
        {
            let sheet = if $obj.is_instance_of::<PyDict>() {
                XLSXSheetRead {
                    $($attr: $obj.get_item(stringify!($attr)).unwrap().extract().unwrap(),)+
                    ..Default::default()
                }
            } else {
                XLSXSheetRead {
                    $($attr: $obj.getattr(stringify!($attr)).unwrap().extract().unwrap(),)+
                    ..Default::default()
                }
            };

            WrapperXLSXSheetRead(sheet)
        }
    };
}

#[pyclass]
#[pyo3(module = "report_core", name = "XLSXSheetRead")]
#[derive(Debug, Clone)]
pub struct WrapperXLSXSheetRead(pub(crate) XLSXSheetRead);

impl From<&Bound<'_, PyAny>> for WrapperXLSXSheetRead {
    fn from(obj: &Bound<'_, PyAny>) -> Self {
        let mut wrapper = extract_sheet!(obj, name, index, max_row, max_column);

        let cells_iter = if obj.is_instance_of::<PyDict>() {
            obj.get_item("cells").unwrap()
        } else {
            obj.getattr("cells").unwrap()
        }
        .downcast::<PyList>()
        .unwrap()
        .iter();

        let cells = cells_iter
            .map(|c| WrapperXLSXSheetCellRead::from(&c))
            .map(|w| ((w.0.row, w.0.column), w.0))
            .collect();

        wrapper.0._cells = cells;

        wrapper
    }
}

#[pymethods]
impl WrapperXLSXSheetRead {
    pub fn __repr__(slf: &Bound<'_, Self>) -> PyResult<String> {
        Python::with_gil(|_py| {
            let slf = slf.borrow();
            let len = slf.0.cells().count();

            Ok(format!("XLSXSheetRead ({}) cells: {}", slf.0.name, len))
        })
    }

    #[getter]
    pub fn name(&self) -> PyResult<String> {
        Python::with_gil(|_py| Ok(self.0.name.clone()))
    }

    #[getter]
    pub fn max_row(&self) -> PyResult<u32> {
        Python::with_gil(|_py| Ok(self.0.max_row))
    }

    #[getter]
    pub fn max_column(&self) -> PyResult<u16> {
        Python::with_gil(|_py| Ok(self.0.max_column))
    }

    #[getter]
    pub fn index(&self) -> PyResult<i32> {
        Python::with_gil(|_py| Ok(self.0.index))
    }

    #[getter]
    pub fn cells(&self) -> PyResult<Vec<WrapperXLSXSheetCellRead>> {
        Python::with_gil(|_py| {
            let cells = self
                .0
                .cells()
                .map(|c| WrapperXLSXSheetCellRead(c.clone()))
                .collect();

            Ok(cells)
        })
    }

    #[pyo3(signature=(min_row=None, max_row=None, min_col=None, max_col=None))]
    pub fn iter_cells(
        &self,
        min_row: Option<u32>,
        max_row: Option<u32>,
        min_col: Option<u16>,
        max_col: Option<u16>,
    ) -> PyResult<Vec<WrapperXLSXSheetCellRead>> {
        Python::with_gil(|_py| {
            self.0
                .iter_cells(min_row, max_row, min_col, max_col)
                .map(|cells| cells.into_iter().map(WrapperXLSXSheetCellRead).collect())
                .map_err(|e| PyRuntimeError::new_err(format!("{}", e)))
        })
    }

    /// Поиск ячейки по шаблону
    pub fn find_cell_by_pattern_regex(
        &self,
        pattern: &str,
    ) -> PyResult<Option<WrapperXLSXSheetCellRead>> {
        Python::with_gil(|_py| {
            self.0
                .find_cell_by_pattern_regex(pattern)
                .map(|cell| cell.map(WrapperXLSXSheetCellRead))
                .map_err(|e| PyRuntimeError::new_err(format!("{}", e)))
        })
    }

    /// Поиск ячеек по шаблону
    pub fn find_cells_by_pattern_regex(
        &self,
        pattern: &str,
    ) -> PyResult<Vec<WrapperXLSXSheetCellRead>> {
        Python::with_gil(|_py| {
            self.0
                .find_cells_by_pattern_regex(pattern)
                .map(|cells| cells.into_iter().map(WrapperXLSXSheetCellRead).collect())
                .map_err(|e| PyRuntimeError::new_err(format!("{}", e)))
        })
    }

    /// Поиск ячеек колонок для строк которые соответствуют патерну
    #[pyo3(signature=(pattern, col_stop=None))]
    pub fn find_cells_for_rows_pattern_regex(
        &self,
        pattern: &str,
        col_stop: Option<u16>,
    ) -> PyResult<Vec<WrapperXLSXSheetCellRead>> {
        Python::with_gil(|_py| {
            self.0
                .find_cells_for_rows_pattern_regex(pattern, col_stop)
                .map(|cells| cells.into_iter().map(WrapperXLSXSheetCellRead).collect())
                .map_err(|e| PyRuntimeError::new_err(format!("{}", e)))
        })
    }

    /// Поиск ячеек строк для колонок которые соответствуют патерну
    #[pyo3(signature=(pattern, row_stop=None))]
    pub fn find_cells_for_cols_pattern_regex(
        &self,
        pattern: &str,
        row_stop: Option<u32>,
    ) -> PyResult<Vec<WrapperXLSXSheetCellRead>> {
        Python::with_gil(|_py| {
            self.0
                .find_cells_for_cols_pattern_regex(pattern, row_stop)
                .map(|cells| cells.into_iter().map(WrapperXLSXSheetCellRead).collect())
                .map_err(|e| PyRuntimeError::new_err(format!("{}", e)))
        })
    }

    /// Поиск ячеек с помощью ИЛИ ячейки по патернам
    pub fn find_cells_multi_pattern_regex(
        &self,
        pattern_1: &str,
        pattern_2: &str,
    ) -> PyResult<Vec<WrapperXLSXSheetCellRead>> {
        Python::with_gil(|_py| {
            self.0
                .find_cells_multi_pattern_regex(pattern_1, pattern_2)
                .map(|cells| cells.into_iter().map(WrapperXLSXSheetCellRead).collect())
                .map_err(|e| PyRuntimeError::new_err(format!("{}", e)))
        })
    }

    /// Поиск ячейки по буквенной координате A1 (cell)
    pub fn find_cell_by_cell(&self, cell: &str) -> PyResult<Option<WrapperXLSXSheetCellRead>> {
        Python::with_gil(|_py| {
            self.0
                .find_cell_by_cell(cell)
                .map(|cell| cell.map(WrapperXLSXSheetCellRead))
                .map_err(|e| PyRuntimeError::new_err(format!("{}", e)))
        })
    }

    /// Поиск ячейки по координате
    pub fn find_cell_by_coords(
        &self,
        row: u32,
        col: u16,
    ) -> PyResult<Option<WrapperXLSXSheetCellRead>> {
        Python::with_gil(|_py| {
            self.0
                .find_cell_by_coords(row, col)
                .map(|cell| cell.map(WrapperXLSXSheetCellRead))
                .map_err(|e| PyRuntimeError::new_err(format!("{}", e)))
        })
    }

    /// Поиск ячеек между шаьлонами
    pub fn find_cells_between_patterns(
        &self,
        pattern_after: &str,
        pattern_before: &str,
    ) -> PyResult<Vec<WrapperXLSXSheetCellRead>> {
        Python::with_gil(|_py| {
            self.0
                .find_cells_between_patterns(pattern_after, pattern_before)
                .map(|cells| cells.into_iter().map(WrapperXLSXSheetCellRead).collect())
                .map_err(|e| PyRuntimeError::new_err(format!("{}", e)))
        })
    }

    /// Возвращаем все ячейки, которые находятся в диапазоне строк
    pub fn find_cells_by_range_rows(
        &self,
        start_row: u32,
        end_row: u32,
    ) -> PyResult<Vec<WrapperXLSXSheetCellRead>> {
        Python::with_gil(|_py| {
            self.0
                .find_cells_by_range_rows(start_row, end_row)
                .map(|cells| cells.into_iter().map(WrapperXLSXSheetCellRead).collect())
                .map_err(|e| PyRuntimeError::new_err(format!("{}", e)))
        })
    }

    /// Возвращаем все ячейки, которые находятся в диапазоне колонок
    pub fn find_cells_by_range_cols(
        &self,
        start_col: u16,
        end_col: u16,
    ) -> PyResult<Vec<WrapperXLSXSheetCellRead>> {
        Python::with_gil(|_py| {
            self.0
                .find_cells_by_range_cols(start_col, end_col)
                .map(|cells| cells.into_iter().map(WrapperXLSXSheetCellRead).collect())
                .map_err(|e| PyRuntimeError::new_err(format!("{}", e)))
        })
    }

    /// Поиск значенияячейки по координате
    pub fn find_value_by_coords(&self, row: u32, col: u16) -> PyResult<PyObject> {
        Python::with_gil(|py| match self.0.find_value_by_coords(row, col) {
            Ok(Some(value)) => {
                let value = raw_value_to_py(py, &value.raw_value)?;
                Ok(value)
            }
            Ok(None) => Ok(py.None()),
            Err(e) => Err(PyRuntimeError::new_err(format!("{}", e))),
        })
    }
}
