use std::sync::Arc;

use ahash::HashMap;
use core_rs::{
    structs::{
        cell::Cell,
        coordinate::Coordinate,
        range::{MergedRange, Range},
        sheet::Sheet,
    },
    traits::{ReadableCell, ReadableSheet},
};
use parking_lot::RwLock;
use pyo3::{
    prelude::*,
    types::{PyDict, PyList, PyString},
};

use super::cell::WrapperCell;
use crate::py_extract;

/// Вспомогптельная функция для преобразования cells в rust тип
type CellMap = HashMap<(u32, u16), Arc<RwLock<Cell>>>;

fn extract_cells(obj: &Bound<'_, PyAny>) -> PyResult<CellMap> {
    let cells_attr = if obj.is_instance_of::<PyDict>() {
        obj.get_item("cells")?
    } else {
        obj.getattr("cells")?
    };

    let cells_list = cells_attr.cast::<PyList>()?;
    cells_list
        .iter()
        .map(|c| {
            let wrapper = WrapperCell::try_from(&c)?;
            let guard = wrapper.0.read();
            let coord = guard.get_coordinate();
            Ok(((coord.row, coord.column), wrapper.0.clone()))
        })
        .collect()
}

#[pyclass]
#[pyo3(module = "report_core.readable", name = "ReadableSheet")]
#[derive(Debug, Clone)]
pub struct WrapperSheet(pub(crate) Arc<RwLock<Sheet>>);

impl TryFrom<&Bound<'_, PyAny>> for WrapperSheet {
    type Error = PyErr;

    fn try_from(obj: &Bound<'_, PyAny>) -> PyResult<Self> {
        Python::attach(|_py| {
            let name = py_extract!(obj, name)?.as_string_direct();
            let sheet_state = py_extract!(obj, sheet_state)?.as_string_direct();
            let merge_cells = py_extract!(obj, merge_cells)?
                .as_u32_vec_array::<4>()
                .into_iter()
                .map(|v| Range::new(v[0], v[1], v[2] as u16, v[3] as u16))
                .collect();

            let map = extract_cells(obj)?;

            let sheet = Sheet::extract(&name, &sheet_state, merge_cells, map);
            Ok(Self(Arc::new(RwLock::new(sheet))))
        })
    }
}

#[pymethods]
impl WrapperSheet {
    pub fn __repr__(slf: &Bound<'_, Self>) -> PyResult<String> {
        Python::attach(|_py| {
            let class_name: Bound<'_, PyString> = slf.get_type().qualname()?;

            let slf = slf.borrow();
            let slf = slf.0.read();

            Ok(format!(
                "{} ({}) cells: {}",
                class_name,
                slf.get_name(),
                slf.get_cell_collection().len()
            ))
        })
    }

    #[getter]
    pub fn name(&self) -> PyResult<String> {
        Python::attach(|_py| {
            let slf = self.0.read();
            Ok(slf.get_name())
        })
    }

    #[getter]
    pub fn sheet_state(&self) -> PyResult<String> {
        Python::attach(|_py| {
            let slf = self.0.read();
            Ok(slf.get_sheet_state())
        })
    }

    #[getter]
    pub fn get_max_row(&self) -> PyResult<u32> {
        Python::attach(|_py| {
            let slf = self.0.read();

            Ok(slf.get_max_row())
        })
    }

    #[getter]
    pub fn get_max_column(&self) -> PyResult<u16> {
        Python::attach(|_py| {
            let slf = self.0.read();

            Ok(slf.get_max_column())
        })
    }

    #[getter]
    pub fn get_cells(&self) -> PyResult<Vec<WrapperCell>> {
        Python::attach(|_py| {
            let slf = self.0.read();

            let cells = slf
                .get_cell_collection_sorted()
                .into_iter()
                .map(|cell| WrapperCell(cell.clone()))
                .collect();

            Ok(cells)
        })
    }

    #[getter]
    pub fn get_merge_cells(&self) -> PyResult<Vec<MergedRange>> {
        Python::attach(|_py| {
            let merged_cells = self
                .0
                .read()
                .get_merge_cell_collection()
                .iter()
                .map(|range| range.into())
                .collect();

            Ok(merged_cells)
        })
    }

    pub fn get_value_cell(&self, row: u32, col: u16) -> PyResult<String> {
        Python::attach(|_py| {
            let slf = self.0.read();

            let coord = Coordinate::from((row, col));
            Ok(slf.get_cell_value(coord))
        })
    }

    #[pyo3(signature = (start_row=None, end_row=None, start_col=None, end_col=None))]
    pub fn get_cells_by_range(
        &self,
        py: Python<'_>,
        start_row: Option<u32>,
        end_row: Option<u32>,
        start_col: Option<u16>,
        end_col: Option<u16>,
    ) -> PyResult<Vec<WrapperCell>> {
        py.detach(|| {
            let slf = self.0.read();

            let cells = slf
                .get_cell_collection_by_range(start_row, end_row, start_col, end_col)
                .map(|cell| WrapperCell(cell.clone()))
                .collect::<Vec<_>>();

            Ok(cells)
        })
    }

    pub fn find_cell_by_regex(&self, py: Python<'_>, regex: &str) -> PyResult<Option<WrapperCell>> {
        py.detach(|| {
            let slf = self.0.read();

            Ok(slf
                .find_cell_by_regex(regex)?
                .map(|c| WrapperCell(c.clone())))
        })
    }

    pub fn find_cell_by_str(&self, py: Python<'_>, value: &str) -> PyResult<Option<WrapperCell>> {
        py.detach(|| {
            let slf = self.0.read();

            Ok(slf.find_cell_by_str(value)?.map(|c| WrapperCell(c.clone())))
        })
    }

    pub fn find_cell_by_coords(
        &self,
        py: Python<'_>,
        row: u32,
        col: u16,
    ) -> PyResult<Option<WrapperCell>> {
        py.detach(|| {
            let slf = self.0.read();

            Ok(slf
                .find_cell_by_coords(row, col)?
                .map(|c| WrapperCell(c.clone())))
        })
    }

    pub fn find_cell_by_letter(
        &self,
        py: Python<'_>,
        letter: &str,
    ) -> PyResult<Option<WrapperCell>> {
        py.detach(|| {
            let slf = self.0.read();

            Ok(slf
                .find_cell_by_letter(letter)?
                .map(|c| WrapperCell(c.clone())))
        })
    }

    pub fn find_cells_by_regex(&self, py: Python<'_>, regex: &str) -> PyResult<Vec<WrapperCell>> {
        py.detach(|| {
            let slf = self.0.read();

            let wrapper_cells = slf
                .find_cells_by_regex(regex)?
                .into_iter()
                .map(|cell| WrapperCell(cell.clone()))
                .collect();

            Ok(wrapper_cells)
        })
    }

    pub fn find_cells_by_str(&self, py: Python<'_>, value: &str) -> PyResult<Vec<WrapperCell>> {
        py.detach(|| {
            let slf = self.0.read();

            let wrapper_cells = slf
                .find_cells_by_str(value)?
                .into_iter()
                .map(|cell| WrapperCell(cell.clone()))
                .collect();

            Ok(wrapper_cells)
        })
    }

    pub fn find_cells_for_rows_by_regex(
        &self,
        py: Python<'_>,
        regex: &str,
        col_stop: u16,
    ) -> PyResult<Vec<WrapperCell>> {
        py.detach(|| {
            let slf = self.0.read();

            let wrapper_cells = slf
                .find_cells_for_rows_by_regex(regex, col_stop)?
                .into_iter()
                .map(|cell| WrapperCell(cell.clone()))
                .collect();

            Ok(wrapper_cells)
        })
    }

    pub fn find_cells_for_cols_by_regex(
        &self,
        py: Python<'_>,
        regex: &str,
        row_stop: u32,
    ) -> PyResult<Vec<WrapperCell>> {
        py.detach(|| {
            let slf = self.0.read();

            let wrapper_cells = slf
                .find_cells_for_cols_by_regex(regex, row_stop)?
                .into_iter()
                .map(|cell| WrapperCell(cell.clone()))
                .collect();

            Ok(wrapper_cells)
        })
    }

    pub fn find_cells_multi_regex(
        &self,
        py: Python<'_>,
        before_regex: &str,
        after_regex: &str,
    ) -> PyResult<Vec<WrapperCell>> {
        py.detach(|| {
            let slf = self.0.read();

            let wrapper_cells = slf
                .find_cells_multi_regex(before_regex, after_regex)?
                .into_iter()
                .map(|cell| WrapperCell(cell.clone()))
                .collect();

            Ok(wrapper_cells)
        })
    }

    pub fn find_cells_between_regex(
        &self,
        py: Python<'_>,
        before_regex: &str,
        after_regex: &str,
    ) -> PyResult<Vec<WrapperCell>> {
        py.detach(|| {
            let slf = self.0.read();

            let wrapper_cells = slf
                .find_cells_between_regex(before_regex, after_regex)?
                .into_iter()
                .map(|cell| WrapperCell(cell.clone()))
                .collect();

            Ok(wrapper_cells)
        })
    }

    pub fn find_cells_range_rows(
        &self,
        py: Python<'_>,
        start_row: u32,
        end_row: u32,
    ) -> PyResult<Vec<WrapperCell>> {
        py.detach(|| {
            let slf = self.0.read();

            let wrapper_cells = slf
                .find_cells_range_rows(start_row, end_row)?
                .into_iter()
                .map(|cell| WrapperCell(cell.clone()))
                .collect();

            Ok(wrapper_cells)
        })
    }

    pub fn find_cells_range_cols(
        &self,
        py: Python<'_>,
        start_col: u16,
        end_col: u16,
    ) -> PyResult<Vec<WrapperCell>> {
        py.detach(|| {
            let slf = self.0.read();

            let wrapper_cells = slf
                .find_cells_range_cols(start_col, end_col)?
                .into_iter()
                .map(|cell| WrapperCell(cell.clone()))
                .collect();

            Ok(wrapper_cells)
        })
    }

    pub fn find_values_by_col_rows(
        &self,
        py: Python<'_>,
        col: u16,
        rows: Vec<u32>,
    ) -> PyResult<Vec<String>> {
        py.detach(|| {
            let slf = self.0.read();

            Ok(slf.find_values_by_col_rows(col, rows)?)
        })
    }

    pub fn find_values_by_row_cols(
        &self,
        py: Python<'_>,
        row: u32,
        cols: Vec<u16>,
    ) -> PyResult<Vec<String>> {
        py.detach(|| {
            let slf = self.0.read();

            Ok(slf.find_values_by_row_cols(row, cols)?)
        })
    }

    pub fn find_value_by_coords(
        &self,
        py: Python<'_>,
        row: u32,
        col: u16,
    ) -> PyResult<Option<String>> {
        py.detach(|| {
            let slf = self.0.read();

            Ok(slf.find_value_by_coords(row, col)?)
        })
    }
}
