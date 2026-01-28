mod error;
mod snippet;
mod storage;

use crate::storage::SnippetStorage;
use crate::error::AppError;

fn main() {
    if let Err(e) = run() {
        eprintln!("Application error: {}", e);
    }
}

fn run() -> Result<(), AppError> {
    let mut storage = SnippetStorage::load("snippets.json")?;

    storage.add_snippet("Rust", "Rust is fast and safe")?;
    storage.add_snippet("Error handling", "Use Result and thiserror")?;

    let snippet = storage.get_snippet("Rust")?;
    println!("Found snippet: {}", snippet.content);

    storage.save()?;
 
   Ok(())
}
//Для pull request

