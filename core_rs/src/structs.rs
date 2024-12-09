use std::{
    collections::HashMap,
    sync::{Arc, Weak},
};

use parking_lot::RwLock;
use serde::Serialize;

use crate::datatype::CellValue;

#[derive(Clone, Debug, Default, Serialize)]
pub struct XLSXBook {
    // todo
    pub sheets: Vec<Arc<RwLock<XLSXSheet>>>,
    self_ref: Option<Arc<RwLock<Self>>>,
}

impl XLSXBook {
    pub fn new() -> Arc<RwLock<Self>> {
        let book = Arc::new(RwLock::new(Self {
            sheets: vec![],
            self_ref: None,
        }));

        let mut guard = book.write();
        guard.self_ref = Some(Arc::clone(&book));
        drop(guard);

        book
    }
}

#[derive(Clone, Debug, Default)]
pub struct XLSXSheet {
    pub name: String,
    pub max_row: u32,
    pub max_column: u16,
    pub index: i32,
    // todo
    pub _cells: HashMap<(u32, u16), Arc<RwLock<XLSXSheetCell>>>,

    _current_workbook: Weak<RwLock<XLSXBook>>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct XLSXSheetCell {
    pub row: u32,
    pub column: u16,
    pub cell: String,
    pub value: Box<CellValue>,
    pub formula: Option<String>,
    pub data_type: String,
    pub number_format: String,
    pub is_merge: bool,
    pub start_row: Option<u32>,
    pub end_row: Option<u32>,
    pub start_column: Option<u16>,
    pub end_column: Option<u16>,
    pub style_id: Option<String>,
    pub hidden_value: Option<String>,
    pub comment: Option<String>,

    // Слабая ссылка на текущую страницу
    #[serde(skip)]
    pub _current_sheet: Weak<RwLock<XLSXSheet>>,
}
