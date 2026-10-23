use std::sync::{Arc, LazyLock};

use ahash::HashMap;
use fancy_regex::Regex;
use parking_lot::Mutex;

use crate::error::{Error, Result};

/// Максимальная длина шаблона: заведомо больше любого легитимного заголовка
/// отчёта, но отсекает случайные и злонамеренные гигантские паттерны.
pub(crate) const MAX_PATTERN_LEN: usize = 512;

/// Ёмкость кэша компиляции: при переполнении кэш сбрасывается целиком —
/// дешевле вытесняющей политики, а рабочие наборы шаблонов в отчётах
/// умещаются с запасом.
const CACHE_CAPACITY: usize = 128;

/// Кэш скомпилированных шаблонов: повторные поиски с тем же шаблоном
/// (типовой случай — один и тот же отчёт по многим листам) не
/// перекомпилируются.
static CACHE: LazyLock<Mutex<HashMap<Box<str>, Arc<Regex>>>> =
    LazyLock::new(|| Mutex::new(HashMap::default()));

/// Компиляция шаблона без учёта регистра (семантика всех `find_*_by_regex`):
/// лимит длины, кэш компиляции, типизированная ошибка.
///
/// # Errors
///
/// `PatternTooLong` — шаблон длиннее `MAX_PATTERN_LEN`; `InvalidRegex` —
/// шаблон не компилируется движком regex.
pub(crate) fn compile_case_insensitive(pattern: &str) -> Result<Arc<Regex>> {
    if pattern.len() > MAX_PATTERN_LEN {
        return Err(Error::pattern_too_long(
            pattern,
            MAX_PATTERN_LEN,
            pattern.len(),
        ));
    }

    let mut cache = CACHE.lock();

    if let Some(re) = cache.get(pattern) {
        return Ok(Arc::clone(re));
    }

    let re = Arc::new(
        Regex::new(&format!("(?i){pattern}")).map_err(|e| Error::invalid_regex(pattern, e))?,
    );

    if cache.len() >= CACHE_CAPACITY {
        cache.clear();
    }
    cache.insert(pattern.into(), Arc::clone(&re));

    Ok(re)
}

/// Совпадение текста с шаблоном: ошибка выполнения (например, исчерпанный
/// лимит обратной трассировки) — это ошибка, а не «не совпало».
///
/// # Errors
///
/// `RegexExec` — сбой движка regex на этом тексте.
pub(crate) fn is_match(re: &Regex, text: &str) -> Result<bool> {
    re.is_match(text).map_err(Error::regex_exec)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Длинный шаблон отклоняется с текстом о превышении предела.
    #[test]
    fn too_long_pattern_is_rejected() {
        let pattern = "a".repeat(MAX_PATTERN_LEN + 1);

        let err = compile_case_insensitive(&pattern).unwrap_err();
        assert!(
            err.to_string().contains("exceeds the maximum"),
            "получено: {err}"
        );

        // Граничное значение допустимо
        assert!(compile_case_insensitive(&"a".repeat(MAX_PATTERN_LEN)).is_ok());
    }
}
