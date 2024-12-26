use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Column {
    col_num: u16,
    pub(crate) width: f64,
    pub(crate) hidden: bool,
}
