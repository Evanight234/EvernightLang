use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorLevel {
    Bahaya,
    Peringatan,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EvernightError {
    pub level: ErrorLevel,
    pub code: String,
    pub line: usize,
    pub column: usize,
    pub message: String,
}

impl EvernightError {
    pub fn bahaya(
        code: impl Into<String>,
        line: usize,
        column: usize,
        message: impl Into<String>,
    ) -> Self {
        Self {
            level: ErrorLevel::Bahaya,
            code: code.into(),
            line,
            column,
            message: message.into(),
        }
    }

    pub fn peringatan(
        code: impl Into<String>,
        line: usize,
        column: usize,
        message: impl Into<String>,
    ) -> Self {
        Self {
            level: ErrorLevel::Peringatan,
            code: code.into(),
            line,
            column,
            message: message.into(),
        }
    }
}

impl fmt::Display for EvernightError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let level_str = match self.level {
            ErrorLevel::Bahaya => "BAHAYA",
            ErrorLevel::Peringatan => "PERINGATAN",
        };
        write!(
            f,
            "{} [{}]: Baris {} (Kolom {}) - {}",
            level_str, self.code, self.line, self.column, self.message
        )
    }
}

impl std::error::Error for EvernightError {}
