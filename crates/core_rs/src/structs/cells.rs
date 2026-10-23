use ahash::HashMap;
use anyhow::{Result, bail};
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

    let mut items: Vec<_> = map.iter().collect();
    items.sort_unstable_by_key(|(key, _)| *key);

    let mut seq = serializer.serialize_seq(Some(items.len()))?;
    for (_, cell) in items {
        let guard = cell.read();
        seq.serialize_element(&*guard)?;
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

/// Проверяет, что диапазон удаления `[idx; idx + amount)` лежит в домене
/// Excel `[1; max]`. Арифметика — в широком типе: в исходных
/// типах (u16/u32) `idx + amount` переполнялось с коллизиями ключей
/// карты в release и паникой в debug.
#[inline]
fn validate_delete_range(idx: u64, amount: u64, max: u64) -> Result<()> {
    if idx == 0 {
        bail!("Индекс начала удаления должен быть ≥ 1, получено: {idx}");
    }

    if amount == 0 {
        bail!("Количество удаляемых строк/колонок должно быть ≥ 1, получено: {amount}");
    }

    let end = idx + amount - 1;
    if end > max {
        bail!("Диапазон удаления [{idx}; {end}] выходит за границы листа (максимум: {max})");
    }

    Ok(())
}

impl Cells {
    pub fn new(map: HashMap<(u32, u16), Arc<RwLock<Cell>>>) -> Self {
        Self {
            map,
            ..Default::default()
        }
    }

    /// Глубокое клонирование: каждая ячейка получает собственную обёртку
    /// `Arc<RwLock<Cell>>`, чтобы мутации копии не меняли оригинал.
    pub fn clone_deep(&self) -> Self {
        let map = self
            .map
            .iter()
            .map(|(key, cell)| (*key, Arc::new(RwLock::new(cell.read().clone()))))
            .collect();

        Self {
            map,
            default_cell_value: self.default_cell_value.clone(),
        }
    }

    /// Метод для получения коллекции ячеек
    #[inline]
    pub fn get_collection(&self) -> Vec<&Arc<RwLock<Cell>>> {
        self.map.values().collect::<Vec<_>>()
    }

    /// Метод для получения коллекции ячеек, отсортированной по (row, col).
    ///
    /// Детерминированный обход: одиночные find_cell_by_* опираются на порядок —
    /// при нескольких совпадениях побеждает ячейка с наименьшими координатами.
    #[inline]
    pub fn get_collection_sorted(&self) -> Vec<&Arc<RwLock<Cell>>> {
        let mut items: Vec<_> = self.map.iter().map(|(key, cell)| (*key, cell)).collect();

        // Сортируем по ключу (row, col) - быстрая сортировка примитивов
        items.par_sort_unstable_by_key(|(key, _)| *key);

        // Возвращаем только ячейки в отсортированном порядке
        items.into_iter().map(|(_, cell)| cell).collect()
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
        let coord = Coordinate::try_from(letter).ok()?;

        self.map.get(&(coord.row, coord.column))
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

        let mut cells: Vec<_> = self
            .map
            .par_iter()
            .filter(|((row, col), _)| {
                *row >= start_row && *row <= end_row && *col >= start_col && *col <= end_col
            })
            .map(|(key, cell)| (*key, cell))
            .collect();

        // Сортируем по ключам без блокировок!
        cells.par_sort_unstable_by_key(|(key, _)| *key);

        // Возвращаем только ячейки
        cells.into_iter().map(|(_, cell)| cell)
    }

    /// Get-or-create ячейки по координатам.
    ///
    /// Существующая ячейка не мутируется: при `Some(value)` значение
    /// перезаписывается, при `None` возвращается ячейка как есть.
    #[inline]
    pub fn cell<T>(&mut self, coordinate: T, value: Option<&str>) -> &Arc<RwLock<Cell>>
    where
        T: Into<Coordinate>,
    {
        let Coordinate { row, column } = coordinate.into();

        let cell = self.map.entry((row, column)).or_insert_with(|| {
            Arc::new(RwLock::new(Cell::new(Coordinate::new(row, column), value)))
        });

        // Get-or-create: существующую ячейку не мутируем. Новая пустая ячейка
        // уже получает data_type "s" в Cell::new (Empty → "s"), поэтому touch
        // без значения не требует никаких правок (issue #19).
        if let Some(value) = value {
            cell.write().set_value(value);
        }

        cell
    }

    /// Удаляет колонки `[idx; idx + amount)`, сдвигая оставшиеся влево.
    ///
    /// # Errors
    /// Возвращает ошибку, если `idx` или `amount` меньше 1 либо диапазон
    /// выходит за границы листа (`MAX_COL`).
    #[inline]
    pub fn delete_cols(&mut self, idx: u16, amount: u16) -> Result<()> {
        validate_delete_range(idx as u64, amount as u64, MAX_COL as u64)?;

        let new_map: HashMap<_, _> = self
            .map
            .drain()
            .filter_map(|((row, col), cell)| {
                if col < idx || col >= idx + amount {
                    // Ячейка сохраняется
                    if col > idx {
                        // Нужно обновить координату в Cell
                        cell.write()
                            .set_coordinate(Coordinate::new(row, col - amount));
                        Some(((row, col - amount), cell))
                    } else {
                        Some(((row, col), cell))
                    }
                } else {
                    // Ячейка удаляется
                    None
                }
            })
            .collect();

        self.map = new_map;

        Ok(())
    }

    /// Удаляет строки `[idx; idx + amount)`, сдвигая оставшиеся вверх.
    ///
    /// # Errors
    /// Возвращает ошибку, если `idx` или `amount` меньше 1 либо диапазон
    /// выходит за границы листа (`MAX_ROW`).
    #[inline]
    pub fn delete_rows(&mut self, idx: u32, amount: u32) -> Result<()> {
        validate_delete_range(idx as u64, amount as u64, MAX_ROW as u64)?;

        let new_map: HashMap<_, _> = self
            .map
            .drain()
            .filter_map(|((row, col), cell)| {
                if row < idx || row >= idx + amount {
                    // Ячейка сохраняется
                    if row > idx {
                        // Нужно обновить координату в Cell
                        cell.write()
                            .set_coordinate(Coordinate::new(row - amount, col));
                        Some(((row - amount, col), cell))
                    } else {
                        Some(((row, col), cell))
                    }
                } else {
                    // Ячейка удаляется
                    None
                }
            })
            .collect();

        self.map = new_map;

        Ok(())
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
        let cells = self.get_collection();

        find_values_by_col_rows(col, rows, cells)
    }

    #[inline]
    pub fn find_values_by_row_cols(&self, row: u32, cols: Vec<u16>) -> Result<Vec<String>> {
        let cells = self.get_collection();

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
