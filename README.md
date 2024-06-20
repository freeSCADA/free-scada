# Free SCADA: A general-porpoise SCADA solution

<!--toc:start-->
- [Free SCADA: A general-porpoise SCADA solution](#free-scada-a-general-porpoise-scada-solution)
  - [Features](#features)
    - [Data Acquisition: Collection-first model](#data-acquisition-collection-first-model)
    - [Control and Supervision: Reliability-first model](#control-and-supervision-reliability-first-model)
    - [Compatibility: Plugin-driven architecture](#compatibility-plugin-driven-architecture)
    - [License: FREE](#license-free)
  - [Installation](#installation)
  - [Contributing](#contributing)
<!--toc:end-->

Free SCADA is a free, open source SCADA system for everything you
need, from smart houses, to country-wide power systems.

It is built entirely in Rust to ensure safety, reliability and
performance.

Everything is...

For free SCADA development!

## Features

### Data Acquisition: Collection-first model

The data is collected on real time and saved to the database
first, even before the user can see it. This allows Free SCADA
to be flexible, because we converted the difficult problem of
serving directly all the events and data to the clients, into
a typical CRUD application, where each client requests only
what they need.

### Control and Supervision: Reliability-first model

Free SCADA stores the logs of every command on the database to
make sure everything's going correctly. In case something fails,
you can get the logs from the database and know exactly what went wrong
with the command execution.

### Compatibility: Plugin-driven architecture

Free SCADA uses a plugin-driven system to achieve a wide compatibility
with any kind of device. It relies on open protocols and standards by
default, but you can extend this behavior with plugins and configuring
the system properly to accept the new protocol.

### License: FREE

Free SCADA is distributed under the Apache License 2.0, so you are
free to fork this project and mantain your own variant to make Free SCADA
fit your needs. We also aim to become a standard on the SCADA market,
and being open is the most important part to reach that goal.

## Installation

For now, it is impossible to install Free SCADA. Installable binaries will
be released as soon as possible.

## Contributing

If you wish to contribute to this project, feel free to send either a pull
request or a merge request. Any changes to the code are welcome.
