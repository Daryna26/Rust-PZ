use snippets_app::{Snippet, SnippetStorage, validate_title};

#[test]
fn integration_add_and_read_snippet() {
    let mut storage = SnippetStorage::new();

    let snippet = Snippet {
        id: 10,
        title: "Integration".into(),
        content: "Test".into(),
    };

    storage.add(snippet);

    let result = storage.get(10).unwrap();
    assert_eq!(result.content, "Test");
}

#[test]
fn integration_title_validation() {
    assert!(validate_title("OK").is_ok());
    assert!(validate_title("").is_err());
}
