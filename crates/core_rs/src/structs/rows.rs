use ahash::HashMap;
use serde::Serialize;

use crate::DEFAULT_ROW_HEIGHT;

use super::row::Row;

#[derive(Clone, Debug, Serialize)]
pub(crate) struct Rows {
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    rows: HashMap<u32, Box<Row>>,
    default_height: f64,
}

impl Default for Rows {
    fn default() -> Self {
        Self {
            rows: HashMap::default(),
            default_height: DEFAULT_ROW_HEIGHT,
        }
    }
}

impl Rows {
    #[inline]
    pub(crate) fn set_height(&mut self, row_num: u32, val: f64) {
        let row = self
            .rows
            .entry(row_num)
            .or_insert(Box::new(Row::new(row_num)));

        row.set_height(val);
    }

    #[inline]
    pub(crate) fn set_hidden(&mut self, row_num: u32, val: bool) {
        let row = self
            .rows
            .entry(row_num)
            .or_insert(Box::new(Row::new(row_num)));

        row.set_hidden(val);
    }

    #[inline]
    pub(crate) fn get_heignt(&self, row_num: u32) -> &f64 {
        self.rows
            .get(&row_num)
            .map(|r| r.get_height())
            .unwrap_or_else(|| &self.default_height)
    }

    #[inline]
    pub(crate) fn get_hidden(&self, row_num: u32) -> &bool {
        self.rows
            .get(&row_num)
            .map(|r| r.get_hidden())
            .unwrap_or_else(|| &false)
    }
}

#[cfg(test)]
mod tests {
    use crate::DEFAULT_ROW_HEIGHT;

    use super::*;

    #[test]
    fn new_rows() {
        let rows = Rows::default();

        assert!(rows.rows.is_empty());
    }

    #[test]
    fn new_set_height() {
        let mut rows = Rows::default();

        rows.set_height(1, 100.0);

        assert_eq!(*rows.get_heignt(1), 100.0);
    }

    #[test]
    fn new_set_hidden() {
        let mut rows = Rows::default();

        rows.set_hidden(1, true);

        assert!(*rows.get_hidden(1));
        assert_eq!(*rows.get_heignt(1), DEFAULT_ROW_HEIGHT);
    }
}
