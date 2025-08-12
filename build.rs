use std::env;
use std::process::Command;

fn main() {
    // Git Commit Hash
    let git_hash = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "unknown".into());

    // Git Tag (falls vorhanden)
    let git_tag = Command::new("git")
        .args(["describe", "--tags", "--abbrev=0"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "no-tag".into());

    // Setze Umgebungsvariablen für den Build
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
    println!("cargo:rustc-env=GIT_TAG={}", git_tag);

    // Crate Version aus Cargo.toml
    let crate_version = env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "unknown".into());
    println!("cargo:rustc-env=CRATE_VERSION={}", crate_version);
}
