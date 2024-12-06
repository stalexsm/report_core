pub mod book;
pub mod cell;
pub mod sheet;

const DATA_TYPES: [&str; 5] = ["s", "n", "d", "b", "f"];

const MAX_COL: u16 = 16_384;
const MAX_ROW: u32 = 1_048_576;

pub const DEFAULT_COL: u16 = 5;
pub const DEFAULT_ROW: u32 = 5;
