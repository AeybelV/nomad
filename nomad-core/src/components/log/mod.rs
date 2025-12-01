//! Nomad LogService
//!
//! A Logger service provided as part of the Nomad Standard Components Collection

use crate::{
    LogLevel,
    component::ComponentId,
    logger::{LogBuffer, LogHandle, LogRecord, LogSink},
};

/// Standard logging service provided by Nomad
pub struct LogService<const CAP: usize> {
    // NOTE: NOT THREADSAFE
    // TODO: Make threadsafe once threading and syncronization is provided
    buf: LogBuffer<CAP>,
}

impl<const CAP: usize> LogService<CAP> {
    pub fn new() -> Self {
        Self {
            buf: LogBuffer::new(),
        }
    }

    /// Drain all pending records into a single sink.
    pub fn drain<S: LogSink>(&mut self, sink: &mut S) {
        self.buf.drain(|rec| sink.write(rec));
    }

    /// Drain into multiple sinks (console + file etc).
    pub fn drain_multi<S: LogSink, const N: usize>(&mut self, sinks: &mut [S; N]) {
        self.buf.drain(|rec| {
            for s in sinks.iter_mut() {
                s.write(rec);
            }
        });
    }

    /// Check if there are logs pending.
    pub fn has_pending(&self) -> bool {
        !self.buf.is_empty()
    }

    /// Expose read-only view if you want, e.g. for telemetry
    pub fn snapshot<'a>(&'a self) -> impl Iterator<Item = &'a LogRecord> {
        self.buf.iter()
    }
}

// Implements the LogHandle trait to create a proper Logger
impl<const CAP: usize> LogHandle for LogService<CAP> {
    fn log_message(&mut self, component: ComponentId, level: LogLevel, message: &'static str) {
        // This simple logger will only publish the record to the ringbuffer and nothing more
        // TODO: When threading is implemented, implement another task/thread which drains the
        // buffer periodically
        self.buf.push(LogRecord {
            level,
            component,
            message,
        });
    }
}
