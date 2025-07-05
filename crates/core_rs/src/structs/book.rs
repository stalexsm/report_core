use std::{collections::HashMap, sync::Arc};

use anyhow::{Result, bail};
use parking_lot::RwLock;
use serde::Serialize;
use serde_json::Value;

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

    #[inline]
    pub fn copy_sheet(&mut self, sheet: Arc<RwLock<Sheet>>) -> Arc<RwLock<Sheet>> {
        let new_sheet = Arc::new(RwLock::new((*sheet.read()).clone()));

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
        match serde_json::to_string(self) {
            Ok(j) => Ok(j),
            Err(e) => {
                bail!("Failed to convert in JSON: {}", e);
            }
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
