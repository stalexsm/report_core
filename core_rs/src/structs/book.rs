use std::collections::HashMap;

use anyhow::{bail, Result};
use serde::Serialize;
use serde_json::Value;

use super::sheet::Sheet;

#[derive(Clone, Debug, Default, Serialize)]
pub struct Book {
    sheets: Vec<Sheet>,
}

impl Book {
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn add_sheet(&mut self, sheet: Sheet) -> &mut Self {
        self.sheets.push(sheet);

        self
    }

    #[inline]
    pub fn get_sheet_name(&self, name: &str) -> Option<&Sheet> {
        self.sheets.iter().find(|sheet| sheet.name == name)
    }

    #[inline]
    pub fn get_sheet_name_mut(&mut self, name: &str) -> Option<&mut Sheet> {
        self.sheets.iter_mut().find(|sheet| sheet.name == name)
    }

    #[inline]
    pub fn get_sheet_index(&self, idx: i32) -> Option<&Sheet> {
        self.sheets.get(idx as usize)
    }

    #[inline]
    pub fn get_sheet_index_mut(&mut self, idx: i32) -> Option<&mut Sheet> {
        self.sheets.get_mut(idx as usize)
    }

    #[inline]
    pub fn get_sheet_collection(&self) -> &[Sheet] {
        &self.sheets
    }

    #[inline]
    pub fn get_sheet_collection_mut(&mut self) -> &mut [Sheet] {
        &mut self.sheets
    }

    #[inline]
    pub fn to_json(&self) -> Result<String> {
        if let Ok(j) = serde_json::to_string(self) {
            Ok(j)
        } else {
            bail!("Failed to convert in JSON");
        }
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

    fn test_book() -> Book {
        let mut book = Book::new();
        book.add_sheet(Sheet::new("ЦП"));

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
        book.add_sheet(Sheet::new("ЦП"));

        assert_eq!(book.sheets.len(), 1);
    }

    #[test]
    fn test_get_sheet_name() {
        let book = test_book();

        assert_eq!(book.get_sheet_name("ЦП").unwrap().name, "ЦП");
    }

    #[test]
    fn test_get_sheet_name_mut() {
        let mut book = test_book();

        assert_eq!(book.get_sheet_name_mut("ЦП").unwrap().name, "ЦП");
    }

    #[test]
    fn test_get_sheet_index() {
        let book = test_book();

        assert_eq!(book.get_sheet_index(0).unwrap().name, "ЦП");
    }

    #[test]
    fn test_get_sheet_index_mut() {
        let mut book = test_book();

        assert_eq!(book.get_sheet_index_mut(0).unwrap().name, "ЦП");
    }

    #[test]
    fn test_get_sheet_collection() {
        let book = test_book();

        assert_eq!(book.get_sheet_collection().len(), 1);
    }

    #[test]
    fn test_get_sheet_collection_mut() {
        let mut book = test_book();

        assert_eq!(book.get_sheet_collection_mut().len(), 1);
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
