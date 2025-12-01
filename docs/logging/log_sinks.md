# Log Sinks

A sink consumes log records.

The sink is where logs ultimately go for their final destination. Examples of sinks include:
- stdout (POSIX)
- RTT (baremetal)
- file (POSIX)
- UART (baremetal or POSIX)
- remote telemetry

Sinks implement the LogSink trait. Anything can implement a LogSink as a LogSink can be implemented at varying levels of abstraction. For example
`nomad-osal` provides a LogSink for stdout and file for POSIX systems to print logs to stdout or to a file.
`nomad-osal` also provides a LogSink for sending out logs onto RTT on baremetal. A LogSink can be implemeted
as a FSW component for more complex sinks, such as maybe broadcasting over LoRa and performing some packet encapsulation.

A Logger is responsible for handling LogSinks and routing log entires to the sink.
