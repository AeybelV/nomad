# Logging Primitives (`logger.rs`)

This module contains the lowest-level logging structures.


## LogLevel

Logs can be performed at varying levels or severity.

- Debug
- Info
- Warn
- Error

## LogRecord

A minimal representation of a log entry that consists of

- log level
- component ID
- static message string

Timestamps will be added in a later phase.

## LogBuffer

A fixed-size ring buffer that:
- stores LogRecords,
- overwrites old entries when full,
- supports iteration,
- supports draining (removing entries while processing them).

The LogBuffer is strictly in-memory, does not allocate, does not depend on OSAL. When implementing a Logger
you are writing something that ultimately will be managing a LogBuffer(s).

## LogSink trait

Represents the final destination for log records.

Examples:
- stdout sink (POSIX)
- RTT sink (baremetal)
- file sink (POSIX)
- radio telemetry sink (future)

Users can implement this trait to create custom logging backends. A Logger is responsible for
draining LogRecords from a LogBuffer into a LogSink

## LogHandle trait

Represents something capable of receiving log requests from components. Loggers are implemented
by implementing this trait. At a minimum, a Logger will take log requests from components and
put them onto a LogBuffer for intermediate storage, and at sometime drain the entries into a Sink.

LogService is a component part of the standard collection that implements LogHandle. It can 
manage a LogBuffer and multiple sinks, and route log requests to them.
