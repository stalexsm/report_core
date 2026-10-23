use fancy_regex;

/// Ошибки ядра.
///
/// Доменные варианты (`InvalidCellLetter`, `InvalidRegex`, `PatternTooLong`,
/// `Domain`) означают невалидные входные данные вызывающего кода и доходят
/// до Python как `ValueError`; внутренние сбои (`RegexExec`, `Json`) —
/// как `RuntimeError`.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Буква не является координатой ячейки (например, «1A») либо выходит
    /// за границы листа.
    #[error("invalid cell letter: {letter}")]
    InvalidCellLetter { letter: String },

    /// Шаблон не компилируется движком regex.
    #[error("invalid regex {pattern:?}: {source}")]
    InvalidRegex {
        pattern: String,
        source: fancy_regex::Error,
    },

    /// Длина шаблона превышает допустимый предел.
    #[error("regex pattern length {len} exceeds the maximum of {limit}")]
    PatternTooLong {
        pattern: String,
        limit: usize,
        len: usize,
    },

    /// Шаблон валиден, но выполнение упало (например, исчерпан лимит
    /// обратной трассировки на катастрофическом шаблоне).
    #[error("regex execution failed: {source}")]
    RegexExec { source: fancy_regex::Error },

    /// Нарушение домена Excel при валидации диапазонов (удаление строк/колонок).
    #[error("{message}")]
    Domain { message: String },

    /// Сбой сериализации книги в JSON.
    #[error("Failed to convert in JSON: {source}")]
    Json {
        #[from]
        source: serde_json::Error,
    },
}

impl Error {
    /// Ошибка невалидной буквенной координаты.
    pub(crate) fn invalid_cell_letter(letter: impl Into<String>) -> Self {
        Self::InvalidCellLetter {
            letter: letter.into(),
        }
    }

    /// Ошибка компиляции шаблона.
    pub(crate) fn invalid_regex(pattern: impl Into<String>, source: fancy_regex::Error) -> Self {
        Self::InvalidRegex {
            pattern: pattern.into(),
            source,
        }
    }

    /// Ошибка превышения лимита длины шаблона.
    pub(crate) fn pattern_too_long(pattern: impl Into<String>, limit: usize, len: usize) -> Self {
        Self::PatternTooLong {
            pattern: pattern.into(),
            limit,
            len,
        }
    }

    /// Ошибка выполнения валидного шаблона.
    pub(crate) fn regex_exec(source: fancy_regex::Error) -> Self {
        Self::RegexExec { source }
    }

    /// Ошибка домена Excel: текст сообщения идёт вызывающему коду как есть.
    pub(crate) fn domain(message: impl Into<String>) -> Self {
        Self::Domain {
            message: message.into(),
        }
    }
}

/// Псевдоним результата с ошибкой ядра.
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    /// Доменные ошибки несут человекочитаемые тексты, которые доходят
    /// до вызывающего кода как есть.
    #[test]
    fn display_messages() {
        assert_eq!(
            Error::invalid_cell_letter("1A").to_string(),
            "invalid cell letter: 1A"
        );

        let err = Error::domain("Тест сообщения домена");
        assert_eq!(err.to_string(), "Тест сообщения домена");
    }
}
