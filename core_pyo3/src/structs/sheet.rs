use std::sync::Arc;

use core_rs::{
    structs::{
        comment::Comment,
        coordinate::Coordinate,
        range::{MergedRange, Range},
        sheet::Sheet,
    },
    traits::{ReadableSheet, WriteableSheet},
};
use parking_lot::RwLock;
use pyo3::prelude::*;

use super::{cell::WrapperCell, comment::WrapperComment};

#[pyclass(from_py_object)]
#[pyo3(module = "report_core", name = "Sheet")]
#[derive(Debug, Clone)]
pub struct WrapperSheet(pub(crate) Arc<RwLock<Sheet>>);

#[pymethods]
impl WrapperSheet {
    pub fn __repr__(slf: &Bound<'_, Self>) -> PyResult<String> {
        let slf = slf.borrow();
        let slf = slf.0.read();
        Ok(format!(
            "Sheet ({}) cells: {}",
            slf.get_name(),
            slf.get_cell_collection().len()
        ))
    }

    #[getter]
    pub fn name(&self) -> PyResult<String> {
        let slf = self.0.read();
        Ok(slf.get_name())
    }

    #[setter]
    pub fn set_name(&self, name: String) -> PyResult<()> {
        let mut slf = self.0.write();
        slf.set_name(name.as_str());

        Ok(())
    }

    #[getter]
    pub fn sheet_state(&self) -> PyResult<String> {
        let slf = self.0.read();
        Ok(slf.get_sheet_state())
    }

    pub fn set_sheet_state(&self, state: &str) -> PyResult<()> {
        let mut slf = self.0.write();
        slf.set_sheet_state(state);

        Ok(())
    }

    #[getter]
    pub fn get_max_row(&self) -> PyResult<u32> {
        let slf = self.0.read();

        Ok(slf.get_max_row())
    }

    #[getter]
    pub fn get_max_column(&self) -> PyResult<u16> {
        let slf = self.0.read();

        Ok(slf.get_max_column())
    }

    #[getter]
    pub fn get_cells(&self) -> PyResult<Vec<WrapperCell>> {
        let slf = self.0.read();

        let cells = slf
            .get_cell_collection_sorted()
            .into_iter()
            .map(|cell| WrapperCell(Arc::clone(cell)))
            .collect();

        Ok(cells)
    }

    #[getter]
    pub fn get_merge_cells(&self) -> PyResult<Vec<MergedRange>> {
        let merged_cells = self
            .0
            .read()
            .get_merge_cell_collection()
            .iter()
            .map(|range| range.into())
            .collect();

        Ok(merged_cells)
    }

    pub fn add_merge_cells(
        &mut self,
        start_row: u32,
        end_row: u32,
        start_col: u16,
        end_col: u16,
    ) -> PyResult<()> {
        let range = Range::new(start_row, end_row, start_col, end_col);

        self.0.write().add_merge_range(range);

        Ok(())
    }

    pub fn add_comment(&mut self, row: u32, col: u16, text: &str, author: &str) -> PyResult<()> {
        let coord = Coordinate::new(row, col);
        let mut comment = Comment::new(coord, author);
        comment.set_text(text);

        self.0.write().add_comments(comment);

        Ok(())
    }

    #[getter]
    pub fn get_comments(&self) -> PyResult<Vec<WrapperComment>> {
        let comments = self
            .0
            .read()
            .get_comments()
            .iter()
            .map(|c| WrapperComment(Arc::clone(c)))
            .collect();

        Ok(comments)
    }

    #[pyo3(signature = (row, col, value=None))]
    pub fn cell(&self, row: u32, col: u16, value: Option<&str>) -> PyResult<WrapperCell> {
        let coord = Coordinate::from((row, col));
        let mut slf = self.0.write();

        let cell = slf.cell(coord, value);

        Ok(WrapperCell(Arc::clone(cell)))
    }

    pub fn get_value_cell(&self, row: u32, col: u16) -> PyResult<String> {
        let slf = self.0.read();

        let coord = Coordinate::from((row, col));
        Ok(slf.get_cell_value(coord))
    }

    pub fn delete_cols(&self, idx: u16, amount: u16) -> PyResult<()> {
        let mut slf = self.0.write();
        slf.delete_cols(idx, amount);

        Ok(())
    }

    pub fn delete_rows(&self, idx: u32, amount: u32) -> PyResult<()> {
        let mut slf = self.0.write();
        slf.delete_rows(idx, amount);

        Ok(())
    }

    pub fn set_height_row(&self, row_num: u32, val: f64) -> PyResult<()> {
        let mut slf = self.0.write();

        slf.set_height_row(row_num, val);
        Ok(())
    }

    pub fn set_hidden_row(&self, row_num: u32, val: bool) -> PyResult<()> {
        let mut slf = self.0.write();

        slf.set_hidden_row(row_num, val);
        Ok(())
    }

    pub fn set_width_column(&self, col_num: u16, val: f64) -> PyResult<()> {
        let mut slf = self.0.write();

        slf.set_width_column(col_num, val);
        Ok(())
    }

    pub fn set_hidden_column(&self, col_num: u16, val: bool) -> PyResult<()> {
        let mut slf = self.0.write();

        slf.set_hidden_column(col_num, val);
        Ok(())
    }

    #[pyo3(signature = (start_row=None, end_row=None, start_col=None, end_col=None))]
    pub fn get_cells_by_range(
        &self,
        start_row: Option<u32>,
        end_row: Option<u32>,
        start_col: Option<u16>,
        end_col: Option<u16>,
    ) -> PyResult<Vec<WrapperCell>> {
        let slf = self.0.read();

        let cells = slf
            .get_cell_collection_by_range(start_row, end_row, start_col, end_col)
            .map(|cell| WrapperCell(Arc::clone(cell)))
            .collect::<Vec<_>>();

        Ok(cells)
    }

    pub fn get_height_by_row(&self, row_num: u32) -> PyResult<f64> {
        let slf = self.0.read();

        let val = slf.get_height_by_row(row_num);
        Ok(*val)
    }

    pub fn get_hidden_by_row(&self, row_num: u32) -> PyResult<bool> {
        let slf = self.0.read();

        let val = slf.get_hidden_by_row(row_num);
        Ok(*val)
    }

    pub fn get_width_by_column(&self, col_num: u16) -> PyResult<f64> {
        let slf = self.0.read();

        let val = slf.get_width_by_column(col_num);
        Ok(*val)
    }

    pub fn get_hidden_by_column(&self, col_num: u16) -> PyResult<bool> {
        let slf = self.0.read();

        let val = slf.get_hidden_by_column(col_num);
        Ok(*val)
    }

    pub fn find_cell_by_regex(&self, regex: &str) -> PyResult<Option<WrapperCell>> {
        let slf = self.0.read();

        Ok(slf
            .find_cell_by_regex(regex)?
            .map(|c| WrapperCell(Arc::clone(c))))
    }

    pub fn find_cell_by_str(&self, value: &str) -> PyResult<Option<WrapperCell>> {
        let slf = self.0.read();

        Ok(slf
            .find_cell_by_str(value)?
            .map(|c| WrapperCell(Arc::clone(c))))
    }

    pub fn find_cell_by_letter(&self, letter: &str) -> PyResult<Option<WrapperCell>> {
        let slf = self.0.read();

        Ok(slf
            .find_cell_by_letter(letter)?
            .map(|c| WrapperCell(Arc::clone(c))))
    }

    pub fn find_cells_by_regex(&self, regex: &str) -> PyResult<Vec<WrapperCell>> {
        let slf = self.0.read();

        let cells = slf.find_cells_by_regex(regex)?;
        let wrapped_cells = cells
            .into_iter()
            .map(|cell| WrapperCell(Arc::clone(cell)))
            .collect();

        Ok(wrapped_cells)
    }

    pub fn find_cells_by_str(&self, value: &str) -> PyResult<Vec<WrapperCell>> {
        let slf = self.0.read();

        let cells = slf.find_cells_by_str(value)?;
        let wrapped_cells = cells
            .into_iter()
            .map(|cell| WrapperCell(Arc::clone(cell)))
            .collect();

        Ok(wrapped_cells)
    }

    pub fn find_cells_for_rows_by_regex(
        &self,
        regex: &str,
        col_stop: u16,
    ) -> PyResult<Vec<WrapperCell>> {
        let slf = self.0.read();

        let cells = slf.find_cells_for_rows_by_regex(regex, col_stop)?;
        let wrapped_cells = cells
            .into_iter()
            .map(|cell| WrapperCell(Arc::clone(cell)))
            .collect();

        Ok(wrapped_cells)
    }

    pub fn find_cells_for_cols_by_regex(
        &self,
        regex: &str,
        row_stop: u32,
    ) -> PyResult<Vec<WrapperCell>> {
        let slf = self.0.read();

        let cells = slf.find_cells_for_cols_by_regex(regex, row_stop)?;
        let wrapped_cells = cells
            .into_iter()
            .map(|cell| WrapperCell(Arc::clone(cell)))
            .collect();

        Ok(wrapped_cells)
    }

    pub fn find_cells_multi_regex(
        &self,
        before_regex: &str,
        after_regex: &str,
    ) -> PyResult<Vec<WrapperCell>> {
        let slf = self.0.read();

        let cells = slf.find_cells_multi_regex(before_regex, after_regex)?;
        let wrapped_cells = cells
            .into_iter()
            .map(|cell| WrapperCell(Arc::clone(cell)))
            .collect();

        Ok(wrapped_cells)
    }

    pub fn find_cells_between_regex(
        &self,
        before_regex: &str,
        after_regex: &str,
    ) -> PyResult<Vec<WrapperCell>> {
        let slf = self.0.read();

        let cells = slf.find_cells_between_regex(before_regex, after_regex)?;
        let wrapped_cells = cells
            .into_iter()
            .map(|cell| WrapperCell(Arc::clone(cell)))
            .collect();

        Ok(wrapped_cells)
    }

    pub fn find_cells_range_rows(
        &self,
        start_row: u32,
        end_row: u32,
    ) -> PyResult<Vec<WrapperCell>> {
        let slf = self.0.read();

        let cells = slf.find_cells_range_rows(start_row, end_row)?;
        let wrapped_cells = cells
            .into_iter()
            .map(|cell| WrapperCell(Arc::clone(cell)))
            .collect();

        Ok(wrapped_cells)
    }

    pub fn find_cells_range_cols(
        &self,
        start_col: u16,
        end_col: u16,
    ) -> PyResult<Vec<WrapperCell>> {
        let slf = self.0.read();

        let cells = slf.find_cells_range_cols(start_col, end_col)?;
        let wrapped_cells = cells
            .into_iter()
            .map(|cell| WrapperCell(Arc::clone(cell)))
            .collect();

        Ok(wrapped_cells)
    }

    pub fn find_value_by_coords(&self, row: u32, col: u16) -> PyResult<Option<String>> {
        let slf = self.0.read();

        Ok(slf.find_value_by_coords(row, col)?)
    }
}
