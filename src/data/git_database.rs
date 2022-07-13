use std::{
    fs::File,
    io::{BufReader, BufWriter},
};

extern crate serde_json;
use serde::{Deserialize, Serialize};

#[path = "../git/git.rs"]
mod git;

#[derive(Eq, Hash, PartialEq, Clone, Serialize, Deserialize)]
pub struct GitDatabaseEntry {
    pub url: String,
    pub commit_hash: String,
    pub channel_id: u64,
}

pub struct GitDatabase {
    entries: Vec<GitDatabaseEntry>,
}

impl GitDatabase {
    pub fn new() -> GitDatabase {
        GitDatabase {
            entries: Vec::new(),
        }
    }

    pub fn load_or_create_db(&mut self) {
        let path = "database.json";
        if !std::path::Path::new(path).exists() {
            self.save();
        } else {
            self.load();
        }
    }

    // Add a repo to the database.
    pub fn add(&mut self, url: &str, commit_hash: &str, channel_id: u64) {
        // inset data into entries
        self.entries.push(GitDatabaseEntry {
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

        // Save db to file
        self.save();
        Ok(commit_hash)
    }

    // remove a repo from the database.
    pub fn remove(&mut self, url: &str) -> Result<(), ()> {
        // remove entry from entries where url == url
        let size = self.entries.len();

        //TODO: check also channel id - because of duplicate git urls from different channels
        self.entries.retain(|entry| entry.url != url);
        if size > self.entries.len() {
            // Save db to file
            self.save();
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
    pub fn check_for_updates(&mut self) -> Vec<GitDatabaseEntry> {
        let mut return_values: Vec<GitDatabaseEntry> = Vec::new();

        // Loop over entries, check for updates and update commit_hash if needed.
        for entry in &mut self.entries {
            let commit_hash = git::get_latest_commit_hash(&entry.url);
            if commit_hash != entry.commit_hash {
                entry.commit_hash = commit_hash;
                return_values.push(entry.clone());
            }
        }

        if return_values.len() > 0 {
            // Save db to file
            self.save();

            println!("Changes found: {}", return_values.len());
            for entry in return_values.iter() {
                println!("db -> {} {}", entry.url, entry.commit_hash);
            }
        }

        return return_values;
    }

    fn save(&self) {
        let file = File::create("database.json").unwrap();
        let mut writer = BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, &self.entries).unwrap();
    }

    fn load(&mut self) {
        let file = File::open("database.json").unwrap();
        let mut reader = BufReader::new(file);
        self.entries = serde_json::from_reader(&mut reader).unwrap();
    }
}
