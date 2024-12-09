use std::sync::{Arc, Weak};

use crate::{
    datatype::{CellRawValue, CellValue},
    utils::{get_letter_coordinate, get_number_format_by_datatype},
};
use anyhow::{bail, Result};
use chrono::NaiveDateTime;
use parking_lot::Mutex;
use serde::Serialize;

use super::{sheet::XLSXSheet, DATA_TYPES};

#[derive(Clone, Debug, Default, Serialize)]
pub struct XLSXSheetCell {
    pub row: u32,
    pub column: u16,
    pub cell: String,
    pub value: CellValue,
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
    pub _current_sheet: Weak<Mutex<XLSXSheet>>,
}

impl XLSXSheetCell {
    pub fn new(
        sheet: Arc<Mutex<XLSXSheet>>,
        row: u32,
        col: u16,
        value: Option<String>,
    ) -> Arc<Mutex<Self>> {
        // Получение  letter (cell)
        let cell = get_letter_coordinate(row, col);

        // Обработка значения
        let mut raw_value = CellRawValue::Empty;
        if let Some(val) = value {
            raw_value = CellValue::quess_typed_value(&val)
        }

        let value = CellValue {
            raw_value: raw_value.clone(),
        };
        // Определение datetype
        let data_type = raw_value.get_date_type().to_string();
        // Определение number format
        let number_format = get_number_format_by_datatype(&data_type);

        Arc::new(Mutex::new(Self {
            row,
            column: col,
            cell,
            value,
            data_type,
            number_format,
            _current_sheet: Arc::downgrade(&sheet),
            ..Default::default()
        }))
    }

    pub fn set_value(&mut self, value: String) -> Result<()> {
        let cell_value = self.value.set_value(value);

        if self.formula.is_none() {
            let data_type = cell_value.get_data_type().to_string();

            self.number_format = get_number_format_by_datatype(&data_type);
            self.data_type = data_type;
        }

        Ok(())
    }

    pub fn set_value_number(&mut self, value: f64) -> Result<()> {
        self.value.set_value_number(value);

        Ok(())
    }

    pub fn set_value_integer(&mut self, value: i32) -> Result<()> {
        self.value.set_value_integer(value);

        Ok(())
    }

    pub fn set_value_bool(&mut self, value: bool) -> Result<()> {
        self.value.set_value_bool(value);

        Ok(())
    }

    pub fn set_value_str(&mut self, value: String) -> Result<()> {
        self.value.set_value_str(value);

        Ok(())
    }

    pub fn set_value_datetime(&mut self, value: NaiveDateTime) -> Result<()> {
        self.value.set_value_datatime(value);

        Ok(())
    }

    pub fn set_formula(&mut self, value: String) -> Result<()> {
        self.formula = Some(value);
        // если идет установка формулы, то и тип установим как формула
        self.data_type = "f".to_string();

        Ok(())
    }

    pub fn set_data_type(&mut self, value: String) -> Result<()> {
        if !DATA_TYPES.contains(&value.as_str()) {
            bail!(format!("value not in {:?}", DATA_TYPES))
        }

        self.data_type = value;

        Ok(())
    }

    pub fn set_number_format(&mut self, value: String) -> Result<()> {
        self.number_format = value;

        Ok(())
    }

    pub fn set_style_id(&mut self, value: String) -> Result<()> {
        self.style_id = Some(value);

        Ok(())
    }

    pub fn set_hidden_value(&mut self, value: String) -> Result<()> {
        self.hidden_value = Some(value);

        Ok(())
    }

    pub fn set_comment(&mut self, value: String) -> Result<()> {
        self.comment = Some(value);

        Ok(())
    }

    pub fn is_formula(&self) -> bool {
        self.formula.is_some() && self.data_type == "f"
    }

    pub fn is_value_bool(&self) -> bool {
        self.value.is_bool()
    }

    pub fn is_value_numeric(&self) -> bool {
        self.value.is_numeric()
    }

    pub fn is_value_datetime(&self) -> bool {
        self.value.is_datetime()
    }

    pub fn is_value_empty(&self) -> bool {
        self.value.is_empty()
    }
}
