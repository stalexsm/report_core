use std::sync::Arc;

use ::pyo3::prelude::*;
use core_rs::structs::comment::Comment;
use parking_lot::RwLock;

/// Тип данных комментарий.
#[pyclass(from_py_object)]
#[pyo3(module = "report_core", name = "Comment")]
#[derive(Debug, Clone)]
pub struct WrapperComment(pub(crate) Arc<RwLock<Comment>>);

#[pymethods]
impl WrapperComment {
    /// Автор комментария
    #[getter]
    pub fn get_author(&self) -> PyResult<String> {
        let slf = self.0.read();
        let author = slf.get_author();

        Ok(author.into())
    }

    /// Текст комментария
    #[getter]
    pub fn get_text(&self) -> PyResult<String> {
        let slf = self.0.read();
        let author = slf.get_text();

        Ok(author.into())
    }

    /// Получение координаты комментария
    /// ---
    ///
    /// Returns:
    /// ---
    ///     tuple[int, int]
    #[getter]
    pub fn get_coordinate(&self) -> PyResult<(u32, u16)> {
        let slf = self.0.read();
        let coordinate = slf.get_coordinate();

        Ok((coordinate.row, coordinate.column))
    }

    /// Строка комментария
    #[getter]
    pub fn get_row(&self) -> PyResult<u32> {
        let slf = self.0.read();
        let coordinate = slf.get_coordinate();

        Ok(coordinate.row)
    }

    /// Колонка комментария
    #[getter]
    pub fn get_column(&self) -> PyResult<u16> {
        let slf = self.0.read();
        let coordinate = slf.get_coordinate();

        Ok(coordinate.column)
    }

    /// Установка автора комментария
    #[setter]
    pub fn set_author(&mut self, val: &str) -> PyResult<()> {
        let mut slf = self.0.write();
        slf.set_author(val);

        Ok(())
    }

    /// Установка текста комментария
    #[setter]
    pub fn set_text(&mut self, val: &str) -> PyResult<()> {
        let mut slf = self.0.write();
        slf.set_text(val);

        Ok(())
    }
}
