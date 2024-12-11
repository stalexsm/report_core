use serde::Serialize;

#[derive(Clone, Default, Debug, Serialize)]
pub struct Range {
    pub start_row: Option<u32>,
    pub end_row: Option<u32>,
    pub start_col: Option<u16>,
    pub end_col: Option<u16>,
}

impl Range {
    /// Интициализирует диапазон с заданными координатами
    pub fn new(start_row: u32, end_row: u32, start_col: u16, end_col: u16) -> Self {
        Range {
            start_row: Some(start_row),
            end_row: Some(end_row),
            start_col: Some(start_col),
            end_col: Some(end_col),
        }
    }
}
