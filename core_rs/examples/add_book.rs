use std::time::Instant;

use core_rs::writer::book::XLSXBook;

fn main() -> anyhow::Result<()> {
    let start = Instant::now();

    let book = XLSXBook::new();
    let _ = book.lock().add_sheet("A".to_string(), None, None);

    let sheet = book.lock().get_sheet_name("A".to_string());
    println!("Find NameSheet {:?}", sheet.map(|s| s.lock().name.clone()));

    let sheet = book.lock().get_sheet_index(0);
    println!("Find Idx Sheet {:?}", sheet.map(|s| s.lock().name.clone()));

    let end = start.elapsed();
    println!(
        "Выполнено за: {}.{:03} сек.",
        end.as_secs(),
        end.subsec_millis(),
    );

    Ok(())
}
