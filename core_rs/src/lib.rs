pub mod datatype;
pub mod helper;
pub mod reader;
pub mod utils;
pub mod writer;

/// Функция для получения версии.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
