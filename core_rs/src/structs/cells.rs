use anyhow::Result;
use fancy_regex::Regex;
use parking_lot::RwLock;
use rayon::{
    iter::{IntoParallelRefIterator, ParallelIterator},
    slice::ParallelSliceMut,
};
use serde::Serialize;
use std::{collections::HashMap, sync::Arc};

use super::{cell::Cell, coordinate::Coordinate, range::Range};
use crate::{datatype::CellValue, MAX_COL, MAX_ROW};

#[derive(Clone, Default, Debug, Serialize)]
pub struct Cells {
    map: HashMap<(u32, u16), Arc<RwLock<Cell>>>,
    default_cell_value: CellValue,
}

impl Cells {
    /// Метод для получения коллекции ячеек
    #[inline]
    pub fn get_collection(&self) -> Vec<&Arc<RwLock<Cell>>> {
        self.map.values().collect::<Vec<_>>()
    }

    /// Метод для получения коллекции ячеек
    #[inline]
    pub fn get_collection_sorted(&self) -> Vec<&Arc<RwLock<Cell>>> {
        let mut cells = self.map.values().collect::<Vec<_>>();
        cells.par_sort_by_key(|cell| {
            let guard = cell.read();
            let cord = guard.get_coordinate();

            (cord.row, cord.column)
        });

        cells
    }

    /// Метод для получения ячейки по координатам
    #[inline]
    pub fn get_cell<T>(&self, coordinate: T) -> Option<&Arc<RwLock<Cell>>>
    where
        T: Into<Coordinate>,
    {
        let Coordinate { row, column } = coordinate.into();
        self.map.get(&(row, column))
    }

    #[inline]
    pub fn get_cell_by_letter(&self, letter: &str) -> Option<&Arc<RwLock<Cell>>> {
        let Coordinate { row, column } = Coordinate::from(letter);

        self.map.get(&(row, column))
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
            .map(|c| c.read().get_value())
            .unwrap_or(self.default_cell_value.get_value())
    }

    #[inline]
    pub fn get_cell_collection_by_range(
        &self,
        range: &Range,
    ) -> impl Iterator<Item = &Arc<RwLock<Cell>>> {
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
                let guard = cell.read();
                let coord = guard.get_coordinate();
                coord.row >= start_row
                    && coord.row <= end_row
                    && coord.column >= start_col
                    && coord.column <= end_col
            })
            .map(|(_, cell)| cell)
            .collect::<Vec<_>>();

        cells.par_sort_by_key(|cell| {
            let guard = cell.read();
            let cord = guard.get_coordinate();

            (cord.row, cord.column)
        });

        cells.into_iter()
    }

    #[inline]
    pub fn write_cell<T>(&mut self, coordinate: T, value: &str) -> &Arc<RwLock<Cell>>
    where
        T: Into<Coordinate>,
    {
        let Coordinate { row, column } = coordinate.into();

        let cell = self.map.entry((row, column)).or_insert_with(|| {
            Arc::new(RwLock::new(Cell::new(
                Coordinate::new(row, column),
                Some(value),
            )))
        });

        cell.write().set_value(value);

        cell
    }

    #[inline]
    pub fn delete_cols(&mut self, idx: u16, amount: u16) {
        self.map.retain(|_, cell| {
            let guard = cell.read();
            let Coordinate { column, .. } = guard.get_coordinate();
            *column < idx || *column >= idx + amount
        });

        for (_, cell) in self.map.iter() {
            let guard = cell.read();
            let Coordinate { column, .. } = guard.get_coordinate();
            if *column > idx {
                cell.write().set_coordinate(Coordinate {
                    row: guard.get_coordinate().row,
                    column: column - amount,
                });
            }
        }
    }

    #[inline]
    pub fn delete_rows(&mut self, idx: u32, amount: u32) {
        self.map.retain(|_, cell| {
            let guard = cell.read();
            let Coordinate { row, .. } = guard.get_coordinate();
            *row < idx || *row >= idx + amount
        });

        for (_, cell) in self.map.iter() {
            let guard = cell.read();
            let Coordinate { row, .. } = guard.get_coordinate();
            if *row > idx {
                cell.write().set_coordinate(Coordinate {
                    row: row - amount,
                    column: guard.get_coordinate().column,
                });
            }
        }
    }

    #[inline]
    pub fn find_cell_by_regex(&self, regex: &str) -> Result<Option<&Arc<RwLock<Cell>>>> {
        let cells = self.get_collection_sorted();

        let re = Regex::new(regex)?;
        Ok(cells.par_iter().find_map_first(|cell| {
            let guard = cell.read();
            if re.is_match(&guard.get_value()).unwrap_or(false) {
                Some(*cell)
            } else {
                None
            }
        }))
    }

    #[inline]
    pub fn find_cell_by_letter(&self, letter: &str) -> Result<Option<&Arc<RwLock<Cell>>>> {
        let cells = self.get_collection_sorted();
        let letter_coord = &Coordinate::from(letter);

        Ok(cells.par_iter().find_map_first(|cell| {
            let guard = cell.read();
            let coord = guard.get_coordinate();
            if coord == letter_coord {
                Some(*cell)
            } else {
                None
            }
        }))
    }

    #[inline]
    pub fn find_cells_by_regex(&self, regex: &str) -> Result<Vec<&Arc<RwLock<Cell>>>> {
        let cells = self.get_collection_sorted();

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

    #[inline]
    pub fn find_cells_for_rows_by_regex(
        &self,
        regex: &str,
        col_stop: u16,
    ) -> Result<Vec<&Arc<RwLock<Cell>>>> {
        let cells = self.get_collection_sorted();

        let re = Regex::new(regex)?;
        Ok(cells
            .par_iter()
            .filter_map(|cell| {
                let guard = cell.read();
                if re.is_match(&guard.get_value()).unwrap_or(false) {
                    if guard.get_coordinate().column <= col_stop {
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

    #[inline]
    pub fn find_cells_for_cols_by_regex(
        &self,
        regex: &str,
        row_stop: u32,
    ) -> Result<Vec<&Arc<RwLock<Cell>>>> {
        let cells = self.get_collection_sorted();

        let re = Regex::new(regex)?;
        Ok(cells
            .par_iter()
            .filter_map(|cell| {
                let guard = cell.read();
                if re.is_match(&guard.get_value()).unwrap_or(false) {
                    if guard.get_coordinate().row <= row_stop {
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

    #[inline]
    pub fn find_cells_multi_regex(
        &self,
        before_regex: &str,
        after_regex: &str,
    ) -> Result<Vec<&Arc<RwLock<Cell>>>> {
        let cells = self.get_collection_sorted();

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

    #[inline]
    pub fn find_cells_between_regex(
        &self,
        before_regex: &str,
        after_regex: &str,
    ) -> Result<Vec<&Arc<RwLock<Cell>>>> {
        let cells = self.get_collection_sorted();

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

    pub fn find_cells_range_rows(
        &self,
        start_row: u32,
        end_row: u32,
    ) -> Result<Vec<&Arc<RwLock<Cell>>>> {
        let cells = self.get_collection_sorted();

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

    pub fn find_cells_range_cols(
        &self,
        start_col: u16,
        end_col: u16,
    ) -> Result<Vec<&Arc<RwLock<Cell>>>> {
        let cells = self.get_collection_sorted();

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
}
