pub mod datatype;
pub mod structs;
pub mod traits;
pub mod utils;

/// Функция для получения версии.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

const MAX_COL: u16 = 16_384;
const MAX_ROW: u32 = 1_048_576;

const _DATA_TYPES: [&str; 5] = ["s", "n", "d", "b", "f"];
