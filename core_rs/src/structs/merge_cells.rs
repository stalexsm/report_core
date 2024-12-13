use serde::Serialize;

use super::range::Range;

#[derive(Clone, Default, Debug, Serialize)]
pub struct MergeCells {
    #[serde(rename = "merge_cells")]
    range: Vec<Range>,
}

impl MergeCells {
    /// Интициализирует коллекцию диапазонов слияний
    pub fn new(range: Vec<Range>) -> Self {
        MergeCells { range }
    }

    /// Метод для получения коллекции диапазонов слияний
    pub fn get_collection(&self) -> &[Range] {
        &self.range
    }

    /// Метод для добавления диапазона слияния
    pub fn add_range(&mut self, range: Range) {
        self.range.push(range);
    }
}
