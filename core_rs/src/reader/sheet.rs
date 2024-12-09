use crate::helper::HelperCell;

use super::cell::XLSXSheetCellRead;
use anyhow::Result;
use rayon::slice::ParallelSliceMut;
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct XLSXSheetRead {
    pub name: String,
    pub max_row: u32,
    pub max_column: u16,
    pub index: i32,
    pub _cells: HashMap<(u32, u16), XLSXSheetCellRead>,
}

impl XLSXSheetRead {
    pub fn cells(&self) -> impl Iterator<Item = &XLSXSheetCellRead> {
        let mut cells: Vec<_> = self._cells.values().collect();
        cells.sort_by_key(|cell| (cell.row, cell.column));

        cells.into_iter()
    }

    pub fn iter_cells(
        &self,
        min_row: Option<u32>,
        max_row: Option<u32>,
        min_col: Option<u16>,
        max_col: Option<u16>,
    ) -> Result<Vec<XLSXSheetCellRead>> {
        // Получение значений, так как они необязательные.
        let min_row = min_row.unwrap_or(1);
        let max_row = max_row.unwrap_or(self.max_row);
        let min_col = min_col.unwrap_or(1);
        let max_col = max_col.unwrap_or(self.max_column);

        let mut cells = self._cells.values().cloned().collect::<Vec<_>>();
        cells.par_sort_by_key(|cell| (cell.row, cell.column));

        HelperCell::iter_cells(min_row, max_row, min_col, max_col, cells)
    }

    pub fn find_cell_by_pattern_regex(&self, pattern: &str) -> Result<Option<XLSXSheetCellRead>> {
        let mut cells = self._cells.values().cloned().collect::<Vec<_>>();
        cells.sort_by_key(|cell| (cell.row, cell.column));

        HelperCell::find_cell_by_pattern_regex(pattern, cells)
    }

    // Поиск ячеек по шаблону
    pub fn find_cells_by_pattern_regex(&self, pattern: &str) -> Result<Vec<XLSXSheetCellRead>> {
        let mut cells = self._cells.values().cloned().collect::<Vec<_>>();
        cells.par_sort_by_key(|cell| (cell.row, cell.column));

        HelperCell::find_cells_by_pattern_regex(pattern, cells)
    }

    /// Поиск ячеек колонок для строк которые соответствуют патерну
    pub fn find_cells_for_rows_pattern_regex(
        &self,
        pattern: &str,
        col_stop: Option<u16>,
    ) -> Result<Vec<XLSXSheetCellRead>> {
        let mut cells = self._cells.values().cloned().collect::<Vec<_>>();
        cells.par_sort_by_key(|cell| (cell.row, cell.column));

        HelperCell::find_cells_for_rows_pattern_regex(pattern, cells, col_stop)
    }

    /// Поиск ячеек строк для колонок которые соответствуют патерну
    pub fn find_cells_for_cols_pattern_regex(
        &self,
        pattern: &str,
        row_stop: Option<u32>,
    ) -> Result<Vec<XLSXSheetCellRead>> {
        let mut cells = self._cells.values().cloned().collect::<Vec<_>>();
        cells.par_sort_by_key(|cell| (cell.row, cell.column));

        HelperCell::find_cells_for_cols_pattern_regex(pattern, cells, row_stop)
    }

    /// Поиск ячеек с помощью ИЛИ ячейки по патернам
    pub fn find_cells_multi_pattern_regex(
        &self,
        pattern_1: &str,
        pattern_2: &str,
    ) -> Result<Vec<XLSXSheetCellRead>> {
        let mut cells = self._cells.values().cloned().collect::<Vec<_>>();
        cells.par_sort_by_key(|cell| (cell.row, cell.column));

        HelperCell::find_cells_multi_pattern_regex(pattern_1, pattern_2, cells)
    }

    /// Поиск ячейки по буквенной координате A1 (cell)
    pub fn find_cell_by_cell(&self, cell: &str) -> Result<Option<XLSXSheetCellRead>> {
        let mut cells = self._cells.values().cloned().collect::<Vec<_>>();
        cells.par_sort_by_key(|cell| (cell.row, cell.column));

        HelperCell::find_cell_by_cell(cell, cells)
    }

    /// Поиск ячейки по координате
    pub fn find_cell_by_coords(&self, row: u32, col: u16) -> Result<Option<XLSXSheetCellRead>> {
        let mut cells = self._cells.values().cloned().collect::<Vec<_>>();
        cells.par_sort_by_key(|cell| (cell.row, cell.column));

        HelperCell::find_cell_by_coords(row, col, cells)
    }

    /// Поиск ячеек между шаьлонами
    pub fn find_cells_between_patterns(
        &self,
        pattern_start: &str,
        pattern_end: &str,
    ) -> Result<Vec<XLSXSheetCellRead>> {
        let mut cells = self._cells.values().cloned().collect::<Vec<_>>();
        cells.par_sort_by_key(|cell| (cell.row, cell.column));

        HelperCell::find_cells_between_patterns(pattern_start, pattern_end, cells)
    }

    /// Возвращаем все ячейки, которые находятся в диапазоне строк
    pub fn find_cells_by_range_rows(
        &self,
        start_row: u32,
        end_row: u32,
    ) -> Result<Vec<XLSXSheetCellRead>> {
        let mut cells = self._cells.values().cloned().collect::<Vec<_>>();
        cells.par_sort_by_key(|cell| (cell.row, cell.column));

        HelperCell::find_cells_by_range_rows(start_row, end_row, cells)
    }

    /// Возвращаем все ячейки, которые находятся в диапазоне колонок
    pub fn find_cells_by_range_cols(
        &self,
        start_col: u16,
        end_col: u16,
    ) -> Result<Vec<XLSXSheetCellRead>> {
        let mut cells = self._cells.values().cloned().collect::<Vec<_>>();
        cells.par_sort_by_key(|cell| (cell.row, cell.column));

        HelperCell::find_cells_by_range_cols(start_col, end_col, cells)
    }
}
