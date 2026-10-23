//! Сериализация serde-типов прямо в Python-объекты.
//!
//! `to_dict` строит dict без промежуточной JSON-строки и обратного разбора:
//! пиковое потребление памяти не удваивается, а CPU-затраты на парсинг
//! исчезают. Семантика значений повторяет JSON: `None`/unit → `null`,
//! seq/кортежи → `list`, map/структуры → `dict`, варианты enum — по правилам
//! `serde_json` (unit-вариант — строка, остальные — `{вариант: значение}`).

use pyo3::{
    BoundObject,
    exceptions::PyValueError,
    prelude::*,
    types::{PyBytes, PyDict, PyList, PyString},
};
use serde::ser::{self, Error as _};

/// Ошибка сериализации: обёртка `PyErr`, реализующая `serde::ser::Error`.
///
/// Прямая реализация для `PyErr` невозможна (orphan rule), поэтому все
/// методы сериализатора отдают этот тип, а наружу он конвертируется в
/// `PyErr` через `From`.
#[derive(Debug)]
pub(crate) struct PySerError(PyErr);

impl std::fmt::Display for PySerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for PySerError {}

impl ser::Error for PySerError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        PySerError(PyValueError::new_err(msg.to_string()))
    }
}

impl From<PySerError> for PyErr {
    fn from(err: PySerError) -> PyErr {
        err.0
    }
}

impl From<PyErr> for PySerError {
    fn from(err: PyErr) -> Self {
        PySerError(err)
    }
}

/// Сериализатор `serde`, выдающий Python-объекты.
///
/// Требует GIL: построение Python-объектов без него невозможно, поэтому
/// вызывающий код держит его на время построения dict.
pub(crate) struct PySerializer<'py> {
    py: Python<'py>,
}

impl<'py> PySerializer<'py> {
    pub(crate) fn new(py: Python<'py>) -> Self {
        Self { py }
    }

    /// Python-объект из значения с готовой конверсией `IntoPyObject`
    /// (примитивы): цикл через `Serialize` недопустим — он снова попадает
    /// в этот сериализатор.
    fn convert<T>(self, v: T) -> Result<Py<PyAny>, PySerError>
    where
        T: IntoPyObject<'py>,
        T::Error: Into<PyErr>,
    {
        let bound = v.into_pyobject(self.py).map_err(Into::into)?;

        Ok(bound.into_any().unbind())
    }

    fn value<T: serde::Serialize + ?Sized>(&self, value: &T) -> Result<Py<PyAny>, PySerError> {
        value.serialize(PySerializer::new(self.py))
    }
}

impl<'py> serde::Serializer for PySerializer<'py> {
    type Ok = Py<PyAny>;
    type Error = PySerError;
    type SerializeSeq = PySeqSerializer<'py>;
    type SerializeTuple = PySeqSerializer<'py>;
    type SerializeTupleStruct = PySeqSerializer<'py>;
    type SerializeTupleVariant = PyVariantSerializer<'py>;
    type SerializeMap = PyMapSerializer<'py>;
    type SerializeStruct = PyMapSerializer<'py>;
    type SerializeStructVariant = PyVariantSerializer<'py>;

    fn serialize_bool(self, v: bool) -> Result<Py<PyAny>, PySerError> {
        self.convert(v)
    }

    fn serialize_i8(self, v: i8) -> Result<Py<PyAny>, PySerError> {
        self.convert(v)
    }

    fn serialize_i16(self, v: i16) -> Result<Py<PyAny>, PySerError> {
        self.convert(v)
    }

    fn serialize_i32(self, v: i32) -> Result<Py<PyAny>, PySerError> {
        self.convert(v)
    }

    fn serialize_i64(self, v: i64) -> Result<Py<PyAny>, PySerError> {
        self.convert(v)
    }

    fn serialize_i128(self, v: i128) -> Result<Py<PyAny>, PySerError> {
        self.convert(v)
    }

    fn serialize_u8(self, v: u8) -> Result<Py<PyAny>, PySerError> {
        self.convert(v)
    }

    fn serialize_u16(self, v: u16) -> Result<Py<PyAny>, PySerError> {
        self.convert(v)
    }

    fn serialize_u32(self, v: u32) -> Result<Py<PyAny>, PySerError> {
        self.convert(v)
    }

    fn serialize_u64(self, v: u64) -> Result<Py<PyAny>, PySerError> {
        self.convert(v)
    }

    fn serialize_u128(self, v: u128) -> Result<Py<PyAny>, PySerError> {
        self.convert(v)
    }

    fn serialize_f32(self, v: f32) -> Result<Py<PyAny>, PySerError> {
        self.convert(v)
    }

    fn serialize_f64(self, v: f64) -> Result<Py<PyAny>, PySerError> {
        self.convert(v)
    }

    fn serialize_char(self, v: char) -> Result<Py<PyAny>, PySerError> {
        let mut buf = [0u8; 4];

        self.serialize_str(v.encode_utf8(&mut buf))
    }

    fn serialize_str(self, v: &str) -> Result<Py<PyAny>, PySerError> {
        Ok(PyString::new(self.py, v).into_any().unbind())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Py<PyAny>, PySerError> {
        Ok(PyBytes::new(self.py, v).into_any().unbind())
    }

    fn serialize_none(self) -> Result<Py<PyAny>, PySerError> {
        Ok(self.py.None())
    }

    fn serialize_some<T>(self, value: &T) -> Result<Py<PyAny>, PySerError>
    where
        T: serde::Serialize + ?Sized,
    {
        self.value(value)
    }

    fn serialize_unit(self) -> Result<Py<PyAny>, PySerError> {
        self.serialize_none()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Py<PyAny>, PySerError> {
        self.serialize_none()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _index: u32,
        variant: &'static str,
    ) -> Result<Py<PyAny>, PySerError> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Py<PyAny>, PySerError>
    where
        T: serde::Serialize + ?Sized,
    {
        self.value(value)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Py<PyAny>, PySerError>
    where
        T: serde::Serialize + ?Sized,
    {
        let dict = PyDict::new(self.py);

        dict.set_item(variant, self.value(value)?)?;

        Ok(dict.into_any().unbind())
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, PySerError> {
        Ok(PySeqSerializer::new(self.py, len))
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, PySerError> {
        Ok(PySeqSerializer::new(self.py, Some(len)))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, PySerError> {
        Ok(PySeqSerializer::new(self.py, Some(len)))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, PySerError> {
        Ok(PyVariantSerializer::tuple(self.py, variant, len))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, PySerError> {
        Ok(PyMapSerializer::new(self.py))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, PySerError> {
        Ok(PyMapSerializer::new(self.py))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, PySerError> {
        Ok(PyVariantSerializer::dict(self.py, variant))
    }
}

/// Накопитель элементов seq/кортежа с финальной сборкой `list`.
pub(crate) struct PySeqSerializer<'py> {
    py: Python<'py>,
    items: Vec<Py<PyAny>>,
}

impl<'py> PySeqSerializer<'py> {
    fn new(py: Python<'py>, len: Option<usize>) -> Self {
        Self {
            py,
            items: Vec::with_capacity(len.unwrap_or(0)),
        }
    }

    fn push<T: serde::Serialize + ?Sized>(&mut self, value: &T) -> Result<(), PySerError> {
        self.items
            .push(value.serialize(PySerializer::new(self.py))?);

        Ok(())
    }

    fn finish(self) -> Result<Py<PyAny>, PySerError> {
        Ok(PyList::new(self.py, &self.items)?.into_any().unbind())
    }
}

impl ser::SerializeSeq for PySeqSerializer<'_> {
    type Ok = Py<PyAny>;
    type Error = PySerError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize + ?Sized,
    {
        self.push(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.finish()
    }
}

impl ser::SerializeTuple for PySeqSerializer<'_> {
    type Ok = Py<PyAny>;
    type Error = PySerError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize + ?Sized,
    {
        self.push(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.finish()
    }
}

impl ser::SerializeTupleStruct for PySeqSerializer<'_> {
    type Ok = Py<PyAny>;
    type Error = PySerError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize + ?Sized,
    {
        self.push(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.finish()
    }
}

/// Накопитель пар ключ-значение с финальной сборкой `dict`.
pub(crate) struct PyMapSerializer<'py> {
    py: Python<'py>,
    dict: Bound<'py, PyDict>,
    last_key: Option<Py<PyAny>>,
}

impl<'py> PyMapSerializer<'py> {
    fn new(py: Python<'py>) -> Self {
        Self {
            py,
            dict: PyDict::new(py),
            last_key: None,
        }
    }

    fn finish(self) -> Result<Py<PyAny>, PySerError> {
        Ok(self.dict.into_any().unbind())
    }
}

impl ser::SerializeMap for PyMapSerializer<'_> {
    type Ok = Py<PyAny>;
    type Error = PySerError;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize + ?Sized,
    {
        self.last_key = Some(key.serialize(PySerializer::new(self.py))?);

        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize + ?Sized,
    {
        let key = self
            .last_key
            .take()
            .expect("serialize_value вызван без serialize_key");

        self.dict
            .set_item(key, value.serialize(PySerializer::new(self.py))?)?;

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.finish()
    }
}

impl ser::SerializeStruct for PyMapSerializer<'_> {
    type Ok = Py<PyAny>;
    type Error = PySerError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize + ?Sized,
    {
        self.dict
            .set_item(key, value.serialize(PySerializer::new(self.py))?)?;

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.finish()
    }
}

/// Накопитель варианта enum: `{вариант: [значения]}` (кортежный) или
/// `{вариант: {поля...}}` (структурный) — как в `serde_json`.
pub(crate) struct PyVariantSerializer<'py> {
    py: Python<'py>,
    variant: &'static str,
    inner: VariantInner<'py>,
}

enum VariantInner<'py> {
    /// Значения кортежного варианта, собираются в список
    Items(Vec<Py<PyAny>>),
    /// Поля структурного варианта, собираются в dict
    Fields(Bound<'py, PyDict>),
}

impl<'py> PyVariantSerializer<'py> {
    fn tuple(py: Python<'py>, variant: &'static str, len: usize) -> Self {
        Self {
            py,
            variant,
            inner: VariantInner::Items(Vec::with_capacity(len)),
        }
    }

    fn dict(py: Python<'py>, variant: &'static str) -> Self {
        Self {
            py,
            variant,
            inner: VariantInner::Fields(PyDict::new(py)),
        }
    }

    fn finish(self) -> Result<Py<PyAny>, PySerError> {
        let dict = PyDict::new(self.py);
        let value = match self.inner {
            VariantInner::Items(items) => PyList::new(self.py, &items)?.into_any().unbind(),
            VariantInner::Fields(fields) => fields.into_any().unbind(),
        };

        dict.set_item(self.variant, value)?;

        Ok(dict.into_any().unbind())
    }
}

impl ser::SerializeTupleVariant for PyVariantSerializer<'_> {
    type Ok = Py<PyAny>;
    type Error = PySerError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize + ?Sized,
    {
        let VariantInner::Items(items) = &mut self.inner else {
            return Err(PySerError::custom(
                "кортежный вариант перезаписан структурным",
            ));
        };

        items.push(value.serialize(PySerializer::new(self.py))?);

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.finish()
    }
}

impl ser::SerializeStructVariant for PyVariantSerializer<'_> {
    type Ok = Py<PyAny>;
    type Error = PySerError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize + ?Sized,
    {
        let VariantInner::Fields(fields) = &mut self.inner else {
            return Err(PySerError::custom(
                "структурный вариант перезаписан кортежным",
            ));
        };

        fields.set_item(key, value.serialize(PySerializer::new(self.py))?)?;

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.finish()
    }
}
