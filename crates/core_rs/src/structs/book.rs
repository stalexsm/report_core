use std::sync::Arc;

use ahash::HashMap;
use parking_lot::RwLock;
use serde::Serialize;
use serde_json::Value;

use crate::error::Result;
use crate::traits::{ReadableSheet, WriteableSheet};

use super::sheet::Sheet;

#[derive(Clone, Debug, Default, Serialize)]
pub struct Book {
    pub(crate) sheets: Vec<Arc<RwLock<Sheet>>>,
}

impl Book {
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn add_sheet(&mut self, name: &str, sheet_state: &str) -> Arc<RwLock<Sheet>> {
        let sheet = Arc::new(RwLock::new(Sheet::new(name, sheet_state)));
        self.sheets.push(Arc::clone(&sheet));

        sheet
    }

    /// Копирует лист в книгу: ячейки и комментарии клонируются глубоко,
    /// чтобы копия не разделяла их с оригиналом.
    #[inline]
    pub fn copy_sheet(&mut self, sheet: Arc<RwLock<Sheet>>) -> Arc<RwLock<Sheet>> {
        let new_sheet = Arc::new(RwLock::new(sheet.read().deep_clone()));

        let mut guard = new_sheet.write();
        guard.set_name(&format!("Sheet {}", self.sheets.len() + 1));
        drop(guard);

        self.sheets.push(Arc::clone(&new_sheet));
        new_sheet
    }

    #[inline]
    pub fn get_sheet_name(&self, name: &str) -> Option<&Arc<RwLock<Sheet>>> {
        self.sheets
            .iter()
            .find(|sheet| sheet.read().get_name() == name)
    }

    #[inline]
    pub fn get_sheet_index(&self, idx: i32) -> Option<&Arc<RwLock<Sheet>>> {
        self.sheets.get(idx as usize)
    }

    #[inline]
    pub fn get_sheet_collection(&self) -> &[Arc<RwLock<Sheet>>] {
        &self.sheets
    }

    #[inline]
    pub fn to_json(&self) -> Result<String> {
        // From<serde_json::Error> для Error генерирует thiserror (#[from])
        Ok(serde_json::to_string(self)?)
    }

    #[inline]
    pub fn to_hashmap(&self) -> Result<HashMap<String, Value>> {
        let j = self.to_json()?;
        let h = serde_json::from_str(&j)?;

        Ok(h)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::structs::{comment::Comment, coordinate::Coordinate};
    use crate::traits::ReadableCell;

    fn test_book() -> Book {
        let mut book = Book::new();
        book.add_sheet("ЦП", "visible");

        book
    }

    #[test]
    fn test_new() {
        let book = Book::new();

        assert_eq!(book.sheets.len(), 0);
    }

    #[test]
    fn test_add_sheet() {
        let mut book = Book::new();
        book.add_sheet("ЦП", "visible");

        assert_eq!(book.sheets.len(), 1);
    }

    #[test]
    fn test_copy_sheet() {
        let mut book = test_book();
        let sheet = book.get_sheet_index(0).unwrap();

        let sheet2 = book.copy_sheet(sheet.clone());

        assert_eq!(book.sheets.len(), 2);
        assert_eq!(sheet2.read().get_name(), "Sheet 2");
    }

    #[test]
    fn test_copy_sheet_cells_independent() {
        let mut book = test_book();
        let sheet = book.get_sheet_index(0).unwrap().clone();

        sheet.write().cell(Coordinate::new(1, 1), Some("orig"));

        let sheet2 = book.copy_sheet(sheet.clone());
        sheet2.write().cell(Coordinate::new(1, 1), Some("changed"));

        assert_eq!(sheet.read().get_cell_value(Coordinate::new(1, 1)), "orig");
        assert_eq!(
            sheet2.read().get_cell_value(Coordinate::new(1, 1)),
            "changed"
        );
    }

    #[test]
    fn test_copy_sheet_delete_rows_independent() {
        let mut book = test_book();
        let sheet = book.get_sheet_index(0).unwrap().clone();

        sheet.write().cell(Coordinate::new(1, 1), Some("r1"));
        sheet.write().cell(Coordinate::new(2, 1), Some("r2"));

        let sheet2 = book.copy_sheet(sheet.clone());
        sheet2.write().delete_rows(1, 1).unwrap();

        // Оригинал не тронут: обе ячейки на месте, координаты не съехали
        assert_eq!(sheet.read().get_cell_value(Coordinate::new(1, 1)), "r1");
        assert_eq!(sheet.read().get_cell_value(Coordinate::new(2, 1)), "r2");
        let cell = sheet
            .read()
            .find_cell_by_coords(2, 1)
            .unwrap()
            .unwrap()
            .clone();
        assert_eq!(*cell.read().get_coordinate(), Coordinate::new(2, 1));

        // Копия сдвинулась: строка 2 стала строкой 1
        assert_eq!(sheet2.read().get_cell_value(Coordinate::new(1, 1)), "r2");
        assert_eq!(sheet2.read().get_max_row(), 1);
    }

    #[test]
    fn test_copy_sheet_delete_cols_independent() {
        let mut book = test_book();
        let sheet = book.get_sheet_index(0).unwrap().clone();

        sheet.write().cell(Coordinate::new(1, 1), Some("c1"));
        sheet.write().cell(Coordinate::new(1, 2), Some("c2"));

        let sheet2 = book.copy_sheet(sheet.clone());
        sheet2.write().delete_cols(1, 1).unwrap();

        // Оригинал не тронут: обе ячейки на месте, координаты не съехали
        assert_eq!(sheet.read().get_cell_value(Coordinate::new(1, 1)), "c1");
        assert_eq!(sheet.read().get_cell_value(Coordinate::new(1, 2)), "c2");
        let cell = sheet
            .read()
            .find_cell_by_coords(1, 2)
            .unwrap()
            .unwrap()
            .clone();
        assert_eq!(*cell.read().get_coordinate(), Coordinate::new(1, 2));

        // Копия сдвинулась: колонка 2 стала колонкой 1
        assert_eq!(sheet2.read().get_cell_value(Coordinate::new(1, 1)), "c2");
        assert_eq!(sheet2.read().get_max_column(), 1);
    }

    #[test]
    fn test_copy_sheet_comments_independent() {
        let mut book = test_book();
        let sheet = book.get_sheet_index(0).unwrap().clone();

        sheet
            .write()
            .add_comments(Comment::new(Coordinate::new(1, 1), "A.C"));
        sheet.read().get_comments()[0].write().set_text("orig");

        let sheet2 = book.copy_sheet(sheet.clone());
        sheet2.read().get_comments()[0].write().set_text("changed");

        assert_eq!(sheet.read().get_comments()[0].read().get_text(), "orig");
        assert_eq!(sheet2.read().get_comments()[0].read().get_text(), "changed");
    }

    #[test]
    fn test_get_sheet_name() {
        let book = test_book();

        assert_eq!(book.get_sheet_name("ЦП").unwrap().read().get_name(), "ЦП");
    }

    #[test]
    fn test_get_sheet_index() {
        let book = test_book();

        assert_eq!(book.get_sheet_index(0).unwrap().read().get_name(), "ЦП");
    }

    #[test]
    fn test_get_sheet_collection() {
        let book = test_book();

        assert_eq!(book.get_sheet_collection().len(), 1);
    }

    #[test]
    fn test_to_json() {
        let book = test_book();

        assert!(book.to_json().is_ok());
    }

    #[test]
    fn test_to_hashmap() {
        let book = test_book();

        assert!(book.to_hashmap().is_ok());
    }
}
