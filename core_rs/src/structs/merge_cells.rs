use super::range::Range;
use serde::Serialize;

#[derive(Clone, Default, Debug, Serialize)]
pub struct MergeCells {
    #[serde(
        rename = "merge_cells",
        serialize_with = "serialize_vec_range",
        skip_serializing_if = "<[_]>::is_empty"
    )]
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
}

// Для сериализации
fn serialize_vec_range<S>(ranges: &[Range], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let tuples: Vec<(u32, u32, u16, u16)> = ranges.iter().map(|r| r.into()).collect();
    tuples.serialize(serializer)
}
