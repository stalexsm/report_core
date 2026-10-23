use std::fmt::Display;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(untagged)]
pub enum CellRawValue {
    #[default]
    Empty,

    String(Box<str>),
    Numeric(f64),
    Integer(i32),
    Bool(bool),
    Datetime(NaiveDateTime),
}

impl CellRawValue {
    /// Метод для получения типа данных.
    #[inline]
    pub(crate) fn get_date_type(&self) -> &str {
        match &self {
            Self::String(_) => "s",
            Self::Integer(_) => "n",
            Self::Numeric(_) => "n",
            Self::Bool(_) => "b",
            Self::Datetime(_) => "d",
            _ => "s",
        }
    }

    /// Метод для получения данных тип String.
    #[inline]
    pub(crate) fn _get_value_str(&self) -> String {
        match &self {
            Self::String(_)
            | Self::Numeric(_)
            | Self::Integer(_)
            | Self::Bool(_)
            | Self::Datetime(_) => self.to_string(),

            _ => "".to_string(),
        }
    }
}

impl Display for CellRawValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(v) => write!(f, "{v}"),
            Self::Numeric(v) => write!(f, "{v}"),
            Self::Integer(v) => write!(f, "{v}"),
            Self::Bool(v) => write!(f, "{v}"),
            Self::Datetime(v) => write!(f, "{v}"),
            _ => write!(f, ""),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CellValue {
    #[serde(rename = "value")]
    pub raw_value: CellRawValue,
}

impl CellValue {
    #[inline]
    pub fn get_data_type(&self) -> &str {
        self.raw_value.get_date_type()
    }

    #[inline]
    pub fn get_value(&self) -> String {
        self.raw_value.to_string()
    }

    #[inline]
    pub fn get_raw_value(&self) -> &CellRawValue {
        &self.raw_value
    }

    #[inline]
    pub fn set_value<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.raw_value = Self::guess_typed_data(&value.into());

        self
    }

    #[inline]
    pub fn set_value_str<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.raw_value = CellRawValue::String(value.into().into_boxed_str());

        self
    }

    #[inline]
    pub fn set_value_number(&mut self, value: f64) -> &mut Self {
        self.raw_value = CellRawValue::Numeric(value);

        self
    }

    #[inline]
    pub fn set_value_integer(&mut self, value: i32) -> &mut Self {
        self.raw_value = CellRawValue::Integer(value);

        self
    }

    #[inline]
    pub fn set_value_bool(&mut self, value: bool) -> &mut Self {
        self.raw_value = CellRawValue::Bool(value);

        self
    }

    #[inline]
    pub fn set_value_datatime(&mut self, value: NaiveDateTime) -> &mut Self {
        self.raw_value = CellRawValue::Datetime(value);

        self
    }

    #[inline]
    pub fn is_bool(&self) -> bool {
        matches!(self.raw_value, CellRawValue::Bool(_))
    }

    #[inline]
    pub fn is_numeric(&self) -> bool {
        matches!(self.raw_value, CellRawValue::Numeric(_))
    }

    #[inline]
    pub fn is_integer(&self) -> bool {
        matches!(self.raw_value, CellRawValue::Integer(_))
    }

    #[inline]
    pub fn is_datetime(&self) -> bool {
        matches!(self.raw_value, CellRawValue::Datetime(_))
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        matches!(self.raw_value, CellRawValue::Empty)
    }

    #[inline]
    pub(crate) fn guess_typed_data(value: &str) -> CellRawValue {
        let uppercase_value = value.to_uppercase();

        match uppercase_value.as_str() {
            "" => CellRawValue::Empty,
            "TRUE" => CellRawValue::Bool(true),
            "FALSE" => CellRawValue::Bool(false),
            _ => {
                if let Ok(number) = value.parse::<i32>() {
                    CellRawValue::Integer(number)
                } else if let Ok(number) = value.parse::<f64>() {
                    // NaN и inf непредставимы в XLSX: значения, не представимые
                    // конечным числом (включая переполнение до бесконечности),
                    // остаются строками, чтобы тип и значение не расходились.
                    if number.is_finite() {
                        CellRawValue::Numeric(number)
                    } else {
                        CellRawValue::String(value.into())
                    }
                } else if let Ok(datetime) =
                    NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S")
                {
                    CellRawValue::Datetime(datetime)
                } else {
                    CellRawValue::String(value.into())
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_value_keeps_non_finite_numbers_as_string() {
        // NaN и inf непредставимы в XLSX: такие строки остаются строками,
        // чтобы тип данных и значение не расходились при сериализации.
        for value in [
            "nan", "NaN", "inf", "-inf", "Infinity", "1e400", "+nan", "-1e400",
        ] {
            let mut cell = CellValue::default();
            cell.set_value(value);

            assert!(
                matches!(cell.get_raw_value(), CellRawValue::String(_)),
                "значение {value} должно остаться строкой"
            );
            assert_eq!(cell.get_data_type(), "s");
        }
    }

    #[test]
    fn set_value_still_parses_finite_numbers() {
        let mut cell = CellValue::default();

        cell.set_value("1.5");
        assert!(matches!(
            cell.get_raw_value(),
            CellRawValue::Numeric(number) if *number == 1.5
        ));

        cell.set_value("42");
        assert!(matches!(cell.get_raw_value(), CellRawValue::Integer(42)));
    }
}
