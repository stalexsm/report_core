pub(crate) mod funcs;
pub(crate) mod macros;
pub(crate) mod py_serde;
pub(crate) mod structs;

pub(crate) use macros::*;

use pyo3::{exceptions::PyValueError, prelude::*};

/// Преобразование ошибки ядра в `ValueError`.
///
/// Используется на путях, где ошибка ядра означает невалидные входные
/// данные пользователя (например, невалидная буква ячейки), а не
/// внутренний сбой: по умолчанию `anyhow`-ошибки доходят до Python
/// как `RuntimeError`.
pub(crate) fn user_value_error(err: anyhow::Error) -> PyErr {
    PyValueError::new_err(err.to_string())
}

/// Версия ядра, публикуемая как константа `__version__`.
const VERSION: &str = core_rs::version();

/// Rust-ядро для построения отчётов на основе XLSX.
//
// Декларативное (inline) объявление модуля обязательно: только такой синтаксис
// поддерживает интроспекцию PyO3 (`experimental-inspect`) и автогенерацию
// .pyi-стабов (`scripts/generate-stubs.sh`).
#[pymodule]
#[pyo3(name = "_report_core")]
mod report_core {
    use pyo3::prelude::*;

    // Классы
    #[pymodule_export]
    use crate::structs::book::WrapperBook;
    #[pymodule_export]
    use crate::structs::cell::WrapperCell;
    #[pymodule_export]
    use crate::structs::comment::WrapperComment;
    #[pymodule_export]
    use crate::structs::service::WrapperService;
    #[pymodule_export]
    use crate::structs::sheet::WrapperSheet;

    // Функции
    /// Функция для преобразования номера колонки в букву
    /// ---
    ///
    /// Arguments:
    /// ---
    ///     col: u16
    ///         Номер колонки
    ///
    /// Returns:
    /// ---
    ///     str
    #[pyfunction]
    fn column_number_to_letter(col: u16) -> String {
        core_rs::utils::index_to_alpha(col)
    }

    /// Функция для получения координаты ячейки в стиле A1
    /// ---
    ///
    /// Arguments:
    /// ---
    ///     row: u32
    ///         Номер строки
    ///     col: u16
    ///         Номер колонки
    ///
    /// Returns:
    /// ---
    ///     str
    #[pyfunction]
    fn get_letter_coordinate(row: u32, col: u16) -> String {
        core_rs::utils::get_letter_coordinate(row, col)
    }

    /// Для получения версии
    /// ---------------------
    ///
    /// Returns:
    /// --------
    ///     str
    #[pyfunction]
    fn version() -> String {
        core_rs::version().to_string()
    }

    // Константы
    /// Версия ядра.
    // Имя задано Python-API (`__version__`), а не соглашениями Rust.
    #[allow(non_upper_case_globals)]
    #[pymodule_export]
    const __version__: &str = crate::VERSION;

    /// Ширина колонки по умолчанию (~140 пикселей).
    #[pymodule_export]
    const DEFAULT_COLUMN_WIDTH: f64 = core_rs::DEFAULT_COLUMN_WIDTH;

    /// Высота строки по умолчанию (~20 пикселей).
    #[pymodule_export]
    const DEFAULT_ROW_HEIGHT: f64 = core_rs::DEFAULT_ROW_HEIGHT;

    // Сабмодуль
    /// Сабмодуль для работы с готовыми XLSX-файлами (только чтение).
    // Для вложенных модулей `submodule` неявный, явно указывать нельзя.
    #[pymodule]
    #[pyo3(name = "readable")]
    mod readable {
        // Классы
        #[pymodule_export]
        use crate::structs::readable::cell::WrapperCell;
        #[pymodule_export]
        use crate::structs::readable::finder::WrapperFinder;
        #[pymodule_export]
        use crate::structs::readable::sheet::WrapperSheet;

        // Функции
        #[pymodule_export]
        use crate::structs::readable::create_finder;

        #[pymodule_export]
        use crate::funcs::find_cell_by_coords;

        #[pymodule_export]
        use crate::funcs::find_value_by_coords;

        #[pymodule_export]
        use crate::funcs::find_cell_by_regex;

        #[pymodule_export]
        use crate::funcs::find_cell_by_str;

        #[pymodule_export]
        use crate::funcs::find_cell_by_letter;

        #[pymodule_export]
        use crate::funcs::find_cells_by_regex;

        #[pymodule_export]
        use crate::funcs::find_cells_by_str;

        #[pymodule_export]
        use crate::funcs::find_cells_for_rows_by_regex;

        #[pymodule_export]
        use crate::funcs::find_cells_for_cols_by_regex;

        #[pymodule_export]
        use crate::funcs::find_cells_multi_regex;

        #[pymodule_export]
        use crate::funcs::find_cells_between_regex;

        #[pymodule_export]
        use crate::funcs::find_cells_range_rows;

        #[pymodule_export]
        use crate::funcs::find_cells_range_cols;

        #[pymodule_export]
        use crate::funcs::find_values_by_col_rows;

        #[pymodule_export]
        use crate::funcs::find_values_by_row_cols;
    }
}
