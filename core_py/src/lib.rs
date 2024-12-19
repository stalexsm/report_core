pub(crate) mod funcs;
pub(crate) mod structs;

use funcs::{
    find_cell_by_coords, find_cell_by_letter, find_cell_by_regex, find_cells_between_regex,
    find_cells_by_regex, find_cells_for_cols_by_regex, find_cells_for_rows_by_regex,
    find_cells_multi_regex, find_cells_range_cols, find_cells_range_rows, find_value_by_coords,
};
use pyo3::prelude::*;
use structs::{
    book::WrapperBook,
    cell::{self, WrapperCell},
    readable::finder,
    service::WrapperService,
    sheet::{self, WrapperSheet},
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
    add_classes!(m, WrapperService, WrapperBook, WrapperSheet, WrapperCell);

    // functions
    m.add_function(wrap_pyfunction!(version, m)?)?;
    m.add_function(wrap_pyfunction!(column_number_to_letter, m)?)?;
    m.add_function(wrap_pyfunction!(get_letter_coordinate, m)?)?;

    // Sub Module
    let readable = PyModule::new(m.py(), "readable")?;
    readable.add_class::<finder::WrapperFinder>()?;
    readable.add_class::<sheet::WrapperSheet>()?;
    readable.add_class::<cell::WrapperCell>()?;

    // funcs
    readable.add_function(wrap_pyfunction!(find_cell_by_coords, &readable)?)?;
    readable.add_function(wrap_pyfunction!(find_value_by_coords, &readable)?)?;
    readable.add_function(wrap_pyfunction!(find_cell_by_regex, &readable)?)?;
    readable.add_function(wrap_pyfunction!(find_cell_by_letter, &readable)?)?;
    readable.add_function(wrap_pyfunction!(find_cells_by_regex, &readable)?)?;
    readable.add_function(wrap_pyfunction!(find_cells_for_rows_by_regex, &readable)?)?;
    readable.add_function(wrap_pyfunction!(find_cells_for_cols_by_regex, &readable)?)?;
    readable.add_function(wrap_pyfunction!(find_cells_multi_regex, &readable)?)?;
    readable.add_function(wrap_pyfunction!(find_cells_between_regex, &readable)?)?;
    readable.add_function(wrap_pyfunction!(find_cells_range_rows, &readable)?)?;
    readable.add_function(wrap_pyfunction!(find_cells_range_cols, &readable)?)?;

    m.add_submodule(&readable)?;

    m.py()
        .import("sys")?
        .getattr("modules")?
        .set_item("report_core.readable", readable)?;

    Ok(())
}
