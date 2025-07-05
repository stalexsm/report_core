pub(crate) mod funcs;
pub(crate) mod macros;
pub(crate) mod structs;

pub(crate) use macros::*;

use core_rs::{DEFAULT_COLUMN_WIDTH, DEFAULT_ROW_HEIGHT};
use funcs::{
    find_cell_by_coords, find_cell_by_letter, find_cell_by_regex, find_cell_by_str,
    find_cells_between_regex, find_cells_by_regex, find_cells_by_str, find_cells_for_cols_by_regex,
    find_cells_for_rows_by_regex, find_cells_multi_regex, find_cells_range_cols,
    find_cells_range_rows, find_value_by_coords, find_values_by_col_rows, find_values_by_row_cols,
};
use pyo3::prelude::*;
use structs::{
    book::WrapperBook, cell::WrapperCell, comment::WrapperComment, readable,
    service::WrapperService, sheet::WrapperSheet,
};

/// Преобразование номера колонки в букву.
#[pyfunction]
fn column_number_to_letter(col: u16) -> PyResult<String> {
    Python::with_gil(|_py| {
        let letter = core_rs::utils::index_to_alpha(col);

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
    m.add("__version__", version())?;
    // classes
    add_classes!(
        m,
        WrapperService,
        WrapperBook,
        WrapperSheet,
        WrapperCell,
        WrapperComment
    );

    // functions
    m.add_function(wrap_pyfunction!(version, m)?)?;
    m.add_function(wrap_pyfunction!(column_number_to_letter, m)?)?;
    m.add_function(wrap_pyfunction!(get_letter_coordinate, m)?)?;

    // constants
    m.add("DEFAULT_ROW_HEIGHT", DEFAULT_ROW_HEIGHT)?;
    m.add("DEFAULT_COLUMN_WIDTH", DEFAULT_COLUMN_WIDTH)?;

    // Sub Module
    let readable = PyModule::new(m.py(), "readable")?;
    readable.add_class::<readable::finder::WrapperFinder>()?;
    readable.add_class::<readable::sheet::WrapperSheet>()?;
    readable.add_class::<readable::cell::WrapperCell>()?;

    // funcs
    readable.add_function(wrap_pyfunction!(readable::create_finder, &readable)?)?;
    readable.add_function(wrap_pyfunction!(find_cell_by_coords, &readable)?)?;
    readable.add_function(wrap_pyfunction!(find_value_by_coords, &readable)?)?;
    readable.add_function(wrap_pyfunction!(find_cell_by_regex, &readable)?)?;
    readable.add_function(wrap_pyfunction!(find_cell_by_str, &readable)?)?;
    readable.add_function(wrap_pyfunction!(find_cell_by_letter, &readable)?)?;
    readable.add_function(wrap_pyfunction!(find_cells_by_regex, &readable)?)?;
    readable.add_function(wrap_pyfunction!(find_cells_by_str, &readable)?)?;
    readable.add_function(wrap_pyfunction!(find_cells_for_rows_by_regex, &readable)?)?;
    readable.add_function(wrap_pyfunction!(find_cells_for_cols_by_regex, &readable)?)?;
    readable.add_function(wrap_pyfunction!(find_cells_multi_regex, &readable)?)?;
    readable.add_function(wrap_pyfunction!(find_cells_between_regex, &readable)?)?;
    readable.add_function(wrap_pyfunction!(find_cells_range_rows, &readable)?)?;
    readable.add_function(wrap_pyfunction!(find_cells_range_cols, &readable)?)?;
    readable.add_function(wrap_pyfunction!(find_values_by_col_rows, &readable)?)?;
    readable.add_function(wrap_pyfunction!(find_values_by_row_cols, &readable)?)?;

    m.add_submodule(&readable)?;

    m.py()
        .import("sys")?
        .getattr("modules")?
        .set_item("report_core.readable", readable)?;

    Ok(())
}
