# Discord Bot using Serenity Framework for Rust

Features:
- [x] Ping? Pong!
- [ ] Add repo to watchlist
- [ ] Scan watchlist for new commits, emit event when found
- [ ] Delete watchlist entry
- [ ] Add game jam note taking system
- [ ] Display game jam notes
- [ ] Clear game jam notes



Methods to get latest commit:
- Github webhooks
- Git clone repo
- Git remote ?

<!-- fn perform_git_clone(url: &str) {
    let _ = std::fs::remove_dir_all("temp");

    let _ = Command::new("git")
        .arg("clone")
        .arg("--depth=1")
        .arg("-n")
        .arg(url)
        .arg("temp")
        .output();

    let output = Command::new("git").arg("log").current_dir("temp").output();
    println!("{:?}", output);
} -->
