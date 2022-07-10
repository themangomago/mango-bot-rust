use std::collections::HashSet;

#[path = "../git/git.rs"]
mod git;

#[derive(Eq, Hash, PartialEq)]
pub struct DatabaseEntry {
    pub url: String,
    pub commit_hash: String,
    pub channel_id: u64,
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

    // Add a repo to the database.
    pub fn add(&mut self, url: &str, commit_hash: &str, channel_id: u64) {
        self.entries.insert(DatabaseEntry {
            url: url.to_string(),
            commit_hash: commit_hash.to_string(),
            channel_id: channel_id,
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
    pub fn remove(&mut self, url: &str) {
        self.entries.remove(&DatabaseEntry {
            url: url.to_string(),
            commit_hash: "".to_string(),
            channel_id: 0,
        });
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
        let mut update_entries: HashSet<DatabaseEntry> = HashSet::new();
        let mut return_values: Vec<DatabaseEntry> = Vec::new();

        for entry in self.entries.iter() {
            let commit_hash = git::get_latest_commit_hash(&entry.url);
            if commit_hash != entry.commit_hash {
                println!("{} has been updated!", entry.url);
                update_entries.insert(DatabaseEntry {
                    url: entry.url.clone(),
                    commit_hash: commit_hash,
                    channel_id: entry.channel_id,
                });
            }
        }

        for entry in update_entries.iter() {
            let copy: DatabaseEntry = DatabaseEntry {
                url: entry.url.clone(),
                commit_hash: entry.commit_hash.clone(),
                channel_id: entry.channel_id,
            };
            return_values.push(copy);
            self.entries.remove(entry);
            self.add(&entry.url, &entry.commit_hash, entry.channel_id);
        }

        for entry in self.entries.iter() {
            println!("{} {}", entry.url, entry.commit_hash);
        }

        return return_values;
    }
}
