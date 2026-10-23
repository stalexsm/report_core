use std::sync::Arc;

use crate::{
    error::Result,
    regex::{compile_case_insensitive, is_match},
    structs::coordinate::Coordinate,
    traits::ReadableCell,
};
use ahash::HashMap;
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
    // Тот же скан, что у find_cell_by_coords: отличается только извлечением
    // значения найденной ячейки
    let cell = find_cell_by_coords(row, col, cells)?;

    Ok(cell.map(|cell| cell.read().get_value()))
}

pub fn find_cell_by_regex<T: ReadableCell + Send + Sync>(
    regex: String,
    cells: Vec<&Arc<RwLock<T>>>,
) -> Result<Option<&Arc<RwLock<T>>>> {
    let re = compile_case_insensitive(&regex)?;

    // Совпадения собираются в Vec с сохранением порядка: ошибка выполнения
    // шаблона (например, исчерпанный лимит обратной трассировки) не глотается,
    // а уходит вызывающему коду
    let cell = cells
        .par_iter()
        .map(|cell| {
            let v = cell.read().get_value();
            is_match(&re, &v).map(|hit| hit.then_some(*cell))
        })
        .collect::<Result<Vec<Option<_>>>>()?
        .into_iter()
        .flatten()
        .next();

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
    let re = compile_case_insensitive(&regex)?;

    Ok(cells
        .par_iter()
        .map(|cell| {
            let v = cell.read().get_value();
            is_match(&re, &v).map(|hit| hit.then_some(*cell))
        })
        .collect::<Result<Vec<Option<_>>>>()?
        .into_iter()
        .flatten()
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
    let re = compile_case_insensitive(&regex)?;

    Ok(cells
        .par_iter()
        .map(|cell| {
            let guard = cell.read();
            if guard.get_coordinate().column > col_stop {
                return Ok(None);
            }

            let v = guard.get_value();
            is_match(&re, &v).map(|hit| hit.then_some(*cell))
        })
        .collect::<Result<Vec<Option<_>>>>()?
        .into_iter()
        .flatten()
        .collect())
}

pub fn find_cells_for_cols_by_regex<T: ReadableCell + Send + Sync>(
    regex: String,
    row_stop: u32,
    cells: Vec<&Arc<RwLock<T>>>,
) -> Result<Vec<&Arc<RwLock<T>>>> {
    let re = compile_case_insensitive(&regex)?;

    Ok(cells
        .par_iter()
        .map(|cell| {
            let guard = cell.read();
            if guard.get_coordinate().row > row_stop {
                return Ok(None);
            }

            let v = guard.get_value();
            is_match(&re, &v).map(|hit| hit.then_some(*cell))
        })
        .collect::<Result<Vec<Option<_>>>>()?
        .into_iter()
        .flatten()
        .collect())
}

pub fn find_cells_multi_regex<T: ReadableCell + Send + Sync>(
    before_regex: String,
    after_regex: String,
    cells: Vec<&Arc<RwLock<T>>>,
) -> Result<Vec<&Arc<RwLock<T>>>> {
    let before_regex = compile_case_insensitive(&before_regex)?;
    let after_regex = compile_case_insensitive(&after_regex)?;

    let mut b = false;
    let mut matched_cells = Vec::new();

    for cell in cells {
        let v = cell.read().get_value();
        // Оба шаблона проверяются всегда: ошибка выполнения любого из них
        // не глотается, а уходит вызывающему коду
        let before_hit = is_match(&before_regex, &v)?;
        let after_hit = is_match(&after_regex, &v)?;

        if (before_hit && !b) || (after_hit && b) {
            b = !b;
            matched_cells.push(cell);
        }
    }

    Ok(matched_cells)
}

pub fn find_cells_between_regex<T: ReadableCell + Send + Sync>(
    before_regex: String,
    after_regex: String,
    cells: Vec<&Arc<RwLock<T>>>,
) -> Result<Vec<&Arc<RwLock<T>>>> {
    let before_regex = compile_case_insensitive(&before_regex)?;
    let after_regex = compile_case_insensitive(&after_regex)?;

    let mut b = false;
    let mut rows_idx = Vec::new();

    for cell in cells.iter() {
        let guard = cell.read();
        let v = guard.get_value();
        let before_hit = is_match(&before_regex, &v)?;
        let after_hit = is_match(&after_regex, &v)?;

        if (before_hit && !b) || (after_hit && b) {
            b = !b;
            rows_idx.push(guard.get_coordinate().row);
        }
    }

    Ok(cells
        .par_iter()
        .filter_map(|cell| {
            let row = cell.read().get_coordinate().row;
            match rows_idx.len() {
                0 => None,
                1 if row >= rows_idx[0] => Some(*cell),
                _ if row >= rows_idx[0] && row <= rows_idx[1] => Some(*cell),
                _ => None,
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

#[cfg(test)]
mod regex_limit_tests {
    use super::*;
    use crate::structs::cell::Cell;
    use crate::structs::coordinate::Coordinate;
    use parking_lot::RwLock;

    fn one_cell(value: &str) -> Vec<Arc<RwLock<Cell>>> {
        vec![Arc::new(RwLock::new(Cell::new(
            Coordinate::new(1, 1),
            Some(value),
        )))]
    }

    /// Шаблон длиннее предела — ошибка, а не молчаливый поиск по куску шаблона.
    #[test]
    fn pattern_length_limit() {
        let cells = one_cell("v");
        let cells: Vec<_> = cells.iter().collect();
        let pattern = "a".repeat(crate::regex::MAX_PATTERN_LEN + 1);

        let err = find_cells_by_regex(pattern.clone(), cells.clone()).unwrap_err();
        assert!(err.to_string().contains("exceeds the maximum"));

        let err = find_cell_by_regex(pattern, cells).unwrap_err();
        assert!(err.to_string().contains("exceeds the maximum"));
    }

    /// Катастрофический шаблон: исчерпание лимита обратной трассировки —
    /// ошибка выполнения, а не «не совпало». Атомарная группа `(?>c)` прячет
    /// литерал от префильтров движка, поэтому перебор действительно
    /// выполняется и упирается в дефолтный лимит fancy-regex.
    #[test]
    fn backtrack_limit_is_error_not_silent_miss() {
        let cells = one_cell(&"ab".repeat(56));
        let cells: Vec<_> = cells.iter().collect();

        let result = find_cell_by_regex("(a|b|ab)*(?>c)".to_string(), cells);

        let err = result.unwrap_err();
        assert!(
            err.to_string().contains("regex execution failed"),
            "получено: {err}"
        );
    }

    /// Повторная компиляция того же шаблона переиспользует результат кэша.
    #[test]
    fn compiled_regex_reused_from_cache() {
        let re1 = crate::regex::compile_case_insensitive("итого").unwrap();
        let re2 = crate::regex::compile_case_insensitive("итого").unwrap();

        assert!(Arc::ptr_eq(&re1, &re2), "кэш не переиспользуется");

        // Валидация и учёт регистра не изменились
        assert!(re1.is_match("ИТОГО").unwrap());
        assert!(!re1.is_match("итогз").unwrap());
    }

    /// Невалидный шаблон — ошибка компиляции с текстом шаблона.
    #[test]
    fn invalid_pattern_reports_pattern() {
        let err = crate::regex::compile_case_insensitive("(").unwrap_err();
        assert!(err.to_string().contains("invalid regex"), "получено: {err}");
    }
}
