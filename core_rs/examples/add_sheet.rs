use std::time::Instant;

use core_rs::writer::book::XLSXBook;

fn main() -> anyhow::Result<()> {
    let start = Instant::now();

    let book = XLSXBook::new();

    let sheet = book.lock().add_sheet("A".to_string(), Some(50), Some(30));

    for cell in sheet.lock().cells() {
        let mut cell = cell.lock();

        if cell.row == 2 && cell.column == 2 {
            cell.set_value("100".to_string())?;
        }

        if cell.row == 3 && cell.column == 3 {
            cell.set_value("100.0".to_string())?;
        }

        if cell.row == 5 && cell.column == 5 {
            cell.set_value("AAAAA".to_string())?;
            cell.set_formula("=SUM:A10".to_string())?;
        }
    }

    let cell = sheet.lock().find_cell_by_coords(2, 2)?;
    println!("Find Coords {:?}", cell);

    let cell = sheet.lock().find_cell_by_coords(3, 3)?;
    println!("Find Coords {:?}", cell);

    let cell = sheet.lock().find_cell_by_coords(5, 5)?;
    println!("Find Coords {:?}", cell);

    let end = start.elapsed();
    println!(
        "Выполнено за: {}.{:03} сек.",
        end.as_secs(),
        end.subsec_millis(),
    );

    let start = Instant::now();
    let sheet = book.lock().add_sheet("B".to_string(), Some(50), Some(30));

    for r in 1..=1000 {
        for c in 1..=1000 {
            let cell = sheet.lock().write_cell(r, c, &format!("Yop! {}{}", r, c))?;

            let mut guarg_cell = cell.lock();
            if guarg_cell.row == 20 && guarg_cell.column == 20 {
                guarg_cell.set_value("AAAAA".to_string())?;
                guarg_cell.set_formula("=SUM(A1:A10)".to_string())?;
            }
        }
    }

    sheet.lock().write_cell(120, 120, "Yop! 120x120")?;

    sheet.lock().generate_empty_cells()?;

    println!(
        "Sheet len cells {:?}",
        sheet.lock().cells().collect::<Vec<_>>().len()
    );

    let cell = sheet.lock().find_cell_by_coords(1, 1)?;
    println!("Find Coords 1x1 {:?}", cell);

    let cell = sheet.lock().find_cell_by_coords(20, 20)?;
    println!("Find Coords 20x20 {:?}", cell);

    let cell = sheet.lock().find_cell_by_coords(105, 105)?;
    println!("Find Coords 105x105 {:?}", cell);

    let end = start.elapsed();
    println!(
        "Выполнено за: {}.{:03} сек.",
        end.as_secs(),
        end.subsec_millis(),
    );

    Ok(())
}
