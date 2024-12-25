use anyhow::Result;
use parking_lot::RwLock;
use serde::Serialize;
use serde_json::Value;
use std::{collections::HashMap, sync::Arc};

use super::{book::Book, sheet::Sheet};

#[derive(Clone, Debug, Default, Serialize)]
pub struct Service {
    book: Arc<RwLock<Book>>,
}

impl Service {
    pub fn new() -> Self {
        let book = Arc::new(RwLock::new(Book::new()));

        Self { book }
    }

    #[inline]
    pub fn add_sheet(&mut self, name: &str) -> Arc<RwLock<Sheet>> {
        let sheet = self.book.write().add_sheet(name);

        sheet
    }

    #[inline]
    pub fn copy_sheet(&mut self, sheet: Arc<RwLock<Sheet>>) -> Arc<RwLock<Sheet>> {
        let sheet = self.book.write().copy_sheet(sheet);

        sheet
    }

    #[inline]
    pub fn get_sheet_name(&self, name: &str) -> Option<Arc<RwLock<Sheet>>> {
        self.book.read().get_sheet_name(name).cloned()
    }

    #[inline]
    pub fn get_sheet_index(&self, idx: i32) -> Option<Arc<RwLock<Sheet>>> {
        self.book.read().get_sheet_index(idx).cloned()
    }

    #[inline]
    pub fn get_sheet_collection(&self) -> Vec<Arc<RwLock<Sheet>>> {
        self.book.read().get_sheet_collection().to_vec()
    }

    #[inline]
    pub fn to_json(&self) -> Result<String> {
        self.book.read().to_json()
    }

    #[inline]
    pub fn to_hashmap(&self) -> Result<HashMap<String, Value>> {
        self.book.read().to_hashmap()
    }
}
