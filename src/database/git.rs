use std::process::Command;

pub fn get_latest_commit_hash(url: &str) -> String {
    let output = Command::new("git")
        .args(&["ls-remote", url])
        .output()
        .expect("Failed to fetch git remote");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut lines = stdout.lines();
    let line = lines.next().unwrap();
    let mut line_split = line.split("\t");
    let hash = line_split.next().unwrap();
    hash.to_string()
}
