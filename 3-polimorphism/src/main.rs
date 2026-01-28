use std::borrow::Cow;
use std::collections::HashMap;
use std::env;
use std::fs;
use serde::{Serialize, Deserialize};
use rusqlite::{params, Connection};
use chrono::Utc;

// Part 1: UserRepository 

trait Storage<K, V> {
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}

struct User {
    id: u64,
    email: Cow<'static, str>,
    activated: bool,
}

// HashMap storage
struct HashMapStorage<K, V> {
    data: HashMap<K, V>,
}

impl<K: std::hash::Hash + Eq, V> HashMapStorage<K, V> {
    fn new() -> Self {
        Self { data: HashMap::new() }
    }
}

impl<K: std::hash::Hash + Eq, V> Storage<K, V> for HashMapStorage<K, V> {
    fn set(&mut self, key: K, val: V) {
        self.data.insert(key, val);
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.data.get(key)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.data.remove(key)
    }
}

// Dynamic dispatch 
struct UserRepositoryDynamic {
    storage: Box<dyn Storage<u64, User>>,
}

impl UserRepositoryDynamic {
    fn new(storage: Box<dyn Storage<u64, User>>) -> Self {
        Self { storage }
    }

    fn add_user(&mut self, user: User) {
        self.storage.set(user.id, user);
    }

    fn get_user(&self, id: u64) -> Option<&User> {
        self.storage.get(&id)
    }

    fn update_user(&mut self, user: User) {
        self.storage.set(user.id, user);
    }

    fn remove_user(&mut self, id: u64) -> Option<User> {
        self.storage.remove(&id)
    }
}

// Static dispatch
struct UserRepositoryStatic<S: Storage<u64, User>> {
    storage: S,
}

impl<S: Storage<u64, User>> UserRepositoryStatic<S> {
    fn new(storage: S) -> Self {
        Self { storage }
    }

    fn add_user(&mut self, user: User) {
        self.storage.set(user.id, user);
    }

    fn get_user(&self, id: u64) -> Option<&User> {
        self.storage.get(&id)
    }

    fn update_user(&mut self, user: User) {
        self.storage.set(user.id, user);
    }

    fn remove_user(&mut self, id: u64) -> Option<User> {
        self.storage.remove(&id)
    }
}

// Part 2: Snippets App

#[derive(Serialize, Deserialize)]
struct Snippet {
    id: u64,
    code: String,
    created_at: String,
}

fn run_snippets_app() {
    let storage_env = env::var("SNIPPETS_APP_STORAGE").unwrap_or_else(|_| "JSON:snippets.json".to_string());
    let parts: Vec<&str> = storage_env.splitn(2, ':').collect();

    match parts[0] {
        "JSON" => {
            let path = parts[1];
            let mut snippets: Vec<Snippet> = serde_json::from_str(&fs::read_to_string(path).unwrap_or_default()).unwrap_or_default();

            // Приклад додавання нового snippet
            let new_snippet = Snippet {
                id: snippets.len() as u64 + 1,
                code: "println!(\"Hello World\");".to_string(),
                created_at: Utc::now().to_rfc3339(),
            };
            snippets.push(new_snippet);

            fs::write(path, serde_json::to_string_pretty(&snippets).unwrap()).unwrap();
            println!("JSON storage updated. Total snippets: {}", snippets.len());
        }
        "SQLITE" => {
            let conn = Connection::open(parts[1]).unwrap();
            conn.execute(
                "CREATE TABLE IF NOT EXISTS snippets (id INTEGER PRIMARY KEY, code TEXT, created_at TEXT)",
                [],
            ).unwrap();

            let new_snippet = Snippet {
                id: 1,
                code: "println!(\"Hello SQLite\");".to_string(),
                created_at: Utc::now().to_rfc3339(),
            };

            conn.execute(
                "INSERT INTO snippets (id, code, created_at) VALUES (?1, ?2, ?3)",
                params![new_snippet.id, new_snippet.code, new_snippet.created_at],
            ).unwrap();

            println!("SQLite storage updated.");
        }
        _ => panic!("Unsupported storage type"),
    }
}

// Main 

fn main() {
    // Тести UserRepository
    let mut dynamic_repo = UserRepositoryDynamic::new(Box::new(HashMapStorage::new()));
    dynamic_repo.add_user(User { id: 1, email: Cow::Borrowed("a@example.com"), activated: true });
    assert!(dynamic_repo.get_user(1).is_some());
    dynamic_repo.remove_user(1);
    assert!(dynamic_repo.get_user(1).is_none());

    let mut static_repo = UserRepositoryStatic::new(HashMapStorage::new());
    static_repo.add_user(User { id: 2, email: Cow::Borrowed("b@example.com"), activated: false });
    assert!(static_repo.get_user(2).is_some());
    static_repo.remove_user(2);
    assert!(static_repo.get_user(2).is_none());

    println!("UserRepository tests passed!");

    // Run snippets app
    run_snippets_app();
}

//Для pull request

