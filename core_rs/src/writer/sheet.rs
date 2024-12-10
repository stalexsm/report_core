use anyhow::{bail, Result};
use parking_lot::Mutex;
use rayon::prelude::*;
use serde::Serialize;
use std::{
    collections::HashMap,
    sync::{Arc, Weak},
};

use super::{book::XLSXBook, cell::XLSXSheetCell, MAX_COL, MAX_ROW};
use crate::utils;

#[derive(Clone, Debug, Default)]
pub struct XLSXSheet {
    pub name: String,
    pub max_row: u32,
    pub max_column: u16,
    pub index: i32,
    // todo
    pub _cells: HashMap<(u32, u16), Arc<Mutex<XLSXSheetCell>>>,

    _current_workbook: Weak<Mutex<XLSXBook>>,
}

impl Serialize for XLSXSheet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("XLSXSheet", 5)?;

        state.serialize_field("name", &self.name)?;
        state.serialize_field("max_row", &self.max_row)?;
        state.serialize_field("max_column", &self.max_column)?;
        state.serialize_field("index", &self.index)?;

        let cells: Vec<_> = self.cells().map(|s| s.lock().clone()).collect();
        state.serialize_field("cells", &cells)?;

        state.end()
    }
}

impl XLSXSheet {
    pub fn new(
        book: Arc<Mutex<XLSXBook>>,
        name: String,
        index: i32,
        rows: u32,
        cols: u16,
    ) -> Arc<Mutex<XLSXSheet>> {
        if rows > MAX_ROW || cols > MAX_COL {
            panic!("Row or Column is out of range");
        }

        // Создаем лист
        let sheet = Arc::new(Mutex::new(Self {
            name,
            max_row: rows,
            max_column: cols,
            index,
            _current_workbook: Arc::downgrade(&book),
            ..Default::default()
        }));

        // Создаем список ячеек по умолчанию
        let cells: HashMap<_, _> = (1..=rows)
            .into_par_iter()
            .flat_map(|r| {
                let sheet = Arc::clone(&sheet);
                (1..=cols).into_par_iter().map(move |c| {
                    let cell = XLSXSheetCell::new(Arc::clone(&sheet), r, c, None);
                    ((r, c), cell)
                })
            })
            .collect();

        // Заполняем список ячеек
        sheet.lock()._cells = cells;

        sheet
    }

    pub fn cells(&self) -> impl Iterator<Item = &Arc<Mutex<XLSXSheetCell>>> {
        let mut cells = self._cells.values().collect::<Vec<_>>();

        cells.par_sort_by_key(|k| {
            let cell = k.lock();
            (cell.row, cell.column)
        });

        cells.into_iter()
    }

    pub fn iter_cells(
        &self,
        min_row: Option<u32>,
        max_row: Option<u32>,
        min_col: Option<u16>,
        max_col: Option<u16>,
    ) -> Result<impl Iterator<Item = &Arc<Mutex<XLSXSheetCell>>>> {
        let min_row = min_row.unwrap_or(1);
        let max_row = max_row.unwrap_or(self.max_row);
        let min_col = min_col.unwrap_or(1);
        let max_col = max_col.unwrap_or(self.max_column);

        if min_row > max_row || min_col > max_col {
            bail!("The coordinates of the cells were incorrectly transmitted");
        }

        let mut cells = self
            ._cells
            .par_iter()
            .filter(move |(_, cell)| {
                let cell = cell.lock();
                cell.row >= min_row
                    && cell.row <= max_row
                    && cell.column >= min_col
                    && cell.column <= max_col
            })
            .map(|(_, cell)| cell)
            .collect::<Vec<_>>();

        cells.par_sort_by_key(|cell| {
            let cell = cell.lock();
            (cell.row, cell.column)
        });

        Ok(cells.into_iter())
    }

    pub fn find_cell_by_coords(
        &self,
        row: u32,
        col: u16,
    ) -> Result<Option<Arc<Mutex<XLSXSheetCell>>>> {
        let cell = self._cells.get(&(row, col));
        Ok(cell.cloned())
    }

    pub fn find_cell_by_cell(&self, cell: &str) -> Result<Option<Arc<Mutex<XLSXSheetCell>>>> {
        let found_cell = self._cells.par_iter().find_map_first(|(_, c)| {
            if c.lock().cell == cell {
                Some(Arc::clone(c))
            } else {
                None
            }
        });

        Ok(found_cell)
    }

    pub fn write_cell(
        &mut self,
        row: u32,
        col: u16,
        value: &str,
    ) -> Result<Arc<Mutex<XLSXSheetCell>>> {
        if row < 1 || row > MAX_ROW || col < 1 || col > MAX_COL {
            bail!(
                "Row or Column is valid. 0 < Row < {} and 0 < Column < {}",
                MAX_ROW,
                MAX_COL
            );
        }

        if let Some(cell) = self._cells.get_mut(&(row, col)) {
            let mut cell_guard = cell.lock();
            cell_guard.set_value(value.to_string())?;

            Ok(cell.clone())
        } else {
            // Добавление ячейки в список ячеек
            let current_sheet = Arc::new(Mutex::new(self.clone()));
            let cell = XLSXSheetCell::new(
                Arc::clone(&current_sheet),
                row,
                col,
                Some(value.to_string()),
            );

            self._cells.insert((row, col), Arc::clone(&cell));

            // Обновим максимальные значения
            self.max_row = self.max_row.max(row);
            self.max_column = self.max_column.max(col);

            Ok(cell)
        }
    }

    pub fn delete_cols(&mut self, idx: u16, amount: u16) -> Result<()> {
        // Remove cells in the specified columns
        self._cells.retain(|_, cell| {
            let cell = cell.lock();
            cell.column < idx || cell.column >= idx + amount
        });

        // Update column numbers for remaining cells
        for (_, cell) in self._cells.iter() {
            let mut cell = cell.lock();
            if cell.column > idx {
                cell.column -= amount;
                // Update the cell's letter coordinate
                let new_letter = utils::get_letter_coordinate(cell.row, cell.column);
                cell.cell = new_letter;
            }
        }

        // Update max_column if necessary
        self.max_column = self.max_column.saturating_sub(amount);

        Ok(())
    }

    pub fn delete_rows(&mut self, idx: u32, amount: u32) -> Result<()> {
        // Remove cells in the specified columns
        self._cells.retain(|_, cell| {
            let cell = cell.lock();
            cell.row < idx || cell.row >= idx + amount
        });

        // Update column numbers for remaining cells
        for (_, cell) in self._cells.iter() {
            let mut cell = cell.lock();
            if cell.row > idx {
                cell.row -= amount;
                // Update the cell's letter coordinate
                let new_letter = utils::get_letter_coordinate(cell.row, cell.column);
                cell.cell = new_letter;
            }
        }

        // Update max_column if necessary
        self.max_row = self.max_row.saturating_sub(amount);

        Ok(())
    }

    pub fn set_merged_cells(
        &mut self,
        start_row: u32,
        end_row: u32,
        start_column: u16,
        end_column: u16,
    ) -> Result<()> {
        // Iterate through all cells in the merge range
        for row in start_row..=end_row {
            for col in start_column..=end_column {
                if let Some(cell) = self._cells.get(&(row, col)) {
                    let mut cell = cell.lock();
                    cell.is_merge = true;
                    cell.start_row = Some(start_row);
                    cell.end_row = Some(end_row);
                    cell.start_column = Some(start_column);
                    cell.end_column = Some(end_column);
                }
            }
        }

        Ok(())
    }

    pub fn generate_empty_cells(&mut self) -> Result<()> {
        // Создаем новые ячейки которые не существуют
        let sheet_ref = Arc::new(Mutex::new(self.clone()));

        let cells: HashMap<_, _> = (1..=self.max_row)
            .into_par_iter()
            .flat_map(|r| {
                let sheet = Arc::clone(&sheet_ref);
                (1..=self.max_column).into_par_iter().filter_map(move |c| {
                    let exists = sheet.lock()._cells.contains_key(&(r, c));
                    if !exists {
                        let cell = XLSXSheetCell::new(Arc::clone(&sheet), r, c, None);
                        Some(((r, c), cell))
                    } else {
                        None
                    }
                })
            })
            .collect();

        self._cells.extend(cells);

        Ok(())
    }
}
