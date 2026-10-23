use std::sync::Arc;

use crate::{structs::coordinate::Coordinate, traits::ReadableCell};
use ahash::HashMap;
use anyhow::Result;
use fancy_regex::Regex;
use parking_lot::RwLock;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub fn find_cell_by_coords<T: ReadableCell + Send + Sync>(
    row: u32,
    col: u16,
    cells: Vec<&Arc<RwLock<T>>>,
) -> Result<Option<&Arc<RwLock<T>>>> {
    let cell = cells.par_iter().find_map_first(|cell| {
        let guard = cell.read();
        let coord = guard.get_coordinate();

        if coord.row == row && coord.column == col {
            Some(*cell)
        } else {
            None
        }
    });

    Ok(cell)
}

pub fn find_value_by_coords<T: ReadableCell + Send + Sync>(
    row: u32,
    col: u16,
    cells: Vec<&Arc<RwLock<T>>>,
) -> Result<Option<String>> {
    let cell = cells.par_iter().find_map_first(|cell| {
        let guard = cell.read();
        let coord = guard.get_coordinate();

        if coord.row == row && coord.column == col {
            Some(guard.get_value())
        } else {
            None
        }
    });

    Ok(cell)
}

pub fn find_cell_by_regex<T: ReadableCell + Send + Sync>(
    regex: String,
    cells: Vec<&Arc<RwLock<T>>>,
) -> Result<Option<&Arc<RwLock<T>>>> {
    let re = Regex::new(&format!("(?i){}", regex))?;

    let cell = cells.par_iter().find_map_first(|cell| {
        let v = cell.read().get_value();
        if re.is_match(&v).unwrap_or(false) {
            Some(*cell)
        } else {
            None
        }
    });

    Ok(cell)
}

pub fn find_cell_by_str<T: ReadableCell + Send + Sync>(
    value: String,
    cells: Vec<&Arc<RwLock<T>>>,
) -> Result<Option<&Arc<RwLock<T>>>> {
    let cell = cells.par_iter().find_map_first(|cell| {
        if cell.read().get_value().trim() == value.trim() {
            Some(*cell)
        } else {
            None
        }
    });

    Ok(cell)
}

pub fn find_cell_by_letter<T: ReadableCell + Send + Sync>(
    letter: String,
    cells: Vec<&Arc<RwLock<T>>>,
) -> Result<Option<&Arc<RwLock<T>>>> {
    let letter_coord = &Coordinate::try_from(letter.as_str())?;

    let cell = cells.par_iter().find_map_first(|cell| {
        let guard = cell.read();
        let coord = guard.get_coordinate();
        if coord == letter_coord {
            Some(*cell)
        } else {
            None
        }
    });

    Ok(cell)
}

pub fn find_cells_by_regex<T: ReadableCell + Send + Sync>(
    regex: String,
    cells: Vec<&Arc<RwLock<T>>>,
) -> Result<Vec<&Arc<RwLock<T>>>> {
    let re = Regex::new(&format!("(?i){}", regex))?;

    Ok(cells
        .par_iter()
        .filter_map(|cell| {
            let v = cell.read().get_value();
            if re.is_match(&v).unwrap_or(false) {
                Some(*cell)
            } else {
                None
            }
        })
        .collect())
}

pub fn find_cells_by_str<T: ReadableCell + Send + Sync>(
    value: String,
    cells: Vec<&Arc<RwLock<T>>>,
) -> Result<Vec<&Arc<RwLock<T>>>> {
    Ok(cells
        .par_iter()
        .filter_map(|cell| {
            if cell.read().get_value().trim() == value.trim() {
                Some(*cell)
            } else {
                None
            }
        })
        .collect())
}

pub fn find_cells_for_rows_by_regex<T: ReadableCell + Send + Sync>(
    regex: String,
    col_stop: u16,
    cells: Vec<&Arc<RwLock<T>>>,
) -> Result<Vec<&Arc<RwLock<T>>>> {
    let re = Regex::new(&format!("(?i){}", regex))?;

    Ok(cells
        .par_iter()
        .filter_map(|cell| {
            let guard = cell.read();
            if guard.get_coordinate().column <= col_stop {
                let v = guard.get_value();
                if re.is_match(&v).unwrap_or(false) {
                    Some(*cell)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect())
}

pub fn find_cells_for_cols_by_regex<T: ReadableCell + Send + Sync>(
    regex: String,
    row_stop: u32,
    cells: Vec<&Arc<RwLock<T>>>,
) -> Result<Vec<&Arc<RwLock<T>>>> {
    let re = Regex::new(&format!("(?i){}", regex))?;

    Ok(cells
        .par_iter()
        .filter_map(|cell| {
            let guard = cell.read();
            if guard.get_coordinate().row <= row_stop {
                let v = guard.get_value();
                if re.is_match(&v).unwrap_or(false) {
                    Some(*cell)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect())
}

pub fn find_cells_multi_regex<T: ReadableCell + Send + Sync>(
    before_regex: String,
    after_regex: String,
    cells: Vec<&Arc<RwLock<T>>>,
) -> Result<Vec<&Arc<RwLock<T>>>> {
    let before_regex = Regex::new(&format!("(?i){}", before_regex))?;
    let after_regex = Regex::new(&format!("(?i){}", after_regex))?;

    let mut b = false;
    Ok(cells
        .iter()
        .filter_map(|cell| {
            let v = cell.read().get_value();
            if ((before_regex.is_match(&v).unwrap_or(false)) && !b)
                || ((after_regex.is_match(&v).unwrap_or(false)) && b)
            {
                b = !b;
                Some(*cell)
            } else {
                None
            }
        })
        .collect())
}

pub fn find_cells_between_regex<T: ReadableCell + Send + Sync>(
    before_regex: String,
    after_regex: String,
    cells: Vec<&Arc<RwLock<T>>>,
) -> Result<Vec<&Arc<RwLock<T>>>> {
    let before_regex = Regex::new(&format!("(?i){}", before_regex))?;
    let after_regex = Regex::new(&format!("(?i){}", after_regex))?;

    let mut b = false;
    let rows_idx = cells
        .iter()
        .filter_map(|cell| {
            let guard = cell.read();
            let v = guard.get_value();
            if ((before_regex.is_match(&v).unwrap_or(false)) && !b)
                || ((after_regex.is_match(&v).unwrap_or(false)) && b)
            {
                b = !b;
                Some(guard.get_coordinate().row)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    Ok(cells
        .par_iter()
        .filter_map(|cell| {
            let row = cell.read().get_coordinate().row;
            if rows_idx.len() >= 2 {
                if row >= rows_idx[0] && row <= rows_idx[1] {
                    Some(*cell)
                } else {
                    None
                }
            } else if rows_idx.len() == 1 {
                if row >= rows_idx[0] {
                    Some(*cell)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect())
}

pub fn find_cells_range_rows<T: ReadableCell + Send + Sync>(
    start_row: u32,
    end_row: u32,
    cells: Vec<&Arc<RwLock<T>>>,
) -> Result<Vec<&Arc<RwLock<T>>>> {
    Ok(cells
        .par_iter()
        .filter_map(|cell| {
            let guard = cell.read();
            let coord = guard.get_coordinate();
            if coord.row >= start_row && coord.row <= end_row {
                Some(*cell)
            } else {
                None
            }
        })
        .collect())
}

pub fn find_cells_range_cols<T: ReadableCell + Send + Sync>(
    start_col: u16,
    end_col: u16,
    cells: Vec<&Arc<RwLock<T>>>,
) -> Result<Vec<&Arc<RwLock<T>>>> {
    Ok(cells
        .par_iter()
        .filter_map(|cell| {
            let guard = cell.read();
            let coord = guard.get_coordinate();
            if coord.column >= start_col && coord.column <= end_col {
                Some(*cell)
            } else {
                None
            }
        })
        .collect())
}

/// Индекс «координата → значение» по коллекции ячеек — для детерминированного
/// поиска значений в порядке запрошенных строк/колонок.
fn index_values_by_coords<T: ReadableCell + Send + Sync>(
    cells: &[&Arc<RwLock<T>>],
) -> HashMap<(u32, u16), String> {
    cells
        .par_iter()
        .map(|cell| {
            let guard = cell.read();
            let coord = guard.get_coordinate();

            ((coord.row, coord.column), guard.get_value())
        })
        .collect()
}

/// Значения по колонке и списку строк — строго в порядке запрошенных строк.
/// Отсутствующая ячейка даёт пустую строку: длина результата всегда равна
/// длине запроса, поэтому позиционный zip с заголовками не искажает отчёт.
pub fn find_values_by_col_rows<T: ReadableCell + Send + Sync>(
    col: u16,
    rows: Vec<u32>,
    cells: Vec<&Arc<RwLock<T>>>,
) -> Result<Vec<String>> {
    let index = index_values_by_coords(&cells);

    Ok(rows
        .into_iter()
        .map(|row| index.get(&(row, col)).cloned().unwrap_or_default())
        .collect())
}

/// Значения по строке и списку колонок — строго в порядке запрошенных колонок.
/// Отсутствующая ячейка даёт пустую строку: длина результата всегда равна
/// длине запроса, поэтому позиционный zip с заголовками не искажает отчёт.
pub fn find_values_by_row_cols<T: ReadableCell + Send + Sync>(
    row: u32,
    cols: Vec<u16>,
    cells: Vec<&Arc<RwLock<T>>>,
) -> Result<Vec<String>> {
    let index = index_values_by_coords(&cells);

    Ok(cols
        .into_iter()
        .map(|col| index.get(&(row, col)).cloned().unwrap_or_default())
        .collect())
}
