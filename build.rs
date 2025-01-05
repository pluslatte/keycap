use std::{env, process::Command};
fn main() {
    let git_hash = env::var("GIT_HASH");
    if git_hash.map(|g| !g.is_empty()) == Ok(true) {
        return;
    }

    let out = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .expect("Failed to execute git command");
    let git_hash = String::from_utf8(out.stdout).unwrap();
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
}
