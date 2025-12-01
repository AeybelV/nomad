//! Nomad FSW
//!
//! Reference Flight Software using Nomad Framework

/// TODO: Import if building for POSIX
use std::env;

// Nomad Core
use nomad_core::component::ComponentId;

// Standard Services
use nomad_core::components::log;

// Logging Macros
use nomad_core::{log_info, log_warn};
use nomad_osal::logsinks::make_default_log_sink;

/// FSW Entrypoint
fn main() {
    // TODO: Only print this if building for POSIX
    print_fsw_info();

    // Instantiates some Component ID's
    // TODO: This will all be handled by the component system when implemented
    const FSW_MAIN: ComponentId = ComponentId(0);
    const IMU: ComponentId = ComponentId(2);

    // Starts the LogService component
    // TODO: Startup the logging service using the component system
    let mut logging = log::LogService::<256>::new();

    // FSW components can use macros to log to a logger
    // TODO: When the component bus exists, the macro should be cleaner
    log_info!(&mut logging, FSW_MAIN, "Nomad FSW starting");
    log_info!(&mut logging, IMU, "IMU init sequence starting");
    log_warn!(&mut logging, IMU, "IMU using default calibration");

    // Drains/flushes logs
    // TODO: When timing and componnts works. LogService
    // should periodically drain logs rather than us manually doing it
    let mut sink = make_default_log_sink();
    logging.drain(&mut sink);
}

/// Prints build information about the FSW binary
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
    println!();
}
