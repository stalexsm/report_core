use std::{collections::HashMap, sync::Arc};

use anyhow::Result;
use parking_lot::RwLock;
use serde::Serialize;

use crate::traits::{ReadableSheet, WriteableSheet};

use super::{
    cell::Cell, cells::Cells, columns::Columns, comment::Comment, coordinate::Coordinate,
    merge_cells::MergeCells, range::Range, rows::Rows,
};

#[derive(Clone, Debug, Default, Serialize)]
pub struct Sheet {
    name: String,
    sheet_state: Box<str>,
    #[serde(flatten)]
    merge_cells: MergeCells,
    #[serde(flatten)]
    cells: Cells,
    row_dimensions: Rows,
    column_dimensions: Columns,
    comments: Vec<Arc<RwLock<Comment>>>,
}

impl Sheet {
    /// Интициализирует лист с заданным именем
    pub fn new(name: &str, sheet_state: &str) -> Self {
        Sheet {
            name: name.to_string(),
            sheet_state: sheet_state.into(),
            ..Default::default()
        }
    }

    #[inline]
    pub fn extract(
        name: &str,
        sheet_state: &str,
        range: Vec<Range>,
        map: HashMap<(u32, u16), Arc<RwLock<Cell>>>,
    ) -> Self {
        Sheet {
            name: name.to_string(),
            sheet_state: sheet_state.into(),
            merge_cells: MergeCells::new(range),
            cells: Cells::new(map),
            ..Default::default()
        }
    }
}

impl ReadableSheet for Sheet {
    type Cell = Cell;

    #[inline]
    fn get_name(&self) -> String {
        self.name.to_string()
    }

    #[inline]
    fn get_sheet_state(&self) -> String {
        self.sheet_state.to_string()
    }

    #[inline]
    fn get_cell_collection(&self) -> Vec<&Arc<RwLock<Cell>>> {
        self.cells.get_collection()
    }

    #[inline]
    fn get_cell_collection_sorted(&self) -> Vec<&Arc<RwLock<Cell>>> {
        self.cells.get_collection_sorted()
    }

    #[inline]
    fn get_max_row(&self) -> u32 {
        self.cells.get_max_row()
    }

    #[inline]
    fn get_max_column(&self) -> u16 {
        self.cells.get_max_column()
    }

    #[inline]
    fn get_cell_value<T>(&self, coordinate: T) -> String
    where
        T: Into<Coordinate>,
    {
        self.cells.get_cell_value(coordinate)
    }

    #[inline]
    fn get_cell_collection_by_range(
        &self,
        start_row: Option<u32>,
        end_row: Option<u32>,
        start_col: Option<u16>,
        end_col: Option<u16>,
    ) -> impl Iterator<Item = &Arc<RwLock<Cell>>> {
        self.cells
            .get_cell_collection_by_range(start_row, end_row, start_col, end_col)
    }

    #[inline]
    fn get_height_by_row(&self, row_num: u32) -> &f64 {
        self.row_dimensions.get_heignt(row_num)
    }

    #[inline]
    fn get_hidden_by_row(&self, row_num: u32) -> &bool {
        self.row_dimensions.get_hidden(row_num)
    }

    #[inline]
    fn get_width_by_column(&self, col_num: u16) -> &f64 {
        self.column_dimensions.get_width(col_num)
    }

    #[inline]
    fn get_hidden_by_column(&self, col_num: u16) -> &bool {
        self.column_dimensions.get_hidden(col_num)
    }

    #[inline]
    fn get_merge_cell_collection(&self) -> &[Range] {
        self.merge_cells.get_collection()
    }

    #[inline]
    fn get_comments(&self) -> &[Arc<RwLock<Comment>>] {
        &self.comments
    }

    #[inline]
    fn find_cell_by_regex(&self, regex: &str) -> Result<Option<&Arc<RwLock<Cell>>>> {
        self.cells.find_cell_by_regex(regex)
    }

    #[inline]
    fn find_cell_by_str(&self, value: &str) -> Result<Option<&Arc<RwLock<Cell>>>> {
        self.cells.find_cell_by_regex(value)
    }

    #[inline]
    fn find_cell_by_coords(&self, row: u32, col: u16) -> Result<Option<&Arc<RwLock<Cell>>>> {
        self.cells.find_cell_by_coords(row, col)
    }

    #[inline]
    fn find_cell_by_letter(&self, letter: &str) -> Result<Option<&Arc<RwLock<Cell>>>> {
        self.cells.find_cell_by_letter(letter)
    }

    #[inline]
    fn find_cells_by_regex(&self, regex: &str) -> Result<Vec<&Arc<RwLock<Cell>>>> {
        self.cells.find_cells_by_regex(regex)
    }

    #[inline]
    fn find_cells_by_str(&self, value: &str) -> Result<Vec<&Arc<RwLock<Cell>>>> {
        self.cells.find_cells_by_str(value)
    }

    #[inline]
    fn find_cells_for_rows_by_regex(
        &self,
        regex: &str,
        col_stop: u16,
    ) -> Result<Vec<&Arc<RwLock<Cell>>>> {
        self.cells.find_cells_for_rows_by_regex(regex, col_stop)
    }

    #[inline]
    fn find_cells_for_cols_by_regex(
        &self,
        regex: &str,
        row_stop: u32,
    ) -> Result<Vec<&Arc<RwLock<Cell>>>> {
        self.cells.find_cells_for_cols_by_regex(regex, row_stop)
    }

    #[inline]
    fn find_cells_multi_regex(
        &self,
        before_regex: &str,
        after_regex: &str,
    ) -> Result<Vec<&Arc<RwLock<Cell>>>> {
        self.cells.find_cells_multi_regex(before_regex, after_regex)
    }

    #[inline]
    fn find_cells_between_regex(
        &self,
        before_regex: &str,
        after_regex: &str,
    ) -> Result<Vec<&Arc<RwLock<Cell>>>> {
        self.cells
            .find_cells_between_regex(before_regex, after_regex)
    }

    #[inline]
    fn find_cells_range_rows(
        &self,
        start_row: u32,
        end_row: u32,
    ) -> Result<Vec<&Arc<RwLock<Cell>>>> {
        self.cells.find_cells_range_rows(start_row, end_row)
    }

    #[inline]
    fn find_cells_range_cols(
        &self,
        start_col: u16,
        end_col: u16,
    ) -> Result<Vec<&Arc<RwLock<Cell>>>> {
        self.cells.find_cells_range_cols(start_col, end_col)
    }

    #[inline]
    fn find_values_by_col_rows(&self, col: u16, rows: Vec<u32>) -> Result<Vec<String>> {
        self.cells.find_values_by_col_rows(col, rows)
    }

    #[inline]
    fn find_values_by_row_cols(&self, row: u32, cols: Vec<u16>) -> Result<Vec<String>> {
        self.cells.find_values_by_row_cols(row, cols)
    }

    #[inline]
    fn find_value_by_coords(&self, row: u32, col: u16) -> Result<Option<String>> {
        self.cells.find_value_by_coords(row, col)
    }
}

impl WriteableSheet for Sheet {
    #[inline]
    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    #[inline]
    fn set_sheet_state(&mut self, state: &str) {
        self.sheet_state = state.into();
    }

    #[inline]
    fn add_merge_range(&mut self, range: Range) {
        self.merge_cells.add_range(range);
    }

    #[inline]
    fn add_comments(&mut self, value: Comment) {
        let value = Arc::new(RwLock::new(value));
        self.comments.push(value);
    }

    #[inline]
    fn cell(&mut self, coordinate: Coordinate, value: Option<&str>) -> &Arc<RwLock<Cell>> {
        self.cells.cell(coordinate, value)
    }

    #[inline]
    fn delete_cols(&mut self, idx: u16, amount: u16) {
        self.cells.delete_cols(idx, amount);
    }

    #[inline]
    fn delete_rows(&mut self, idx: u32, amount: u32) {
        self.cells.delete_rows(idx, amount);
    }

    #[inline]
    fn set_height_row(&mut self, row_num: u32, val: f64) {
        self.row_dimensions.set_height(row_num, val);
    }

    #[inline]
    fn set_hidden_row(&mut self, row_num: u32, val: bool) {
        self.row_dimensions.set_hidden(row_num, val)
    }

    #[inline]
    fn set_width_column(&mut self, col_num: u16, val: f64) {
        self.column_dimensions.set_width(col_num, val);
    }

    #[inline]
    fn set_hidden_column(&mut self, col_num: u16, val: bool) {
        self.column_dimensions.set_hidden(col_num, val)
    }
}

#[cfg(test)]
mod tests {
    use crate::traits::ReadableCell;

    use super::*;

    fn sheet() -> Sheet {
        let mut sheet = Sheet::new("A", "visible");

        for r in 1..=5 {
            for c in 1..=5 {
                let coord = Coordinate::new(r, c);
                let val = format!("Yop! {r}:{c}");

                sheet.cell(coord, Some(&val));
            }
        }

        sheet
    }

    #[test]
    fn new_sheet() {
        let sheet = Sheet::new("test", "visible");

        assert_eq!(sheet.name, "test");
    }

    #[test]
    fn max_row() {
        let sheet = sheet();

        assert_eq!(sheet.get_max_row(), 5);
    }

    #[test]
    fn max_column() {
        let sheet = sheet();

        assert_eq!(sheet.get_max_column(), 5);
    }

    #[test]
    fn write_cell() {
        let mut sheet = Sheet::new("A", "visible");
        sheet.cell(Coordinate::new(1, 1), Some("Привет, мир!"));

        assert_eq!(sheet.get_cell_collection().len(), 1);
    }

    #[test]
    fn get_cell_collection() {
        let mut sheet = Sheet::new("A", "visible");
        sheet.cell(Coordinate::new(1, 1), Some("Привет, мир!"));

        assert_eq!(sheet.get_cell_collection().len(), 1);
    }

    #[test]
    fn get_cell_collection_sorted() {
        let mut sheet = Sheet::new("A", "visible");
        sheet.cell(Coordinate::new(1, 1), Some("Привет, мир!"));

        assert_eq!(sheet.get_cell_collection_sorted().len(), 1);
    }

    #[test]
    fn get_cell_value() {
        let mut sheet = Sheet::new("A", "visible");
        let coord = Coordinate::new(1, 1);

        sheet.cell(coord.clone(), Some("Привет, мир!"));

        assert_eq!(sheet.get_cell_value(coord), "Привет, мир!");
    }

    #[test]
    fn get_cell_collection_by_range() {
        let mut sheet = Sheet::new("A", "visible");

        for r in 1..=5 {
            for c in 1..=5 {
                let coord = Coordinate::new(r, c);
                let val = format!("Привет, мир! {r}:{c}");

                sheet.cell(coord, Some(&val));
            }
        }

        assert_eq!(
            sheet
                .get_cell_collection_by_range(Some(1), Some(2), Some(1), Some(2))
                .count(),
            4
        );
    }

    #[test]
    fn delete_rows() {
        let mut sheet = Sheet::new("A", "visible");

        for r in 1..=5 {
            for c in 1..=5 {
                let coord = Coordinate::new(r, c);
                let val = format!("Привет, мир! {r}:{c}");

                sheet.cell(coord, Some(&val));
            }
        }

        sheet.delete_rows(2, 4);

        assert_eq!(sheet.get_cell_collection().len(), 5);
    }

    #[test]
    fn delete_cols() {
        let mut sheet = Sheet::new("A", "visible");

        for r in 1..=5 {
            for c in 1..=5 {
                let coord = Coordinate::new(r, c);
                let val = format!("Привет, мир! {r}:{c}");

                sheet.cell(coord, Some(&val));
            }
        }

        sheet.delete_cols(2, 4);

        assert_eq!(sheet.get_cell_collection().len(), 5);
    }

    #[test]
    pub fn find_cell_by_regex() {
        let sheet = sheet();
        let regex = "Yop! 3:3";

        let cell = sheet.cells.find_cell_by_regex(regex).unwrap();

        assert!(cell.is_some());
        assert_eq!(cell.unwrap().read().get_value(), "Yop! 3:3");
    }

    #[test]
    pub fn find_cell_by_str() {
        let sheet = sheet();
        let regex = "Yop! 3:3";

        let cell = sheet.cells.find_cell_by_str(regex).unwrap();

        assert!(cell.is_some());
        assert_eq!(cell.unwrap().read().get_value(), "Yop! 3:3");
    }

    #[test]
    pub fn find_cell_by_coords() {
        let sheet = sheet();

        let cell = sheet.cells.find_cell_by_coords(1, 1).unwrap();

        assert!(cell.is_some());
        assert_eq!(cell.unwrap().read().get_value(), "Yop! 1:1");
    }

    #[test]
    pub fn find_cell_by_letter() {
        let sheet = sheet();
        let letter = "C1";

        let cell = sheet.cells.find_cell_by_letter(letter).unwrap();

        assert!(cell.is_some());
        assert_eq!(cell.unwrap().read().get_value(), "Yop! 1:3");
    }

    #[test]
    pub fn find_cells_by_regex() {
        let sheet = sheet();
        let regex = "Yop! 2:2";

        let cells = sheet.cells.find_cells_by_regex(regex).unwrap();

        assert_eq!(cells.len(), 1);
    }

    #[test]
    pub fn find_cells_by_str() {
        let sheet = sheet();
        let regex = "Yop! 2:2";

        let cells = sheet.cells.find_cells_by_str(regex).unwrap();

        assert_eq!(cells.len(), 1);
    }

    #[test]
    pub fn find_cells_for_rows_by_regex() {
        let sheet = sheet();
        let regex = "Yop!";
        let col_stop = 2;

        let cells = sheet
            .cells
            .find_cells_for_rows_by_regex(regex, col_stop)
            .unwrap();

        assert_eq!(cells.len(), 10);
    }

    #[test]
    pub fn find_cells_for_cols_by_regex() {
        let sheet = sheet();
        let regex = "Yop!";
        let row_stop = 2;

        let cells = sheet
            .cells
            .find_cells_for_cols_by_regex(regex, row_stop)
            .unwrap();

        assert_eq!(cells.len(), 10);
    }

    #[test]
    pub fn find_cells_multi_regex() {
        let sheet = sheet();

        let before_regex = "Yop! 1:1";
        let after_regex = "Yop! 5:5";

        let cells = sheet
            .cells
            .find_cells_multi_regex(before_regex, after_regex)
            .unwrap();

        assert_eq!(cells.len(), 2);
    }

    #[test]
    pub fn find_cells_between_regex() {
        let sheet = sheet();

        let before_regex = "Yop! 1:5";
        let after_regex = "Yop! 2:5";

        let cells = sheet
            .cells
            .find_cells_between_regex(before_regex, after_regex)
            .unwrap();

        assert_eq!(cells.len(), 10);
    }

    #[test]
    pub fn find_cells_range_rows() {
        let sheet = sheet();
        let start_row = 1;
        let end_row = 2;

        let cells = sheet
            .cells
            .find_cells_range_rows(start_row, end_row)
            .unwrap();

        assert_eq!(cells.len(), 10);
    }

    #[test]
    pub fn find_cells_range_cols() {
        let sheet = sheet();

        let start_col = 1;
        let end_col = 2;

        let cells = sheet
            .cells
            .find_cells_range_cols(start_col, end_col)
            .unwrap();

        assert_eq!(cells.len(), 10);
    }

    #[test]
    pub fn find_values_by_row_cols() {
        let sheet = sheet();

        let row = 1;
        let cols = vec![1, 2];

        let cells = sheet.cells.find_values_by_row_cols(row, cols).unwrap();

        assert_eq!(cells.len(), 2);
    }

    #[test]
    pub fn find_values_by_col_rows() {
        let sheet = sheet();

        let col = 2;
        let rows = vec![1, 2, 3];

        let cells = sheet.cells.find_values_by_col_rows(col, rows).unwrap();

        assert_eq!(cells.len(), 3);
    }

    #[test]
    pub fn find_value_by_coords() {
        let sheet = sheet();

        let value = sheet.cells.find_value_by_coords(1, 1).unwrap();

        assert_eq!(value, Some(format!("Yop! {}:{}", 1, 1)));
    }

    #[test]
    pub fn add_merge_range() {
        let mut sheet = sheet();

        let range = Range::new(1, 2, 1, 2);
        sheet.add_merge_range(range);

        let range = sheet.get_merge_cell_collection();

        assert_eq!(range.len(), 1);
    }

    #[test]
    pub fn get_merge_cell_collection() {
        let sheet = sheet();
        let range = sheet.get_merge_cell_collection();

        assert_eq!(range.len(), 0);
    }

    #[test]
    pub fn get_comments() {
        let sheet = sheet();
        let comments = sheet.get_comments();

        assert_eq!(comments.len(), 0);
    }

    #[test]
    pub fn add_comments() {
        let mut sheet = sheet();
        sheet.add_comments(Comment::new(Coordinate::new(1, 1), "A.C"));

        assert_eq!(sheet.get_comments().len(), 1);
    }
}
