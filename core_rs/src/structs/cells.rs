use rayon::{
    iter::{IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator},
    slice::ParallelSliceMut,
};
use serde::Serialize;
use std::collections::HashMap;

use crate::{datatype::CellValue, MAX_COL, MAX_ROW};

use super::{cell::Cell, coordinate::Coordinate, range::Range};

#[derive(Clone, Default, Debug, Serialize)]
pub struct Cells {
    map: HashMap<(u32, u16), Cell>,
    default_cell_value: CellValue,
}

impl Cells {
    /// Метод для получения коллекции ячеек
    #[inline]
    pub fn get_collection(&self) -> Vec<&Cell> {
        self.map.values().collect::<Vec<_>>()
    }

    /// Метод для получения коллекции ячеек
    #[inline]
    pub fn get_collection_sorted(&self) -> Vec<&Cell> {
        let mut cells = self.map.values().collect::<Vec<_>>();
        cells.par_sort_by_key(|cell| {
            let cord = cell.get_coordinate();

            (cord.row, cord.column)
        });

        cells
    }

    /// Метод для получения ячейки по координатам
    #[inline]
    pub fn get_cell<T>(&self, coordinate: T) -> Option<&Cell>
    where
        T: Into<Coordinate>,
    {
        let Coordinate { row, column } = coordinate.into();
        self.map.get(&(row, column))
    }

    #[inline]
    pub fn get_cell_by_letter(&self, letter: &str) -> Option<&Cell> {
        let Coordinate { row, column } = Coordinate::from(letter);

        self.map.get(&(row, column))
    }

    #[inline]
    pub fn get_cell_by_letter_mut(&mut self, letter: &str) -> Option<&mut Cell> {
        let Coordinate { row, column } = Coordinate::from(letter);

        self.map.get_mut(&(row, column))
    }

    /// Метод для получения мутабельной ячейки по координатам
    #[inline]
    pub fn get_cell_mut<T>(&mut self, coordinate: T) -> Option<&mut Cell>
    where
        T: Into<Coordinate>,
    {
        let Coordinate { row, column } = coordinate.into();
        self.map.get_mut(&(row, column))
    }

    /// Метод для получения значения ячейки по координатам
    #[inline]
    pub fn get_cell_value<T>(&self, coordinate: T) -> String
    where
        T: Into<Coordinate>,
    {
        let Coordinate { row, column } = coordinate.into();
        self.map
            .get(&(row, column))
            .map(|c| c.get_value())
            .unwrap_or(self.default_cell_value.get_value())
    }

    #[inline]
    pub fn get_cell_collection_by_range(&self, range: &Range) -> impl Iterator<Item = &Cell> {
        let Range {
            start_col,
            start_row,
            end_col,
            end_row,
        } = range;

        let start_row = start_row.unwrap_or(1);
        let end_row = end_row.unwrap_or(MAX_ROW);
        let start_col = start_col.unwrap_or(1);
        let end_col = end_col.unwrap_or(MAX_COL);

        let mut cells = self
            .map
            .par_iter()
            .filter(|(_, cell)| {
                let coord = cell.get_coordinate();
                coord.row >= start_row
                    && coord.row <= end_row
                    && coord.column >= start_col
                    && coord.column <= end_col
            })
            .map(|(_, cell)| cell)
            .collect::<Vec<_>>();

        cells.par_sort_by_key(|cell| {
            let cord = cell.get_coordinate();

            (cord.row, cord.column)
        });

        cells.into_iter()
    }

    #[inline]
    pub fn get_cell_collection_by_range_mut(
        &mut self,
        range: &Range,
    ) -> impl Iterator<Item = &mut Cell> {
        let Range {
            start_col,
            start_row,
            end_col,
            end_row,
        } = range;

        let start_row = start_row.unwrap_or(1);
        let end_row = end_row.unwrap_or(MAX_ROW);
        let start_col = start_col.unwrap_or(1);
        let end_col = end_col.unwrap_or(MAX_COL);

        let mut cells = self
            .map
            .par_iter_mut()
            .filter(|(_, cell)| {
                let coord = cell.get_coordinate();
                coord.row >= start_row
                    && coord.row <= end_row
                    && coord.column >= start_col
                    && coord.column <= end_col
            })
            .map(|(_, cell)| cell)
            .collect::<Vec<_>>();

        cells.par_sort_by_key(|cell| {
            let cord = cell.get_coordinate();

            (cord.row, cord.column)
        });

        cells.into_iter()
    }

    #[inline]
    pub fn write_cell<T>(&mut self, coordinate: T, value: &str) -> &mut Cell
    where
        T: Into<Coordinate>,
    {
        let Coordinate { row, column } = coordinate.into();

        let cell = self
            .map
            .entry((row, column))
            .or_insert_with(|| Cell::new(Coordinate::new(row, column), Some(value)));

        cell.set_value(value);

        cell
    }

    #[inline]
    pub fn delete_cols(&mut self, idx: u16, amount: u16) {
        self.map.retain(|_, cell| {
            let Coordinate { column, .. } = cell.get_coordinate();
            *column < idx || *column >= idx + amount
        });

        for (_, cell) in self.map.iter_mut() {
            let Coordinate { column, .. } = cell.get_coordinate();
            if *column > idx {
                cell.set_coordinate(Coordinate {
                    row: cell.get_coordinate().row,
                    column: column - amount,
                });
            }
        }
    }

    #[inline]
    pub fn delete_rows(&mut self, idx: u32, amount: u32) {
        self.map.retain(|_, cell| {
            let Coordinate { row, .. } = cell.get_coordinate();
            *row < idx || *row >= idx + amount
        });

        for (_, cell) in self.map.iter_mut() {
            let Coordinate { row, .. } = cell.get_coordinate();
            if *row > idx {
                cell.set_coordinate(Coordinate {
                    row: row - amount,
                    column: cell.get_coordinate().column,
                });
            }
        }
    }
}
