//! Nomad Logging Macros
//!
//! Provides macros to interact with loggers

use crate::{LogHandle, LogLevel, component::ComponentId};

/// Wrapper used internally by logging macros.
/// Logs a message to the provided Logger
pub fn log_raw<L: LogHandle>(
    logger: &mut L,
    component: ComponentId,
    level: LogLevel,
    message: &'static str,
) {
    logger.log_message(component, level, message);
}

#[macro_export]
/// Logs at DEBUG level
macro_rules! log_debug {
    ($logger:expr, $comp:expr, $msg:expr) => {
        $crate::log_macros::log_raw($logger, $comp, $crate::LogLevel::Debug, $msg)
    };
}

#[macro_export]
/// Logs at INFO level
macro_rules! log_info {
    ($logger:expr, $comp:expr, $msg:expr) => {
        $crate::log_macros::log_raw($logger, $comp, $crate::LogLevel::Info, $msg)
    };
}

#[macro_export]
/// Logs at WARN level
macro_rules! log_warn {
    ($logger:expr, $comp:expr, $msg:expr) => {
        $crate::log_macros::log_raw($logger, $comp, $crate::LogLevel::Warn, $msg)
    };
}

#[macro_export]
/// Logs at ERROR level
macro_rules! log_error {
    ($logger:expr, $comp:expr, $msg:expr) => {
        $crate::log_macros::log_raw($logger, $comp, $crate::LogLevel::Error, $msg)
    };
}
