use chrono::Utc;
use std::process::Command;

fn main() {
    let git_hash = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let git_branch = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let release_name =
        std::env::var("NOMAD_RELEASE_NAME").unwrap_or_else(|_| "development".to_string());

    let build_time = Utc::now().to_rfc3339();

    let profile = std::env::var("PROFILE").unwrap_or_else(|_| "unknown".to_string());
    let target = std::env::var("TARGET").unwrap_or_else(|_| "unknown".to_string());

    println!("cargo:rustc-env=NOMAD_GIT_HASH={}", git_hash);
    println!("cargo:rustc-env=NOMAD_GIT_BRANCH={}", git_branch);
    println!("cargo:rustc-env=NOMAD_RELEASE_NAME={}", release_name);
    println!("cargo:rustc-env=NOMAD_BUILD_TIME={}", build_time);
    println!("cargo:rustc-env=NOMAD_BUILD_PROFILE={}", profile);
    println!("cargo:rustc-env=NOMAD_BUILD_TARGET={}", target);

    // Re-run build script when HEAD changes.
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs/heads");
}
