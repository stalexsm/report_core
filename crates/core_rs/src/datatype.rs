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
    pub(crate) fn get_data_type(&self) -> &str {
        match &self {
            Self::String(_) => "s",
            Self::Integer(_) => "n",
            Self::Numeric(_) => "n",
            Self::Bool(_) => "b",
            Self::Datetime(_) => "d",
            _ => "s",
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
        self.raw_value.get_data_type()
    }

    #[inline]
    pub fn get_value(&self) -> String {
        self.raw_value.to_string()
    }

    /// Сравнение отрисованного значения с образцом (уже обрезанным) без
    /// промежуточной String: для строковых ячеек — сравнение ссылки, для
    /// остальных типов отрисовка неизбежна.
    #[inline]
    pub(crate) fn value_eq_str(&self, needle: &str) -> bool {
        match &self.raw_value {
            CellRawValue::String(s) => s.trim() == needle,
            raw => raw.to_string().trim() == needle,
        }
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
    pub fn set_value_datetime(&mut self, value: NaiveDateTime) -> &mut Self {
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
        // TRUE/FALSE распознаются без аллокации: регистр учитывается только
        // для ASCII-литералов, а to_uppercase() на каждое значение — лишний
        // проход с аллокацией строки в горячем пути set_value.
        match value {
            "" => CellRawValue::Empty,
            v if v.eq_ignore_ascii_case("true") => CellRawValue::Bool(true),
            v if v.eq_ignore_ascii_case("false") => CellRawValue::Bool(false),
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

            assert!(!cell.is_empty(), "значение {value} не должно быть пустым");
            assert_eq!(
                cell.get_data_type(),
                "s",
                "значение {value} должно остаться строкой"
            );
        }
    }

    #[test]
    fn set_value_recognizes_bool_in_any_case() {
        // Регистр литералов true/false не важен, распознавание — без аллокации
        for (raw, expected) in [
            ("true", true),
            ("TRUE", true),
            ("True", true),
            ("false", false),
            ("FALSE", false),
            ("False", false),
        ] {
            let mut cell = CellValue::default();
            cell.set_value(raw);

            assert!(
                cell.is_bool(),
                "значение {raw} должно распознаться как Bool"
            );
            assert_eq!(cell.get_value(), expected.to_string());
        }
    }

    #[test]
    fn set_value_unicode_lookalikes_stay_strings() {
        // Регистр учитывается только для ASCII: символы, совпадающие с
        // литералами лишь после Unicode-приведения ('ſ' → 'S'), остаются
        // строками — тип и значение не расходятся
        for raw in ["ſalse", "ſtrue", "TURE"] {
            let mut cell = CellValue::default();
            cell.set_value(raw);

            assert!(!cell.is_empty(), "значение {raw} не должно быть пустым");
            assert_eq!(
                cell.get_data_type(),
                "s",
                "значение {raw} должно остаться строкой"
            );
        }
    }

    #[test]
    fn set_value_still_parses_finite_numbers() {
        let mut cell = CellValue::default();

        cell.set_value("1.5");
        assert!(cell.is_numeric(), "значение 1.5 должно стать числом");
        assert_eq!(cell.get_value(), "1.5");

        cell.set_value("42");
        assert!(cell.is_integer(), "значение 42 должно стать целым числом");
        assert_eq!(cell.get_value(), "42");
    }
}
