# Components

## Components

Nomad is a framework that allows you to develop your own FSW components for Nomad.

## Creating a Component

TODO

## Component Bus

TODO

### Standard Components

Nomad provides a standard collection of components in `nomad-core`

TODO: Maybe breakout to a separate page

## LogService

Core logging component. Nomad exposes a logging API, which allows you to
implement your own Logger, and allows you to use said Logger. LogService
is a component that implements a Logger as part of the standard collection.
The intention is to provide a simple logger which can be used for most purposes
just fine, but serve as a reference for implementing your own Logger if you wish.

It can manage the Nomad internal LogBuffer, handles formatting and routing of log messages.
It provides LogSinks management such as file logging, console logging, or broadcast based logging.

## TimeService

Provides:
- mission time
- monotonic time
- scheduling data
