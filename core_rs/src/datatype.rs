use std::fmt::Display;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum CellRawValue {
    #[default]
    Empty,

    String(String),
    Numeric(f64),
    Integer(i32),
    Bool(bool),
    Datetime(NaiveDateTime),
}

impl CellRawValue {
    /// Метод для получения типа данных.
    pub(crate) fn get_date_type(&self) -> &str {
        match &self {
            Self::String(_) => "s",
            Self::Integer(_) => "n",
            Self::Numeric(_) => "n",
            Self::Bool(_) => "b",
            Self::Datetime(_) => "d",
            _ => "",
        }
    }

    /// Метод для получения данных тип String.
    pub(crate) fn get_value_str(&self) -> String {
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

#[derive(Debug, Clone, Default)]
pub struct CellValue {
    pub raw_value: CellRawValue,
}

impl Serialize for CellValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.raw_value.get_value_str())
    }
}

impl CellValue {
    /// Метод для получения типа ячейки.
    pub fn get_data_type(&self) -> &str {
        self.raw_value.get_date_type()
    }

    /// Метод для получения значения ячейки.
    pub fn get_value_str(&self) -> String {
        self.raw_value.get_value_str()
    }

    /// Метод для установки значения как Number
    pub fn set_value_number(&mut self, value: f64) -> &mut Self {
        self.raw_value = CellRawValue::Numeric(value);

        self
    }

    /// Метод для установки значения как Integer
    pub fn set_value_integer(&mut self, value: i32) -> &mut Self {
        self.raw_value = CellRawValue::Integer(value);

        self
    }

    /// Метод для установки значения как Boolean
    pub fn set_value_bool(&mut self, value: bool) -> &mut Self {
        self.raw_value = CellRawValue::Bool(value);

        self
    }

    /// Метод для установки значения как String
    pub fn set_value_str(&mut self, value: String) -> &mut Self {
        self.raw_value = CellRawValue::String(value);

        self
    }

    /// Метод для установки значения как String
    pub fn set_value_datatime(&mut self, value: NaiveDateTime) -> &mut Self {
        self.raw_value = CellRawValue::Datetime(value);

        self
    }

    /// Метод для установки значения как String
    pub fn set_value(&mut self, value: String) -> &mut Self {
        self.raw_value = Self::quess_typed_value(&value);

        self
    }

    /// Проверить, является ли значение ячейки boolean
    pub fn is_bool(&self) -> bool {
        matches!(self.raw_value, CellRawValue::Bool(_))
    }

    /// Проверить, является ли значение ячейки numeric
    pub fn is_numeric(&self) -> bool {
        matches!(self.raw_value, CellRawValue::Numeric(_))
    }

    /// Проверить, является ли значение ячейки numeric
    pub fn is_integer(&self) -> bool {
        matches!(self.raw_value, CellRawValue::Integer(_))
    }

    /// Проверить, является ли значение ячейки Datetime
    pub fn is_datetime(&self) -> bool {
        matches!(self.raw_value, CellRawValue::Datetime(_))
    }

    /// Проверить, является ли значение ячейки Empty
    pub fn is_empty(&self) -> bool {
        matches!(self.raw_value, CellRawValue::Empty)
    }

    /// Метод для определения типа значения
    pub fn quess_typed_value(value: &str) -> CellRawValue {
        match value {
            "" => CellRawValue::Empty,
            _ => {
                if let Ok(number) = value.parse::<i32>() {
                    CellRawValue::Integer(number)
                } else if let Ok(number) = value.parse::<f64>() {
                    CellRawValue::Numeric(number)
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
