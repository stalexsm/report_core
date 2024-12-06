use std::time::Instant;

use core_rs::writer::book::XLSXBook;

fn main() -> anyhow::Result<()> {
    let start = Instant::now();
    let book = XLSXBook::new();
    let sheet = book
        .lock()
        .unwrap()
        .add_sheet("A".to_string(), Some(50), Some(30));

    let mut guard_sheet = sheet.lock().unwrap();
    guard_sheet.set_merged_cells(1, 10, 1, 10)?;

    for cell in guard_sheet.cells() {
        let cell = cell.lock().unwrap();

        if cell.is_merge {
            println!(
                "{} -> {}:{} __ is_merge: {:?} [{:?}, {:?}, {:?}, {:?}]",
                cell.cell,
                cell.row,
                cell.column,
                cell.is_merge,
                cell.start_row,
                cell.end_row,
                cell.start_column,
                cell.end_column
            );
        }
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
