use serde::Serialize;

use crate::DEFAULT_ROW_HEIGHT;

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize, Default)]
pub struct Row {
    row_num: u32,
    height: f64,
    hidden: bool,
    collapsed: bool,
    level: u32,
}

impl Row {
    pub fn new(row_num: u32) -> Self {
        Self {
            row_num,
            height: DEFAULT_ROW_HEIGHT,
            ..Default::default()
        }
    }

    pub fn get_height(&self) -> &f64 {
        &self.height
    }

    pub fn get_hidden(&self) -> &bool {
        &self.hidden
    }

    pub fn set_height(&mut self, val: f64) -> &mut Self {
        self.height = val;

        self
    }

    pub fn set_hidden(&mut self, val: bool) -> &mut Self {
        self.hidden = val;

        self
    }
}
