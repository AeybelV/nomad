# Nomad

A modular component-based flight-software platform for aeronautic robotics. Designed
originally for the [Laika Flight Computer](https://github.com/AeybelV/Laika-FC/)

***This project is very much in its infancy***

## Modules

The design of Nomad is inspired heavily by Nasa cFS.

- `nomad-core`: message bus, core service traits, component model and registration,
 events, system parameters.
- `nomad-osal`: OS Abstraction Layer. Tasks/threads, mutexes/locks,
time primitives. Wraps POSIX and RTOS (RTIC) APIS
- `nomad-hal`: Platform/Hardware abstraction layer. Board-/SoC-specific 
crates implement these (or wrap embedded-hal)
- `nomad-fsw`: Nomad Reference Flight Software

To utilize a nomad based FSW. You need `nomad-core`, `nomad-osal`, and a board crate that
imlements traits in `nomad-hal`. `nomad-fsw` is a reference fsw program with minimum
services and components. You can extend the reference FSW with your own components
or build your own FSW with nomad components.

## License

Copyright © 2025 Aeybel Varghese

Laika is open source and released under the Apache License Version 2.0
Contributions, bug reports, and feature requests are welcome.

See the [LICENSE](LICENSE) file for full details.
