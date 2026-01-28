use crate::{Snippet, SnippetError};

#[derive(Default)]
pub struct SnippetStorage {
    snippets: Vec<Snippet>,
}

impl SnippetStorage {
    pub fn new() -> Self {
        Self { snippets: Vec::new() }
    }

    pub fn add(&mut self, snippet: Snippet) {
        self.snippets.push(snippet);
    }

    pub fn get(&self, id: usize) -> Result<&Snippet, SnippetError> {
        self.snippets
            .iter()
            .find(|s| s.id == id)
            .ok_or(SnippetError::NotFound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_and_get_snippet() {
        let mut storage = SnippetStorage::new();
        storage.add(Snippet {
            id: 1,
            title: "A".into(),
            content: "B".into(),
        });

        let s = storage.get(1).unwrap();
        assert_eq!(s.title, "A");
    }

    #[test]
    fn get_missing_snippet_returns_error() {
        let storage = SnippetStorage::new();
        assert!(matches!(storage.get(99), Err(SnippetError::NotFound)));
    }
}
