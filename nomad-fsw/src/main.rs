use std::env;

fn main() {
    print_fsw_info();
}

fn print_fsw_info() {
    let pkg_name = env!("CARGO_PKG_NAME");
    let pkg_version = env!("CARGO_PKG_VERSION");
    let pkg_description = option_env!("CARGO_PKG_DESCRIPTION").unwrap_or("");

    let git_hash = option_env!("NOMAD_GIT_HASH").unwrap_or("unknown");
    let git_branch = option_env!("NOMAD_GIT_BRANCH").unwrap_or("unknown");
    let release_name = option_env!("NOMAD_RELEASE_NAME").unwrap_or("development");
    let build_time = option_env!("NOMAD_BUILD_TIME").unwrap_or("unknown");
    let build_profile = option_env!("NOMAD_BUILD_PROFILE").unwrap_or("unknown");
    let build_target = option_env!("NOMAD_BUILD_TARGET").unwrap_or("unknown");

    println!("\n{pkg_name}");
    if !pkg_description.is_empty() {
        println!("{pkg_description}");
    }
    println!();

    println!("Version       : {pkg_version}");
    println!("Release name  : {release_name}");
    println!("Git branch    : {git_branch}");
    println!("Git commit    : {git_hash}");
    println!("Build time    : {build_time}");
    println!("Build profile : {build_profile}");
    println!("Build target  : {build_target}");
}
