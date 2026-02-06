use std::fmt;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Duplicate item: front '{0}' already exists")]
    Duplicate(String),

    #[error("Item '{0}' not found")]
    NotFound(String),

    #[error("Ambiguous ID '{0}': matches multiple items")]
    AmbiguousId(String),

    #[error("Data file not found: {0}")]
    NoDataFile(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}

impl AppError {
    pub fn code(&self) -> &'static str {
        match self {
            AppError::Duplicate(_) => "duplicate",
            AppError::NotFound(_) => "not_found",
            AppError::AmbiguousId(_) => "ambiguous_id",
            AppError::NoDataFile(_) => "no_data_file",
            AppError::InvalidInput(_) => "invalid_input",
            AppError::IoError(_) | AppError::JsonError(_) => "io_error",
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "ok": false,
            "error": self.code(),
            "message": self.to_string()
        })
    }
}

/// Wrapper for success output
pub fn success_json(data: serde_json::Value) -> serde_json::Value {
    serde_json::json!({ "ok": true, "data": data })
}

/// Pretty-print JSON to a writer
pub fn write_json(writer: &mut impl fmt::Write, value: &serde_json::Value) -> fmt::Result {
    write!(writer, "{}", serde_json::to_string_pretty(value).unwrap())
}
