#![warn(
    missing_docs,
    broken_intra_doc_links,
    unreachable_pub,
    clippy::missing_panics_doc,
    clippy::clone_on_ref_ptr,
    clippy::similar_names
)]

mod snippet;
mod storage;

pub use snippet::Snippet;
pub use storage::SnippetStorage;
