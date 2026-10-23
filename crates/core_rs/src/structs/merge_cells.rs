use super::range::Range;
use serde::Serialize;

#[derive(Clone, Default, Debug, Serialize)]
pub struct MergeCells {
    #[serde(rename = "merge_cells", serialize_with = "serialize_vec_range")]
    range: Vec<Range>,
}

impl MergeCells {
    /// Интициализирует коллекцию диапазонов слияний
    pub fn new(range: Vec<Range>) -> Self {
        MergeCells { range }
    }

    /// Метод для получения коллекции диапазонов слияний
    #[inline]
    pub fn get_collection(&self) -> &[Range] {
        &self.range
    }

    /// Метод для добавления диапазона слияния
    #[inline]
    pub fn add_range(&mut self, range: Range) {
        self.range.push(range);
    }

    /// Сдвигает/усекает диапазоны слияний при удалении строк `[idx; idx + amount)`.
    ///
    /// Диапазоны, целиком попавшие в удаление и усечённые до одной ячейки,
    /// удаляются; одноячеечные диапазоны, созданные до удаления, сохраняются.
    #[inline]
    pub(crate) fn delete_rows(&mut self, idx: u32, amount: u32) {
        self.range = self
            .range
            .iter()
            .filter_map(|range| {
                let shifted = range.delete_rows(idx, amount)?;

                (shifted.is_single_cell() == range.is_single_cell()).then_some(shifted)
            })
            .collect();
    }

    /// Сдвигает/усекает диапазоны слияний при удалении колонок
    /// `[idx; idx + amount)`. Симметрично `delete_rows`.
    #[inline]
    pub(crate) fn delete_cols(&mut self, idx: u16, amount: u16) {
        self.range = self
            .range
            .iter()
            .filter_map(|range| {
                let shifted = range.delete_cols(idx, amount)?;

                (shifted.is_single_cell() == range.is_single_cell()).then_some(shifted)
            })
            .collect();
    }
}

// Для сериализации
fn serialize_vec_range<S>(ranges: &[Range], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let tuples: Vec<(u32, u32, u16, u16)> = ranges.iter().map(|r| r.into()).collect();
    tuples.serialize(serializer)
}
