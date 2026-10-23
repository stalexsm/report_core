use pyo3::{
    exceptions::{PyAttributeError, PyKeyError, PyValueError},
    prelude::*,
    types::{PyBool, PyDict, PyFloat, PyInt, PyList, PyString, PyTuple},
};

// Универсальный enum для всех Python типов
#[derive(Debug, Clone)]
pub enum PyValue {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    List(Vec<PyValue>),
    None,
}

impl PyValue {
    // Методы для конвертации в конкретные типы
    pub fn as_string(&self) -> Option<String> {
        match self {
            PyValue::String(s) => Some(s.clone()),
            PyValue::Int(i) => Some(i.to_string()),
            PyValue::Float(f) => Some(f.to_string()),
            PyValue::Bool(b) => Some(b.to_string()),
            PyValue::List(list) => {
                let strings: Vec<String> = list.iter().map(|v| v.as_string_direct()).collect();
                Some(format!("[{}]", strings.join(", ")))
            }
            PyValue::None => None,
        }
    }

    /// Строгое преобразование в `u32`: `ValueError` вместо тихих
    /// `as`-приведений и `unwrap_or` — мусорное значение не превращается
    /// молча в валидную координату.
    pub fn as_u32(&self) -> PyResult<u32> {
        self.strict_unsigned(
            "u32",
            |i| u32::try_from(i).ok(),
            f64_to_u32,
            |s| s.parse().ok(),
        )
    }

    /// Строгое преобразование в `u16`, см. `as_u32`.
    pub fn as_u16(&self) -> PyResult<u16> {
        self.strict_unsigned(
            "u16",
            |i| u16::try_from(i).ok(),
            f64_to_u16,
            |s| s.parse().ok(),
        )
    }

    /// Общая механика строгих конверсий: `Int` — через `try_from`, `Float` —
    /// только целые в диапазоне, `String` — через `parse`; всё остальное —
    /// `ValueError` с Python-представлением значения.
    fn strict_unsigned<T>(
        &self,
        type_name: &str,
        from_int: impl Fn(i64) -> Option<T>,
        from_float: impl Fn(f64) -> Option<T>,
        from_str: impl Fn(&str) -> Option<T>,
    ) -> PyResult<T> {
        let (repr, value) = match self {
            PyValue::Int(i) => (i.to_string(), from_int(*i)),
            PyValue::Float(f) => (f.to_string(), from_float(*f)),
            PyValue::String(s) => (format!("{s:?}"), from_str(s)),
            PyValue::Bool(_) | PyValue::List(_) | PyValue::None => (self.py_repr(), None),
        };

        value.ok_or_else(|| {
            PyValueError::new_err(format!(
                "expected unsigned integer ({type_name}), got: {repr}"
            ))
        })
    }

    /// Python-подобное представление значения для сообщений об ошибках.
    pub(crate) fn py_repr(&self) -> String {
        match self {
            PyValue::String(s) => format!("{s:?}"),
            PyValue::Int(i) => i.to_string(),
            PyValue::Float(f) => f.to_string(),
            PyValue::Bool(b) => {
                if *b {
                    "True".to_string()
                } else {
                    "False".to_string()
                }
            }
            PyValue::List(list) => {
                let items: Vec<String> = list.iter().map(PyValue::py_repr).collect();
                format!("[{}]", items.join(", "))
            }
            PyValue::None => "None".to_string(),
        }
    }

    pub fn as_string_direct(&self) -> String {
        match self {
            PyValue::String(s) => s.to_string(),
            PyValue::Int(i) => i.to_string(),
            PyValue::Float(f) => f.to_string(),
            PyValue::Bool(b) => b.to_string(),
            PyValue::List(list) => {
                let strings: Vec<String> = list.iter().map(|v| v.as_string_direct()).collect();
                format!("[{}]", strings.join(", "))
            }
            PyValue::None => String::new(),
        }
    }
}

/// Строгая конверсия `f64` в `u32`: только целые числа в диапазоне типа.
fn f64_to_u32(f: f64) -> Option<u32> {
    if f.is_finite() && f >= 0.0 && f <= u32::MAX as f64 && f.fract() == 0.0 {
        Some(f as u32)
    } else {
        None
    }
}

/// Строгая конверсия `f64` в `u16`: только целые числа в диапазоне типа.
fn f64_to_u16(f: f64) -> Option<u16> {
    if f.is_finite() && f >= 0.0 && f <= u16::MAX as f64 && f.fract() == 0.0 {
        Some(f as u16)
    } else {
        None
    }
}

// Функция для автоматического извлечения Python значения
pub(crate) fn extract_py_value_auto(py_value: &Bound<'_, PyAny>) -> PyResult<PyValue> {
    if py_value.is_none() {
        Ok(PyValue::None)
    } else if py_value.is_instance_of::<PyBool>() {
        // Проверяем bool перед int, так как bool является подклассом int в Python
        Ok(PyValue::Bool(py_value.extract::<bool>()?))
    } else if py_value.is_instance_of::<PyInt>() {
        Ok(PyValue::Int(py_value.extract::<i64>()?))
    } else if py_value.is_instance_of::<PyFloat>() {
        Ok(PyValue::Float(py_value.extract::<f64>()?))
    } else if py_value.is_instance_of::<PyString>() {
        Ok(PyValue::String(py_value.extract::<String>()?))
    } else if py_value.is_instance_of::<PyList>() || py_value.is_instance_of::<PyTuple>() {
        // Кортежи обрабатываем как списки: публичный API библиотеки возвращает
        // list[tuple] (например, Sheet.merge_cells), и readable-слой обязан
        // читать собственные типы, а не только duck-typed объекты.
        let mut values = Vec::new();
        for item in py_value.try_iter()? {
            values.push(extract_py_value_auto(&item?)?);
        }
        Ok(PyValue::List(values))
    } else {
        // Для любого другого типа пытаемся преобразовать в строку
        Ok(PyValue::String(py_value.str()?.to_string()))
    }
}

// Один универсальный макрос для извлечения любого атрибута
//
// Цепочка имён через `|` — фолбэки: берётся первый существующий атрибут
// (для readable-типов имена идут от duck-typed объектов к собственным,
// например `style_id | style`).
#[macro_export]
macro_rules! py_extract {
    ($obj:expr, $attr:ident $(| $fallback:ident)*) => {
        $crate::extract_py_attr_chain(
            &$obj,
            &[stringify!($attr) $(, stringify!($fallback))*],
        )
    };
}

/// Извлечение атрибута по цепочке имён: первый найденный побеждает. Для dict
/// отсутствие имени — это `KeyError`, для объекта — `AttributeError`; любой
/// другой переход к следующему имени не считается и отдаётся как есть.
pub(crate) fn extract_py_attr_chain(obj: &Bound<'_, PyAny>, attrs: &[&str]) -> PyResult<PyValue> {
    let is_dict = obj.is_instance_of::<PyDict>();

    for (idx, attr) in attrs.iter().enumerate() {
        let last = idx + 1 == attrs.len();

        let value = if is_dict {
            obj.get_item(*attr)
        } else {
            obj.getattr(*attr)
        };

        match value {
            Ok(value) => return extract_py_value_auto(&value),
            Err(err)
                if !last && (is_dict && err.is_instance_of::<PyKeyError>(obj.py()))
                    || (!is_dict && err.is_instance_of::<PyAttributeError>(obj.py())) =>
            {
                continue;
            }
            Err(err) => return Err(err),
        }
    }

    unreachable!("attrs не пуст: макрос гарантирует хотя бы одно имя")
}
