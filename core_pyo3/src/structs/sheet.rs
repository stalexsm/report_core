use std::sync::Arc;

use core_rs::{
    structs::{
        comment::Comment,
        coordinate::Coordinate,
        range::{MergedRange, Range},
        sheet::Sheet,
    },
    traits::{ReadableSheet, WriteableSheet},
};
use parking_lot::RwLock;
use pyo3::prelude::*;

use super::{cell::WrapperCell, comment::WrapperComment};

/// Тип данных листа с которыми работает парсер.
#[pyclass(from_py_object)]
#[pyo3(module = "report_core", name = "Sheet")]
#[derive(Debug, Clone)]
pub struct WrapperSheet(pub(crate) Arc<RwLock<Sheet>>);

#[pymethods]
impl WrapperSheet {
    pub fn __repr__(slf: &Bound<'_, Self>) -> PyResult<String> {
        let slf = slf.borrow();
        let slf = slf.0.read();
        Ok(format!(
            "Sheet ({}) cells: {}",
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

    /// Установка имени листа
    #[setter]
    pub fn set_name(&self, name: String) -> PyResult<()> {
        let mut slf = self.0.write();
        slf.set_name(name.as_str());

        Ok(())
    }

    /// Состояние и видимость листа
    #[getter]
    pub fn sheet_state(&self) -> PyResult<String> {
        let slf = self.0.read();
        Ok(slf.get_sheet_state())
    }

    /// Метод установки видимости листа
    /// ---
    ///
    /// Arguments:
    /// ---
    ///     state: str
    ///         Тип видимости: Literal["visible", "hidden"]
    ///
    /// Returns:
    /// ---
    ///     None
    pub fn set_sheet_state(&self, state: &str) -> PyResult<()> {
        let mut slf = self.0.write();
        slf.set_sheet_state(state);

        Ok(())
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
    ///     list[Cell]
    #[getter]
    pub fn get_cells(&self) -> PyResult<Vec<WrapperCell>> {
        let slf = self.0.read();

        let cells = slf
            .get_cell_collection_sorted()
            .into_iter()
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

    /// Функция для добавления объединённых ячеек на лист.
    /// ---
    ///
    /// Arguments:
    /// ---
    ///     start_row: int
    ///         Начальная строка
    ///     end_row: int
    ///         Конечная строка
    ///     start_col: int
    ///         Начальная колонка
    ///     end_col: int
    ///         Конечная колонка
    pub fn add_merge_cells(
        &mut self,
        start_row: u32,
        end_row: u32,
        start_col: u16,
        end_col: u16,
    ) -> PyResult<()> {
        let range = Range::new(start_row, end_row, start_col, end_col);

        self.0.write().add_merge_range(range);

        Ok(())
    }

    /// Добавление комментария в ячейку
    /// ---
    ///
    /// Arguments:
    /// ---
    ///     row: int
    ///         Строка ячейки
    ///     col: int
    ///         Колонка ячейки
    ///     text: str
    ///         Текст комментария
    ///     author: str
    ///         Автор комментария
    pub fn add_comment(&mut self, row: u32, col: u16, text: &str, author: &str) -> PyResult<()> {
        let coord = Coordinate::new(row, col);
        let mut comment = Comment::new(coord, author);
        comment.set_text(text);

        self.0.write().add_comments(comment);

        Ok(())
    }

    /// Комментарии листа
    /// ---
    ///
    /// Returns:
    /// ---
    ///     list[Comment]
    #[getter]
    pub fn get_comments(&self) -> PyResult<Vec<WrapperComment>> {
        let comments = self
            .0
            .read()
            .get_comments()
            .iter()
            .map(|c| WrapperComment(Arc::clone(c)))
            .collect();

        Ok(comments)
    }

    /// Функция для получения/добавления ячейки.
    /// ---
    ///
    /// Arguments:
    /// ---
    ///     row: int
    ///         Строка ячейки
    ///     col: int
    ///         Колонка ячейки
    ///     value: str | None
    ///         Значение ячейки
    ///
    /// Returns:
    /// ---
    ///     Cell
    #[pyo3(signature = (row, col, value=None))]
    pub fn cell(&self, row: u32, col: u16, value: Option<&str>) -> PyResult<WrapperCell> {
        let coord = Coordinate::from((row, col));
        let mut slf = self.0.write();

        let cell = slf.cell(coord, value);

        Ok(WrapperCell(Arc::clone(cell)))
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

    /// Метод удаления колонок
    /// ---
    ///
    /// Arguments:
    /// ---
    ///     idx: int
    ///         Номер колонки
    ///     amount: int
    ///         Количество колонок
    pub fn delete_cols(&self, idx: u16, amount: u16) -> PyResult<()> {
        let mut slf = self.0.write();
        slf.delete_cols(idx, amount);

        Ok(())
    }

    /// Метод удаления строк
    /// ---
    ///
    /// Arguments:
    /// ---
    ///     idx: int
    ///         Номер строки
    ///     amount: int
    ///         Количество строк
    pub fn delete_rows(&self, idx: u32, amount: u32) -> PyResult<()> {
        let mut slf = self.0.write();
        slf.delete_rows(idx, amount);

        Ok(())
    }

    /// Метод установки высоты ячеек
    /// ---
    ///
    /// Arguments:
    /// ---
    ///     row_num: int
    ///         Номер строки
    ///     val: float
    ///         Высота ячейки
    ///
    /// Returns:
    /// ---
    ///     None
    pub fn set_height_row(&self, row_num: u32, val: f64) -> PyResult<()> {
        let mut slf = self.0.write();

        slf.set_height_row(row_num, val);
        Ok(())
    }

    /// Метод установки скрытости строки
    /// ---
    ///
    /// Arguments:
    /// ---
    ///     row_num: int
    ///         Номер строки
    ///     val: bool
    ///         Скрытость строки
    ///
    /// Returns:
    /// ---
    ///     None
    pub fn set_hidden_row(&self, row_num: u32, val: bool) -> PyResult<()> {
        let mut slf = self.0.write();

        slf.set_hidden_row(row_num, val);
        Ok(())
    }

    /// Метод установки ширины колонки
    /// ---
    ///
    /// Arguments:
    /// ---
    ///     col_num: int
    ///         Номер колонки
    ///     val: float
    ///         Ширина колонки
    ///
    /// Returns:
    /// ---
    ///     None
    pub fn set_width_column(&self, col_num: u16, val: f64) -> PyResult<()> {
        let mut slf = self.0.write();

        slf.set_width_column(col_num, val);
        Ok(())
    }

    /// Метод установки скрытости колонки
    /// ---
    ///
    /// Arguments:
    /// ---
    ///     col_num: int
    ///         Номер колонки
    ///     val: bool
    ///         Скрытость колонки
    ///
    /// Returns:
    /// ---
    ///     None
    pub fn set_hidden_column(&self, col_num: u16, val: bool) -> PyResult<()> {
        let mut slf = self.0.write();

        slf.set_hidden_column(col_num, val);
        Ok(())
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
    ///     list[Cell]
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

    /// Метод для получения высоты строки
    /// ---
    ///
    /// Arguments:
    /// ---
    ///     row_num: int
    ///         Номер строки
    ///
    /// Returns:
    /// ---
    ///     float
    pub fn get_height_by_row(&self, row_num: u32) -> PyResult<f64> {
        let slf = self.0.read();

        let val = slf.get_height_by_row(row_num);
        Ok(*val)
    }

    /// Метод для получения скрытости строки
    /// ---
    ///
    /// Arguments:
    /// ---
    ///     row_num: int
    ///         Номер строки
    ///
    /// Returns:
    /// ---
    ///     bool
    pub fn get_hidden_by_row(&self, row_num: u32) -> PyResult<bool> {
        let slf = self.0.read();

        let val = slf.get_hidden_by_row(row_num);
        Ok(*val)
    }

    /// Метод для получения ширины колонки
    /// ---
    ///
    /// Arguments:
    /// ---
    ///     col_num: int
    ///         Номер колонки
    ///
    /// Returns:
    /// ---
    ///     float
    pub fn get_width_by_column(&self, col_num: u16) -> PyResult<f64> {
        let slf = self.0.read();

        let val = slf.get_width_by_column(col_num);
        Ok(*val)
    }

    /// Метод для получения скрытости колонки
    /// ---
    ///
    /// Arguments:
    /// ---
    ///     col_num: int
    ///         Номер колонки
    ///
    /// Returns:
    /// ---
    ///     bool
    pub fn get_hidden_by_column(&self, col_num: u16) -> PyResult<bool> {
        let slf = self.0.read();

        let val = slf.get_hidden_by_column(col_num);
        Ok(*val)
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
    ///     Cell | None
    pub fn find_cell_by_regex(&self, regex: &str) -> PyResult<Option<WrapperCell>> {
        let slf = self.0.read();

        Ok(slf
            .find_cell_by_regex(regex)?
            .map(|c| WrapperCell(Arc::clone(c))))
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
    ///     Cell | None
    pub fn find_cell_by_str(&self, value: &str) -> PyResult<Option<WrapperCell>> {
        let slf = self.0.read();

        Ok(slf
            .find_cell_by_str(value)?
            .map(|c| WrapperCell(Arc::clone(c))))
    }

    /// Функция для получения ячейки по буквенной координате (A1).
    /// ---
    ///
    /// Arguments:
    /// ---------
    ///     letter: str
    ///         Координата (буквенная) A1
    ///
    /// Returns:
    /// --------
    ///     Cell | None
    pub fn find_cell_by_letter(&self, letter: &str) -> PyResult<Option<WrapperCell>> {
        let slf = self.0.read();

        Ok(slf
            .find_cell_by_letter(letter)?
            .map(|c| WrapperCell(Arc::clone(c))))
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
    ///     list[Cell]
    pub fn find_cells_by_regex(&self, regex: &str) -> PyResult<Vec<WrapperCell>> {
        let slf = self.0.read();

        let cells = slf.find_cells_by_regex(regex)?;
        let wrapped_cells = cells
            .into_iter()
            .map(|cell| WrapperCell(Arc::clone(cell)))
            .collect();

        Ok(wrapped_cells)
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
    ///     list[Cell]
    pub fn find_cells_by_str(&self, value: &str) -> PyResult<Vec<WrapperCell>> {
        let slf = self.0.read();

        let cells = slf.find_cells_by_str(value)?;
        let wrapped_cells = cells
            .into_iter()
            .map(|cell| WrapperCell(Arc::clone(cell)))
            .collect();

        Ok(wrapped_cells)
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
    ///     list[Cell]
    pub fn find_cells_for_rows_by_regex(
        &self,
        regex: &str,
        col_stop: u16,
    ) -> PyResult<Vec<WrapperCell>> {
        let slf = self.0.read();

        let cells = slf.find_cells_for_rows_by_regex(regex, col_stop)?;
        let wrapped_cells = cells
            .into_iter()
            .map(|cell| WrapperCell(Arc::clone(cell)))
            .collect();

        Ok(wrapped_cells)
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
    ///     list[Cell]
    pub fn find_cells_for_cols_by_regex(
        &self,
        regex: &str,
        row_stop: u32,
    ) -> PyResult<Vec<WrapperCell>> {
        let slf = self.0.read();

        let cells = slf.find_cells_for_cols_by_regex(regex, row_stop)?;
        let wrapped_cells = cells
            .into_iter()
            .map(|cell| WrapperCell(Arc::clone(cell)))
            .collect();

        Ok(wrapped_cells)
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
    ///     list[Cell]
    pub fn find_cells_multi_regex(
        &self,
        before_regex: &str,
        after_regex: &str,
    ) -> PyResult<Vec<WrapperCell>> {
        let slf = self.0.read();

        let cells = slf.find_cells_multi_regex(before_regex, after_regex)?;
        let wrapped_cells = cells
            .into_iter()
            .map(|cell| WrapperCell(Arc::clone(cell)))
            .collect();

        Ok(wrapped_cells)
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
    ///     list[Cell]
    pub fn find_cells_between_regex(
        &self,
        before_regex: &str,
        after_regex: &str,
    ) -> PyResult<Vec<WrapperCell>> {
        let slf = self.0.read();

        let cells = slf.find_cells_between_regex(before_regex, after_regex)?;
        let wrapped_cells = cells
            .into_iter()
            .map(|cell| WrapperCell(Arc::clone(cell)))
            .collect();

        Ok(wrapped_cells)
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
    ///     list[Cell]
    pub fn find_cells_range_rows(
        &self,
        start_row: u32,
        end_row: u32,
    ) -> PyResult<Vec<WrapperCell>> {
        let slf = self.0.read();

        let cells = slf.find_cells_range_rows(start_row, end_row)?;
        let wrapped_cells = cells
            .into_iter()
            .map(|cell| WrapperCell(Arc::clone(cell)))
            .collect();

        Ok(wrapped_cells)
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
    ///     list[Cell]
    pub fn find_cells_range_cols(
        &self,
        start_col: u16,
        end_col: u16,
    ) -> PyResult<Vec<WrapperCell>> {
        let slf = self.0.read();

        let cells = slf.find_cells_range_cols(start_col, end_col)?;
        let wrapped_cells = cells
            .into_iter()
            .map(|cell| WrapperCell(Arc::clone(cell)))
            .collect();

        Ok(wrapped_cells)
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
    pub fn find_value_by_coords(&self, row: u32, col: u16) -> PyResult<Option<String>> {
        let slf = self.0.read();

        Ok(slf.find_value_by_coords(row, col)?)
    }
}
