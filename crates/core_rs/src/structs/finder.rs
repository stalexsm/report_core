use std::sync::Arc;

use parking_lot::RwLock;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    error::Result,
    regex::{compile_case_insensitive, is_match},
    traits::ReadableSheet,
};

#[derive(Debug, Clone, Default)]
pub struct Finder<T: ReadableSheet + Send + Sync> {
    pub sheets: Vec<Arc<RwLock<T>>>,
}

impl<T: ReadableSheet + Send + Sync> Finder<T> {
    /// Finder хранит переданные листы как есть — те же Arc, без копий:
    /// создание стоит O(числа листов), данные не дублируются, мутации
    /// исходных листов видны поисковику.
    pub fn new(sheets: Vec<Arc<RwLock<T>>>) -> Self {
        Self { sheets }
    }

    #[inline]
    pub fn get_sheet_collection(&self) -> &[Arc<RwLock<T>>] {
        &self.sheets
    }

    #[inline]
    pub fn find_sheet_by_name(&self, name: &str) -> Option<&Arc<RwLock<T>>> {
        self.sheets
            .par_iter()
            .find_first(|s| s.read().get_name() == name)
    }

    /// Поиск листа по регулярному выражению.
    ///
    /// Шаблон интерпретируется без учёта регистра — как в клеточных
    /// `find_*_by_regex` (у них шаблон неявно обёрнут в `(?i)`).
    ///
    /// # Errors
    ///
    /// Возвращает `Err`, если `pattern` — невалидное регулярное выражение.
    #[inline]
    pub fn find_sheet_by_regex(&self, pattern: &str) -> Result<Option<&Arc<RwLock<T>>>> {
        let re = compile_case_insensitive(pattern)?;

        // Порядок листов сохраняется, ошибка выполнения шаблона (например,
        // исчерпанный лимит обратной трассировки) не глотается, а уходит
        // вызывающему коду
        let sheet = self
            .sheets
            .par_iter()
            .map(|sheet| {
                let name = sheet.read().get_name();
                is_match(&re, &name).map(|hit| hit.then_some(sheet))
            })
            .collect::<Result<Vec<Option<_>>>>()?
            .into_iter()
            .flatten()
            .next();

        Ok(sheet)
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
    use crate::traits::WriteableSheet;

    use super::*;

    /// Лист для Finder, завёрнутый в разделяемый Arc.
    fn shared_sheet(name: &str) -> Arc<RwLock<Sheet>> {
        Arc::new(RwLock::new(Sheet::new(name, "visible")))
    }

    #[test]
    fn new_finder() {
        let f = Finder::new(vec![shared_sheet("A")]);

        assert_eq!(f.sheets.len(), 1)
    }

    /// Finder хранит те же Arc, что ему переданы: мутация исходного
    /// листа видна поисковику, копий нет.
    #[test]
    fn new_finder_shares_sheets() {
        let sheet = shared_sheet("A");
        let f = Finder::new(vec![Arc::clone(&sheet)]);

        sheet.write().set_name("B");

        assert_eq!(f.sheets[0].read().get_name(), "B");
    }

    #[test]
    fn find_sheet_by_name() {
        let f = Finder::new(vec![shared_sheet("A")]);

        let f = f.find_sheet_by_name("A").unwrap();
        let f = f.read();

        assert_eq!(f.get_name(), "A")
    }

    #[test]
    fn find_sheet_by_regex() {
        let f = Finder::new(vec![shared_sheet("A")]);

        let f = f.find_sheet_by_regex("A").unwrap().unwrap();
        let f = f.read();

        assert_eq!(f.get_name(), "A")
    }

    /// Невалидный шаблон — Err, а не паника.
    #[test]
    fn find_sheet_by_regex_invalid_pattern() {
        let f = Finder::new(vec![shared_sheet("A")]);

        assert!(f.find_sheet_by_regex("(").is_err());
    }

    /// Валидный шаблон без совпадений — Ok(None).
    #[test]
    fn find_sheet_by_regex_no_match() {
        let f = Finder::new(vec![shared_sheet("A")]);

        let f = f.find_sheet_by_regex("B").unwrap();

        assert!(f.is_none());
    }

    /// Поиск без учёта регистра — семантика выровнена с клеточными
    /// find_*_by_regex (у них шаблон неявно обёрнут в (?i)).
    #[test]
    fn find_sheet_by_regex_case_insensitive() {
        let f = Finder::new(vec![shared_sheet("A")]);

        let f = f.find_sheet_by_regex("a").unwrap().unwrap();
        let f = f.read();

        assert_eq!(f.get_name(), "A")
    }

    #[test]
    fn get_sheet_index() {
        let f = Finder::new(vec![shared_sheet("A")]);

        let f = f.get_sheet_index(0).unwrap();
        let f = f.read();

        assert_eq!(f.get_name(), "A")
    }

    #[test]
    fn get_sheets_without_names() {
        let f = Finder::new(vec![shared_sheet("A")]);

        let f = f.get_sheets_without_names(vec!["A"]);

        assert!(f.is_empty())
    }

    #[test]
    fn get_sheets_with_names() {
        let f = Finder::new(vec![shared_sheet("A")]);

        let f = f.get_sheets_with_names(vec!["A"]);

        assert!(!f.is_empty())
    }
}
