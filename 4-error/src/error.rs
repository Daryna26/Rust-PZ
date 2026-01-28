use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Snippet with title '{0}' not found")]
    SnippetNotFound(String),

    #[error("Snippet with title '{0}' already exists")]
    DuplicateSnippet(String),
}
