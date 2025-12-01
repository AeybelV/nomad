# Logging Macros

For the sake of convenience. Macros are provided to create LogRecords and send them to a Logger.

The macro constructs the LogRecord and forward the request to a Logger (usually LogService).

For most purposes, this should be the intended way to perform logging and interacting with the logging system
in nomad from other components.
