use serde::Serialize;

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

impl From<String> for Coordinate {
    #[inline]
    fn from(value: String) -> Self {
        let str_ref: &str = value.as_ref();

        str_ref.into()
    }
}

impl From<&str> for Coordinate {
    #[inline]
    fn from(value: &str) -> Self {
        let (col, row) = index_from_coordinate(value.to_uppercase());

        Coordinate::new(col.unwrap(), row.unwrap())
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
