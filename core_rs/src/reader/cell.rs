use crate::datatype::CellValue;

#[derive(Clone, Debug, Default)]
pub struct XLSXSheetCellRead {
    pub row: u32,
    pub column: u16,
    pub cell: String,
    pub value: Box<CellValue>,
    pub formula: Option<String>,
    pub data_type: String,
    pub number_format: String,
    pub is_merge: bool,
    pub start_row: Option<u32>,
    pub end_row: Option<u32>,
    pub start_column: Option<u16>,
    pub end_column: Option<u16>,
    pub style_id: Option<String>,
    pub hidden_value: Option<String>,
    pub comment: Option<String>,
}

impl XLSXSheetCellRead {
    pub fn is_formula(&self) -> bool {
        self.formula.is_some() && self.data_type == "f"
    }

    pub fn is_value_bool(&self) -> bool {
        self.value.is_bool()
    }

    pub fn is_value_numeric(&self) -> bool {
        self.value.is_numeric()
    }

    pub fn is_value_integer(&self) -> bool {
        self.value.is_integer()
    }

    pub fn is_value_datetime(&self) -> bool {
        self.value.is_datetime()
    }

    pub fn is_value_empty(&self) -> bool {
        self.value.is_empty()
    }
}
