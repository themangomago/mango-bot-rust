struct DatabaseEntry {
    url: String,
    commit_hash: String,
}

struct Database {
    entries: HashSet<DatabaseEntry>,
}
