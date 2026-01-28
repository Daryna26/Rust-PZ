#![warn(
    missing_docs,
    broken_intra_doc_links,
    unreachable_pub,
    clippy::missing_panics_doc,
    clippy::clone_on_ref_ptr,
    clippy::similar_names
)]

use snippets_app::{Snippet, SnippetStorage};

/// Application entry point.
fn main() {
    let mut storage = SnippetStorage::new();

    let snippet = Snippet::new("hello", "Hello, Rust!");
    storage.add(snippet);

    if let Some(found) = storage.get("hello") {
        println!("{}", found.content());
    }
}
//Для pull request

