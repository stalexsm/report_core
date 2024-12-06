use anyhow::{bail, Result};
use rayon::prelude::*;

use crate::reader::{cell::XLSXSheetCellRead, sheet::XLSXSheetRead};

#[derive(Debug, Clone)]
pub struct HelperSheet {
    pub sheets: Vec<XLSXSheetRead>,
}

impl HelperSheet {
    pub fn new(sheets: Vec<XLSXSheetRead>) -> Self {
        Self { sheets }
    }

    /// Поиск листа по наименованию
    pub fn find_sheet_by_name(&self, name: &str) -> Result<Option<XLSXSheetRead>> {
        let sheet = self
            .sheets
            .par_iter()
            .find_first(|s| s.name == name)
            .cloned();

        Ok(sheet)
    }

    /// Поиск листа по шаблону regex
    pub fn find_sheet_by_pattern(&self, pattern: &str) -> Result<Option<XLSXSheetRead>> {
        let re = regex::Regex::new(pattern).unwrap();

        let cell = self
            .sheets
            .par_iter()
            .find_first(|s| re.is_match(&s.name))
            .cloned();

        Ok(cell)
    }

    /// Поиск листа по индексу
    pub fn find_sheet_by_index(&self, idx: i32) -> Result<Option<XLSXSheetRead>> {
        let cell = self
            .sheets
            .par_iter()
            .find_first(|s| s.index == idx)
            .cloned();

        Ok(cell)
    }

    /// Получение списка листов, исключая передаваесый список.
    pub fn get_sheets_without_names(&self, name_list: Vec<String>) -> Result<Vec<XLSXSheetRead>> {
        let cells = self
            .sheets
            .par_iter()
            .filter(|c| !name_list.contains(&c.name))
            .cloned()
            .collect();

        Ok(cells)
    }

    /// Получение списка листов, передаваемого списка листов .
    pub fn get_sheets_with_names(&self, name_list: Vec<String>) -> Result<Vec<XLSXSheetRead>> {
        let cells = self
            .sheets
            .par_iter()
            .filter(|c| name_list.contains(&c.name))
            .cloned()
            .collect();

        Ok(cells)
    }
}

#[derive(Debug, Clone)]
pub struct HelperCell;

impl HelperCell {
    /// Поиск ячейки по шаблону
    pub fn find_cell_by_pattern_regex(
        pattern: &str,
        cells: Vec<XLSXSheetCellRead>,
    ) -> Result<Option<XLSXSheetCellRead>> {
        let re = regex::Regex::new(&regex::escape(pattern))?;

        let cell = cells.par_iter().find_map_first(|cell| {
            if re.is_match(&cell.value.get_value_str()) {
                Some(cell.clone())
            } else {
                None
            }
        });

        Ok(cell)
    }

    // Поиск ячеек по шаблону
    pub fn find_cells_by_pattern_regex(
        pattern: &str,
        cells: Vec<XLSXSheetCellRead>,
    ) -> Result<Vec<XLSXSheetCellRead>> {
        let re = regex::Regex::new(&regex::escape(pattern))?;

        let cells = cells
            .par_iter()
            .filter_map(|cell| {
                if re.is_match(&cell.value.get_value_str()) {
                    Some(cell.clone())
                } else {
                    None
                }
            })
            .collect();

        Ok(cells)
    }

    /// Поиск ячеек колонок для строк которые соответствуют патерну
    pub fn find_cells_for_rows_pattern_regex(
        pattern: &str,
        cells: Vec<XLSXSheetCellRead>,
        col_stop: Option<u16>,
    ) -> Result<Vec<XLSXSheetCellRead>> {
        let re = regex::Regex::new(&regex::escape(pattern))?;

        let cells = cells
            .par_iter()
            .filter_map(|cell| {
                if re.is_match(&cell.value.get_value_str()) {
                    if let Some(col_stop) = col_stop {
                        if cell.column >= col_stop {
                            return None;
                        }
                    }
                    Some(cell.clone())
                } else {
                    None
                }
            })
            .collect();

        Ok(cells)
    }

    /// Поиск ячеек строк для колонок которые соответствуют патерну
    pub fn find_cells_for_cols_pattern_regex(
        pattern: &str,
        cells: Vec<XLSXSheetCellRead>,
        row_stop: Option<u32>,
    ) -> Result<Vec<XLSXSheetCellRead>> {
        let re = regex::Regex::new(&regex::escape(pattern))?;

        let cells = cells
            .par_iter()
            .filter_map(|cell| {
                if re.is_match(&cell.value.get_value_str()) {
                    if let Some(row_stop) = row_stop {
                        if cell.row >= row_stop {
                            return None;
                        }
                    }
                    Some(cell.clone())
                } else {
                    None
                }
            })
            .collect();

        Ok(cells)
    }

    /// Поиск ячеек с помощью ИЛИ ячейки по патернам
    pub fn find_cells_multi_pattern_regex(
        pattern_1: &str,
        pattern_2: &str,
        cells: Vec<XLSXSheetCellRead>,
    ) -> Result<Vec<XLSXSheetCellRead>> {
        let re1 = regex::Regex::new(&regex::escape(pattern_1))?;
        let re2 = regex::Regex::new(&regex::escape(pattern_2))?;

        let mut b = false;
        let cells = cells
            .iter()
            .filter_map(|cell| {
                let val = cell.value.get_value_str();

                if (re1.is_match(&val) && !b) || (re2.is_match(&val) && b) {
                    b = !b;
                    Some(cell.clone())
                } else {
                    None
                }
            })
            .collect();

        Ok(cells)
    }

    /// Поиск ячейки по буквенной координате A1 (cell)
    pub fn find_cell_by_cell(
        cell: &str,
        cells: Vec<XLSXSheetCellRead>,
    ) -> Result<Option<XLSXSheetCellRead>> {
        let found_cell = cells.par_iter().find_map_first(|c| {
            if c.cell == cell {
                Some(c.clone())
            } else {
                None
            }
        });

        Ok(found_cell)
    }

    /// Поиск ячейки по координате
    pub fn find_cell_by_coords(
        row: u32,
        col: u16,
        cells: Vec<XLSXSheetCellRead>,
    ) -> Result<Option<XLSXSheetCellRead>> {
        let found_cell = cells.par_iter().find_map_first(|cell| {
            if cell.row == row && cell.column == col {
                Some(cell.clone())
            } else {
                None
            }
        });

        Ok(found_cell)
    }

    /// Поиск ячеек между шаьлонами
    pub fn find_cells_between_patterns(
        pattern_start: &str,
        pattern_end: &str,
        cells: Vec<XLSXSheetCellRead>,
    ) -> Result<Vec<XLSXSheetCellRead>> {
        let re_start = regex::Regex::new(&regex::escape(pattern_start))?;
        let re_end = regex::Regex::new(&regex::escape(pattern_end))?;

        let mut b = false;
        let rows_idx = cells
            .iter()
            .filter_map(|cell| {
                let val = cell.value.get_value_str();

                if (re_start.is_match(&val) && !b) || (re_end.is_match(&val) && b) {
                    b = !b;
                    Some(cell.row)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let cells = cells
            .par_iter()
            .filter_map(|cell| {
                if rows_idx.len() >= 2 {
                    if cell.row >= rows_idx[0] && cell.row <= rows_idx[1] {
                        Some(cell.clone())
                    } else {
                        None
                    }
                } else if rows_idx.len() == 1 {
                    if cell.row >= rows_idx[0] {
                        Some(cell.clone())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        Ok(cells)
    }

    /// Получить список всех ячеек в заданном диапазоне.
    pub fn iter_cells(
        min_row: u32,
        max_row: u32,
        min_col: u16,
        max_col: u16,
        cells: Vec<XLSXSheetCellRead>,
    ) -> Result<Vec<XLSXSheetCellRead>> {
        if min_row > max_row || min_col > max_col {
            bail!("The coordinates of the cells were incorrectly transmitted");
        }

        let cells = cells
            .par_iter()
            .filter_map(|cell| {
                if cell.row >= min_row
                    && cell.row <= max_row
                    && cell.column >= min_col
                    && cell.column <= max_col
                {
                    Some(cell.clone())
                } else {
                    None
                }
            })
            .collect();

        Ok(cells)
    }

    /// Возвращаем все ячейки, которые находятся в диапазоне строк
    pub fn find_cells_by_range_rows(
        start_row: u32,
        end_row: u32,
        cells: Vec<XLSXSheetCellRead>,
    ) -> Result<Vec<XLSXSheetCellRead>> {
        let cells = cells
            .par_iter()
            .filter_map(|cell| {
                if cell.row >= start_row && cell.row <= end_row {
                    Some(cell.clone())
                } else {
                    None
                }
            })
            .collect();

        Ok(cells)
    }

    /// Возвращаем все ячейки, которые находятся в диапазоне колонок
    pub fn find_cells_by_range_cols(
        start_col: u16,
        end_col: u16,
        cells: Vec<XLSXSheetCellRead>,
    ) -> Result<Vec<XLSXSheetCellRead>> {
        let cells = cells
            .par_iter()
            .filter_map(|cell| {
                if cell.column >= start_col && cell.column <= end_col {
                    Some(cell.clone())
                } else {
                    None
                }
            })
            .collect();

        Ok(cells)
    }
}
