use std::sync::Arc;

use crate::error::Result;
use ahash::HashMap;
use parking_lot::RwLock;
use serde::Serialize;

use crate::traits::{ReadableSheet, WriteableSheet};

use super::{
    cell::Cell, cells::Cells, columns::Columns, comment::Comment, coordinate::Coordinate,
    merge_cells::MergeCells, range::Range, rows::Rows,
};

#[derive(Clone, Debug, Default, Serialize)]
pub struct Sheet {
    name: String,
    sheet_state: Box<str>,
    #[serde(flatten)]
    merge_cells: MergeCells,
    #[serde(flatten)]
    cells: Cells,
    row_dimensions: Rows,
    column_dimensions: Columns,
    comments: Vec<Arc<RwLock<Comment>>>,
}

impl Sheet {
    /// Интициализирует лист с заданным именем
    pub fn new(name: &str, sheet_state: &str) -> Self {
        Sheet {
            name: name.to_string(),
            sheet_state: sheet_state.into(),
            ..Default::default()
        }
    }

    #[inline]
    pub fn extract(
        name: &str,
        sheet_state: &str,
        range: Vec<Range>,
        map: HashMap<(u32, u16), Arc<RwLock<Cell>>>,
    ) -> Self {
        Sheet {
            name: name.to_string(),
            sheet_state: sheet_state.into(),
            merge_cells: MergeCells::new(range),
            cells: Cells::new(map),
            ..Default::default()
        }
    }

    /// Глубокая копия листа: ячейки и комментарии получают собственные обёртки
    /// `Arc<RwLock<..>>`, чтобы мутации и `delete_rows/cols` на копии
    /// не меняли оригинал.
    pub fn deep_clone(&self) -> Self {
        Sheet {
            cells: self.cells.clone_deep(),
            comments: self
                .comments
                .iter()
                .map(|comment| Arc::new(RwLock::new(comment.read().clone())))
                .collect(),
            ..self.clone()
        }
    }

    /// Сдвигает координаты комментариев при удалении строк `[idx; idx + amount)`,
    /// комментарии из удаляемого диапазона удаляются — как ячейки.
    ///
    /// Ожидается валидный диапазон удаления: `idx ≥ 1`, `amount ≥ 1`.
    fn remap_comments_rows(&mut self, idx: u32, amount: u32) {
        let delete_end = (idx as u64 + amount as u64 - 1) as u32;

        self.comments = self
            .comments
            .drain(..)
            .filter_map(|comment| {
                let row = comment.read().get_coordinate().row;

                if row < idx {
                    // Выше удаления — без изменений
                    Some(comment)
                } else if row > delete_end {
                    // Ниже удаления — сдвиг вверх
                    let column = comment.read().get_coordinate().column;

                    comment
                        .write()
                        .set_coordinate(Coordinate::new(row - amount, column));
                    Some(comment)
                } else {
                    // Комментарий удалённой ячейки исчезает вместе с ней
                    None
                }
            })
            .collect();
    }

    /// Сдвигает координаты комментариев при удалении колонок
    /// `[idx; idx + amount)`. Симметрично `remap_comments_rows`.
    fn remap_comments_cols(&mut self, idx: u16, amount: u16) {
        let delete_end = (idx as u64 + amount as u64 - 1) as u16;

        self.comments = self
            .comments
            .drain(..)
            .filter_map(|comment| {
                let column = comment.read().get_coordinate().column;

                if column < idx {
                    // Левее удаления — без изменений
                    Some(comment)
                } else if column > delete_end {
                    // Правее удаления — сдвиг влево
                    let row = comment.read().get_coordinate().row;

                    comment
                        .write()
                        .set_coordinate(Coordinate::new(row, column - amount));
                    Some(comment)
                } else {
                    // Комментарий удалённой ячейки исчезает вместе с ней
                    None
                }
            })
            .collect();
    }
}

impl ReadableSheet for Sheet {
    type Cell = Cell;

    #[inline]
    fn get_name(&self) -> String {
        self.name.to_string()
    }

    #[inline]
    fn get_sheet_state(&self) -> String {
        self.sheet_state.to_string()
    }

    #[inline]
    fn get_cell_collection(&self) -> Vec<&Arc<RwLock<Cell>>> {
        self.cells.get_collection()
    }

    #[inline]
    fn get_cell_collection_sorted(&self) -> Vec<&Arc<RwLock<Cell>>> {
        self.cells.get_collection_sorted()
    }

    #[inline]
    fn get_cell_count(&self) -> usize {
        self.cells.len()
    }

    #[inline]
    fn get_max_row(&self) -> u32 {
        self.cells.get_max_row()
    }

    #[inline]
    fn get_max_column(&self) -> u16 {
        self.cells.get_max_column()
    }

    #[inline]
    fn get_cell_value<T>(&self, coordinate: T) -> String
    where
        T: Into<Coordinate>,
    {
        self.cells.get_cell_value(coordinate)
    }

    #[inline]
    fn get_cell_collection_by_range(
        &self,
        start_row: Option<u32>,
        end_row: Option<u32>,
        start_col: Option<u16>,
        end_col: Option<u16>,
    ) -> impl Iterator<Item = &Arc<RwLock<Cell>>> {
        self.cells
            .get_cell_collection_by_range(start_row, end_row, start_col, end_col)
    }

    #[inline]
    fn get_height_by_row(&self, row_num: u32) -> &f64 {
        self.row_dimensions.get_heignt(row_num)
    }

    #[inline]
    fn get_hidden_by_row(&self, row_num: u32) -> &bool {
        self.row_dimensions.get_hidden(row_num)
    }

    #[inline]
    fn get_width_by_column(&self, col_num: u16) -> &f64 {
        self.column_dimensions.get_width(col_num)
    }

    #[inline]
    fn get_hidden_by_column(&self, col_num: u16) -> &bool {
        self.column_dimensions.get_hidden(col_num)
    }

    #[inline]
    fn get_merge_cell_collection(&self) -> &[Range] {
        self.merge_cells.get_collection()
    }

    #[inline]
    fn get_comments(&self) -> &[Arc<RwLock<Comment>>] {
        &self.comments
    }

    #[inline]
    fn find_cell_by_regex(&self, regex: &str) -> Result<Option<&Arc<RwLock<Cell>>>> {
        self.cells.find_cell_by_regex(regex)
    }

    #[inline]
    fn find_cell_by_str(&self, value: &str) -> Result<Option<&Arc<RwLock<Cell>>>> {
        self.cells.find_cell_by_str(value)
    }

    #[inline]
    fn find_cell_by_coords(&self, row: u32, col: u16) -> Result<Option<&Arc<RwLock<Cell>>>> {
        self.cells.find_cell_by_coords(row, col)
    }

    #[inline]
    fn find_cell_by_letter(&self, letter: &str) -> Result<Option<&Arc<RwLock<Cell>>>> {
        self.cells.find_cell_by_letter(letter)
    }

    #[inline]
    fn find_cells_by_regex(&self, regex: &str) -> Result<Vec<&Arc<RwLock<Cell>>>> {
        self.cells.find_cells_by_regex(regex)
    }

    #[inline]
    fn find_cells_by_str(&self, value: &str) -> Result<Vec<&Arc<RwLock<Cell>>>> {
        self.cells.find_cells_by_str(value)
    }

    #[inline]
    fn find_cells_for_rows_by_regex(
        &self,
        regex: &str,
        col_stop: u16,
    ) -> Result<Vec<&Arc<RwLock<Cell>>>> {
        self.cells.find_cells_for_rows_by_regex(regex, col_stop)
    }

    #[inline]
    fn find_cells_for_cols_by_regex(
        &self,
        regex: &str,
        row_stop: u32,
    ) -> Result<Vec<&Arc<RwLock<Cell>>>> {
        self.cells.find_cells_for_cols_by_regex(regex, row_stop)
    }

    #[inline]
    fn find_cells_multi_regex(
        &self,
        before_regex: &str,
        after_regex: &str,
    ) -> Result<Vec<&Arc<RwLock<Cell>>>> {
        self.cells.find_cells_multi_regex(before_regex, after_regex)
    }

    #[inline]
    fn find_cells_between_regex(
        &self,
        before_regex: &str,
        after_regex: &str,
    ) -> Result<Vec<&Arc<RwLock<Cell>>>> {
        self.cells
            .find_cells_between_regex(before_regex, after_regex)
    }

    #[inline]
    fn find_cells_range_rows(
        &self,
        start_row: u32,
        end_row: u32,
    ) -> Result<Vec<&Arc<RwLock<Cell>>>> {
        self.cells.find_cells_range_rows(start_row, end_row)
    }

    #[inline]
    fn find_cells_range_cols(
        &self,
        start_col: u16,
        end_col: u16,
    ) -> Result<Vec<&Arc<RwLock<Cell>>>> {
        self.cells.find_cells_range_cols(start_col, end_col)
    }

    #[inline]
    fn find_values_by_col_rows(&self, col: u16, rows: Vec<u32>) -> Result<Vec<String>> {
        self.cells.find_values_by_col_rows(col, rows)
    }

    #[inline]
    fn find_values_by_row_cols(&self, row: u32, cols: Vec<u16>) -> Result<Vec<String>> {
        self.cells.find_values_by_row_cols(row, cols)
    }

    #[inline]
    fn find_value_by_coords(&self, row: u32, col: u16) -> Result<Option<String>> {
        self.cells.find_value_by_coords(row, col)
    }
}

impl WriteableSheet for Sheet {
    #[inline]
    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    #[inline]
    fn set_sheet_state(&mut self, state: &str) {
        self.sheet_state = state.into();
    }

    #[inline]
    fn add_merge_range(&mut self, range: Range) {
        self.merge_cells.add_range(range);
    }

    #[inline]
    fn add_comments(&mut self, value: Comment) {
        let value = Arc::new(RwLock::new(value));
        self.comments.push(value);
    }

    #[inline]
    fn cell(&mut self, coordinate: Coordinate, value: Option<&str>) -> &Arc<RwLock<Cell>> {
        self.cells.cell(coordinate, value)
    }

    #[inline]
    fn delete_cols(&mut self, idx: u16, amount: u16) -> Result<()> {
        self.cells.delete_cols(idx, amount)?;
        self.merge_cells.delete_cols(idx, amount);
        self.column_dimensions.delete_cols(idx, amount);
        self.remap_comments_cols(idx, amount);

        Ok(())
    }

    #[inline]
    fn delete_rows(&mut self, idx: u32, amount: u32) -> Result<()> {
        self.cells.delete_rows(idx, amount)?;
        self.merge_cells.delete_rows(idx, amount);
        self.row_dimensions.delete_rows(idx, amount);
        self.remap_comments_rows(idx, amount);

        Ok(())
    }

    #[inline]
    fn set_height_row(&mut self, row_num: u32, val: f64) {
        self.row_dimensions.set_height(row_num, val);
    }

    #[inline]
    fn set_hidden_row(&mut self, row_num: u32, val: bool) {
        self.row_dimensions.set_hidden(row_num, val)
    }

    #[inline]
    fn set_width_column(&mut self, col_num: u16, val: f64) {
        self.column_dimensions.set_width(col_num, val);
    }

    #[inline]
    fn set_hidden_column(&mut self, col_num: u16, val: bool) {
        self.column_dimensions.set_hidden(col_num, val)
    }
}

#[cfg(test)]
mod tests {
    use crate::traits::{ReadableCell, WriteableCell};
    use crate::{DEFAULT_COLUMN_WIDTH, DEFAULT_ROW_HEIGHT, MAX_COL, MAX_ROW};

    use super::*;

    fn sheet() -> Sheet {
        let mut sheet = Sheet::new("A", "visible");

        for r in 1..=5 {
            for c in 1..=5 {
                let coord = Coordinate::new(r, c);
                let val = format!("Yop! {r}:{c}");

                sheet.cell(coord, Some(&val));
            }
        }

        sheet
    }

    #[test]
    fn new_sheet() {
        let sheet = Sheet::new("test", "visible");

        assert_eq!(sheet.name, "test");
    }

    #[test]
    fn max_row() {
        let sheet = sheet();

        assert_eq!(sheet.get_max_row(), 5);
    }

    #[test]
    fn max_column() {
        let sheet = sheet();

        assert_eq!(sheet.get_max_column(), 5);
    }

    #[test]
    fn write_cell() {
        let mut sheet = Sheet::new("A", "visible");
        sheet.cell(Coordinate::new(1, 1), Some("Привет, мир!"));

        assert_eq!(sheet.get_cell_collection().len(), 1);
    }

    #[test]
    fn get_cell_collection() {
        let mut sheet = Sheet::new("A", "visible");
        sheet.cell(Coordinate::new(1, 1), Some("Привет, мир!"));

        assert_eq!(sheet.get_cell_collection().len(), 1);
    }

    #[test]
    fn get_cell_collection_sorted() {
        let mut sheet = Sheet::new("A", "visible");
        sheet.cell(Coordinate::new(1, 1), Some("Привет, мир!"));

        assert_eq!(sheet.get_cell_collection_sorted().len(), 1);
    }

    #[test]
    fn get_cell_value() {
        let mut sheet = Sheet::new("A", "visible");
        let coord = Coordinate::new(1, 1);

        sheet.cell(coord.clone(), Some("Привет, мир!"));

        assert_eq!(sheet.get_cell_value(coord), "Привет, мир!");
    }

    /// Повторный `cell(r, c)` без значения (touch) не мутирует тип и значение
    /// существующей ячейки: числовая ячейка остаётся "n".
    #[test]
    fn cell_touch_preserves_existing_data_type() {
        let mut sheet = Sheet::new("A", "visible");
        let coord = Coordinate::new(1, 1);

        sheet.cell(coord.clone(), Some("3.14"));

        let cell = sheet.cell(coord, None);

        assert_eq!(cell.read().get_data_type(), "n");
        assert_eq!(cell.read().get_value(), "3.14");
    }

    /// Touch не мутирует и строковую ячейку: тип "s" и значение на месте.
    #[test]
    fn cell_touch_preserves_string_cell() {
        let mut sheet = Sheet::new("A", "visible");
        let coord = Coordinate::new(1, 1);

        sheet.cell(coord.clone(), Some("текст"));

        let cell = sheet.cell(coord, None);

        assert_eq!(cell.read().get_data_type(), "s");
        assert_eq!(cell.read().get_value(), "текст");
    }

    /// Touch не сбрасывает тип формульной ячейки с пустым значением: "f"
    /// сохраняется, ячейка остаётся формульной.
    #[test]
    fn cell_touch_preserves_formula_cell() {
        let mut sheet = Sheet::new("A", "visible");
        let coord = Coordinate::new(1, 1);

        let cell = sheet.cell(coord.clone(), None);
        cell.write().set_formula("SUM(B1:B10)");

        let cell = sheet.cell(coord, None);

        assert!(cell.read().is_formula());
        assert_eq!(cell.read().get_data_type(), "f");
    }

    /// Touch по новой координате создаёт пустую ячейку с типом "s" —
    /// исходное поведение ветки создания сохранено.
    #[test]
    fn cell_touch_creates_empty_string_typed_cell() {
        let mut sheet = Sheet::new("A", "visible");

        let cell = sheet.cell(Coordinate::new(1, 1), None);

        assert!(cell.read().is_value_empty());
        assert_eq!(cell.read().get_data_type(), "s");
    }

    #[test]
    fn get_cell_collection_by_range() {
        let mut sheet = Sheet::new("A", "visible");

        for r in 1..=5 {
            for c in 1..=5 {
                let coord = Coordinate::new(r, c);
                let val = format!("Привет, мир! {r}:{c}");

                sheet.cell(coord, Some(&val));
            }
        }

        assert_eq!(
            sheet
                .get_cell_collection_by_range(Some(1), Some(2), Some(1), Some(2))
                .count(),
            4
        );
    }

    #[test]
    fn delete_rows() {
        let mut sheet = Sheet::new("A", "visible");

        for r in 1..=5 {
            for c in 1..=5 {
                let coord = Coordinate::new(r, c);
                let val = format!("Привет, мир! {r}:{c}");

                sheet.cell(coord, Some(&val));
            }
        }

        sheet.delete_rows(2, 4).unwrap();

        assert_eq!(sheet.get_cell_collection().len(), 5);
    }

    #[test]
    fn delete_cols() {
        let mut sheet = Sheet::new("A", "visible");

        for r in 1..=5 {
            for c in 1..=5 {
                let coord = Coordinate::new(r, c);
                let val = format!("Привет, мир! {r}:{c}");

                sheet.cell(coord, Some(&val));
            }
        }

        sheet.delete_cols(2, 4).unwrap();

        assert_eq!(sheet.get_cell_collection().len(), 5);
    }

    /// Delete_rows сдвигает merge-диапазоны, оставшиеся ниже удаления,
    /// и не трогает диапазоны выше.
    #[test]
    fn delete_rows_shifts_merge_cells_below() {
        let mut sheet = Sheet::new("A", "visible");

        sheet.add_merge_range(Range::new(1, 1, 1, 2)); // выше удаления
        sheet.add_merge_range(Range::new(7, 8, 1, 2)); // ниже удаления

        sheet.delete_rows(2, 4).unwrap(); // удалены строки 2–5

        let merges = sheet.get_merge_cell_collection();

        assert_eq!(merges.len(), 2);
        assert!(merges.contains(&Range::new(1, 1, 1, 2)));
        assert!(merges.contains(&Range::new(3, 4, 1, 2)));
    }

    /// Delete_rows: merge внутри диапазона удаления удаляется, частично
    /// пересекающийся усекается, а усечённый до одной ячейки удаляется.
    /// Одноячеечный до удаления диапазон сохраняется.
    #[test]
    fn delete_rows_truncates_and_drops_intersecting_merge_cells() {
        let mut sheet = Sheet::new("A", "visible");

        sheet.add_merge_range(Range::new(2, 3, 1, 2)); // целиком внутри удаления
        sheet.add_merge_range(Range::new(1, 2, 4, 5)); // хвост в удалении
        sheet.add_merge_range(Range::new(5, 6, 7, 8)); // голова в удалении
        sheet.add_merge_range(Range::new(5, 6, 9, 9)); // усечётся до одной ячейки
        sheet.add_merge_range(Range::new(9, 10, 1, 2)); // ниже удаления
        sheet.add_merge_range(Range::new(9, 9, 4, 4)); // одноячеечный до удаления

        sheet.delete_rows(2, 4).unwrap(); // удалены строки 2–5

        let merges = sheet.get_merge_cell_collection();

        assert_eq!(merges.len(), 4);
        // Хвост (строка 2) удалён, осталась строка 1
        assert!(merges.contains(&Range::new(1, 1, 4, 5)));
        // Голова (строка 5) удалена, строка 6 сдвинулась на 2
        assert!(merges.contains(&Range::new(2, 2, 7, 8)));
        // (2,3) удалён целиком, (5,6,9,9) схлопнулся до одной ячейки,
        // одноячеечный (9,9,4,4) сохранился, сдвинувшись на (5,5,4,4)
        assert!(merges.contains(&Range::new(5, 6, 1, 2)));
        assert!(merges.contains(&Range::new(5, 5, 4, 4)));
    }

    /// Delete_cols: merge-диапазоны сдвигаются влево, усекаются или удаляются.
    #[test]
    fn delete_cols_shifts_and_truncates_merge_cells() {
        let mut sheet = Sheet::new("A", "visible");

        sheet.add_merge_range(Range::new(1, 2, 2, 3)); // целиком внутри удаления
        sheet.add_merge_range(Range::new(1, 2, 1, 3)); // хвост в удалении
        sheet.add_merge_range(Range::new(1, 2, 5, 6)); // голова в удалении
        sheet.add_merge_range(Range::new(9, 9, 5, 6)); // усечётся до одной ячейки
        sheet.add_merge_range(Range::new(1, 2, 9, 10)); // правее удаления
        sheet.add_merge_range(Range::new(1, 1, 9, 9)); // одноячеечный до удаления

        sheet.delete_cols(2, 4).unwrap(); // удалены колонки 2–5

        let merges = sheet.get_merge_cell_collection();

        assert_eq!(merges.len(), 4);
        // Хвост (колонки 2–3) удалён, осталась колонка 1
        assert!(merges.contains(&Range::new(1, 2, 1, 1)));
        // Голова (колонка 5) удалена, колонка 6 сдвинулась на 2
        assert!(merges.contains(&Range::new(1, 2, 2, 2)));
        // (2,3) удалён целиком, (9,9,5,6) схлопнулся до одной ячейки,
        // одноячеечный (1,1,9,9) сохранился, сдвинувшись на (1,1,5,5)
        assert!(merges.contains(&Range::new(1, 2, 5, 6)));
        assert!(merges.contains(&Range::new(1, 1, 5, 5)));
    }

    /// Delete_rows пересчитывает ключи row_dimensions: размеры строк ниже
    /// удаления сдвигаются вверх, размеры удалённых строк исчезают.
    #[test]
    fn delete_rows_shifts_row_dimensions() {
        let mut sheet = Sheet::new("A", "visible");

        sheet.set_height_row(1, 10.0); // выше удаления
        sheet.set_height_row(3, 30.0); // внутри удаления
        sheet.set_height_row(7, 70.0); // ниже удаления
        sheet.set_hidden_row(7, true);

        sheet.delete_rows(2, 4).unwrap(); // удалены строки 2–5

        assert_eq!(*sheet.get_height_by_row(1), 10.0); // выше — на месте
        assert_eq!(*sheet.get_height_by_row(3), 70.0); // 7 → 3
        assert!(*sheet.get_hidden_by_row(3));
        assert_eq!(*sheet.get_height_by_row(7), DEFAULT_ROW_HEIGHT); // ключ исчез
    }

    /// Delete_cols пересчитывает ключи column_dimensions: размеры колонок
    /// правее удаления сдвигаются влево, размеры удалённых колонок исчезают.
    #[test]
    fn delete_cols_shifts_column_dimensions() {
        let mut sheet = Sheet::new("A", "visible");

        sheet.set_width_column(1, 10.0); // левее удаления
        sheet.set_width_column(3, 30.0); // внутри удаления
        sheet.set_width_column(7, 70.0); // правее удаления
        sheet.set_hidden_column(7, true);

        sheet.delete_cols(2, 4).unwrap(); // удалены колонки 2–5

        assert_eq!(*sheet.get_width_by_column(1), 10.0); // левее — на месте
        assert_eq!(*sheet.get_width_by_column(3), 70.0); // 7 → 3
        assert!(*sheet.get_hidden_by_column(3));
        assert_eq!(*sheet.get_width_by_column(7), DEFAULT_COLUMN_WIDTH); // ключ исчез
    }

    /// Delete_rows сдвигает координаты комментариев, а комментарии из
    /// удаляемого диапазона удаляются — как ячейки и как в Excel.
    #[test]
    fn delete_rows_shifts_and_drops_comments() {
        let mut sheet = Sheet::new("A", "visible");

        sheet.add_comments(Comment::new(Coordinate::new(1, 1), "A")); // выше удаления
        sheet.add_comments(Comment::new(Coordinate::new(3, 2), "B")); // внутри удаления
        sheet.add_comments(Comment::new(Coordinate::new(7, 3), "C")); // ниже удаления

        sheet.delete_rows(2, 4).unwrap(); // удалены строки 2–5

        let comments = sheet.get_comments();

        assert_eq!(comments.len(), 2);
        assert_eq!(comments[0].read().get_coordinate(), &Coordinate::new(1, 1));
        assert_eq!(comments[1].read().get_coordinate(), &Coordinate::new(3, 3));
    }

    /// Delete_cols сдвигает координаты комментариев, а комментарии из
    /// удаляемого диапазона удаляются.
    #[test]
    fn delete_cols_shifts_and_drops_comments() {
        let mut sheet = Sheet::new("A", "visible");

        sheet.add_comments(Comment::new(Coordinate::new(1, 1), "A")); // левее удаления
        sheet.add_comments(Comment::new(Coordinate::new(2, 3), "B")); // внутри удаления
        sheet.add_comments(Comment::new(Coordinate::new(3, 7), "C")); // правее удаления

        sheet.delete_cols(2, 4).unwrap(); // удалены колонки 2–5

        let comments = sheet.get_comments();

        assert_eq!(comments.len(), 2);
        assert_eq!(comments[0].read().get_coordinate(), &Coordinate::new(1, 1));
        assert_eq!(comments[1].read().get_coordinate(), &Coordinate::new(3, 3));
    }

    /// Ошибочный delete_rows/cols (за границами листа) не мутирует ничего:
    /// merge, размеры и комментарии остаются на месте.
    #[test]
    fn delete_out_of_bounds_leaves_metadata_untouched() {
        let mut sheet = Sheet::new("A", "visible");

        sheet.cell(Coordinate::new(1, 1), Some("A1"));
        sheet.add_merge_range(Range::new(2, 3, 1, 2));
        sheet.set_height_row(2, 20.0);
        sheet.set_width_column(2, 20.0);
        sheet.add_comments(Comment::new(Coordinate::new(2, 2), "A"));

        assert!(sheet.delete_rows(MAX_ROW, 2).is_err());
        assert!(sheet.delete_cols(MAX_COL, 2).is_err());

        assert!(
            sheet
                .get_merge_cell_collection()
                .contains(&Range::new(2, 3, 1, 2))
        );
        assert_eq!(*sheet.get_height_by_row(2), 20.0);
        assert_eq!(*sheet.get_width_by_column(2), 20.0);
        assert_eq!(sheet.get_comments().len(), 1);
    }

    /// Диапазон удаления за границами листа → Err, а не переполнение u16
    /// с коллизиями ключей.
    #[test]
    fn delete_cols_out_of_bounds() {
        let mut sheet = Sheet::new("A", "visible");
        sheet.cell(Coordinate::new(1, 1), Some("A1"));

        // 60000 + 10000 = 70000 > u16::MAX — раньше переполнялось
        assert!(sheet.delete_cols(60000, 10000).is_err());

        // Карта не опустошена: ошибочный вызов ничего не удаляет
        assert_eq!(sheet.get_cell_collection().len(), 1);
    }

    /// Диапазон удаления строк за границами листа → Err.
    #[test]
    fn delete_rows_out_of_bounds() {
        let mut sheet = Sheet::new("A", "visible");
        sheet.cell(Coordinate::new(1, 1), Some("A1"));

        // 1_048_577 — за границей листа (MAX_ROW = 1_048_576)
        assert!(sheet.delete_rows(MAX_ROW, 2).is_err());
        assert_eq!(sheet.get_cell_collection().len(), 1);
    }

    /// Нулевые idx/amount вне домена Excel → Err.
    #[test]
    fn delete_zero_idx_or_amount() {
        let mut sheet = Sheet::new("A", "visible");

        assert!(sheet.delete_cols(0, 1).is_err());
        assert!(sheet.delete_cols(1, 0).is_err());
        assert!(sheet.delete_rows(0, 1).is_err());
        assert!(sheet.delete_rows(1, 0).is_err());
    }

    /// Граничные валидные вызовы → Ok, содержимое корректно.
    #[test]
    fn delete_cols_boundaries() {
        // Удалить последнюю колонку (XFD = MAX_COL) — валидно
        let mut sheet = Sheet::new("A", "visible");
        sheet.cell(Coordinate::new(1, MAX_COL), Some("последняя"));

        sheet.delete_cols(MAX_COL, 1).unwrap();
        assert_eq!(sheet.get_cell_collection().len(), 0);

        // Удалить все колонки одним вызовом — валидно
        let mut sheet = Sheet::new("A", "visible");
        sheet.cell(Coordinate::new(1, 1), Some("A1"));
        sheet.cell(Coordinate::new(1, MAX_COL), Some("XFD1"));

        sheet.delete_cols(1, MAX_COL).unwrap();
        assert_eq!(sheet.get_cell_collection().len(), 0);
    }

    /// Граничные валидные вызовы для строк → Ok, содержимое корректно.
    #[test]
    fn delete_rows_boundaries() {
        // Удалить последнюю строку (MAX_ROW) — валидно
        let mut sheet = Sheet::new("A", "visible");
        sheet.cell(Coordinate::new(MAX_ROW, 1), Some("последняя"));

        sheet.delete_rows(MAX_ROW, 1).unwrap();
        assert_eq!(sheet.get_cell_collection().len(), 0);

        // Удалить все строки одним вызовом — валидно
        let mut sheet = Sheet::new("A", "visible");
        sheet.cell(Coordinate::new(1, 1), Some("A1"));
        sheet.cell(Coordinate::new(MAX_ROW, 1), Some("A1048576"));

        sheet.delete_rows(1, MAX_ROW).unwrap();
        assert_eq!(sheet.get_cell_collection().len(), 0);
    }

    /// После валидного удаления ячейки правее/ниже диапазона сдвигаются
    /// с корректными значениями и координатами.
    #[test]
    fn delete_cols_shifts_content() {
        let mut sheet = Sheet::new("A", "visible");

        for r in 1..=2 {
            for c in 1..=5 {
                let coord = Coordinate::new(r, c);
                let val = format!("V{r}:{c}");

                sheet.cell(coord, Some(&val));
            }
        }

        // Удаляем колонки 2–3: 4→2, 5→3, колонка 1 не двигается
        sheet.delete_cols(2, 2).unwrap();

        assert_eq!(sheet.get_cell_collection().len(), 6);
        assert_eq!(sheet.get_cell_value(Coordinate::new(1, 1)), "V1:1");
        assert_eq!(sheet.get_cell_value(Coordinate::new(2, 2)), "V2:4");
        assert_eq!(sheet.get_cell_value(Coordinate::new(1, 3)), "V1:5");
        assert!(sheet.find_cell_by_coords(1, 4).unwrap().is_none());
    }

    /// После валидного удаления строк содержимое корректно.
    #[test]
    fn delete_rows_shifts_content() {
        let mut sheet = Sheet::new("A", "visible");

        for r in 1..=5 {
            for c in 1..=2 {
                let coord = Coordinate::new(r, c);
                let val = format!("V{r}:{c}");

                sheet.cell(coord, Some(&val));
            }
        }

        // Удаляем строки 2–3: 4→2, 5→3, строка 1 не двигается
        sheet.delete_rows(2, 2).unwrap();

        assert_eq!(sheet.get_cell_collection().len(), 6);
        assert_eq!(sheet.get_cell_value(Coordinate::new(1, 1)), "V1:1");
        assert_eq!(sheet.get_cell_value(Coordinate::new(2, 2)), "V4:2");
        assert_eq!(sheet.get_cell_value(Coordinate::new(3, 1)), "V5:1");
        assert!(sheet.find_cell_by_coords(4, 1).unwrap().is_none());
    }

    #[test]
    pub fn find_cell_by_regex() {
        let sheet = sheet();
        let regex = "Yop! 3:3";

        let cell = sheet.cells.find_cell_by_regex(regex).unwrap();

        assert!(cell.is_some());
        assert_eq!(cell.unwrap().read().get_value(), "Yop! 3:3");
    }

    #[test]
    pub fn find_cell_by_str() {
        let sheet = sheet();
        let regex = "Yop! 3:3";

        let cell = sheet.cells.find_cell_by_str(regex).unwrap();

        assert!(cell.is_some());
        assert_eq!(cell.unwrap().read().get_value(), "Yop! 3:3");
    }

    #[test]
    pub fn find_cell_by_coords() {
        let sheet = sheet();

        let cell = sheet.cells.find_cell_by_coords(1, 1).unwrap();

        assert!(cell.is_some());
        assert_eq!(cell.unwrap().read().get_value(), "Yop! 1:1");
    }

    #[test]
    pub fn find_cell_by_letter() {
        let sheet = sheet();
        let letter = "C1";

        let cell = sheet.cells.find_cell_by_letter(letter).unwrap();

        assert!(cell.is_some());
        assert_eq!(cell.unwrap().read().get_value(), "Yop! 1:3");
    }

    #[test]
    pub fn find_cell_by_letter_invalid() {
        let sheet = sheet();

        for letter in ["1A", "", "HELLO", "A99999999999999999"] {
            let err = sheet.cells.find_cell_by_letter(letter).unwrap_err();

            assert_eq!(err.to_string(), format!("invalid cell letter: {letter}"));
        }
    }

    /// Невалидная буква — не паника, а отсутствие ячейки.
    #[test]
    pub fn get_cell_by_letter_invalid() {
        let sheet = sheet();

        assert!(sheet.cells.get_cell_by_letter("1A").is_none());
        assert!(sheet.cells.get_cell_by_letter("C1").is_some());
    }

    #[test]
    pub fn find_cells_by_regex() {
        let sheet = sheet();
        let regex = "Yop! 2:2";

        let cells = sheet.cells.find_cells_by_regex(regex).unwrap();

        assert_eq!(cells.len(), 1);
    }

    #[test]
    pub fn find_cells_by_str() {
        let sheet = sheet();
        let regex = "Yop! 2:2";

        let cells = sheet.cells.find_cells_by_str(regex).unwrap();

        assert_eq!(cells.len(), 1);
    }

    #[test]
    pub fn find_cells_for_rows_by_regex() {
        let sheet = sheet();
        let regex = "Yop!";
        let col_stop = 2;

        let cells = sheet
            .cells
            .find_cells_for_rows_by_regex(regex, col_stop)
            .unwrap();

        assert_eq!(cells.len(), 10);
    }

    #[test]
    pub fn find_cells_for_cols_by_regex() {
        let sheet = sheet();
        let regex = "Yop!";
        let row_stop = 2;

        let cells = sheet
            .cells
            .find_cells_for_cols_by_regex(regex, row_stop)
            .unwrap();

        assert_eq!(cells.len(), 10);
    }

    #[test]
    pub fn find_cells_multi_regex() {
        let sheet = sheet();

        let before_regex = "Yop! 1:1";
        let after_regex = "Yop! 5:5";

        let cells = sheet
            .cells
            .find_cells_multi_regex(before_regex, after_regex)
            .unwrap();

        assert_eq!(cells.len(), 2);
    }

    #[test]
    pub fn find_cells_between_regex() {
        let sheet = sheet();

        let before_regex = "Yop! 1:5";
        let after_regex = "Yop! 2:5";

        let cells = sheet
            .cells
            .find_cells_between_regex(before_regex, after_regex)
            .unwrap();

        assert_eq!(cells.len(), 10);
    }

    #[test]
    pub fn find_cells_range_rows() {
        let sheet = sheet();
        let start_row = 1;
        let end_row = 2;

        let cells = sheet
            .cells
            .find_cells_range_rows(start_row, end_row)
            .unwrap();

        assert_eq!(cells.len(), 10);
    }

    #[test]
    pub fn find_cells_range_cols() {
        let sheet = sheet();

        let start_col = 1;
        let end_col = 2;

        let cells = sheet
            .cells
            .find_cells_range_cols(start_col, end_col)
            .unwrap();

        assert_eq!(cells.len(), 10);
    }

    #[test]
    pub fn find_values_by_row_cols() {
        let sheet = sheet();

        let row = 1;
        let cols = vec![1, 2];

        let cells = sheet.cells.find_values_by_row_cols(row, cols).unwrap();

        assert_eq!(cells.len(), 2);
    }

    #[test]
    pub fn find_values_by_col_rows() {
        let sheet = sheet();

        let col = 2;
        let rows = vec![1, 2, 3];

        let cells = sheet.cells.find_values_by_col_rows(col, rows).unwrap();

        assert_eq!(cells.len(), 3);
    }

    /// При нескольких совпадениях find_cell_by_regex возвращает ячейку
    /// с наименьшими координатами (детерминизм между запусками процесса).
    #[test]
    pub fn find_cell_by_regex_multiple_matches() {
        let mut sheet = Sheet::new("A", "visible");

        for coord in [(3, 3), (1, 2), (2, 1)] {
            sheet.cell(Coordinate::new(coord.0, coord.1), Some("dup"));
        }

        let cell = sheet.cells.find_cell_by_regex("dup").unwrap().unwrap();
        let guard = cell.read();
        let coord = guard.get_coordinate();

        assert_eq!((coord.row, coord.column), (1, 2));
    }

    /// При нескольких совпадениях find_cell_by_str возвращает ячейку
    /// с наименьшими координатами (детерминизм между запусками процесса).
    #[test]
    pub fn find_cell_by_str_multiple_matches() {
        let mut sheet = Sheet::new("A", "visible");

        for coord in [(3, 3), (1, 2), (2, 1)] {
            sheet.cell(Coordinate::new(coord.0, coord.1), Some("dup"));
        }

        let cell = sheet.cells.find_cell_by_str("dup").unwrap().unwrap();
        let guard = cell.read();
        let coord = guard.get_coordinate();

        assert_eq!((coord.row, coord.column), (1, 2));
    }

    /// Значения идут в порядке запрошенных строк, а не в порядке обхода
    /// HashMap; отсутствующая строка даёт пустую строку, длина результата
    /// равна длине запроса (позиционный zip безопасен).
    #[test]
    pub fn find_values_by_col_rows_request_order() {
        let sheet = sheet();

        let values = sheet
            .cells
            .find_values_by_col_rows(2, vec![5, 1, 3])
            .unwrap();

        assert_eq!(values, vec!["Yop! 5:2", "Yop! 1:2", "Yop! 3:2"]);

        let values = sheet.cells.find_values_by_col_rows(2, vec![2, 99]).unwrap();

        assert_eq!(values, vec!["Yop! 2:2", ""]);
    }

    /// Значения идут в порядке запрошенных колонок, а не в порядке обхода
    /// HashMap; отсутствующая колонка даёт пустую строку, длина результата
    /// равна длине запроса (позиционный zip безопасен).
    #[test]
    pub fn find_values_by_row_cols_request_order() {
        let sheet = sheet();

        let values = sheet
            .cells
            .find_values_by_row_cols(1, vec![4, 1, 5])
            .unwrap();

        assert_eq!(values, vec!["Yop! 1:4", "Yop! 1:1", "Yop! 1:5"]);

        let values = sheet.cells.find_values_by_row_cols(1, vec![3, 99]).unwrap();

        assert_eq!(values, vec!["Yop! 1:3", ""]);
    }

    #[test]
    pub fn find_value_by_coords() {
        let sheet = sheet();

        let value = sheet.cells.find_value_by_coords(1, 1).unwrap();

        assert_eq!(value, Some(format!("Yop! {}:{}", 1, 1)));
    }

    #[test]
    pub fn add_merge_range() {
        let mut sheet = sheet();

        let range = Range::new(1, 2, 1, 2);
        sheet.add_merge_range(range);

        let range = sheet.get_merge_cell_collection();

        assert_eq!(range.len(), 1);
    }

    #[test]
    pub fn get_merge_cell_collection() {
        let sheet = sheet();
        let range = sheet.get_merge_cell_collection();

        assert_eq!(range.len(), 0);
    }

    #[test]
    pub fn get_comments() {
        let sheet = sheet();
        let comments = sheet.get_comments();

        assert_eq!(comments.len(), 0);
    }

    #[test]
    pub fn add_comments() {
        let mut sheet = sheet();
        sheet.add_comments(Comment::new(Coordinate::new(1, 1), "A.C"));

        assert_eq!(sheet.get_comments().len(), 1);
    }
}
