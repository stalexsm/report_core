use anyhow::Result;
use serde::Serialize;

use super::{
    cell::Cell, cells::Cells, coordinate::Coordinate, merge_cells::MergeCells, range::Range,
};

#[derive(Clone, Debug, Default, Serialize)]
pub struct Sheet {
    pub name: String,
    merge_cells: MergeCells,
    cells: Cells,
}

impl Sheet {
    /// Интициализирует лист с заданным именем
    pub fn new(name: &str) -> Self {
        Sheet {
            name: name.to_string(),
            ..Default::default()
        }
    }

    #[inline]
    pub fn get_cell_collection(&self) -> Vec<&Cell> {
        self.cells.get_collection()
    }

    #[inline]
    pub fn get_cell_collection_sorted(&self) -> Vec<&Cell> {
        self.cells.get_collection_sorted()
    }

    #[inline]
    pub fn get_cell<T>(&self, coordinate: T) -> Option<&Cell>
    where
        T: Into<Coordinate>,
    {
        self.cells.get_cell(coordinate)
    }

    #[inline]
    pub fn get_cell_mut<T>(&mut self, coordinate: T) -> Option<&mut Cell>
    where
        T: Into<Coordinate>,
    {
        self.cells.get_cell_mut(coordinate)
    }

    #[inline]
    pub fn get_cell_value<T>(&self, coordinate: T) -> String
    where
        T: Into<Coordinate>,
    {
        self.cells.get_cell_value(coordinate)
    }

    #[inline]
    pub fn get_cell_collection_by_range(&self, range: &Range) -> impl Iterator<Item = &Cell> {
        self.cells.get_cell_collection_by_range(range)
    }

    #[inline]
    pub fn get_cell_collection_by_range_mut(
        &mut self,
        range: &Range,
    ) -> impl Iterator<Item = &mut Cell> {
        self.cells.get_cell_collection_by_range_mut(range)
    }

    #[inline]
    pub fn get_merge_cell_collection(&self) -> &[Range] {
        self.merge_cells.get_collection()
    }

    #[inline]
    pub fn add_merge_range(&mut self, range: Range) {
        self.merge_cells.add_range(range);
    }

    #[inline]
    pub fn write_cell(&mut self, coordinate: Coordinate, value: &str) -> &mut Cell {
        self.cells.write_cell(coordinate, value)
    }

    #[inline]
    pub fn delete_cols(&mut self, idx: u16, amount: u16) {
        self.cells.delete_cols(idx, amount);
    }

    #[inline]
    pub fn delete_rows(&mut self, idx: u32, amount: u32) {
        self.cells.delete_rows(idx, amount);
    }

    #[inline]
    pub fn find_cell_by_regex(&self, regex: &str) -> Result<Option<&Cell>> {
        self.cells.find_cell_by_regex(regex)
    }

    #[inline]
    pub fn find_cell_by_letter(&self, letter: &str) -> Result<Option<&Cell>> {
        self.cells.find_cell_by_letter(letter)
    }

    #[inline]
    pub fn find_cells_by_regex(&self, regex: &str) -> Result<Vec<&Cell>> {
        self.cells.find_cells_by_regex(regex)
    }

    #[inline]
    pub fn find_cells_for_rows_by_regex(&self, regex: &str, col_stop: u16) -> Result<Vec<&Cell>> {
        self.cells.find_cells_for_rows_by_regex(regex, col_stop)
    }

    #[inline]
    pub fn find_cells_for_cols_by_regex(&self, regex: &str, row_stop: u32) -> Result<Vec<&Cell>> {
        self.cells.find_cells_for_cols_by_regex(regex, row_stop)
    }

    #[inline]
    pub fn find_cells_multi_regex(
        &self,
        before_regex: &str,
        after_regex: &str,
    ) -> Result<Vec<&Cell>> {
        self.cells.find_cells_multi_regex(before_regex, after_regex)
    }

    #[inline]
    pub fn find_cells_between_regex(
        &self,
        before_regex: &str,
        after_regex: &str,
    ) -> Result<Vec<&Cell>> {
        self.cells
            .find_cells_between_regex(before_regex, after_regex)
    }

    #[inline]
    pub fn find_cells_range_rows(&self, start_row: u32, end_row: u32) -> Result<Vec<&Cell>> {
        self.cells.find_cells_range_rows(start_row, end_row)
    }

    #[inline]
    pub fn find_cells_range_cols(&self, start_col: u16, end_col: u16) -> Result<Vec<&Cell>> {
        self.cells.find_cells_range_cols(start_col, end_col)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sheet() -> Sheet {
        let mut sheet = Sheet::new("A");

        for r in 1..=5 {
            for c in 1..=5 {
                let coord = Coordinate::new(r, c);
                let val = format!("Yop! {}:{}", r, c);

                sheet.write_cell(coord, &val);
            }
        }

        sheet
    }

    #[test]
    fn new_sheet() {
        let sheet = Sheet::new("test");

        assert_eq!(sheet.name, "test");
    }

    #[test]
    fn write_cell() {
        let mut sheet = Sheet::new("A");
        sheet.write_cell(Coordinate::new(1, 1), "Привет, мир!");

        assert_eq!(sheet.get_cell_collection().len(), 1);
    }

    #[test]
    fn get_cell_collection() {
        let mut sheet = Sheet::new("A");
        sheet.write_cell(Coordinate::new(1, 1), "Привет, мир!");

        assert_eq!(sheet.get_cell_collection().len(), 1);
    }

    #[test]
    fn get_cell_collection_sorted() {
        let mut sheet = Sheet::new("A");
        sheet.write_cell(Coordinate::new(1, 1), "Привет, мир!");

        assert_eq!(sheet.get_cell_collection_sorted().len(), 1);
    }

    #[test]
    fn get_cell() {
        let mut sheet = Sheet::new("A");
        let coord = Coordinate::new(1, 1);

        sheet.write_cell(coord.clone(), "Привет, мир!");

        assert!(sheet.get_cell(coord).is_some());
    }

    #[test]
    fn get_cell_mut() {
        let mut sheet = Sheet::new("A");
        let coord = Coordinate::new(1, 1);

        sheet.write_cell(coord.clone(), "Привет, мир!");

        assert!(sheet.get_cell_mut(coord).is_some());
    }

    #[test]
    fn get_cell_value() {
        let mut sheet = Sheet::new("A");
        let coord = Coordinate::new(1, 1);

        sheet.write_cell(coord.clone(), "Привет, мир!");

        assert_eq!(sheet.get_cell_value(coord), "Привет, мир!");
    }

    #[test]
    fn get_cell_collection_by_range() {
        let mut sheet = Sheet::new("A");

        for r in 1..=5 {
            for c in 1..=5 {
                let coord = Coordinate::new(r, c);
                let val = format!("Привет, мир! {}:{}", r, c);

                sheet.write_cell(coord, &val);
            }
        }

        assert_eq!(
            sheet
                .get_cell_collection_by_range(&Range::new(1, 2, 1, 2))
                .count(),
            4
        );
    }

    #[test]
    fn get_cell_collection_by_range_mut() {
        let mut sheet = Sheet::new("A");

        for r in 1..=5 {
            for c in 1..=5 {
                let coord = Coordinate::new(r, c);
                let val = format!("Привет, мир! {}:{}", r, c);

                sheet.write_cell(coord, &val);
            }
        }

        assert_eq!(
            sheet
                .get_cell_collection_by_range_mut(&Range::new(1, 2, 1, 2))
                .count(),
            4
        );
    }

    #[test]
    fn delete_rows() {
        let mut sheet = Sheet::new("A");

        for r in 1..=5 {
            for c in 1..=5 {
                let coord = Coordinate::new(r, c);
                let val = format!("Привет, мир! {}:{}", r, c);

                sheet.write_cell(coord, &val);
            }
        }

        sheet.delete_rows(2, 4);

        assert_eq!(sheet.get_cell_collection().len(), 5);
    }

    #[test]
    fn delete_cols() {
        let mut sheet = Sheet::new("A");

        for r in 1..=5 {
            for c in 1..=5 {
                let coord = Coordinate::new(r, c);
                let val = format!("Привет, мир! {}:{}", r, c);

                sheet.write_cell(coord, &val);
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
        assert_eq!(cell.unwrap().get_value(), "Yop! 3:3");
    }

    #[test]
    pub fn find_cell_by_letter() {
        let sheet = sheet();
        let letter = "C1";

        let cell = sheet.cells.find_cell_by_letter(letter).unwrap();

        assert!(cell.is_some());
        assert_eq!(cell.unwrap().get_value(), "Yop! 1:3");
    }

    #[test]
    pub fn find_cells_by_regex() {
        let sheet = sheet();
        let regex = "Yop! 2:2";

        let cells = sheet.cells.find_cells_by_regex(regex).unwrap();

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
}
