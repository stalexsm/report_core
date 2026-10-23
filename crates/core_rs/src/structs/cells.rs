use ahash::HashMap;
use parking_lot::RwLock;
use rayon::slice::ParallelSliceMut;
use serde::Serialize;
use std::sync::Arc;

use super::{cell::Cell, coordinate::Coordinate};
use crate::{
    MAX_COL, MAX_ROW,
    datatype::CellValue,
    error::{Error, Result},
    funcs::{
        find_cell_by_regex, find_cell_by_str, find_cells_by_regex, find_cells_by_str,
        find_cells_for_cols_by_regex, find_cells_for_rows_by_regex, find_cells_multi_regex,
        find_cells_range_cols, find_cells_range_rows,
    },
    regex::{compile_case_insensitive, is_match},
    traits::{ReadableCell, WriteableCell},
};

/// Ячейки, снимаемые с карты при сдвиге: старый ключ и обёртка.
type ShiftedCells = Vec<((u32, u16), Arc<RwLock<Cell>>)>;

/// Вспомоогательная функция для сериализации HashMap только Value, как вектор.
fn serialize_cells_to_vec<S>(
    map: &HashMap<(u32, u16), Arc<RwLock<Cell>>>,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde::ser::SerializeSeq;

    let mut items: Vec<_> = map.iter().collect();
    items.sort_unstable_by_key(|(key, _)| *key);

    let mut seq = serializer.serialize_seq(Some(items.len()))?;
    for (_, cell) in items {
        let guard = cell.read();
        seq.serialize_element(&*guard)?;
    }
    seq.end()
}

/// Ленивый кэш сортированных по (row, col) ключей карты.
///
/// Порядок обхода определяется только ключами карты, а значения ячеек живут за
/// `Arc<RwLock<..>>` — мутации значений порядок не меняют. Поэтому кэш
/// инвалидается только в точках изменения набора ключей (`cell` с новой
/// координатой, `delete_rows/cols`) и строится один раз при первом обращении.
type SortedKeysCache = RwLock<Option<Arc<[(u32, u16)]>>>;

#[derive(Default, Debug, Serialize)]
pub struct Cells {
    #[serde(serialize_with = "serialize_cells_to_vec")]
    #[serde(rename = "cells")]
    map: HashMap<(u32, u16), Arc<RwLock<Cell>>>,
    #[serde(skip)]
    default_cell_value: CellValue,
    /// Максимальные координаты карты, поддерживаются в точках мутаций;
    /// геттеры отдают их за O(1) вместо полного прохода.
    #[serde(skip)]
    max_row: u32,
    #[serde(skip)]
    max_column: u16,
    #[serde(skip)]
    sorted_keys: SortedKeysCache,
}

impl Clone for Cells {
    fn clone(&self) -> Self {
        Cells {
            map: self.map.clone(),
            default_cell_value: self.default_cell_value.clone(),
            max_row: self.max_row,
            max_column: self.max_column,
            // Набор ключей у копии тот же — кэш переиспользуется
            sorted_keys: RwLock::new(self.sorted_keys.read().clone()),
        }
    }
}

/// Проверяет, что диапазон удаления `[idx; idx + amount)` лежит в домене
/// Excel `[1; max]`. Арифметика — в широком типе: в исходных
/// типах (u16/u32) `idx + amount` переполнялось с коллизиями ключей
/// карты в release и паникой в debug.
#[inline]
fn validate_delete_range(idx: u64, amount: u64, max: u64) -> Result<()> {
    if idx == 0 {
        return Err(Error::domain(format!(
            "Индекс начала удаления должен быть ≥ 1, получено: {idx}"
        )));
    }

    if amount == 0 {
        return Err(Error::domain(format!(
            "Количество удаляемых строк/колонок должно быть ≥ 1, получено: {amount}"
        )));
    }

    let end = idx + amount - 1;
    if end > max {
        return Err(Error::domain(format!(
            "Диапазон удаления [{idx}; {end}] выходит за границы листа (максимум: {max})"
        )));
    }

    Ok(())
}

impl Cells {
    pub fn new(map: HashMap<(u32, u16), Arc<RwLock<Cell>>>) -> Self {
        // Максимумы считаются один раз при сборке; кэш сортированных ключей
        // строится лениво при первом обращении
        let (max_row, max_column) = map
            .keys()
            .fold((0, 0), |(row, col), &(r, c)| (row.max(r), col.max(c)));

        Self {
            map,
            max_row,
            max_column,
            ..Default::default()
        }
    }

    /// Количество ячеек за O(1) — для repr и прочих счётчиков без проходов.
    #[inline]
    pub fn len(&self) -> usize {
        self.map.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Глубокое клонирование: каждая ячейка получает собственную обёртку
    /// `Arc<RwLock<Cell>>`, чтобы мутации копии не меняли оригинал.
    pub fn clone_deep(&self) -> Self {
        let map = self
            .map
            .iter()
            .map(|(key, cell)| (*key, Arc::new(RwLock::new(cell.read().clone()))))
            .collect();

        Self {
            map,
            default_cell_value: self.default_cell_value.clone(),
            // Набор ключей у копии тот же — кэш и максимумы переиспользуются
            max_row: self.max_row,
            max_column: self.max_column,
            sorted_keys: RwLock::new(self.sorted_keys.read().clone()),
        }
    }

    /// Сортированные по (row, col) ключи карты: из кэша или с разовой сборкой.
    fn sorted_keys_cloned(&self) -> Arc<[(u32, u16)]> {
        if let Some(keys) = self.sorted_keys.read().clone() {
            return keys;
        }

        let mut keys: Vec<(u32, u16)> = self.map.keys().copied().collect();

        // Сортировка по ключу (row, col) — быстрая сортировка примитивов
        keys.par_sort_unstable();
        let keys: Arc<[(u32, u16)]> = keys.into();

        // Карта иммутабельна под &self, параллельные строители соберут тот же
        // набор — перезапись кэша безопасна
        *self.sorted_keys.write() = Some(Arc::clone(&keys));

        keys
    }

    /// Метод для получения коллекции ячеек
    #[inline]
    pub fn get_collection(&self) -> Vec<&Arc<RwLock<Cell>>> {
        self.map.values().collect::<Vec<_>>()
    }

    /// Метод для получения коллекции ячеек, отсортированной по (row, col).
    ///
    /// Детерминированный обход: одиночные find_cell_by_* опираются на порядок —
    /// при нескольких совпадениях побеждает ячейка с наименьшими координатами.
    /// Порядок берётся из кэша ключей, сортировка на вызов не повторяется.
    pub fn get_collection_sorted(&self) -> Vec<&Arc<RwLock<Cell>>> {
        self.sorted_keys_cloned()
            .iter()
            // Инвалидация кэша синхронна с мутациями карты: ключ из кэша
            // всегда есть в карте
            .filter_map(|key| self.map.get(key))
            .collect()
    }

    #[inline]
    pub fn get_max_row(&self) -> u32 {
        self.max_row
    }

    #[inline]
    pub fn get_max_column(&self) -> u16 {
        self.max_column
    }

    #[inline]
    pub fn get_cell_by_letter(&self, letter: &str) -> Option<&Arc<RwLock<Cell>>> {
        let coord = Coordinate::try_from(letter).ok()?;

        self.map.get(&(coord.row, coord.column))
    }

    /// Метод для получения значения ячейки по координатам
    #[inline]
    pub fn get_cell_value<T>(&self, coordinate: T) -> String
    where
        T: Into<Coordinate>,
    {
        let Coordinate { row, column } = coordinate.into();
        self.map
            .get(&(row, column))
            .map(|c| c.read().get_value())
            .unwrap_or(self.default_cell_value.get_value())
    }

    #[inline]
    pub fn get_cell_collection_by_range(
        &self,
        start_row: Option<u32>,
        end_row: Option<u32>,
        start_col: Option<u16>,
        end_col: Option<u16>,
    ) -> impl Iterator<Item = &Arc<RwLock<Cell>>> {
        let start_row = start_row.unwrap_or(1);
        let end_row = end_row.unwrap_or(MAX_ROW);
        let start_col = start_col.unwrap_or(1);
        let end_col = end_col.unwrap_or(MAX_COL);

        // Ключи из кэша уже отсортированы: фильтрация сохраняет порядок,
        // отдельная сортировка не нужна. Материализуем в Vec: возвращаемый
        // итератор не может заимствовать локальный Arc кэша
        self.sorted_keys_cloned()
            .iter()
            .filter(|(row, col)| {
                *row >= start_row && *row <= end_row && *col >= start_col && *col <= end_col
            })
            .filter_map(|key| self.map.get(key))
            .collect::<Vec<_>>()
            .into_iter()
    }

    /// Get-or-create ячейки по координатам.
    ///
    /// Существующая ячейка не мутируется: при `Some(value)` значение
    /// перезаписывается, при `None` возвращается ячейка как есть.
    #[inline]
    pub fn cell<T>(&mut self, coordinate: T, value: Option<&str>) -> &Arc<RwLock<Cell>>
    where
        T: Into<Coordinate>,
    {
        use std::collections::hash_map::Entry;

        let Coordinate { row, column } = coordinate.into();

        // Get-or-create: существующую ячейку не мутируем, новая — вставляется
        // с обновлением максимумов и инвалидацией кэша ключей
        let existed = match self.map.entry((row, column)) {
            Entry::Occupied(_) => true,
            Entry::Vacant(entry) => {
                self.max_row = self.max_row.max(row);
                self.max_column = self.max_column.max(column);
                self.sorted_keys.write().take();

                entry.insert(Arc::new(RwLock::new(Cell::new(
                    Coordinate::new(row, column),
                    value,
                ))));

                false
            }
        };

        let cell = self
            .map
            .get(&(row, column))
            .expect("ячейка только что создана или уже существовала");

        // Новая пустая ячейка уже получает data_type "s" в Cell::new
        // (Empty → "s"), поэтому без значения не требует никаких правок
        if existed && let Some(value) = value {
            cell.write().set_value(value);
        }

        cell
    }

    /// Удаляет колонки `[idx; idx + amount)`, сдвигая оставшиеся влево.
    ///
    /// Вместо полной пересборки карты: удаляемые ключи снимаются `retain`,
    /// ключи правее диапазона переучитываются точечно.
    ///
    /// # Errors
    /// Возвращает ошибку, если `idx` или `amount` меньше 1 либо диапазон
    /// выходит за границы листа (`MAX_COL`).
    #[inline]
    pub fn delete_cols(&mut self, idx: u16, amount: u16) -> Result<()> {
        validate_delete_range(idx as u64, amount as u64, MAX_COL as u64)?;

        let delete_end = idx + amount;
        // Затронутые сдвигом ячейки снимаются целиком до вставок: цель сдвига
        // (col - amount ≥ idx) не пересекается с оставшимися ключами (col < idx),
        // поэтому переучёт не задевает нетронутые записи
        let shifted_keys: Vec<(u32, u16)> = self
            .map
            .keys()
            .copied()
            .filter(|(_, col)| *col >= delete_end)
            .collect();
        let mut shifted: ShiftedCells = shifted_keys
            .into_iter()
            .map(|key| (key, self.map.remove(&key).expect("ключ из итератора карты")))
            .collect();

        let before = self.map.len();
        self.map
            .retain(|(_, col), _| *col < idx || *col >= delete_end);
        let deleted = self.map.len() != before;

        if deleted || !shifted.is_empty() {
            for ((row, col), cell) in shifted.drain(..) {
                cell.write()
                    .set_coordinate(Coordinate::new(row, col - amount));
                self.map.insert((row, col - amount), cell);
            }

            self.invalidate_derived_state();
        }

        Ok(())
    }

    /// Удаляет строки `[idx; idx + amount)`, сдвигая оставшиеся вверх.
    /// Вместо полной пересборки карты: удаляемые ключи снимаются `retain`,
    /// ключи ниже диапазона переучитываются точечно.
    ///
    /// # Errors
    /// Возвращает ошибку, если `idx` или `amount` меньше 1 либо диапазон
    /// выходит за границы листа (`MAX_ROW`).
    #[inline]
    pub fn delete_rows(&mut self, idx: u32, amount: u32) -> Result<()> {
        validate_delete_range(idx as u64, amount as u64, MAX_ROW as u64)?;

        let delete_end = idx + amount;
        // Затронутые сдвигом ячейки снимаются целиком до вставок: цель сдвига
        // (row - amount ≥ idx) не пересекается с оставшимися ключами (row < idx)
        let shifted_keys: Vec<(u32, u16)> = self
            .map
            .keys()
            .copied()
            .filter(|(row, _)| *row >= delete_end)
            .collect();
        let mut shifted: ShiftedCells = shifted_keys
            .into_iter()
            .map(|key| (key, self.map.remove(&key).expect("ключ из итератора карты")))
            .collect();

        let before = self.map.len();
        self.map
            .retain(|(row, _), _| *row < idx || *row >= delete_end);
        let deleted = self.map.len() != before;

        if deleted || !shifted.is_empty() {
            for ((row, col), cell) in shifted.drain(..) {
                cell.write()
                    .set_coordinate(Coordinate::new(row - amount, col));
                self.map.insert((row - amount, col), cell);
            }

            self.invalidate_derived_state();
        }

        Ok(())
    }

    /// Пересчёт производных данных карты после мутации набора ключей:
    /// кэш сортированных ключей сбрасывается, максимумы пересчитываются одним
    /// проходом. Вызывается только из точек мутаций, поэтому O(n) здесь —
    /// разовая цена редкой операции.
    fn invalidate_derived_state(&mut self) {
        let (max_row, max_column) = self
            .map
            .keys()
            .fold((0, 0), |(row, col), &(r, c)| (row.max(r), col.max(c)));

        self.max_row = max_row;
        self.max_column = max_column;
        self.sorted_keys.write().take();
    }

    #[inline]
    pub fn find_cell_by_regex(&self, regex: &str) -> Result<Option<&Arc<RwLock<Cell>>>> {
        let cells = self.get_collection_sorted();

        find_cell_by_regex(regex.into(), cells)
    }

    #[inline]
    pub fn find_cell_by_str(&self, value: &str) -> Result<Option<&Arc<RwLock<Cell>>>> {
        let cells = self.get_collection_sorted();

        find_cell_by_str(value.into(), cells)
    }

    #[inline]
    pub fn find_cell_by_coords(&self, row: u32, col: u16) -> Result<Option<&Arc<RwLock<Cell>>>> {
        Ok(self.map.get(&(row, col)))
    }

    #[inline]
    pub fn find_cell_by_letter(&self, letter: &str) -> Result<Option<&Arc<RwLock<Cell>>>> {
        // Ячейка с координатой уникальна в карте: O(1) по ключу вместо скана
        let coord = Coordinate::try_from(letter)?;

        Ok(self.map.get(&(coord.row, coord.column)))
    }

    #[inline]
    pub fn find_cells_by_regex(&self, regex: &str) -> Result<Vec<&Arc<RwLock<Cell>>>> {
        let cells = self.get_collection_sorted();

        find_cells_by_regex(regex.into(), cells)
    }

    #[inline]
    pub fn find_cells_by_str(&self, value: &str) -> Result<Vec<&Arc<RwLock<Cell>>>> {
        let cells = self.get_collection_sorted();

        find_cells_by_str(value.into(), cells)
    }

    #[inline]
    pub fn find_cells_for_rows_by_regex(
        &self,
        regex: &str,
        col_stop: u16,
    ) -> Result<Vec<&Arc<RwLock<Cell>>>> {
        let cells = self.get_collection_sorted();

        find_cells_for_rows_by_regex(regex.into(), col_stop, cells)
    }

    #[inline]
    pub fn find_cells_for_cols_by_regex(
        &self,
        regex: &str,
        row_stop: u32,
    ) -> Result<Vec<&Arc<RwLock<Cell>>>> {
        let cells = self.get_collection_sorted();

        find_cells_for_cols_by_regex(regex.into(), row_stop, cells)
    }

    #[inline]
    pub fn find_cells_multi_regex(
        &self,
        before_regex: &str,
        after_regex: &str,
    ) -> Result<Vec<&Arc<RwLock<Cell>>>> {
        let cells = self.get_collection_sorted();

        find_cells_multi_regex(before_regex.into(), after_regex.into(), cells)
    }

    #[inline]
    pub fn find_cells_between_regex(
        &self,
        before_regex: &str,
        after_regex: &str,
    ) -> Result<Vec<&Arc<RwLock<Cell>>>> {
        let before_regex = compile_case_insensitive(before_regex)?;
        let after_regex = compile_case_insensitive(after_regex)?;

        // Проход 1 — строки границ. Значения читаются только до второй
        // границы: дальнейшие совпадения на результат не влияют. Ошибка
        // выполнения шаблона не глотается, а уходит вызывающему коду
        let mut boundaries: [Option<u32>; 2] = [None, None];
        let mut after_before = false;

        for cell in self.get_collection_sorted() {
            let guard = cell.read();
            let v = guard.get_value();
            let hit = if !after_before {
                is_match(&before_regex, &v)?
            } else {
                is_match(&after_regex, &v)?
            };

            if hit {
                let slot = if after_before {
                    &mut boundaries[1]
                } else {
                    &mut boundaries[0]
                };
                *slot = Some(guard.get_coordinate().row);

                if after_before {
                    break;
                }

                after_before = !after_before;
            }
        }

        let Some(start_row) = boundaries[0] else {
            return Ok(Vec::new());
        };

        // Диапазон строк — непрерывный отрезок сортированных ключей: границы
        // отрезка находятся бинарным поиском вместо прохода
        let keys = self.sorted_keys_cloned();
        let lower = keys.partition_point(|&(row, _)| row < start_row);
        let upper = match boundaries[1] {
            Some(end_row) => keys.partition_point(|&(row, _)| row <= end_row),
            None => keys.len(),
        };

        Ok(keys[lower..upper]
            .iter()
            .filter_map(|key| self.map.get(key))
            .collect())
    }

    #[inline]
    pub fn find_cells_range_rows(
        &self,
        start_row: u32,
        end_row: u32,
    ) -> Result<Vec<&Arc<RwLock<Cell>>>> {
        let cells = self.get_collection_sorted();

        find_cells_range_rows(start_row, end_row, cells)
    }

    #[inline]
    pub fn find_cells_range_cols(
        &self,
        start_col: u16,
        end_col: u16,
    ) -> Result<Vec<&Arc<RwLock<Cell>>>> {
        let cells = self.get_collection_sorted();

        find_cells_range_cols(start_col, end_col, cells)
    }

    /// Значения по колонке и списку строк — строго в порядке запрошенных строк.
    /// Отсутствующая ячейка даёт пустую строку: длина результата всегда равна
    /// длине запроса, поэтому позиционный zip с заголовками не искажает отчёт.
    /// Ключи карты уникальны — прямой O(1) поиск на запрошенную координату.
    #[inline]
    pub fn find_values_by_col_rows(&self, col: u16, rows: Vec<u32>) -> Result<Vec<String>> {
        Ok(rows
            .into_iter()
            .map(|row| self.get_cell_value((row, col)))
            .collect())
    }

    /// Значения по строке и списку колонок — строго в порядке запрошенных
    /// колонок. Семантика как у [`Cells::find_values_by_col_rows`].
    #[inline]
    pub fn find_values_by_row_cols(&self, row: u32, cols: Vec<u16>) -> Result<Vec<String>> {
        Ok(cols
            .into_iter()
            .map(|col| self.get_cell_value((row, col)))
            .collect())
    }

    #[inline]
    pub fn find_value_by_coords(&self, row: u32, col: u16) -> Result<Option<String>> {
        let value = self.map.get(&(row, col)).map(|cell| {
            let guard = cell.read();
            guard.get_value().to_string()
        });

        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cells_with(pairs: &[((u32, u16), &str)]) -> Cells {
        let map: HashMap<_, _> = pairs
            .iter()
            .map(|&(key, value)| {
                (
                    key,
                    Arc::new(RwLock::new(Cell::new(
                        Coordinate::new(key.0, key.1),
                        Some(value),
                    ))),
                )
            })
            .collect();

        Cells::new(map)
    }

    /// Кэш сортированных ключей: порядок детерминированный, а вставка новой
    /// ячейки после построения кэша не теряется в выдаче.
    #[test]
    fn sorted_collection_reflects_new_cells() {
        let mut cells = cells_with(&[((2, 2), "b"), ((1, 1), "a")]);

        let sorted: Vec<(u32, u16)> = cells
            .get_collection_sorted()
            .iter()
            .map(|cell| {
                let guard = cell.read();
                let coord = guard.get_coordinate();
                (coord.row, coord.column)
            })
            .collect();

        assert_eq!(sorted, vec![(1, 1), (2, 2)]);

        // Кэш уже построен — новая ячейка обязана появиться в выдаче
        cells.cell(Coordinate::new(1, 2), Some("c"));

        let sorted: Vec<(u32, u16)> = cells
            .get_collection_sorted()
            .iter()
            .map(|cell| {
                let guard = cell.read();
                let coord = guard.get_coordinate();
                (coord.row, coord.column)
            })
            .collect();

        assert_eq!(sorted, vec![(1, 1), (1, 2), (2, 2)]);
    }

    /// Максимумы поддерживаются при вставках и удалениях, а не пересчитываются
    /// на каждом обращении.
    #[test]
    fn max_row_and_column_tracked_on_mutations() {
        let mut cells = cells_with(&[((1, 1), "a"), ((5, 3), "b")]);

        assert_eq!(cells.get_max_row(), 5);
        assert_eq!(cells.get_max_column(), 3);

        cells.cell(Coordinate::new(7, 2), Some("c"));

        assert_eq!(cells.get_max_row(), 7);
        assert_eq!(cells.get_max_column(), 3);

        cells.delete_rows(7, 1).unwrap();

        assert_eq!(cells.get_max_row(), 5);

        cells.delete_cols(3, 1).unwrap();

        assert_eq!(cells.get_max_column(), 1);
    }

    /// Поиск по букве идёт по ключу карты: найденная ячейка, промах и ошибка
    /// разбора буквы сохраняют прежние контракты.
    #[test]
    fn find_cell_by_letter_uses_map_key() {
        let cells = cells_with(&[((1, 1), "a"), ((2, 5), "b")]);

        let found = cells.find_cell_by_letter("E2").unwrap().unwrap();
        assert_eq!(found.read().get_value(), "b");

        assert!(cells.find_cell_by_letter("Z9").unwrap().is_none());
        assert!(cells.find_cell_by_letter("1A").is_err());
    }

    /// Границы between: без совпадений — пусто; без второй границы — от первой
    /// до конца; с двумя — замкнутый диапазон строк.
    #[test]
    fn find_cells_between_regex_boundaries() {
        let cells = cells_with(&[
            ((1, 1), "шапка"),
            ((2, 1), "строка 1"),
            ((3, 1), "строка 2"),
            ((4, 1), "подвал"),
            ((5, 1), "хвост"),
        ]);

        let letters = |result: Vec<&Arc<RwLock<Cell>>>| {
            result
                .iter()
                .map(|cell| cell.read().get_letter())
                .collect::<Vec<_>>()
        };

        assert!(
            letters(
                cells
                    .find_cells_between_regex("нет такого", "подвал")
                    .unwrap()
            )
            .is_empty()
        );

        assert_eq!(
            letters(
                cells
                    .find_cells_between_regex("подвал", "нет такого")
                    .unwrap()
            ),
            vec!["A4", "A5"],
            "одна граница: от неё до конца"
        );

        assert_eq!(
            letters(cells.find_cells_between_regex("шапка", "подвал").unwrap()),
            vec!["A1", "A2", "A3", "A4"],
            "две границы: замкнутый диапазон"
        );
    }

    /// Сдвиг при удалении не должен перезаписать ещё не сдвинутую ячейку:
    /// колонки удаляемого диапазона и правее чередуются.
    #[test]
    fn delete_cols_keeps_cells_on_key_collision() {
        // Удаляем колонку 2: B уходит, D сдвигается на место бывшей C
        let mut cells = cells_with(&[((1, 1), "A"), ((1, 2), "B"), ((1, 3), "C"), ((1, 4), "D")]);

        cells.delete_cols(2, 1).unwrap();

        let values: Vec<String> = cells
            .get_collection_sorted()
            .iter()
            .map(|cell| cell.read().get_value())
            .collect();

        assert_eq!(values, vec!["A", "C", "D"]);
        assert_eq!(cells.get_max_column(), 3);
    }

    /// Удаление последних строк: сдвигаемых ячеек нет, но карта меняется —
    /// кэш ключей и максимум обязаны обновиться.
    #[test]
    fn delete_last_rows_invalidates_cache_and_max() {
        let mut cells = cells_with(&[((1, 1), "a"), ((3, 1), "c")]);

        // Строим кэш до удаления
        assert_eq!(cells.get_collection_sorted().len(), 2);

        cells.delete_rows(2, 2).unwrap();

        assert_eq!(cells.get_max_row(), 1);
        assert_eq!(cells.get_collection_sorted().len(), 1);
    }

    /// Удаление без единой ячейки в диапазоне — карта не меняется, кэш жив.
    #[test]
    fn delete_empty_range_keeps_cache() {
        let mut cells = cells_with(&[((1, 1), "a"), ((2, 2), "b")]);

        assert_eq!(cells.get_collection_sorted().len(), 2);

        cells.delete_rows(10, 5).unwrap();
        cells.delete_cols(10, 5).unwrap();

        assert_eq!(cells.get_collection_sorted().len(), 2);
        assert_eq!(cells.get_max_row(), 2);
        assert_eq!(cells.get_max_column(), 2);
    }

    /// Значения по списку строк идут в порядке запроса, отсутствующие ячейки
    /// дают пустую строку.
    #[test]
    fn find_values_by_col_rows_order_and_gaps() {
        let cells = cells_with(&[((1, 2), "h"), ((3, 2), "x")]);

        let values = cells.find_values_by_col_rows(2, vec![1, 2, 3]).unwrap();

        assert_eq!(values, vec!["h", "", "x"]);
    }

    /// Глубокая копия разделяет набор ключей: кэш валиден, а мутации копии
    /// не видны в оригинале.
    #[test]
    fn clone_deep_shares_sorted_keys_cache() {
        let cells = cells_with(&[((2, 1), "b"), ((1, 1), "a")]);

        // Строим кэш
        assert_eq!(cells.get_collection_sorted().len(), 2);

        let mut copy = cells.clone_deep();
        copy.cell(Coordinate::new(1, 1), Some("изменено"));

        assert_eq!(cells.find_value_by_coords(1, 1).unwrap().unwrap(), "a");
        assert_eq!(
            copy.find_value_by_coords(1, 1).unwrap().unwrap(),
            "изменено"
        );
        assert_eq!(copy.get_collection_sorted().len(), 2);
    }
}
