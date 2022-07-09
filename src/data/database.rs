use std::collections::HashSet;

#[path = "../git/git.rs"]
mod git;

#[derive(Eq, Hash, PartialEq)]
struct DatabaseEntry {
    url: String,
    commit_hash: String,
}

pub struct Database {
    entries: HashSet<DatabaseEntry>,
}

impl Database {
    pub fn new() -> Database {
        Database {
            entries: HashSet::new(),
        }
    }

    // Add a new repo to the database.
    pub fn add(&mut self, url: &str, commit_hash: &str) {
        self.entries.insert(DatabaseEntry {
            url: url.to_string(),
            commit_hash: commit_hash.to_string(),
        });
    }

    // remove a repo from the database.
    pub fn remove(&mut self, url: &str) {
        self.entries.remove(&DatabaseEntry {
            url: url.to_string(),
            commit_hash: "".to_string(),
        });
    }

    pub fn list(&self) -> Vec<String> {
        self.entries
            .iter()
            .map(|entry| format!("{} {}", entry.url, entry.commit_hash))
            .collect()
    }

    // Check if a repo had recent updates.
    pub fn check_for_updates(&mut self) {
        let mut update_entries: HashSet<DatabaseEntry> = HashSet::new();

        for entry in self.entries.iter() {
            let commit_hash = git::get_latest_commit_hash(&entry.url);
            if commit_hash != entry.commit_hash {
                println!("{} has been updated!", entry.url);
                update_entries.insert(DatabaseEntry {
                    url: entry.url.clone(),
                    commit_hash: commit_hash,
                });
                break;
            }
        }

        for entry in update_entries.iter() {
            self.entries.remove(entry);
            self.add(&entry.url, &entry.commit_hash);
        }
    }
}
