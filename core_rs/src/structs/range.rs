use serde::Serialize;

pub type MergedRange = (u32, u32, u16, u16);

#[derive(Clone, Default, Debug, Serialize, PartialEq, Eq)]
pub struct Range {
    pub start_row: u32,
    pub end_row: u32,
    pub start_col: u16,
    pub end_col: u16,
}

impl Range {
    /// Интициализирует диапазон с заданными координатами
    pub fn new(start_row: u32, end_row: u32, start_col: u16, end_col: u16) -> Self {
        Range {
            start_row,
            end_row,
            start_col,
            end_col,
        }
    }
}

impl From<(u32, u32, u16, u16)> for Range {
    #[inline]
    fn from(value: (u32, u32, u16, u16)) -> Self {
        Range::new(value.0, value.1, value.2, value.3)
    }
}

impl From<Range> for (u32, u32, u16, u16) {
    #[inline]
    fn from(range: Range) -> Self {
        (
            range.start_row,
            range.end_row,
            range.start_col,
            range.end_col,
        )
    }
}

impl From<&Range> for (u32, u32, u16, u16) {
    #[inline]
    fn from(range: &Range) -> Self {
        (
            range.start_row,
            range.end_row,
            range.start_col,
            range.end_col,
        )
    }
}
