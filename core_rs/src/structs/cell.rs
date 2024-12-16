use chrono::NaiveDateTime;
use serde::Serialize;

use super::{coordinate::Coordinate, style::Style};
use crate::datatype::CellValue;

#[derive(Clone, Debug, Default, Serialize)]
pub struct Cell {
    #[serde(flatten)]
    coordinate: Coordinate,
    #[serde(flatten)]
    value: Box<CellValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    formula: Option<String>,
    data_type: String,
    #[serde(flatten)]
    style: Option<Style>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub fn get_letter(&self) -> String {
        String::from(&self.coordinate)
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
    pub fn is_value_integer(&self) -> bool {
        self.value.is_integer()
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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn cell() -> Cell {
        Cell::new(Coordinate { row: 1, column: 1 }, Some("Тестовая ячейка"))
    }

    #[test]
    fn new_cell() {
        let cell = cell();

        assert_eq!(cell.get_coordinate(), &Coordinate::new(1, 1))
    }

    #[test]
    fn get_coordinate() {
        let cell = cell();

        assert_eq!(cell.get_coordinate(), &Coordinate::new(1, 1))
    }

    #[test]
    fn set_coordinate() {
        let mut cell = cell();

        let coord = Coordinate::new(1, 2);
        cell.set_coordinate(coord.clone());

        assert_eq!(cell.get_coordinate(), &coord)
    }

    #[test]
    fn get_value() {
        let cell = cell();

        assert_eq!(cell.get_value(), "Тестовая ячейка")
    }

    #[test]
    fn get_formula() {
        let cell = Cell {
            coordinate: Coordinate::new(1, 1),
            formula: Some("=A1".to_string()),
            ..Default::default()
        };

        assert_eq!(cell.get_formula().unwrap(), "=A1".to_string())
    }

    #[test]
    fn get_data_type() {
        let cell = Cell {
            coordinate: Coordinate::new(1, 1),
            formula: Some("=A1".to_string()),
            data_type: "f".to_string(),
            ..Default::default()
        };

        assert_eq!(cell.get_data_type(), "f".to_string())
    }

    #[test]
    fn get_style() {
        let style = Style::new("AAAA");
        let cell = Cell {
            coordinate: Coordinate::new(1, 1),
            formula: Some("=A1".to_string()),
            data_type: "f".to_string(),
            style: Some(style.clone()),
            ..Default::default()
        };

        assert_eq!(cell.get_style().unwrap(), style)
    }

    #[test]
    fn set_value() {
        let val = "AAA";
        let mut cell = cell();
        cell.set_value(val);

        assert_eq!(cell.get_value(), val)
    }

    #[test]
    fn set_value_number() {
        let val = 34.8;
        let mut cell = cell();
        cell.set_value_number(val);

        assert_eq!(cell.get_value(), "34.8")
    }

    #[test]
    fn set_value_integer() {
        let val = 34;
        let mut cell = cell();
        cell.set_value_integer(val);

        assert_eq!(cell.get_value(), "34")
    }

    #[test]
    fn set_value_bool() {
        let val = true;
        let mut cell = cell();
        cell.set_value_bool(val);

        assert_eq!(cell.get_value(), "true")
    }

    #[test]
    fn set_value_datetime() {
        let val = Utc::now().naive_utc();
        let mut cell = cell();
        cell.set_value_datetime(val);

        assert_eq!(cell.get_value(), val.to_string())
    }

    #[test]
    fn set_formula() {
        let val = "=A1";
        let mut cell = cell();
        cell.set_formula(val);

        assert_eq!(cell.get_formula().unwrap(), val.to_string())
    }

    #[test]
    fn set_style() {
        let val = "A1";
        let mut cell = cell();
        cell.set_style(val);

        assert_eq!(cell.get_style().unwrap().get_id(), "A1")
    }

    #[test]
    fn set_hidden_value() {
        let val = "Hidden";
        let mut cell = cell();
        cell.set_hidden_value(val);

        assert_eq!(cell.hidden_value.unwrap(), "Hidden")
    }

    #[test]
    fn remove_formula() {
        let val = "=A1";
        let mut cell = cell();
        cell.set_formula(val);

        cell.remove_formula();

        assert!(cell.formula.is_none());
        assert_eq!(cell.data_type, "s")
    }

    #[test]
    fn is_formula() {
        let val = "=A1";
        let mut cell = cell();
        cell.set_formula(val);

        assert!(cell.is_formula());
    }

    #[test]
    fn is_value_bool() {
        let val = "true";
        let mut cell = cell();
        cell.set_value(val);

        assert!(cell.is_value_bool());
    }

    #[test]
    fn is_value_numeric() {
        let val = "38.9";
        let mut cell = cell();
        cell.set_value(val);

        assert!(cell.is_value_numeric());
    }

    #[test]
    fn is_value_integer() {
        let val = "38";
        let mut cell = cell();
        cell.set_value(val);

        assert!(cell.is_value_integer());
    }

    #[test]
    fn is_value_datetime() {
        let val = Utc::now().naive_utc();
        let mut cell = cell();
        cell.set_value_datetime(val);

        assert!(cell.is_value_datetime());
    }

    #[test]
    fn is_value_empty() {
        let val = "";
        let mut cell = cell();
        cell.set_value(val);

        assert!(cell.is_value_empty());
    }

    #[test]
    fn get_letter() {
        let cell = cell();

        assert_eq!(cell.get_letter(), "A1");
    }
}
