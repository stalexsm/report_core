use std::sync::Arc;

use core_rs::{structs::cell::Cell, structs::sheet::Sheet};
use parking_lot::RwLock;
use pyo3::prelude::*;

/// Общая механика листовых поисков `find_cell_*`: лок листа, выполнение
/// поиска без GIL (`py.detach`), заворачивание найденной ячейки в py-обёртку
/// через конструктор `wrap`. Единая точка избавляет Sheet и ReadableSheet
/// от копии служебного кода в каждом методе.
pub(crate) fn find_one<W: Send>(
    py: Python<'_>,
    sheet: &Arc<RwLock<Sheet>>,
    wrap: impl Fn(Arc<RwLock<Cell>>) -> W + Send + Sync,
    search: impl FnOnce(&Sheet) -> core_rs::Result<Option<&Arc<RwLock<Cell>>>> + Send + Sync,
) -> PyResult<Option<W>> {
    let slf = sheet.read();

    py.detach(|| {
        Ok(search(&slf)
            .map_err(crate::py_error)?
            .map(|cell| wrap(Arc::clone(cell))))
    })
}

/// Общая механика листовых поисков `find_cells_*`: как [`find_one`], но
/// возвращает список ячеек в порядке выдачи поиска.
pub(crate) fn find_many<W: Send>(
    py: Python<'_>,
    sheet: &Arc<RwLock<Sheet>>,
    wrap: impl Fn(Arc<RwLock<Cell>>) -> W + Copy + Send + Sync,
    search: impl FnOnce(&Sheet) -> core_rs::Result<Vec<&Arc<RwLock<Cell>>>> + Send + Sync,
) -> PyResult<Vec<W>> {
    let slf = sheet.read();

    py.detach(|| {
        Ok(search(&slf)
            .map_err(crate::py_error)?
            .into_iter()
            .map(|cell| wrap(Arc::clone(cell)))
            .collect())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Проверка сигнатуры на реальных типах: конструктор-кортеж подходит
    /// как `wrap`, поиск — как `search`.
    #[test]
    fn helper_compiles_with_real_types() {
        use core_rs::traits::ReadableSheet;

        pyo3::Python::initialize();
        Python::attach(|py| {
            use core_rs::traits::WriteableSheet;

            let sheet = Arc::new(RwLock::new(Sheet::new("лист", "visible")));
            sheet.write().cell(
                core_rs::structs::coordinate::Coordinate::new(1, 1),
                Some("v"),
            );

            let found = find_one(
                py,
                &sheet,
                WrapperCellLike,
                |s| s.find_cell_by_str("v"),
            )
            .unwrap()
            .expect("ячейка найдена");

            assert_eq!(found.value(), "v");

            let all = find_many(
                py,
                &sheet,
                WrapperCellLike,
                |s| s.find_cells_by_str("v"),
            )
            .unwrap();

            assert_eq!(all.len(), 1);
        });
    }

    /// Подставка вместо py-обёртки: хранит то же значение, что WrapperCell.
    struct WrapperCellLike(Arc<RwLock<Cell>>);

    impl WrapperCellLike {
        fn value(&self) -> String {
            use core_rs::traits::ReadableCell;

            self.0.read().get_value()
        }
    }
}
