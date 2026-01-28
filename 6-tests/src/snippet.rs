use crate::errors::SnippetError;

#[derive(Debug, Clone)]
pub struct Snippet {
    pub id: usize,
    pub title: String,
    pub content: String,
}

///
/// ```
/// use snippets_app::Snippet;
///
/// let s = Snippet {
///     id: 1,
///     title: "Test".into(),
///     content: "Body".into(),
/// };
///
/// assert_eq!(s.title, "Test");
/// ```
pub fn validate_title(title: &str) -> Result<(), SnippetError> {
    if title.trim().is_empty() {
        Err(SnippetError::EmptyTitle)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_title_ok() {
        assert!(validate_title("Hello").is_ok());
    }

    #[test]
    fn empty_title_err() {
        assert!(matches!(
            validate_title(" "),
            Err(SnippetError::EmptyTitle)
        ));
    }
}
