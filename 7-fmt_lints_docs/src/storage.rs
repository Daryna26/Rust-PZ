use std::collections::HashMap;

use crate::Snippet;

/// In-memory storage for [`Snippet`] values.
#[derive(Debug, Default)]
pub struct SnippetStorage {
    snippets: HashMap<String, Snippet>,
}

impl SnippetStorage {
    /// Creates an empty [`SnippetStorage`].
    pub fn new() -> Self {
        Self {
            snippets: HashMap::new(),
        }
    }

    /// Adds a snippet to storage.
    ///
    /// If a snippet with the same name already exists, it is replaced.
    pub fn add(&mut self, snippet: Snippet) {
        self.snippets.insert(snippet.name().to_owned(), snippet);
    }

    /// Returns a snippet by name.
    pub fn get(&self, name: &str) -> Option<&Snippet> {
        self.snippets.get(name)
    }
}
