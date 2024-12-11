use std::iter::successors;

use crate::structs::coordinate::CellIndex;
use fancy_regex::Regex;
use lazy_static::lazy_static;

/// Преобразование номера колонки в букву.
/// Данный метод используется по обработки формирования данных excel.
/// 1. Принимает номер колонки как `u32`.
/// 2. Создает пустую строку для хранения результата.
/// 3. В цикле:
///    - Уменьшает номер колонки на 1 (так как в Excel колонки начинаются с 1, а не с 0).
///    - Вычисляет остаток от деления на 26 (количество букв в английском алфавите).
///    - Преобразует остаток в соответствующую букву и добавляет её к результату.
///    - Делит номер колонки на 26 для перехода к следующей "цифре" в системе счисления с основанием 26.
/// 4. Переворачивает полученную строку, так как буквы были добавлены в обратном порядке.
pub fn column_number_to_letter(mut column: u16) -> String {
    let mut result = String::new();

    while column > 0 {
        column -= 1; // Уменьшаем на 1, так как в Excel колонки начинаются с 1, а не с 0

        let remainder = (column % 26) as u8;
        result.push((b'A' + remainder) as char);
        column /= 26;
    }

    result.chars().rev().collect()
}

/// Получение координат ячейки в стиле A1
pub fn get_letter_coordinate(row: u32, col: u16) -> String {
    format!("{}{}", column_number_to_letter(col), row)
}

/// Определение формата по типу
pub(crate) fn get_number_format_by_datatype(data_type: &str) -> String {
    match data_type {
        "d" => "mm-dd-yy".to_string(),
        _ => "General".to_string(),
    }
}

fn index_to_alpha(index: u32) -> String {
    assert!(index >= 1, "Index cannot be less than one.");

    const BASE_CHAR_CODE: u32 = 'A' as u32;
    // below code is based on the source code of `radix_fmt`
    successors(Some(index - 1), |index| match index / 26u32 {
        0 => None,
        n => Some(n - 1),
    })
    .map(|v| BASE_CHAR_CODE + (v % 26))
    .collect::<Vec<u32>>()
    .into_iter()
    .rev()
    .map(|v| char::from_u32(v).unwrap())
    .collect()
}

fn alpha_to_index<S>(alpha: S) -> u32
where
    S: AsRef<str>,
{
    const BASE_CHAR_CODE: u32 = 'A' as u32;
    // since we only allow up to three characters, we can use pre-computed
    /// powers of 26 `[26^0, 26^1, 26^2]`
    const POSITIONAL_CONSTANTS: [u32; 3] = [1, 26, 676];

    alpha
        .as_ref()
        .chars()
        .rev()
        .enumerate()
        .map(|(index, v)| {
            let vn = (v as u32 - BASE_CHAR_CODE) + 1;

            // 26u32.pow(index as u32) * vn
            POSITIONAL_CONSTANTS[index] * vn
        })
        .sum::<u32>()
}

pub fn index_from_coordinate<T>(coordinate: T) -> CellIndex
where
    T: AsRef<str>,
{
    lazy_static! {
        static ref RE: Regex = Regex::new(r"((\$)?([A-Z]{1,3}))?((\$)?([0-9]+))?").unwrap();
    }

    let caps = RE.captures(coordinate.as_ref()).ok().flatten();

    caps.map(|v| {
        let col = v.get(3).map(|v| alpha_to_index(v.as_str()) as u16); // col number: [A-Z]{1,3}
        let row = v.get(6).and_then(|v| v.as_str().parse::<u32>().ok()); // row number: [0-9]+

        (row, col)
    })
    .unwrap_or_default()
}
