//! Nomad OSAL Timesource
//!
//! Provides Wrappers for Time Sources provided by OS

use core::time::Duration;
use nomad_core::time::{TimeConfig, TimeSource};

#[cfg(feature = "posix")]
mod posix_time {

    use super::*;
    use std::time::{Instant, SystemTime, UNIX_EPOCH};

    /// POSIX Time Source using Instant + System Time
    ///
    /// * `start_instant`: Time reference for monotonic time, recorded and instantiation
    /// * `mission_instant`: Time reference relative to mission epoch.
    pub struct PosixTimeSource {
        start_instant: Instant,
        mission_instant: Instant,
    }

    impl PosixTimeSource {
        pub fn new(cfg: &TimeConfig) -> Self {
            let start_instant = Instant::now();

            // If mission epoch is provided in config, align mission_instant so that:
            // mission_time() = now_unix - mission_epoch.
            // Can now provide time relative since mission started using this
            let mission_instant = if let Some(mission_epoch_secs) = cfg.mission_epoch_unix {
                // Current time
                let now_sys = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or(Duration::from_secs(0));

                // Convert mission epoch to duration
                let mission_epoch = Duration::from_secs(mission_epoch_secs);

                // Calculate difference from current time to epoch
                let offset = now_sys
                    .checked_sub(mission_epoch)
                    .unwrap_or(Duration::from_secs(0));

                // Align that difference to time source start.
                start_instant.checked_sub(offset).unwrap_or(start_instant)
            } else {
                // If no epoch provided, just use start as mission base.
                start_instant
            };

            Self {
                start_instant,
                mission_instant,
            }
        }
    }

    impl TimeSource for PosixTimeSource {
        fn monotonic(&self) -> Duration {
            Instant::now()
                .checked_duration_since(self.start_instant)
                .unwrap_or(Duration::from_secs(0))
        }

        fn mission_time(&self) -> Duration {
            Instant::now()
                .checked_duration_since(self.mission_instant)
                .unwrap_or(Duration::from_secs(0))
        }
    }

    /// Factory for the default POSIX TimeSource.
    /// TODO: Have a better way of handling default time sources
    /// Have the TimeService handle defaults
    pub fn make_default_time_source(cfg: &TimeConfig) -> PosixTimeSource {
        PosixTimeSource::new(cfg)
    }
}

#[cfg(feature = "posix")]
/// TODO: Have a better way of handling default time sources
/// Have the TimeService handle defaults
pub use posix_time::make_default_time_source;
