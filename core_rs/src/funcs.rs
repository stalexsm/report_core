use std::sync::Arc;

use crate::{structs::coordinate::Coordinate, traits::ReadableCell};
use anyhow::Result;
use fancy_regex::{escape, Regex};
use parking_lot::RwLock;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub fn find_cell_by_regex<'a, T: ReadableCell + Send + Sync>(
    pattern: &'a str,
    cells: Vec<&'a Arc<RwLock<T>>>,
) -> Result<Option<&'a Arc<RwLock<T>>>> {
    let re = Regex::new(&escape(pattern))?;

    let cell = cells.par_iter().find_map_first(|cell| {
        if re.is_match(&cell.read().get_value()).unwrap_or(false) {
            Some(*cell)
        } else {
            None
        }
    });

    Ok(cell)
}

pub fn find_cell_by_letter<'a, T: ReadableCell + Send + Sync>(
    letter: &'a str,
    cells: Vec<&'a Arc<RwLock<T>>>,
) -> Result<Option<&'a Arc<RwLock<T>>>> {
    let letter_coord = &Coordinate::from(letter);

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

pub fn find_cells_by_regex<'a, T: ReadableCell + Send + Sync>(
    regex: &str,
    cells: Vec<&'a Arc<RwLock<T>>>,
) -> Result<Vec<&'a Arc<RwLock<T>>>> {
    let re = Regex::new(regex)?;
    Ok(cells
        .par_iter()
        .filter_map(|cell| {
            if re.is_match(&cell.read().get_value()).unwrap_or(false) {
                Some(*cell)
            } else {
                None
            }
        })
        .collect())
}

pub fn find_cells_for_rows_by_regex<'a, T: ReadableCell + Send + Sync>(
    regex: &str,
    col_stop: u16,
    cells: Vec<&'a Arc<RwLock<T>>>,
) -> Result<Vec<&'a Arc<RwLock<T>>>> {
    let re = Regex::new(regex)?;
    Ok(cells
        .par_iter()
        .filter_map(|cell| {
            let guard = cell.read();
            if guard.get_coordinate().column <= col_stop {
                if re.is_match(&guard.get_value()).unwrap_or(false) {
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

pub fn find_cells_for_cols_by_regex<'a, T: ReadableCell + Send + Sync>(
    regex: &str,
    row_stop: u32,
    cells: Vec<&'a Arc<RwLock<T>>>,
) -> Result<Vec<&'a Arc<RwLock<T>>>> {
    let re = Regex::new(regex)?;
    Ok(cells
        .par_iter()
        .filter_map(|cell| {
            let guard = cell.read();
            if guard.get_coordinate().row <= row_stop {
                if re.is_match(&guard.get_value()).unwrap_or(false) {
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

pub fn find_cells_multi_regex<'a, T: ReadableCell + Send + Sync>(
    before_regex: &str,
    after_regex: &str,
    cells: Vec<&'a Arc<RwLock<T>>>,
) -> Result<Vec<&'a Arc<RwLock<T>>>> {
    let before_regex = Regex::new(before_regex)?;
    let after_regex = Regex::new(after_regex)?;

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

pub fn find_cells_between_regex<'a, T: ReadableCell + Send + Sync>(
    before_regex: &str,
    after_regex: &str,
    cells: Vec<&'a Arc<RwLock<T>>>,
) -> Result<Vec<&'a Arc<RwLock<T>>>> {
    let before_regex = Regex::new(before_regex)?;
    let after_regex = Regex::new(after_regex)?;

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

pub fn find_cells_range_rows<'a, T: ReadableCell + Send + Sync>(
    start_row: u32,
    end_row: u32,
    cells: Vec<&'a Arc<RwLock<T>>>,
) -> Result<Vec<&'a Arc<RwLock<T>>>> {
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

pub fn find_cells_range_cols<'a, T: ReadableCell + Send + Sync>(
    start_col: u16,
    end_col: u16,
    cells: Vec<&'a Arc<RwLock<T>>>,
) -> Result<Vec<&'a Arc<RwLock<T>>>> {
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
