use std::{env, fs};
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
        .args(["describe", "--tags"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "no-tag".into());

    // Setze Umgebungsvariablen für den Build
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
    println!("cargo:rustc-env=GIT_TAG={}", git_tag);

    // Crate Version aus Cargo.toml
    // let crate_version = env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "unknown".into());
    // println!("cargo:rustc-env=CRATE_VERSION={}", crate_version);
    set_dependent_crate_versions();
}


fn  set_dependent_crate_versions() {
    // Find Cargo.lock
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let lock_path = format!("{}/Cargo.lock", manifest_dir);

    // Read Cargo.lock contents
    let cargo_lock = fs::read_to_string(&lock_path).expect("Could not read Cargo.lock");

    // Find dependency version
    for (dep_name, var_name) in [
        ("cb-simulator-yew", "DEP_CB_SIMULATOR_YEW_VERSION"),
        ("cb-simulation-util", "DEP_CB_SIMULATION_UTIL_VERSION"),
    ] {
        let version = cargo_lock
            .lines()
            .skip_while(|line| !line.trim().starts_with(&format!("name = \"{}\"", dep_name)))
            .skip(1) // next line should be version
            .next()
            .and_then(|line| line.trim().strip_prefix("version = \""))
            .and_then(|line| line.strip_suffix("\""))
            .unwrap_or("unknown");
        println!("cargo:rustc-env={}={}", var_name, version);
    }
}