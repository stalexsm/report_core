pub mod datatype;
pub mod funcs;
pub mod structs;
pub mod traits;
pub mod utils;

/// Функция для получения версии.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

const MAX_COL: u16 = 16_384;
const MAX_ROW: u32 = 1_048_576;

// Стандартные размеры как в Excel
pub const DEFAULT_COLUMN_WIDTH: f64 = 22.5; // ~ 140 пикселей
pub const DEFAULT_ROW_HEIGHT: f64 = 20.0; // ~ 20 пикселя
