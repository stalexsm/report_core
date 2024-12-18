use std::sync::Arc;

use fancy_regex::Regex;
use parking_lot::RwLock;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::traits::ReadableSheet;

#[derive(Debug, Clone)]
pub struct Finder<T: ReadableSheet + Send + Sync> {
    pub sheets: Vec<Arc<RwLock<T>>>,
}

impl<T: ReadableSheet + Send + Sync> Finder<T> {
    pub fn new(sheets: Vec<T>) -> Self {
        let sheets = sheets
            .into_iter()
            .map(|s| Arc::new(RwLock::new(s)))
            .collect();

        Self { sheets }
    }

    #[inline]
    pub fn find_sheet_by_name(&self, name: &str) -> Option<&Arc<RwLock<T>>> {
        self.sheets
            .par_iter()
            .find_first(|s| s.read().get_name() == name)
    }

    #[inline]
    pub fn find_sheet_by_regex(&self, pattern: &str) -> Option<&Arc<RwLock<T>>> {
        let re = Regex::new(pattern).unwrap();

        self.sheets
            .par_iter()
            .find_first(|s| re.is_match(&s.read().get_name()).unwrap_or(false))
    }

    #[inline]
    pub fn get_sheet_index(&self, idx: i32) -> Option<&Arc<RwLock<T>>> {
        self.sheets.get(idx as usize)
    }

    #[inline]
    pub fn get_sheets_without_names(&self, name_list: Vec<&str>) -> Vec<&Arc<RwLock<T>>> {
        self.sheets
            .par_iter()
            .filter(|s| !name_list.contains(&s.read().get_name().as_str()))
            .collect()
    }

    #[inline]
    pub fn get_sheets_with_names(&self, name_list: Vec<&str>) -> Vec<&Arc<RwLock<T>>> {
        self.sheets
            .par_iter()
            .filter(|s| name_list.contains(&s.read().get_name().as_str()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::structs::sheet::Sheet;

    use super::*;

    #[test]
    fn new_finder() {
        let f = Finder::new(vec![Sheet::new("A")]);

        assert_eq!(f.sheets.len(), 1)
    }

    #[test]
    fn find_sheet_by_name() {
        let f = Finder::new(vec![Sheet::new("A")]);

        let f = f.find_sheet_by_name("A").unwrap();
        let f = f.read();

        assert_eq!(f.get_name(), "A")
    }

    #[test]
    fn find_sheet_by_regex() {
        let f = Finder::new(vec![Sheet::new("A")]);

        let f = f.find_sheet_by_regex("A").unwrap();
        let f = f.read();

        assert_eq!(f.get_name(), "A")
    }

    #[test]
    fn get_sheet_index() {
        let f = Finder::new(vec![Sheet::new("A")]);

        let f = f.get_sheet_index(0).unwrap();
        let f = f.read();

        assert_eq!(f.get_name(), "A")
    }

    #[test]
    fn get_sheets_without_names() {
        let f = Finder::new(vec![Sheet::new("A")]);

        let f = f.get_sheets_without_names(vec!["A"]);

        assert!(f.is_empty())
    }

    #[test]
    fn get_sheets_with_names() {
        let f = Finder::new(vec![Sheet::new("A")]);

        let f = f.get_sheets_with_names(vec!["A"]);

        assert!(!f.is_empty())
    }
}
