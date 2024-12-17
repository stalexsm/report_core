use fancy_regex::Regex;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::traits::ReadableSheet;

#[derive(Debug, Clone)]
pub struct Finder<T: ReadableSheet + Send + Sync> {
    pub sheets: Vec<T>,
}

impl<T: ReadableSheet + Send + Sync> Finder<T> {
    pub fn new(sheets: Vec<T>) -> Self {
        Self { sheets }
    }

    #[inline]
    pub fn find_sheet_by_name(&self, name: &str) -> Option<&T> {
        self.sheets.par_iter().find_first(|s| s.get_name() == name)
    }

    #[inline]
    pub fn find_sheet_by_regex(&self, pattern: &str) -> Option<&T> {
        let re = Regex::new(pattern).unwrap();

        self.sheets
            .par_iter()
            .find_first(|s| re.is_match(&s.get_name()).unwrap_or(false))
    }

    #[inline]
    pub fn get_sheet_index(&self, idx: i32) -> Option<&T> {
        self.sheets.get(idx as usize)
    }

    #[inline]
    pub fn get_sheets_without_names(&self, name_list: Vec<&str>) -> Vec<&T> {
        self.sheets
            .par_iter()
            .filter(|s| !name_list.contains(&s.get_name().as_str()))
            .collect()
    }

    #[inline]
    pub fn get_sheets_with_names(&self, name_list: Vec<&str>) -> Vec<&T> {
        self.sheets
            .par_iter()
            .filter(|s| name_list.contains(&s.get_name().as_str()))
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

        assert_eq!(f.get_name(), "A")
    }

    #[test]
    fn find_sheet_by_regex() {
        let f = Finder::new(vec![Sheet::new("A")]);

        let f = f.find_sheet_by_regex("A").unwrap();

        assert_eq!(f.get_name(), "A")
    }

    #[test]
    fn get_sheet_index() {
        let f = Finder::new(vec![Sheet::new("A")]);

        let f = f.get_sheet_index(0).unwrap();

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
