use fancy_regex::Regex;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use super::sheet::Sheet;

#[derive(Debug, Clone)]
pub struct Finder {
    pub sheets: Vec<Sheet>,
}

impl Finder {
    pub fn new(sheets: Vec<Sheet>) -> Self {
        Self { sheets }
    }

    #[inline]
    pub fn find_sheet_by_name(&self, name: &str) -> Option<&Sheet> {
        self.sheets.par_iter().find_first(|s| s.name == name)
    }

    #[inline]
    pub fn find_sheet_by_regex(&self, pattern: &str) -> Option<&Sheet> {
        let re = Regex::new(pattern).unwrap();

        self.sheets
            .par_iter()
            .find_first(|s| re.is_match(&s.name).unwrap_or(false))
    }

    #[inline]
    pub fn get_sheet_index(&self, idx: i32) -> Option<&Sheet> {
        self.sheets.get(idx as usize)
    }

    #[inline]
    pub fn get_sheets_without_names(&self, name_list: Vec<String>) -> Vec<&Sheet> {
        self.sheets
            .par_iter()
            .filter(|s| !name_list.contains(&s.name))
            .collect()
    }

    #[inline]
    pub fn get_sheets_with_names(&self, name_list: Vec<String>) -> Vec<&Sheet> {
        self.sheets
            .par_iter()
            .filter(|s| name_list.contains(&s.name))
            .collect()
    }
}
