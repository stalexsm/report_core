pub mod datatype;
pub mod error;
pub mod funcs;
pub(crate) mod regex;
pub mod structs;
pub mod traits;
pub mod utils;

pub use error::{Error, Result};

/// Функция для получения версии.
pub const fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

const MAX_COL: u16 = 16_384;
const MAX_ROW: u32 = 1_048_576;

// Стандартные размеры отчёта — больше дефолтных Excel для читаемости.
// Пиксельные оценки: ширина — в символах (px ≈ ширина×7+5, Calibri 11),
// высота — в пунктах (1 pt ≈ 4/3 px при 96 dpi)
pub const DEFAULT_COLUMN_WIDTH: f64 = 22.5; // ≈ 163 px
pub const DEFAULT_ROW_HEIGHT: f64 = 20.0; // ≈ 27 px
