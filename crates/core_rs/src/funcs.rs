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
    // Образец обрезается один раз, сравнение — без аллокации на ячейку
    let needle = value.trim();

    let cell = cells
        .par_iter()
        .find_map_first(|cell| cell.read().value_eq_str(needle).then_some(*cell));

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
    let needle = value.trim();

    Ok(cells
        .par_iter()
        .filter_map(|cell| cell.read().value_eq_str(needle).then_some(*cell))
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
/// поиска значений в порядке запрошенных строк/колонок. Сборка последовательная:
/// при дублях координат (возможны у прямых вызовов с ручной коллекцией)
/// побеждает первое вхождение — результат стабилен между запусками.
fn index_values_by_coords<T: ReadableCell + Send + Sync>(
    cells: &[&Arc<RwLock<T>>],
) -> HashMap<(u32, u16), String> {
    cells.iter().fold(HashMap::default(), |mut index, cell| {
        let guard = cell.read();
        let coord = guard.get_coordinate();

        index
            .entry((coord.row, coord.column))
            .or_insert_with(|| guard.get_value());

        index
    })
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::structs::cell::Cell;

    /// Дубли координат в ручной коллекции: побеждает первое вхождение,
    /// результат стабилен между запусками процесса.
    #[test]
    pub fn find_values_duplicate_coords_first_wins() {
        let first = Arc::new(RwLock::new(Cell::new(Coordinate::new(1, 1), Some("first"))));
        let second = Arc::new(RwLock::new(Cell::new(
            Coordinate::new(1, 1),
            Some("second"),
        )));
        let cells = vec![&first, &second];

        let values = find_values_by_col_rows(1, vec![1], cells.clone()).unwrap();

        assert_eq!(values, vec!["first"]);

        let values = find_values_by_row_cols(1, vec![1], cells).unwrap();

        assert_eq!(values, vec!["first"]);
    }

    /// Поиск по строке работает по отрисованному значению для всех типов:
    /// строки — без аллокации, числа/bool — как раньше через отрисовку.
    #[test]
    pub fn find_cells_by_str_matches_rendered_values() {
        let text = Arc::new(RwLock::new(Cell::new(
            Coordinate::new(1, 1),
            Some("Итого  "),
        )));
        let number = Arc::new(RwLock::new(Cell::new(Coordinate::new(1, 2), Some("34.8"))));
        let flag = Arc::new(RwLock::new(Cell::new(Coordinate::new(1, 3), Some("TRUE"))));
        let cells = vec![&text, &number, &flag];

        assert_eq!(
            find_cells_by_str("итого".into(), cells.clone())
                .unwrap()
                .len(),
            0,
            "сравнение чувствительно к регистру"
        );
        assert_eq!(
            find_cells_by_str("Итого".into(), cells.clone())
                .unwrap()
                .len(),
            1
        );
        assert_eq!(
            find_cells_by_str("34.8".into(), cells.clone())
                .unwrap()
                .len(),
            1
        );
        assert_eq!(find_cells_by_str("true".into(), cells).unwrap().len(), 1);
    }
}
