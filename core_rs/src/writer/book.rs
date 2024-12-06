use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use anyhow::{bail, Result};
use serde::{ser::SerializeStruct, Serialize};
use serde_json::Value;

use super::{sheet::XLSXSheet, DEFAULT_COL, DEFAULT_ROW};

#[derive(Clone, Debug, Default)]
pub struct XLSXBook {
    // todo
    pub sheets: Vec<Arc<Mutex<XLSXSheet>>>,
    self_ref: Option<Arc<Mutex<Self>>>,
}

impl Serialize for XLSXBook {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("XLSXBook", 1)?;

        let sheets: Vec<_> = self
            .sheets
            .iter()
            .map(|s| s.lock().unwrap().clone())
            .collect();

        state.serialize_field("sheets", &sheets)?;
        state.end()
    }
}

impl XLSXBook {
    pub fn new() -> Arc<Mutex<Self>> {
        let book = Arc::new(Mutex::new(Self {
            sheets: vec![],
            self_ref: None,
        }));

        let mut guard = book.lock().unwrap();
        guard.self_ref = Some(Arc::clone(&book));
        drop(guard);

        book
    }

    pub fn add_sheet(
        &mut self,
        name: String,
        rows: Option<u32>,
        cols: Option<u16>,
    ) -> Arc<Mutex<XLSXSheet>> {
        let book = self.self_ref.as_ref().unwrap().clone();

        let rows = rows.unwrap_or(DEFAULT_ROW);
        let cols = cols.unwrap_or(DEFAULT_COL);

        let idx = self.sheets.len() as i32;
        let sheet = XLSXSheet::new(book, name, idx, rows, cols);

        self.sheets.push(Arc::clone(&sheet));

        sheet
    }

    pub fn get_sheet_index(&self, idx: i32) -> Option<Arc<Mutex<XLSXSheet>>> {
        self.sheets
            .iter()
            .find(|sheet| sheet.lock().unwrap().index == idx)
            .map(Arc::clone)
    }

    pub fn get_sheet_name(&self, name: String) -> Option<Arc<Mutex<XLSXSheet>>> {
        self.sheets
            .iter()
            .find(|sheet| sheet.lock().unwrap().name == name)
            .map(Arc::clone)
    }

    pub fn to_json(&self) -> Result<String> {
        if let Ok(j) = serde_json::to_string(self) {
            Ok(j)
        } else {
            bail!("Failed to convert in JSON");
        }
    }

    pub fn to_hashmap(&self) -> Result<HashMap<String, Value>> {
        let j = self.to_json()?;
        let h = serde_json::from_str(&j)?;

        Ok(h)
    }
}
