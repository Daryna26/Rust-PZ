#[derive(Debug)]
pub struct Snippet {
    name: String,
    content: String,
}

impl Snippet {
    /// Creates a new [`Snippet`].
    ///
    /// # Arguments
    ///
    /// * `name` - Unique snippet name
    /// * `content` - Snippet text
    pub fn new(name: &str, content: &str) -> Self {
        Self {
            name: name.to_owned(),
            content: content.to_owned(),
        }
    }

    /// Returns the snippet name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the snippet content.
    pub fn content(&self) -> &str {
        &self.content
    }
}
