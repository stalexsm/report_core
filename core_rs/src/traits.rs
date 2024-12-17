use std::sync::Arc;

use anyhow::Result;
use chrono::NaiveDateTime;
use parking_lot::RwLock;

use crate::structs::{cell::Cell, coordinate::Coordinate, range::Range, style::Style};

pub trait ReadableCell {
    fn get_coordinate(&self) -> &Coordinate;
    fn get_letter(&self) -> String;
    fn get_value(&self) -> String;
    fn get_formula(&self) -> Option<String>;
    fn get_data_type(&self) -> String;
    fn get_style(&self) -> Option<Style>;
    fn is_formula(&self) -> bool;
    fn is_value_bool(&self) -> bool;
    fn is_value_numeric(&self) -> bool;
    fn is_value_integer(&self) -> bool;
    fn is_value_datetime(&self) -> bool;
    fn is_value_empty(&self) -> bool;
}

pub trait WriteableCell {
    fn set_coordinate(&mut self, coordinate: Coordinate);
    fn set_value(&mut self, value: &str) -> &mut Self;
    fn set_value_number(&mut self, value: f64) -> &mut Self;
    fn set_value_integer(&mut self, value: i32) -> &mut Self;
    fn set_value_bool(&mut self, value: bool) -> &mut Self;
    fn set_value_datetime(&mut self, value: NaiveDateTime) -> &mut Self;
    fn set_formula(&mut self, value: &str) -> &mut Self;
    fn set_style(&mut self, value: &str) -> &mut Self;
    fn set_hidden_value(&mut self, value: &str) -> &mut Self;
}

pub trait ReadableSheet {
    type Cell: ReadableCell + Clone + Send; // добавляем дополнительные

    fn get_name(&self) -> String;

    fn get_cell_collection(&self) -> Vec<&Arc<RwLock<Cell>>>;
    fn get_cell_collection_sorted(&self) -> Vec<&Arc<RwLock<Cell>>>;
    fn get_max_row(&self) -> u32;
    fn get_max_column(&self) -> u16;
    fn get_cell_value<T>(&self, coordinate: T) -> String
    where
        T: Into<Coordinate>;
    fn get_cell_collection_by_range(
        &self,
        start_row: Option<u32>,
        end_row: Option<u32>,
        start_col: Option<u16>,
        end_col: Option<u16>,
    ) -> impl Iterator<Item = &Arc<RwLock<Cell>>>;
    fn get_merge_cell_collection(&self) -> &[Range];

    fn find_cell_by_regex(&self, regex: &str) -> Result<Option<&Arc<RwLock<Cell>>>>;
    fn find_cell_by_letter(&self, letter: &str) -> Result<Option<&Arc<RwLock<Cell>>>>;
    fn find_cells_by_regex(&self, regex: &str) -> Result<Vec<&Arc<RwLock<Cell>>>>;
    fn find_cells_for_rows_by_regex(
        &self,
        regex: &str,
        col_stop: u16,
    ) -> Result<Vec<&Arc<RwLock<Cell>>>>;
    fn find_cells_for_cols_by_regex(
        &self,
        regex: &str,
        row_stop: u32,
    ) -> Result<Vec<&Arc<RwLock<Cell>>>>;
    fn find_cells_multi_regex(
        &self,
        before_regex: &str,
        after_regex: &str,
    ) -> Result<Vec<&Arc<RwLock<Cell>>>>;
    fn find_cells_between_regex(
        &self,
        before_regex: &str,
        after_regex: &str,
    ) -> Result<Vec<&Arc<RwLock<Cell>>>>;
    fn find_cells_range_rows(
        &self,
        start_row: u32,
        end_row: u32,
    ) -> Result<Vec<&Arc<RwLock<Cell>>>>;
    fn find_cells_range_cols(
        &self,
        start_col: u16,
        end_col: u16,
    ) -> Result<Vec<&Arc<RwLock<Cell>>>>;
}

pub trait WriteableSheet {
    fn set_name(&mut self, name: &str);
    fn add_merge_range(&mut self, range: Range);
    fn cell(&mut self, coordinate: Coordinate, value: Option<&str>) -> &Arc<RwLock<Cell>>;
    fn delete_cols(&mut self, idx: u16, amount: u16);
    fn delete_rows(&mut self, idx: u32, amount: u32);
}
