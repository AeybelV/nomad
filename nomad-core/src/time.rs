//! Nomad Time Subsystem
//!
//! Core time model provided by Nomad

use core::time::Duration;

/// Mission-relative, monotonic time.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct MissionTime(pub Duration);

/// TimeMode selects between HW time vs simulated time.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TimeMode {
    Real,
    Simulated,
}

#[derive(Copy, Clone, Debug)]
/// Configuration for the Time Subsystem
///
/// * `mode`: Time mode (Real time or simulated)
/// * `mission_epoch_unix`: Optional UNIX timestamp, represent T0 of the mission. Mission time
/// measured relative to it
pub struct TimeConfig {
    pub mode: TimeMode,
    pub mission_epoch_unix: Option<u64>,
}

/// A source of time that can provide monotonic and mission time. Monotonic time is
/// always increasing, used for timeouts, scheduling, etc
/// Mission time is time relative to mission epoch, used for logs and telemetry.
pub trait TimeSource {
    fn monotonic(&self) -> Duration;
    fn mission_time(&self) -> MissionTime;
}
