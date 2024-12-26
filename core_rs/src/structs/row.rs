use serde::Serialize;

#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
pub struct Row {
    row_num: u32,
    height: f64,
    hidden: bool,
}
