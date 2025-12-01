//! Nomad Logging Module
//!
//! Provides a in-memory logging framework

/// Severity levels for logging.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

/// A singular log entry that is stored in the log ring buffer
#[derive(Copy, Clone, Debug)]
pub struct LogRecord {
    pub level: LogLevel,
    pub component: u32,
    pub message: &'static str,
}

/// In-memory ring buffer for log records
pub struct LogBuffer<const LOGGER_CAPACITY: usize> {
    records: [Option<LogRecord>; LOGGER_CAPACITY],
    head: usize,
    len: usize,
}

/// Logger Ringbuffer functions
impl<const LOGGER_CAPACITY: usize> LogBuffer<LOGGER_CAPACITY> {
    /// Instantiates a empty ring buffer of size LOGGER_CAPACITY
    pub fn new() -> Self {
        Self {
            records: [None; LOGGER_CAPACITY],
            head: 0,
            len: 0,
        }
    }

    /// Push a log record, overwriting the oldest when full.
    pub fn push(&mut self, rec: LogRecord) {
        let idx = (self.head + self.len) % LOGGER_CAPACITY;
        self.records[idx] = Some(rec);
        if self.len < LOGGER_CAPACITY {
            self.len += 1;
        } else {
            // buffer full, move head to drop oldest
            self.head = (self.head + 1) % LOGGER_CAPACITY;
        }
    }

    /// Iterate over records from oldest to newest.
    pub fn iter(&self) -> impl Iterator<Item = &LogRecord> {
        (0..self.len).filter_map(move |i| {
            let idx = (self.head + i) % LOGGER_CAPACITY;
            self.records[idx].as_ref()
        })
    }

    /// Clears the entire ring buffer
    pub fn clear(&mut self) {
        for r in self.records.iter_mut() {
            *r = None;
        }
        self.head = 0;
        self.len = 0;
    }
}

// ========== TESTS ==========

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function for extracting messages of LogRecord from a LogBuffer
    /// Collects the messages into a fixed size array
    fn collect_messages<const CAP: usize>(buf: &LogBuffer<CAP>) -> [Option<&'static str>; CAP] {
        let mut out: [Option<&'static str>; CAP] = [None; CAP];
        let mut idx = 0;

        for rec in buf.iter() {
            if idx < CAP {
                out[idx] = Some(rec.message);
                idx += 1;
            }
        }
        out
    }

    #[test]
    /// Tests whether a instantiated LogBuffer
    /// is truly empty.
    fn empty_buffer_iterates_zero_records() {
        const CAP: usize = 4;
        let buf: LogBuffer<CAP> = LogBuffer::new();
        let result = collect_messages(&buf);

        assert_eq!(result, [None, None, None, None]);
    }

    #[test]
    /// Pushes a single record to the log buffer
    fn single_push() {
        const CAP: usize = 4;
        let mut buf: LogBuffer<CAP> = LogBuffer::new();

        buf.push(LogRecord {
            level: LogLevel::Info,
            component: 0,
            message: "Hello World!",
        });

        let result = collect_messages(&buf);

        // Verifies the first record exists, and no more
        assert_eq!(result[0], Some("Hello World!"));
        for remaining in 1..CAP {
            assert_eq!(result[remaining], None);
        }
    }

    #[test]
    /// Pushes multiple records to the ring buffer
    fn multiple_push() {
        const CAP: usize = 4;
        let mut buf: LogBuffer<CAP> = LogBuffer::new();

        buf.push(LogRecord {
            level: LogLevel::Info,
            component: 0,
            message: "one",
        });
        buf.push(LogRecord {
            level: LogLevel::Info,
            component: 1,
            message: "two",
        });
        buf.push(LogRecord {
            level: LogLevel::Info,
            component: 2,
            message: "three",
        });

        let result = collect_messages(&buf);

        assert_eq!(result[0], Some("one"));
        assert_eq!(result[1], Some("two"));
        assert_eq!(result[2], Some("three"));
    }

    #[test]
    /// Pushes to the ringbuffer past the capacity. Oldest record should be
    /// overwritten.
    fn push_over_capacity() {
        const CAP: usize = 2;
        let mut buf: LogBuffer<CAP> = LogBuffer::new();

        buf.push(LogRecord {
            level: LogLevel::Info,
            component: 0,
            message: "one",
        });
        buf.push(LogRecord {
            level: LogLevel::Info,
            component: 0,
            message: "two",
        });
        buf.push(LogRecord {
            level: LogLevel::Info,
            component: 0,
            message: "three",
        });

        let result = collect_messages(&buf);

        assert_eq!(result[0], Some("two"));
        assert_eq!(result[1], Some("three"));
    }

    #[test]
    /// Clears the log
    fn clear_log() {
        const CAP: usize = 3;
        let mut buf: LogBuffer<CAP> = LogBuffer::new();

        buf.push(LogRecord {
            level: LogLevel::Info,
            component: 0,
            message: "one",
        });
        buf.push(LogRecord {
            level: LogLevel::Info,
            component: 0,
            message: "two",
        });

        buf.clear();

        let result = collect_messages(&buf);
        assert_eq!(result, [None, None, None]);

        // Once cleared, the buffer should be like new
        // and can push a record as if it was new
        buf.push(LogRecord {
            level: LogLevel::Warn,
            message: "after clear",
            component: 0,
        });

        let result2 = collect_messages(&buf);
        assert_eq!(result2[0], Some("after clear"));
    }

    #[test]
    // Pushes log entries from multipe components
    fn multiple_components() {
        const CAP: usize = 4;
        let mut buf: LogBuffer<CAP> = LogBuffer::new();

        buf.push(LogRecord {
            level: LogLevel::Info,
            component: 0,
            message: "fsw msg",
        });
        buf.push(LogRecord {
            level: LogLevel::Warn,
            component: 1,
            message: "imu warn",
        });
        buf.push(LogRecord {
            level: LogLevel::Error,
            component: 2,
            message: "nav error",
        });

        // Collect both message & component via match
        let mut comps: [Option<u32>; CAP] = [None; CAP];
        let mut idx = 0;

        for rec in buf.iter() {
            comps[idx] = Some(rec.component);
            idx += 1;
        }

        assert_eq!(comps[0], Some(0));
        assert_eq!(comps[1], Some(1));
        assert_eq!(comps[2], Some(2));
    }
}
