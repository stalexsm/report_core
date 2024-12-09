use std::time::Instant;

use core_rs::writer::book::XLSXBook;

fn main() -> anyhow::Result<()> {
    let start = Instant::now();
    let book = XLSXBook::new();

    let sheet = book.lock().add_sheet("A".to_string(), Some(50), Some(30));
    let mut guard_sheet = sheet.lock();

    for cell in guard_sheet.cells() {
        let mut cell = cell.lock();
        let f = format!("Row: {}, Col: {}", cell.row, cell.column);

        cell.set_value(f)?;
    }

    guard_sheet.delete_cols(3, 28)?;
    guard_sheet.delete_rows(6, 45)?;

    for cell in guard_sheet.cells() {
        let cell = cell.lock();

        println!(
            "Cell {:?}, Row: {}, Col: {}, Value: {:?}",
            cell.cell, cell.row, cell.column, cell.value
        );
    }

    let end = start.elapsed();
    println!(
        "Выполнено за: {}.{:03} сек. Кол-во ячеек: {}",
        end.as_secs(),
        end.subsec_millis(),
        guard_sheet.cells().collect::<Vec<_>>().len()
    );

    Ok(())
}
