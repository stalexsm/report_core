use serde::Serialize;
use std::collections::HashMap;

use super::row::Row;

#[derive(Clone, Default, Debug, Serialize)]
pub(crate) struct _Rows {
    rows: HashMap<u32, Box<Row>>,
}
