use std::fs;
use std::path::Path;

use crate::snippet::Snippet;
use crate::error::AppError;

pub struct SnippetStorage {
    file_path: String,
    snippets: Vec<Snippet>,
}

impl SnippetStorage {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, AppError> {
        let file_path = path.as_ref().to_string_lossy().to_string();

        let snippets = if path.as_ref().exists() {
            let data = fs::read_to_string(&file_path)?;
            serde_json::from_str(&data)?
        } else {
            Vec::new()
        };

        Ok(Self {
            file_path,
            snippets,
        })
    }

    pub fn save(&self) -> Result<(), AppError> {
        let json = serde_json::to_string_pretty(&self.snippets)?;
        fs::write(&self.file_path, json)?;
        Ok(())
    }

    pub fn add_snippet(&mut self, title: &str, content: &str) -> Result<(), AppError> {
        if self.snippets.iter().any(|s| s.title == title) {
            return Err(AppError::DuplicateSnippet(title.to_string()));
        }

        self.snippets.push(Snippet {
            title: title.to_string(),
            content: content.to_string(),
        });

        Ok(())
    }

    pub fn get_snippet(&self, title: &str) -> Result<&Snippet, AppError> {
        self.snippets
            .iter()
            .find(|s| s.title == title)
            .ok_or_else(|| AppError::SnippetNotFound(title.to_string()))
    }
}
