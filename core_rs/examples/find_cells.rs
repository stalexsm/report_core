use core_rs::{
    datatype::{CellRawValue, CellValue},
    reader::{cell::XLSXSheetCellRead, sheet::XLSXSheetRead},
    utils::column_number_to_letter,
};

fn main() -> anyhow::Result<()> {
    let mut sheet = XLSXSheetRead {
        name: "A".to_string(),
        max_row: 50,
        max_column: 30,
        index: 1,
        ..Default::default()
    };

    for r in 1..=100 {
        for c in 1..=5 {
            let mut value = String::new();
            if r == 5 && c == 1 {
                value.push_str("Диаметр 20:");
            } else if r == 20 && c == 1 {
                value.push_str("Диаметр 40:");
            } else {
                value.push_str(&format!("Yop! {}:{}", r, c));
            }

            let cell = XLSXSheetCellRead {
                row: r,
                column: c,
                cell: format!("{}{}", column_number_to_letter(c), r),
                value: Box::new(CellValue {
                    raw_value: CellRawValue::String(value),
                }),
                formula: Some("SUM:A10".to_string()),
                data_type: "s".to_string(),
                number_format: "".to_string(),
                is_merge: false,
                start_row: None,
                end_row: None,
                start_column: None,
                end_column: None,
                style_id: None,
                hidden_value: None,
                comment: None,
            };

            // Add
            sheet._cells.insert((r, c), cell);
        }
    }

    let cell = sheet.find_cell_by_coords(5, 5)?;
    println!("Find Coords {:?}", cell);

    let cell = sheet.find_cell_by_cell("A1")?;
    println!("Find Cell {:?}", cell);

    let cells = sheet.find_cells_between_patterns("Диаметр 20:", "Диаметр 40:")?;
    for cell in cells {
        println!("Find Beetwen Cell {:?}", cell.cell);
    }

    Ok(())
}
