use serde::Serialize;

use crate::DEFAULT_COLUMN_WIDTH;

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Default)]
pub struct Column {
    col_num: u16,
    width: f64,
    hidden: bool,
    collapsed: bool,
    level: u32,
}

impl Column {
    pub fn new(col_num: u16) -> Self {
        Self {
            col_num,
            width: DEFAULT_COLUMN_WIDTH,
            ..Default::default()
        }
    }

    pub fn get_width(&self) -> &f64 {
        &self.width
    }

    pub fn get_hidden(&self) -> &bool {
        &self.hidden
    }

    pub fn set_width(&mut self, val: f64) -> &mut Self {
        self.width = val;

        self
    }

    pub fn set_hidden(&mut self, val: bool) -> &mut Self {
        self.hidden = val;

        self
    }
}
