use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

/// Simple Todo CLI (stores data in todos.json in current dir)
#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "Tiny todo CLI example in Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new todo
    Add { title: String },

    /// List todos
    List,

    /// Mark a todo as done by its id
    Done { id: usize },

    /// Remove a todo by its id
    Remove { id: usize },

    /// Clear all todos
    Clear,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Todo {
    id: usize,
    title: String,
    done: bool,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Store {
    todos: Vec<Todo>,
}

impl Store {
    fn load(path: &PathBuf) -> io::Result<Self> {
        if !path.exists() {
            return Ok(Store::default());
        }
        let data = fs::read_to_string(path)?;
        let store: Store = serde_json::from_str(&data)?;
        Ok(store)
    }

    fn save(&self, path: &PathBuf) -> io::Result<()> {
        let data = serde_json::to_string_pretty(self)?;
        fs::write(path, data)?;
        Ok(())
    }

    fn add(&mut self, title: String) -> usize {
        let id = self.next_id();
        let todo = Todo {
            id,
            title,
            done: false,
        };
        self.todos.push(todo);
        id
    }

    fn next_id(&self) -> usize {
        self.todos.iter().map(|t| t.id).max().unwrap_or(0) + 1
    }

    fn list(&self) -> Vec<Todo> {
        self.todos.clone()
    }

    fn mark_done(&mut self, id: usize) -> bool {
        if let Some(t) = self.todos.iter_mut().find(|t| t.id == id) {
            t.done = true;
            true
        } else {
            false
        }
    }

    fn remove(&mut self, id: usize) -> bool {
        let before = self.todos.len();
        self.todos.retain(|t| t.id != id);
        before != self.todos.len()
    }

    fn clear(&mut self) {
        self.todos.clear();
    }
}

fn default_store_path() -> PathBuf {
    PathBuf::from("todos.json")
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let path = default_store_path();

    let mut store = match Store::load(&path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to load store: {}", e);
            Store::default()
        }
    };

    match cli.command {
        Commands::Add { title } => {
            let id = store.add(title.clone());
            store.save(&path)?;
            println!("Added todo #{}: {}", id, title);
        }
        Commands::List => {
            let todos = store.list();
            if todos.is_empty() {
                println!("No todos yet. Use `todo add \"Buy milk\"`");
            } else {
                for t in todos {
                    println!(
                        "[{}] {} - {}",
                        t.id,
                        if t.done { "x" } else { " " },
                        t.title
                    );
                }
            }
        }
        Commands::Done { id } => {
            if store.mark_done(id) {
                store.save(&path)?;
                println!("Marked #{} done.", id);
            } else {
                eprintln!("No todo with id {}", id);
            }
        }
        Commands::Remove { id } => {
            if store.remove(id) {
                store.save(&path)?;
                println!("Removed #{}.", id);
            } else {
                eprintln!("No todo with id {}", id);
            }
        }
        Commands::Clear => {
            print!("Are you sure you want to delete all todos? (y/N): ");
            io::stdout().flush().ok();
            let mut input = String::new();
            io::stdin().read_line(&mut input).ok();
            if input.trim().to_lowercase() == "y" {
                store.clear();
                store.save(&path)?;
                println!("All todos cleared.");
            } else {
                println!("Aborted.");
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_and_list() {
        let mut s = Store::default();
        assert_eq!(s.list().len(), 0);
        let id = s.add("write tests".into());
        assert_eq!(id, 1);
        let list = s.list();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].title, "write tests");
        assert_eq!(list[0].done, false);
    }

    #[test]
    fn mark_and_remove() {
        let mut s = Store::default();
        let id1 = s.add("a".into());
        let id2 = s.add("b".into());
        assert!(s.mark_done(id1));
        assert!(s.todos.iter().find(|t| t.id == id1).unwrap().done);
        assert!(s.remove(id2));
        assert_eq!(s.list().len(), 1);
    }
}
