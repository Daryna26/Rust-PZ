use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Snippet {
    pub title: String,
    pub content: String,
}
