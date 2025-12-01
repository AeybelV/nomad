//! Nomad OSAL
//!
//! Provides OS abstractions

// TODO: Much of the stuff below is related to stdout and logging
// should be in their own files

use nomad_core::{LogLevel, LogRecord, LogSink};
use std::io::{self, Write};

#[cfg(feature = "console-stdout")]
/// Implement a LogSink for stdout on POSIX platforms
mod stdout_sink {
    use super::*;

    pub struct StdoutLogSink;

    impl StdoutLogSink {
        /// Given a LogLevel, returns the string represenation of the level
        fn format_level(&mut self, level: LogLevel) -> String {
            // Matches the level to string
            let base = match level {
                LogLevel::Error => "ERROR",
                LogLevel::Warn => "WARN",
                LogLevel::Info => "INFO",
                LogLevel::Debug => "DEBUG",
            };

            // If color codes are supported, return the level formatted in color codes
            #[cfg(feature = "stdout-color")]
            {
                match level {
                    LogLevel::Error => format!("\x1b[31m{}\x1b[0m", base), // red
                    LogLevel::Warn => format!("\x1b[33m{}\x1b[0m", base),  // yellow
                    LogLevel::Info => format!("\x1b[32m{}\x1b[0m", base),  // green
                    LogLevel::Debug => format!("\x1b[34m{}\x1b[0m", base), // blue
                }
            }

            // Otherwise return without color
            #[cfg(not(feature = "stdout-color"))]
            base.to_string()
        }
    }

    impl LogSink for StdoutLogSink {
        /// Takes a log records and prints it to stdout
        fn write(&mut self, record: &LogRecord) {
            let level_str = self.format_level(record.level);
            let output = format!(
                "[{}] comp={}: {}",
                level_str, record.component.0, record.message
            );

            match record.level {
                LogLevel::Error | LogLevel::Warn => {
                    // Write to stderr
                    let _ = writeln!(io::stderr(), "{}", output);
                }
                LogLevel::Info | LogLevel::Debug => {
                    // Write to stdout
                    let _ = writeln!(io::stdout(), "{}", output);
                }
            }
        }
    }

    // TODO: Remove in the future
    pub fn make_default_log_sink() -> StdoutLogSink {
        StdoutLogSink
    }
}

// TODO: Have a better way of handling default sinks
// It might be better to remove this and instead let the Logger
// such as LogService handle default LogSinks. OSAL shouldnt dicate
// default sinks, just provide some.
#[cfg(feature = "console-stdout")]
/// Specifies the stdout sink as the default LogSink
pub use stdout_sink::make_default_log_sink;
