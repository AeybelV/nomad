//! Nomad Core API
//!
//! Nomade Core API exposed for developing within the framework and custom components

#![no_std]

// ========== Nomad Core ==========
// Components Subsystem
pub mod component;

// Logging Subsystem
pub mod logger;
pub use logger::{LogBuffer, LogHandle, LogLevel, LogRecord, LogSink};
pub mod log_macros;
pub use log_macros::*;

// ========== Nomad Standard Component Collection =========
pub mod components;
