# Time Primitives

Defines the foundational types and Traits that Nomad uses to reason about time.
These primitives are completely `no_std` and platform-agnostic.

## TimeMode

Represents whether time is operating in:

- `Real`  
  Time is derived from real hardware sources (clock, RTC, OS timers).

- `Simulated`  
  Time is injected externally (useful for SITL flight simulations, playback).

## TimeConfig

Provides configuration data that controls how mission time should be computed.

```rust
pub struct TimeConfig {
    pub mode: TimeMode,
    pub mission_epoch_unix: Option<u64>,
}
```

`mission_epoch_unix` if provided computes Mission Elapsed Time (MET) as `mission_time = now() - mission_epoch_unix`.
If If omitted, mission time defaults to the same zero point as monotonic time, therefore Mission time = elapsed time since FSW boot.

## TimeSource

This trait defines the required interface for any time backend.

```rust
pub trait TimeSource {
    fn monotonic(&self) -> Duration;
    fn mission_time(&self) -> Duration;
}
```

1. `monotonic`
  A strictly increasing clock used for:
  - scheduling
  - timers
  - timeout logic
  - performance measurement

2. `mission_time`
  Time since the mission epoch. Used for:
  - telemetry timestamps
  - log timestamps
  - Syncronization across subsystems
  - Allows mission timeline to exist independently of system uptime.

These two time domain exists for different use cases. The monotonic domain is sourced as a typically a hardware timer, used for scheduling/
Mission time is a derived value that could be a real/simulated value. Its useful for logs and telemetry.
