# LogService

LogService is a standard FSW component provided by the standard collection.

It is a Logger that implements the LogHandle trait. It manages a LogBuffer and can
route log requests from components to the LogBuffer or to sinks. The service can 
drain log entries periodically into one or more LogSinks

It can receive commands and messages over the component bus, or you can use the log macros
to interact in a cleaner way

While anyone can implement their own logger, LogService is a already implemented Logger
that suits most needs. It already does handling of the LogBuffer and various sinks.


