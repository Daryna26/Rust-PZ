use thiserror::Error;

#[derive(Debug, Error)]
pub enum SnippetError {
    #[error("title cannot be empty")]
    EmptyTitle,

    #[error("snippet not found")]
    NotFound,
}
//Для pull request
