use serde::Serialize;

use super::column::Column;

#[derive(Clone, Default, Debug, Serialize)]
pub(crate) struct _Columns {
    column: Vec<Column>,
}
