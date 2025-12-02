# Time System Overview

The Time Subsystem in Doggyware provides platform-agnostic access to both
monotonic time and mission time.

The subsystem consists of:

1. **Time Primitives (time.rs)**
  Defines time behavior and time source abstraction layers.

2. **TimeService (components/time)**
   A FSW service that is provided in the standard collection that wraps and handles `TimeSource`s.
