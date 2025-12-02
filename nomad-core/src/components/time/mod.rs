//! Nomad TimeService
//!
//! A Logger service provided as part of the Nomad Standard Components Collection

use crate::time::TimeSource;
use core::time::Duration;

/// Standard TimeService FSW Components
///
/// Wraps a TimeSource and provides a API to interact with it.
/// At the moment allows other components to query time, in the future
/// this could be upgraded to sync with GNSS/external time, distribute time events
/// on the component bus, provide scheduling helpers.
/// * `source`:
pub struct TimeService<T: TimeSource> {
    source: T,
}

impl<T: TimeSource> TimeService<T> {
    pub fn new(source: T) -> Self {
        Self { source }
    }

    /// Monotonic time since boot/start.
    pub fn monotonic(&self) -> Duration {
        self.source.monotonic()
    }

    /// Mission elapsed time (MET) since mission epoch.
    pub fn mission_time(&self) -> Duration {
        self.source.mission_time()
    }

    /// Borrow the underlying source if you need additional behavior.
    pub fn inner(&self) -> &T {
        &self.source
    }
}
