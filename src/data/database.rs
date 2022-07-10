use std::collections::HashSet;

#[path = "../git/git.rs"]
mod git;

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct DatabaseEntry {
    pub url: String,
    pub commit_hash: String,
    pub channel_id: u64,
}

pub struct Database {
    entries: Vec<DatabaseEntry>,
}

impl Database {
    pub fn new() -> Database {
        Database {
            entries: Vec::new(),
        }
    }

    // Add a repo to the database.
    pub fn add(&mut self, url: &str, commit_hash: &str, channel_id: u64) {
        // inset data into entries
        self.entries.push(DatabaseEntry {
            url: url.to_string(),
            commit_hash: commit_hash.to_string(),
            channel_id,
        });
    }

    // Adds a new repo to the database.
    pub fn add_new(&mut self, url: &str, channel_id: u64) -> Result<String, String> {
        let commit_hash = git::get_latest_commit_hash(url);
        //TODO: error on no commit hash found
        self.add(url, &commit_hash, channel_id);
        Ok(commit_hash)
    }

    // remove a repo from the database.
    pub fn remove(&mut self, url: &str) -> Result<(), ()> {
        // remove entry from entries where url == url
        let size = self.entries.len();
        self.entries.retain(|entry| entry.url != url);
        if size > self.entries.len() {
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn list(&self, channel_id: u64) -> Vec<String> {
        self.entries
            .iter()
            .filter(|entry| entry.channel_id == channel_id)
            .map(|entry| format!("<{}> last commit: {}", entry.url, entry.commit_hash))
            .collect()
    }

    // Check if a repo had recent updates.
    pub fn check_for_updates(&mut self) -> Vec<DatabaseEntry> {
        let mut return_values: Vec<DatabaseEntry> = Vec::new();

        // Loop over entries, check for updates and update commit_hash if needed.
        for entry in &mut self.entries {
            let commit_hash = git::get_latest_commit_hash(&entry.url);
            if commit_hash != entry.commit_hash {
                entry.commit_hash = commit_hash;
                return_values.push(entry.clone());
            }
        }

        println!("Check for updates: {}", self.entries.len());
        for entry in self.entries.iter() {
            println!("db -> {} {}", entry.url, entry.commit_hash);
        }

        return return_values;
    }
}
