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
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
