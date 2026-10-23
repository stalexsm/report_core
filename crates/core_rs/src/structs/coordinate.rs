use serde::Serialize;

use crate::error::{Error, Result};
use crate::utils::{get_letter_coordinate, index_from_coordinate};

pub type CellIndex = (Option<u32>, Option<u16>);

#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Coordinate {
    pub row: u32,
    pub column: u16,
}

impl From<(u32, u16)> for Coordinate {
    #[inline]
    fn from(value: (u32, u16)) -> Self {
        Coordinate::new(value.0, value.1)
    }
}

impl From<(&u32, &u16)> for Coordinate {
    #[inline]
    fn from(value: (&u32, &u16)) -> Self {
        Coordinate::new(*value.0, *value.1)
    }
}

impl TryFrom<String> for Coordinate {
    type Error = Error;

    #[inline]
    fn try_from(value: String) -> Result<Self> {
        Self::try_from(value.as_str())
    }
}

impl TryFrom<&str> for Coordinate {
    type Error = Error;

    #[inline]
    fn try_from(value: &str) -> Result<Self> {
        // CellIndex — это (row, col): u32-строка и u16-колонка.
        let (row, col) = index_from_coordinate(value)?;

        match (row, col) {
            (Some(row), Some(col)) => Ok(Coordinate::new(row, col)),
            _ => Err(Error::invalid_cell_letter(value)),
        }
    }
}

impl From<&Coordinate> for String {
    #[inline]
    fn from(coord: &Coordinate) -> String {
        get_letter_coordinate(coord.row, coord.column)
    }
}

impl Coordinate {
    pub fn new(row: u32, column: u16) -> Self {
        Coordinate { row, column }
    }
}

#[cfg(test)]
mod tests {
    use crate::error::Result;

    use super::*;

    /// Валидная буква ячейки разбирается в координаты.
    #[test]
    fn try_from_valid_letter() -> Result<()> {
        assert_eq!(Coordinate::try_from("A1")?, Coordinate::new(1, 1));
        assert_eq!(Coordinate::try_from("$B$2")?, Coordinate::new(2, 2));
        // Строчные буквы нормализуются к верхнему регистру.
        assert_eq!(Coordinate::try_from("a1")?, Coordinate::new(1, 1));

        Ok(())
    }

    /// Невалидная буква ячейки возвращает ошибку, а не панику.
    #[test]
    fn try_from_invalid_letter() {
        for letter in ["1A", "", "HELLO"] {
            let err = Coordinate::try_from(letter).expect_err(letter);

            assert_eq!(err.to_string(), format!("invalid cell letter: {letter}"));
        }
    }

    /// Координаты за границами листа (MAX_ROW/MAX_COL) отклоняются.
    #[test]
    fn try_from_out_of_bounds() {
        assert!(Coordinate::try_from("A99999999999999999").is_err());
        // XFE = MAX_COL + 1
        assert!(Coordinate::try_from("XFE1").is_err());

        // Граничные значения валидны: XFD = MAX_COL, 1048576 = MAX_ROW.
        assert_eq!(
            Coordinate::try_from("XFD1048576").unwrap(),
            Coordinate::new(1_048_576, 16_384)
        );
    }
}
