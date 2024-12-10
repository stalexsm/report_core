pub(crate) mod helper;
pub(crate) mod reader;
pub(crate) mod services;
pub(crate) mod utils;
pub(crate) mod writer;

use helper::{WrapperHelperSheet, WrapperHelperSheetCell};
use pyo3::prelude::*;
use reader::{cell::WrapperXLSXSheetCellRead, sheet::WrapperXLSXSheetRead};
use services::Service;
use writer::{book::WrapperXLSXBook, cell::WrapperXLSXSheetCell, sheet::WrapperXLSXSheet};

/// Преобразование номера колонки в букву.
#[pyfunction]
fn column_number_to_letter(col: u16) -> PyResult<String> {
    Python::with_gil(|_py| {
        let letter = core_rs::utils::column_number_to_letter(col);

        Ok(letter)
    })
}

/// Преобразование номера колонки в координату.
#[pyfunction]
fn get_letter_coordinate(row: u32, col: u16) -> PyResult<String> {
    Python::with_gil(|_py| {
        let letter = core_rs::utils::get_letter_coordinate(row, col);

        Ok(letter)
    })
}

/// Returns the version of the underlying queue_rs library.
///
/// Returns
/// -------
/// version : str
///   The version of the underlying queue_rs library.
///
#[pyfunction]
fn version() -> String {
    core_rs::version().to_string()
}

macro_rules! add_classes {
    ($m:expr, $($class:ty),*) => {{
        $(
            $m.add_class::<$class>()?;
        )*
    }};
}

#[pymodule]
#[pyo3(name = "_report_core")]
fn report_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // classes
    add_classes!(
        m,
        Service,
        WrapperXLSXBook,
        WrapperXLSXSheet,
        WrapperXLSXSheetCell,
        WrapperXLSXSheetRead,
        WrapperXLSXSheetCellRead,
        WrapperHelperSheet,
        WrapperHelperSheetCell
    );

    // functions
    m.add_function(wrap_pyfunction!(version, m)?)?;
    m.add_function(wrap_pyfunction!(column_number_to_letter, m)?)?;
    m.add_function(wrap_pyfunction!(get_letter_coordinate, m)?)?;

    Ok(())
}
