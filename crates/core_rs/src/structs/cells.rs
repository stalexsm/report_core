use ahash::HashMap;
use anyhow::Result;
use parking_lot::RwLock;
use rayon::{
    iter::{IntoParallelRefIterator, ParallelIterator},
    slice::ParallelSliceMut,
};
use serde::Serialize;
use std::sync::Arc;

use super::{cell::Cell, coordinate::Coordinate};
use crate::{
    MAX_COL, MAX_ROW,
    datatype::CellValue,
    funcs::{
        find_cell_by_letter, find_cell_by_regex, find_cell_by_str, find_cells_between_regex,
        find_cells_by_regex, find_cells_by_str, find_cells_for_cols_by_regex,
        find_cells_for_rows_by_regex, find_cells_multi_regex, find_cells_range_cols,
        find_cells_range_rows, find_values_by_col_rows, find_values_by_row_cols,
    },
    traits::{ReadableCell, WriteableCell},
};

/// Вспомоогательная функция для сериализации HashMap только Value, как вектор.
fn serialize_cells_to_vec<S>(
    map: &HashMap<(u32, u16), Arc<RwLock<Cell>>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde::ser::SerializeSeq;
    let mut keys: Vec<_> = map.keys().collect();
    keys.sort_unstable();

    let mut seq = serializer.serialize_seq(Some(keys.len()))?;
    for key in keys {
        if let Some(cell) = map.get(key) {
            let guard = cell.read();
            seq.serialize_element(&*guard)?;
        }
    }
    seq.end()
}

#[derive(Clone, Default, Debug, Serialize)]
pub struct Cells {
    #[serde(serialize_with = "serialize_cells_to_vec")]
    #[serde(rename = "cells")]
    map: HashMap<(u32, u16), Arc<RwLock<Cell>>>,
    #[serde(skip)]
    default_cell_value: CellValue,
}

impl Cells {
    pub fn new(map: HashMap<(u32, u16), Arc<RwLock<Cell>>>) -> Self {
        Self {
            map,
            ..Default::default()
        }
    }

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

    #[inline]
    pub fn get_max_row(&self) -> u32 {
        self.map.keys().map(|(row, _)| *row).max().unwrap_or(0)
    }

    #[inline]
    pub fn get_max_column(&self) -> u16 {
        self.map
            .keys()
            .map(|(_, column)| *column)
            .max()
            .unwrap_or(0)
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
        start_row: Option<u32>,
        end_row: Option<u32>,
        start_col: Option<u16>,
        end_col: Option<u16>,
    ) -> impl Iterator<Item = &Arc<RwLock<Cell>>> {
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
    pub fn cell<T>(&mut self, coordinate: T, value: Option<&str>) -> &Arc<RwLock<Cell>>
    where
        T: Into<Coordinate>,
    {
        let Coordinate { row, column } = coordinate.into();

        let cell = self.map.entry((row, column)).or_insert_with(|| {
            Arc::new(RwLock::new(Cell::new(Coordinate::new(row, column), value)))
        });

        if let Some(value) = value {
            cell.write().set_value(value);
        } else {
            let mut c = cell.write();
            if !c.is_formula() {
                c.set_data_type("s");
            }
        }

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

        find_cell_by_regex(regex.into(), cells)
    }

    #[inline]
    pub fn find_cell_by_str(&self, value: &str) -> Result<Option<&Arc<RwLock<Cell>>>> {
        let cells = self.get_collection_sorted();

        find_cell_by_str(value.into(), cells)
    }

    #[inline]
    pub fn find_cell_by_coords(&self, row: u32, col: u16) -> Result<Option<&Arc<RwLock<Cell>>>> {
        Ok(self.map.get(&(row, col)))
    }

    #[inline]
    pub fn find_cell_by_letter(&self, letter: &str) -> Result<Option<&Arc<RwLock<Cell>>>> {
        let cells = self.get_collection_sorted();

        find_cell_by_letter(letter.into(), cells)
    }

    #[inline]
    pub fn find_cells_by_regex(&self, regex: &str) -> Result<Vec<&Arc<RwLock<Cell>>>> {
        let cells = self.get_collection_sorted();

        find_cells_by_regex(regex.into(), cells)
    }

    #[inline]
    pub fn find_cells_by_str(&self, value: &str) -> Result<Vec<&Arc<RwLock<Cell>>>> {
        let cells = self.get_collection_sorted();

        find_cells_by_str(value.into(), cells)
    }

    #[inline]
    pub fn find_cells_for_rows_by_regex(
        &self,
        regex: &str,
        col_stop: u16,
    ) -> Result<Vec<&Arc<RwLock<Cell>>>> {
        let cells = self.get_collection_sorted();

        find_cells_for_rows_by_regex(regex.into(), col_stop, cells)
    }

    #[inline]
    pub fn find_cells_for_cols_by_regex(
        &self,
        regex: &str,
        row_stop: u32,
    ) -> Result<Vec<&Arc<RwLock<Cell>>>> {
        let cells = self.get_collection_sorted();

        find_cells_for_cols_by_regex(regex.into(), row_stop, cells)
    }

    #[inline]
    pub fn find_cells_multi_regex(
        &self,
        before_regex: &str,
        after_regex: &str,
    ) -> Result<Vec<&Arc<RwLock<Cell>>>> {
        let cells = self.get_collection_sorted();

        find_cells_multi_regex(before_regex.into(), after_regex.into(), cells)
    }

    #[inline]
    pub fn find_cells_between_regex(
        &self,
        before_regex: &str,
        after_regex: &str,
    ) -> Result<Vec<&Arc<RwLock<Cell>>>> {
        let cells = self.get_collection_sorted();

        find_cells_between_regex(before_regex.into(), after_regex.into(), cells)
    }

    #[inline]
    pub fn find_cells_range_rows(
        &self,
        start_row: u32,
        end_row: u32,
    ) -> Result<Vec<&Arc<RwLock<Cell>>>> {
        let cells = self.get_collection_sorted();

        find_cells_range_rows(start_row, end_row, cells)
    }

    #[inline]
    pub fn find_cells_range_cols(
        &self,
        start_col: u16,
        end_col: u16,
    ) -> Result<Vec<&Arc<RwLock<Cell>>>> {
        let cells = self.get_collection_sorted();

        find_cells_range_cols(start_col, end_col, cells)
    }

    #[inline]
    pub fn find_values_by_col_rows(&self, col: u16, rows: Vec<u32>) -> Result<Vec<String>> {
        let cells = self.get_collection_sorted();

        find_values_by_col_rows(col, rows, cells)
    }

    #[inline]
    pub fn find_values_by_row_cols(&self, row: u32, cols: Vec<u16>) -> Result<Vec<String>> {
        let cells = self.get_collection_sorted();

        find_values_by_row_cols(row, cols, cells)
    }

    #[inline]
    pub fn find_value_by_coords(&self, row: u32, col: u16) -> Result<Option<String>> {
        let value = self.map.get(&(row, col)).map(|cell| {
            let guard = cell.read();
            guard.get_value().to_string()
        });

        Ok(value)
    }
}
