use chrono::NaiveDateTime;
use serde::Serialize;

use super::{coordinate::Coordinate, style::Style};
use crate::datatype::CellValue;

#[derive(Clone, Debug, Default, Serialize)]
pub struct Cell {
    coordinate: Coordinate,
    value: Box<CellValue>,
    formula: Option<String>,
    data_type: String,
    style: Option<Style>,
    hidden_value: Option<String>,
}

impl Cell {
    /// Интициализирует ячейку с заданным координатой и значением
    pub fn new(coordinate: Coordinate, value: Option<&str>) -> Self {
        let mut cell_val = CellValue::default();

        if let Some(val) = value {
            cell_val.set_value(val);
        }
        let data_type = cell_val.get_data_type().to_string();

        Cell {
            coordinate,
            value: Box::new(cell_val),
            data_type,
            ..Default::default()
        }
    }

    /// Метод для получения координаты ячейки
    #[inline]
    pub fn get_coordinate(&self) -> &Coordinate {
        &self.coordinate
    }

    #[inline]
    pub fn set_coordinate(&mut self, coordinate: Coordinate) {
        self.coordinate = coordinate;
    }

    #[inline]
    pub fn get_value(&self) -> String {
        self.value.get_value()
    }

    #[inline]
    pub fn get_formula(&self) -> Option<String> {
        self.formula.clone()
    }

    #[inline]
    pub fn get_data_type(&self) -> String {
        self.data_type.clone()
    }

    #[inline]
    pub fn get_style(&self) -> Option<Style> {
        self.style.clone()
    }

    #[inline]
    pub fn set_value(&mut self, value: &str) -> &mut Self {
        self.value.set_value(value);
        self.remove_formula();

        self
    }

    #[inline]
    pub fn set_value_number(&mut self, value: f64) -> &mut Self {
        self.value.set_value_number(value);
        self.remove_formula();

        self
    }

    #[inline]
    pub fn set_value_integer(&mut self, value: i32) -> &mut Self {
        self.value.set_value_integer(value);
        self.remove_formula();

        self
    }

    #[inline]
    pub fn set_value_bool(&mut self, value: bool) -> &mut Self {
        self.value.set_value_bool(value);
        self.remove_formula();

        self
    }

    #[inline]
    pub fn set_value_datetime(&mut self, value: NaiveDateTime) -> &mut Self {
        self.value.set_value_datatime(value);
        self.remove_formula();

        self
    }

    #[inline]
    pub fn set_formula(&mut self, value: &str) -> &mut Self {
        self.formula = Some(value.to_string());
        self.data_type = "f".to_string();

        self
    }

    #[inline]
    pub fn set_style(&mut self, value: &str) -> &mut Self {
        self.style = Some(Style::new(value));

        self
    }

    #[inline]
    pub fn set_hidden_value(&mut self, value: &str) -> &mut Self {
        self.hidden_value = Some(value.to_string());

        self
    }

    #[inline]
    pub(crate) fn remove_formula(&mut self) {
        self.formula = None;
        // Сбросим и тип данных
        self.data_type = self.value.get_data_type().to_string();
    }

    #[inline]
    pub fn is_formula(&self) -> bool {
        self.formula.is_some() && self.data_type == "f"
    }

    #[inline]
    pub fn is_value_bool(&self) -> bool {
        self.value.is_bool()
    }

    #[inline]
    pub fn is_value_numeric(&self) -> bool {
        self.value.is_numeric()
    }

    #[inline]
    pub fn is_value_datetime(&self) -> bool {
        self.value.is_datetime()
    }

    #[inline]
    pub fn is_value_empty(&self) -> bool {
        self.value.is_empty()
    }
}
