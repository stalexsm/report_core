use std::sync::Arc;

use ahash::HashMap;
use core_rs::{
    structs::{
        cell::Cell,
        coordinate::Coordinate,
        range::{MergedRange, Range},
        sheet::Sheet,
    },
    traits::{ReadableCell, ReadableSheet},
};
use parking_lot::RwLock;
use pyo3::{
    exceptions::PyValueError,
    prelude::*,
    types::{PyDict, PyList, PyString},
};

use super::cell::WrapperCell;
use crate::{PyValue, py_extract};

/// Вспомогательная функция для преобразования cells в rust тип
type CellMap = HashMap<(u32, u16), Arc<RwLock<Cell>>>;

/// Длина списка координат одного объединённого диапазона:
/// `[start_row, end_row, start_col, end_col]`.
const MERGE_RANGE_LEN: usize = 4;

/// Строгий разбор `merge_cells`: список 4-элементных списков или `None`
/// (нет объединений). Плоские и короткие списки — `ValueError`, а не
/// «диапазоны» с мусорными координатами.
fn extract_merge_cells(value: &PyValue) -> PyResult<Vec<Range>> {
    const MSG: &str =
        "merge_cells: expected a list of [start_row, end_row, start_col, end_col] lists";

    let ranges = match value {
        PyValue::None => Vec::new(),
        PyValue::List(items) => items
            .iter()
            .map(|item| match item {
                PyValue::List(parts) if parts.len() == MERGE_RANGE_LEN => {
                    let start_row = parts[0].as_u32()?;
                    let end_row = parts[1].as_u32()?;
                    let start_col = parts[2].as_u16()?;
                    let end_col = parts[3].as_u16()?;
                    Ok(Range::new(start_row, end_row, start_col, end_col))
                }
                _ => Err(PyValueError::new_err(format!(
                    "{MSG}, got: {}",
                    item.py_repr()
                ))),
            })
            .collect::<PyResult<Vec<_>>>()?,
        _ => {
            return Err(PyValueError::new_err(format!(
                "{MSG}, got: {}",
                value.py_repr()
            )));
        }
    };

    Ok(ranges)
}

fn extract_cells(obj: &Bound<'_, PyAny>) -> PyResult<CellMap> {
    let cells_attr = if obj.is_instance_of::<PyDict>() {
        obj.get_item("cells")?
    } else {
        obj.getattr("cells")?
    };

    let cells_list = cells_attr.cast::<PyList>()?;
    cells_list
        .iter()
        .map(|c| {
            let wrapper = WrapperCell::try_from(&c)?;
            let guard = wrapper.0.read();
            let coord = guard.get_coordinate();
            Ok(((coord.row, coord.column), wrapper.0.clone()))
        })
        .collect()
}

/// Лист готового XLSX-файла (только чтение).
#[pyclass(from_py_object)]
#[pyo3(module = "report_core.readable", name = "ReadableSheet")]
#[derive(Debug, Clone)]
pub struct WrapperSheet(pub(crate) Arc<RwLock<Sheet>>);

impl TryFrom<&Bound<'_, PyAny>> for WrapperSheet {
    type Error = PyErr;

    fn try_from(obj: &Bound<'_, PyAny>) -> PyResult<Self> {
        let name = py_extract!(obj, name)?.as_string_or_empty();
        let sheet_state = py_extract!(obj, sheet_state)?.as_string_or_empty();
        let merge_cells = extract_merge_cells(&py_extract!(obj, merge_cells)?)?;

        let map = extract_cells(obj)?;

        let sheet = Sheet::extract(&name, &sheet_state, merge_cells, map);
        Ok(Self(Arc::new(RwLock::new(sheet))))
    }
}

/// Лист для Finder: собственные типы библиотеки (лист книги, ReadableSheet)
/// отдаются тем же Arc — без копирования данных; чужие объекты (dict,
/// duck-typed) извлекаются в новый лист.
pub(crate) fn shared_sheet(obj: &Bound<'_, PyAny>) -> PyResult<Arc<RwLock<Sheet>>> {
    if let Ok(book_sheet) = obj.cast::<crate::structs::sheet::WrapperSheet>() {
        return Ok(book_sheet.borrow().0.clone());
    }

    if let Ok(readable_sheet) = obj.cast::<WrapperSheet>() {
        return Ok(readable_sheet.borrow().0.clone());
    }

    Ok(WrapperSheet::try_from(obj)?.0)
}

#[pymethods]
impl WrapperSheet {
    pub fn __repr__(slf: &Bound<'_, Self>) -> PyResult<String> {
        let class_name: Bound<'_, PyString> = slf.get_type().qualname()?;

        let slf = slf.borrow();
        let slf = slf.0.read();

        Ok(format!(
            "{} ({}) cells: {}",
            class_name,
            slf.get_name(),
            slf.get_cell_collection().len()
        ))
    }

    /// Имя листа
    #[getter]
    pub fn name(&self) -> PyResult<String> {
        let slf = self.0.read();
        Ok(slf.get_name())
    }

    /// Состояние и видимость листа
    #[getter]
    pub fn sheet_state(&self) -> PyResult<String> {
        let slf = self.0.read();
        Ok(slf.get_sheet_state())
    }

    /// Получение максимальной строки в книге
    /// ---
    ///
    /// Returns:
    /// ---
    ///     int
    #[getter]
    pub fn get_max_row(&self) -> PyResult<u32> {
        let slf = self.0.read();

        Ok(slf.get_max_row())
    }

    /// Получение максимальной колонки в книге
    /// ---
    ///
    /// Returns:
    /// ---
    ///     int
    #[getter]
    pub fn get_max_column(&self) -> PyResult<u16> {
        let slf = self.0.read();

        Ok(slf.get_max_column())
    }

    /// Ячейки листа
    /// ---
    ///
    /// Returns:
    /// ---
    ///     list[ReadableCell]
    #[getter]
    pub fn get_cells(&self) -> PyResult<Vec<WrapperCell>> {
        let slf = self.0.read();

        let cells = slf
            .get_cell_collection_sorted()
            .iter()
            .map(|cell| WrapperCell(Arc::clone(cell)))
            .collect();

        Ok(cells)
    }

    /// Объединённые диапазоны ячеек
    /// ---
    ///
    /// Returns:
    /// ---
    ///     list[tuple[int, int, int, int]]
    #[getter]
    pub fn get_merge_cells(&self) -> PyResult<Vec<MergedRange>> {
        let merged_cells = self
            .0
            .read()
            .get_merge_cell_collection()
            .iter()
            .map(|range| range.into())
            .collect();

        Ok(merged_cells)
    }

    /// Функция для получения значения ячейки.
    /// ---
    ///
    /// Arguments:
    /// ---
    ///     row: int
    ///         Строка ячейки
    ///     col: int
    ///         Колонка ячейки
    ///
    /// Returns:
    /// ---
    ///     str
    pub fn get_value_cell(&self, row: u32, col: u16) -> PyResult<String> {
        let slf = self.0.read();

        let coord = Coordinate::from((row, col));
        Ok(slf.get_cell_value(coord))
    }

    /// Получить список всех ячеек в заданном диапазоне.
    /// ---
    ///
    /// Arguments:
    /// ---
    ///     start_row: int | None
    ///         Номер начальной строки
    ///     end_row: int | None
    ///         Номер конечной строки
    ///     start_col: int | None
    ///         Номер начальной колонки
    ///     end_col: int | None
    ///         Номер конечной колонки
    ///
    /// Returns:
    /// ---
    ///     list[ReadableCell]
    #[pyo3(signature = (start_row=None, end_row=None, start_col=None, end_col=None))]
    pub fn get_cells_by_range(
        &self,
        start_row: Option<u32>,
        end_row: Option<u32>,
        start_col: Option<u16>,
        end_col: Option<u16>,
    ) -> PyResult<Vec<WrapperCell>> {
        let slf = self.0.read();

        let cells = slf
            .get_cell_collection_by_range(start_row, end_row, start_col, end_col)
            .map(|cell| WrapperCell(Arc::clone(cell)))
            .collect::<Vec<_>>();

        Ok(cells)
    }

    /// Функция для получения ячейки по регулярному (шаблону) значению.
    /// ---
    ///
    /// Arguments:
    /// ---
    ///     regex: str
    ///         Шаблон (регулярное значение)
    ///
    /// Returns:
    /// ---
    ///     ReadableCell | None
    pub fn find_cell_by_regex(&self, py: Python<'_>, regex: &str) -> PyResult<Option<WrapperCell>> {
        let slf = self.0.read();

        py.detach(|| {
            Ok(slf
                .find_cell_by_regex(regex)?
                .map(|c| WrapperCell(Arc::clone(c))))
        })
    }

    /// Функция для получения ячейки по строковому значению.
    /// ---
    ///
    /// Arguments:
    /// ---
    ///     value: str
    ///         Строка (значение)
    ///
    /// Returns:
    /// ---
    ///     ReadableCell | None
    pub fn find_cell_by_str(&self, py: Python<'_>, value: &str) -> PyResult<Option<WrapperCell>> {
        let slf = self.0.read();

        py.detach(|| {
            Ok(slf
                .find_cell_by_str(value)?
                .map(|c| WrapperCell(Arc::clone(c))))
        })
    }

    /// Функция для получения ячейки по координатам
    /// ---
    ///
    /// Arguments:
    /// ---
    ///     row: int
    ///         Строка ячейки
    ///     col: int
    ///         Колонка ячейки
    ///
    /// Returns:
    /// ---
    ///     ReadableCell | None
    pub fn find_cell_by_coords(
        &self,
        py: Python<'_>,
        row: u32,
        col: u16,
    ) -> PyResult<Option<WrapperCell>> {
        let slf = self.0.read();

        py.detach(|| {
            Ok(slf
                .find_cell_by_coords(row, col)?
                .map(|c| WrapperCell(Arc::clone(c))))
        })
    }

    /// Функция для получения ячейки по буквенной координате (A1).
    /// ---
    ///
    /// Arguments:
    /// ---
    ///     letter: str
    ///         Координата (буквенная) A1
    ///
    /// Returns:
    /// ---
    ///     ReadableCell | None
    ///
    /// Raises:
    /// ---
    ///     ValueError
    ///         Если буква не координата ячейки (например, "1A") или за границами листа
    pub fn find_cell_by_letter(
        &self,
        py: Python<'_>,
        letter: &str,
    ) -> PyResult<Option<WrapperCell>> {
        let slf = self.0.read();

        py.detach(|| {
            let cell = slf
                .find_cell_by_letter(letter)
                .map_err(crate::user_value_error)?;

            Ok(cell.map(|c| WrapperCell(Arc::clone(c))))
        })
    }

    /// Функция для получения ячеек по регулярному (шаблону) значению.
    /// ---
    ///
    /// Arguments:
    /// ---
    ///     regex: str
    ///         Шаблон (регулярное значение)
    ///
    /// Returns:
    /// ---
    ///     list[ReadableCell]
    pub fn find_cells_by_regex(&self, py: Python<'_>, regex: &str) -> PyResult<Vec<WrapperCell>> {
        let slf = self.0.read();

        py.detach(|| {
            let wrapper_cells = slf
                .find_cells_by_regex(regex)?
                .iter()
                .map(|cell| WrapperCell(Arc::clone(cell)))
                .collect();

            Ok(wrapper_cells)
        })
    }

    /// Функция для получения ячеек по строковому значению.
    /// ---
    ///
    /// Arguments:
    /// ---
    ///     value: str
    ///         Строка (значение)
    ///
    /// Returns:
    /// ---
    ///     list[ReadableCell]
    pub fn find_cells_by_str(&self, py: Python<'_>, value: &str) -> PyResult<Vec<WrapperCell>> {
        let slf = self.0.read();

        py.detach(|| {
            let wrapper_cells = slf
                .find_cells_by_str(value)?
                .iter()
                .map(|cell| WrapperCell(Arc::clone(cell)))
                .collect();

            Ok(wrapper_cells)
        })
    }

    /// Функция для получения ячеек по регулярному (шаблону) значению до определенной колонки.
    /// ---
    ///
    /// Arguments:
    /// ---
    ///     regex: str
    ///         Шаблон (регулярное значение)
    ///     col_stop: int
    ///         Значение колонки до которой забирать ячейки
    ///
    /// Returns:
    /// ---
    ///     list[ReadableCell]
    pub fn find_cells_for_rows_by_regex(
        &self,
        py: Python<'_>,
        regex: &str,
        col_stop: u16,
    ) -> PyResult<Vec<WrapperCell>> {
        let slf = self.0.read();

        py.detach(|| {
            let wrapper_cells = slf
                .find_cells_for_rows_by_regex(regex, col_stop)?
                .iter()
                .map(|cell| WrapperCell(Arc::clone(cell)))
                .collect();

            Ok(wrapper_cells)
        })
    }

    /// Функция для получения ячеек по регулярному (шаблону) значению до определенной строки.
    /// ---
    ///
    /// Arguments:
    /// ---
    ///     regex: str
    ///         Шаблон (регулярное значение)
    ///     row_stop: int
    ///         Значение строки до которой забирать ячейки
    ///
    /// Returns:
    /// ---
    ///     list[ReadableCell]
    pub fn find_cells_for_cols_by_regex(
        &self,
        py: Python<'_>,
        regex: &str,
        row_stop: u32,
    ) -> PyResult<Vec<WrapperCell>> {
        let slf = self.0.read();

        py.detach(|| {
            let wrapper_cells = slf
                .find_cells_for_cols_by_regex(regex, row_stop)?
                .iter()
                .map(|cell| WrapperCell(Arc::clone(cell)))
                .collect();

            Ok(wrapper_cells)
        })
    }

    /// Функция для получения ячеек по регулярным (шаблонам) значениям.
    /// Находит ячейки в соответствии шаблонов.
    /// ---
    ///
    /// Arguments:
    /// ---
    ///     before_regex: str
    ///         Первый шаблон (регулярное значение)
    ///     after_regex: str
    ///         Второй шаблон (регулярное значение)
    ///
    /// Returns:
    /// ---
    ///     list[ReadableCell]
    pub fn find_cells_multi_regex(
        &self,
        py: Python<'_>,
        before_regex: &str,
        after_regex: &str,
    ) -> PyResult<Vec<WrapperCell>> {
        let slf = self.0.read();

        py.detach(|| {
            let wrapper_cells = slf
                .find_cells_multi_regex(before_regex, after_regex)?
                .iter()
                .map(|cell| WrapperCell(Arc::clone(cell)))
                .collect();

            Ok(wrapper_cells)
        })
    }

    /// Функция для получения ячеек по регулярным (шаблонам) значениям.
    /// Находит ячейки от первого шаблона до второго.
    /// ---
    ///
    /// Arguments:
    /// ---
    ///     before_regex: str
    ///         Первый шаблон (регулярное значение)
    ///     after_regex: str
    ///         Второй шаблон (регулярное значение)
    ///
    /// Returns:
    /// ---
    ///     list[ReadableCell]
    pub fn find_cells_between_regex(
        &self,
        py: Python<'_>,
        before_regex: &str,
        after_regex: &str,
    ) -> PyResult<Vec<WrapperCell>> {
        let slf = self.0.read();

        py.detach(|| {
            let wrapper_cells = slf
                .find_cells_between_regex(before_regex, after_regex)?
                .iter()
                .map(|cell| WrapperCell(Arc::clone(cell)))
                .collect();

            Ok(wrapper_cells)
        })
    }

    /// Функция для получения ячеек в диапазоне строк.
    /// ---
    ///
    /// Arguments:
    /// ---
    ///     start_row: int
    ///         Стартовая строка
    ///     end_row: int
    ///         Финишная строка
    ///
    /// Returns:
    /// ---
    ///     list[ReadableCell]
    pub fn find_cells_range_rows(
        &self,
        py: Python<'_>,
        start_row: u32,
        end_row: u32,
    ) -> PyResult<Vec<WrapperCell>> {
        let slf = self.0.read();

        py.detach(|| {
            let wrapper_cells = slf
                .find_cells_range_rows(start_row, end_row)?
                .iter()
                .map(|cell| WrapperCell(Arc::clone(cell)))
                .collect();

            Ok(wrapper_cells)
        })
    }

    /// Функция для получения ячеек в диапазоне колонок.
    /// ---
    ///
    /// Arguments:
    /// ---
    ///     start_col: int
    ///         Стартовая колонка
    ///     end_col: int
    ///         Финишная колонка
    ///
    /// Returns:
    /// ---
    ///     list[ReadableCell]
    pub fn find_cells_range_cols(
        &self,
        py: Python<'_>,
        start_col: u16,
        end_col: u16,
    ) -> PyResult<Vec<WrapperCell>> {
        let slf = self.0.read();

        py.detach(|| {
            let wrapper_cells = slf
                .find_cells_range_cols(start_col, end_col)?
                .iter()
                .map(|cell| WrapperCell(Arc::clone(cell)))
                .collect();

            Ok(wrapper_cells)
        })
    }

    /// Функция для получения значений ячеек по колонке и списку строк.
    /// ---
    ///
    /// Значения идут в порядке запрошенных строк; отсутствующая ячейка даёт
    /// пустую строку — длина результата всегда равна длине запроса.
    ///
    /// Arguments:
    /// ---
    ///     col: int
    ///         Колонка
    ///     rows: list[int]
    ///         Список строк
    ///
    /// Returns:
    /// ---
    ///     list[str]
    pub fn find_values_by_col_rows(
        &self,
        py: Python<'_>,
        col: u16,
        rows: Vec<u32>,
    ) -> PyResult<Vec<String>> {
        let slf = self.0.read();

        py.detach(|| Ok(slf.find_values_by_col_rows(col, rows)?))
    }

    /// Функция для получения значений ячеек по строке и списку колонок.
    /// ---
    ///
    /// Значения идут в порядке запрошенных колонок; отсутствующая ячейка даёт
    /// пустую строку — длина результата всегда равна длине запроса.
    ///
    /// Arguments:
    /// ---
    ///     row: int
    ///         Строка
    ///     cols: list[int]
    ///         Список колонок
    ///
    /// Returns:
    /// ---
    ///     list[str]
    pub fn find_values_by_row_cols(
        &self,
        py: Python<'_>,
        row: u32,
        cols: Vec<u16>,
    ) -> PyResult<Vec<String>> {
        let slf = self.0.read();

        py.detach(|| Ok(slf.find_values_by_row_cols(row, cols)?))
    }

    /// Функция для поиска значения ячейки по координатам
    /// ---
    ///
    /// Arguments:
    /// ---
    ///     row: int
    ///         Индекс строки
    ///     col: int
    ///         Индекс колонки
    ///
    /// Returns:
    /// ---
    ///     str | None
    pub fn find_value_by_coords(
        &self,
        py: Python<'_>,
        row: u32,
        col: u16,
    ) -> PyResult<Option<String>> {
        let slf = self.0.read();

        py.detach(|| Ok(slf.find_value_by_coords(row, col)?))
    }
}
