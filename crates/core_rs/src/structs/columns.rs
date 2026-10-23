use ahash::HashMap;
use serde::Serialize;

use super::column::Column;
use crate::DEFAULT_COLUMN_WIDTH;

#[derive(Clone, Debug, Serialize)]
pub(crate) struct Columns {
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    columns: HashMap<u16, Box<Column>>,
    default_width: f64,
}

impl Default for Columns {
    fn default() -> Self {
        Self {
            columns: HashMap::default(),
            default_width: DEFAULT_COLUMN_WIDTH,
        }
    }
}

impl Columns {
    #[inline]
    pub(crate) fn set_width(&mut self, col_num: u16, val: f64) {
        let column = self
            .columns
            .entry(col_num)
            .or_insert(Box::new(Column::new(col_num)));

        column.set_width(val);
    }

    #[inline]
    pub(crate) fn set_hidden(&mut self, col_num: u16, val: bool) {
        let row = self
            .columns
            .entry(col_num)
            .or_insert(Box::new(Column::new(col_num)));

        row.set_hidden(val);
    }

    #[inline]
    pub(crate) fn get_width(&self, col_num: u16) -> &f64 {
        self.columns
            .get(&col_num)
            .map(|c| c.get_width())
            .unwrap_or_else(|| &self.default_width)
    }

    #[inline]
    pub(crate) fn get_hidden(&self, col_num: u16) -> &bool {
        self.columns
            .get(&col_num)
            .map(|c| c.get_hidden())
            .unwrap_or_else(|| &false)
    }

    /// Сдвигает ключи размеров колонок при удалении колонок `[idx; idx + amount)`:
    /// ключи правее удаления сдвигаются влево, ключи из диапазона удаления
    /// исчезают.
    ///
    /// Ожидается валидный диапазон удаления: `idx ≥ 1`, `amount ≥ 1`.
    #[inline]
    pub(crate) fn delete_cols(&mut self, idx: u16, amount: u16) {
        // Арифметика в широком типе: после validate_delete_range значение
        // заведомо ≤ максимуму листа — переполнения и усечения cast'а нет
        let delete_end = (idx as u64 + amount as u64 - 1) as u16;

        self.columns = self
            .columns
            .drain()
            .filter_map(|(col_num, mut column)| {
                if col_num < idx {
                    // Левее удаления — без изменений
                    Some((col_num, column))
                } else if col_num > delete_end {
                    // Правее удаления — сдвиг влево
                    column.set_col_num(col_num - amount);
                    Some((col_num - amount, column))
                } else {
                    // Колонка удалена — размер исчезает вместе с ней
                    None
                }
            })
            .collect();
    }
}

#[cfg(test)]
mod tests {
    use crate::DEFAULT_COLUMN_WIDTH;

    use super::*;

    #[test]
    fn new_rows() {
        let rows = Columns::default();

        assert!(rows.columns.is_empty());
    }

    #[test]
    fn new_set_height() {
        let mut rows = Columns::default();

        rows.set_width(1, 100.0);

        assert_eq!(*rows.get_width(1), 100.0);
    }

    #[test]
    fn new_set_hidden() {
        let mut rows = Columns::default();

        rows.set_hidden(1, true);

        assert!(*rows.get_hidden(1));
        assert_eq!(*rows.get_width(1), DEFAULT_COLUMN_WIDTH);
    }
}
