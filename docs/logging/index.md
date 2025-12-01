# Logging System Overview

Nomad provides a logging system that can be utilized by components.

The logging system is divided into three layers:

1. **Primitives (logging.rs)**  
   In-memory ring buffer, core types, traits. Can be used to implement custom loggers or sinks.

2. **LogService (components/log)**  
   A FSW service that is provided in the standard collection that implements a simple logger.

3. **Logging Macros**  
   Ergonomic wrappers that allow components to log without manually interacting with primitives or Loggers.
