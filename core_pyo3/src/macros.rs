use pyo3::{
    prelude::*,
    types::{PyBool, PyFloat, PyInt, PyList, PyString},
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

    pub fn as_u32(&self) -> u32 {
        match self {
            PyValue::Int(i) => *i as u32,
            PyValue::Float(f) => *f as u32,
            PyValue::String(s) => s.parse().unwrap_or(0),
            PyValue::Bool(b) => {
                if *b {
                    1
                } else {
                    0
                }
            }
            PyValue::List(_) => 0, // Для списков возвращаем 0
            PyValue::None => 0,
        }
    }

    pub fn as_u16(&self) -> u16 {
        match self {
            PyValue::Int(i) => *i as u16,
            PyValue::Float(f) => *f as u16,
            PyValue::String(s) => s.parse().unwrap_or(0),
            PyValue::Bool(b) => {
                if *b {
                    1
                } else {
                    0
                }
            }
            PyValue::List(_) => 0, // Для списков возвращаем 0
            PyValue::None => 0,
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

    // Generic метод для получения Vec<T>
    pub fn as_vec<T>(&self) -> Vec<T>
    where
        T: From<PyValue> + Clone,
    {
        match self {
            PyValue::List(list) => list.iter().map(|v| T::from(v.clone())).collect(),
            _ => vec![], // Для не-списков возвращаем пустой Vec
        }
    }

    // Generic метод для получения массива фиксированного размера
    fn as_array<T, const N: usize>(&self) -> [T; N]
    where
        T: From<PyValue> + Copy + Clone + Default,
    {
        match self {
            PyValue::List(list) => {
                let mut result = [T::default(); N];
                for (i, item) in list.iter().enumerate().take(N) {
                    result[i] = T::from(item.clone());
                }
                result
            }
            _ => [T::default(); N], // Для не-списков возвращаем массив с default значениями
        }
    }

    fn as_vec_array<T, const N: usize>(&self) -> Vec<[T; N]>
    where
        T: From<PyValue> + Copy + Clone + Default,
    {
        match self {
            PyValue::List(list) => {
                list.iter()
                    .map(|item| match item {
                        PyValue::List(inner_list) => {
                            let mut result = [T::default(); N];
                            for (i, inner_item) in inner_list.iter().enumerate().take(N) {
                                result[i] = T::from(inner_item.clone());
                            }
                            result
                        }
                        // Если элемент не список, создаем массив с одним элементом
                        _ => {
                            let mut result = [T::default(); N];
                            result[0] = T::from(item.clone());
                            result
                        }
                    })
                    .collect()
            }
            _ => vec![], // Для не-списков возвращаем пустой Vec
        }
    }

    // Специализированные методы для удобства
    pub fn as_u32_vec(&self) -> Vec<u32> {
        self.as_vec::<u32>()
    }

    pub fn as_u16_vec(&self) -> Vec<u16> {
        self.as_vec::<u16>()
    }

    pub fn as_string_vec(&self) -> Vec<String> {
        self.as_vec::<String>()
    }

    // Методы для получения массивов фиксированного размера
    pub fn as_u32_array<const N: usize>(&self) -> [u32; N] {
        self.as_array::<u32, N>()
    }

    pub fn as_u16_array<const N: usize>(&self) -> [u16; N] {
        self.as_array::<u16, N>()
    }

    pub fn as_u32_vec_array<const N: usize>(&self) -> Vec<[u32; N]> {
        self.as_vec_array::<u32, N>()
    }

    pub fn as_u16_vec_array<const N: usize>(&self) -> Vec<[u16; N]> {
        self.as_vec_array::<u16, N>()
    }

    pub fn as_i64_vec_array<const N: usize>(&self) -> Vec<[i64; N]> {
        self.as_vec_array::<i64, N>()
    }

    pub fn as_f64_vec_array<const N: usize>(&self) -> Vec<[f64; N]> {
        self.as_vec_array::<f64, N>()
    }
}

// Реализация From для автоматического преобразования
impl From<PyValue> for u32 {
    fn from(value: PyValue) -> Self {
        value.as_u32()
    }
}

impl From<PyValue> for u16 {
    fn from(value: PyValue) -> Self {
        value.as_u16()
    }
}

impl From<PyValue> for String {
    fn from(value: PyValue) -> Self {
        value.as_string_direct()
    }
}

impl From<PyValue> for i64 {
    fn from(value: PyValue) -> Self {
        match value {
            PyValue::Int(i) => i,
            PyValue::Float(f) => f as i64,
            PyValue::String(s) => s.parse().unwrap_or(0),
            PyValue::Bool(b) => {
                if b {
                    1
                } else {
                    0
                }
            }
            PyValue::List(_) => 0,
            PyValue::None => 0,
        }
    }
}

impl From<PyValue> for f64 {
    fn from(value: PyValue) -> Self {
        match value {
            PyValue::Int(i) => i as f64,
            PyValue::Float(f) => f,
            PyValue::String(s) => s.parse().unwrap_or(0.0),
            PyValue::Bool(b) => {
                if b {
                    1.0
                } else {
                    0.0
                }
            }
            PyValue::List(_) => 0.0,
            PyValue::None => 0.0,
        }
    }
}

impl From<PyValue> for bool {
    fn from(value: PyValue) -> Self {
        match value {
            PyValue::Bool(b) => b,
            PyValue::Int(i) => i != 0,
            PyValue::Float(f) => f != 0.0,
            PyValue::String(s) => !s.is_empty(),
            PyValue::List(list) => !list.is_empty(),
            PyValue::None => false,
        }
    }
}

// Функция для автоматического извлечения Python значения
pub(crate) fn extract_py_value_auto(py_value: &Bound<'_, PyAny>) -> PyValue {
    if py_value.is_none() {
        PyValue::None
    } else if py_value.is_instance_of::<PyBool>() {
        // Проверяем bool перед int, так как bool является подклассом int в Python
        PyValue::Bool(py_value.extract::<bool>().unwrap())
    } else if py_value.is_instance_of::<PyInt>() {
        PyValue::Int(py_value.extract::<i64>().unwrap())
    } else if py_value.is_instance_of::<PyFloat>() {
        PyValue::Float(py_value.extract::<f64>().unwrap())
    } else if py_value.is_instance_of::<PyString>() {
        PyValue::String(py_value.extract::<String>().unwrap())
    } else if py_value.is_instance_of::<PyList>() {
        let py_list = py_value.cast::<PyList>().unwrap();
        let mut values = Vec::new();
        for item in py_list.iter() {
            values.push(extract_py_value_auto(&item));
        }
        PyValue::List(values)
    } else {
        // Для любого другого типа пытаемся преобразовать в строку
        PyValue::String(py_value.str().unwrap().to_string())
    }
}

// Один универсальный макрос для извлечения любого атрибута
#[macro_export]
macro_rules! py_extract {
    ($obj:expr, $attr:ident) => {{
        let py_value = if $obj.is_instance_of::<PyDict>() {
            $obj.get_item(stringify!($attr)).unwrap()
        } else {
            $obj.getattr(stringify!($attr)).unwrap()
        };

        $crate::extract_py_value_auto(&py_value)
    }};
}
