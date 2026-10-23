use serde::Serialize;

pub type MergedRange = (u32, u32, u16, u16);

#[derive(Clone, Default, Debug, Serialize, PartialEq, Eq)]
pub struct Range {
    pub start_row: u32,
    pub end_row: u32,
    pub start_col: u16,
    pub end_col: u16,
}

impl Range {
    /// Интициализирует диапазон с заданными координатами
    pub fn new(start_row: u32, end_row: u32, start_col: u16, end_col: u16) -> Self {
        Range {
            start_row,
            end_row,
            start_col,
            end_col,
        }
    }

    /// Диапазон вырождается в одну ячейку
    #[inline]
    pub(crate) fn is_single_cell(&self) -> bool {
        self.start_row == self.end_row && self.start_col == self.end_col
    }

    /// Сдвигает/усекает диапазон при удалении строк `[idx; idx + amount)`.
    ///
    /// Диапазоны ниже удаления сдвигаются вверх, выше — остаются на месте,
    /// пересекающиеся — усекаются до не удалённой части. Возвращает `None`,
    /// если диапазон целиком попал в удаление.
    ///
    /// Ожидается валидный диапазон удаления: `idx ≥ 1`, `amount ≥ 1`.
    #[inline]
    pub(crate) fn delete_rows(&self, idx: u32, amount: u32) -> Option<Range> {
        // Арифметика в широком типе: после validate_delete_range значение
        // заведомо ≤ максимуму листа — переполнения и усечения cast'а нет
        let delete_end = (idx as u64 + amount as u64 - 1) as u32;

        if self.end_row < idx {
            // Полностью выше удаления — без изменений
            return Some(self.clone());
        }

        if self.start_row > delete_end {
            // Полностью ниже удаления — сдвиг вверх
            return Some(Range::new(
                self.start_row - amount,
                self.end_row - amount,
                self.start_col,
                self.end_col,
            ));
        }

        // Пересечение — усечение до не удалённой части; голова до удаления
        // остаётся, хвост после удаления сдвигается вверх
        let start_row = self.start_row.min(idx);
        let end_row = if self.end_row > delete_end {
            self.end_row - amount
        } else {
            idx - 1
        };

        (start_row <= end_row).then(|| Range::new(start_row, end_row, self.start_col, self.end_col))
    }

    /// Сдвигает/усекает диапазон при удалении колонок `[idx; idx + amount)`.
    ///
    /// Симметрично `delete_rows`.
    #[inline]
    pub(crate) fn delete_cols(&self, idx: u16, amount: u16) -> Option<Range> {
        let delete_end = (idx as u64 + amount as u64 - 1) as u16;

        if self.end_col < idx {
            return Some(self.clone());
        }

        if self.start_col > delete_end {
            return Some(Range::new(
                self.start_row,
                self.end_row,
                self.start_col - amount,
                self.end_col - amount,
            ));
        }

        let start_col = self.start_col.min(idx);
        let end_col = if self.end_col > delete_end {
            self.end_col - amount
        } else {
            idx - 1
        };

        (start_col <= end_col).then(|| Range::new(self.start_row, self.end_row, start_col, end_col))
    }
}

impl From<(u32, u32, u16, u16)> for Range {
    #[inline]
    fn from(value: (u32, u32, u16, u16)) -> Self {
        Range::new(value.0, value.1, value.2, value.3)
    }
}

impl From<Range> for (u32, u32, u16, u16) {
    #[inline]
    fn from(range: Range) -> Self {
        (
            range.start_row,
            range.end_row,
            range.start_col,
            range.end_col,
        )
    }
}

impl From<&Range> for (u32, u32, u16, u16) {
    #[inline]
    fn from(range: &Range) -> Self {
        (
            range.start_row,
            range.end_row,
            range.start_col,
            range.end_col,
        )
    }
}
