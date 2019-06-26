# Katalyst API Gateway

[![Crate 0.2.0](https://img.shields.io/crates/v/katalyst.svg)](https://crates.io/crates/katalyst)
[![Build Status](https://dev.azure.com/proctorlabs/katalyst/_apis/build/status/proctorlabs.katalyst?branchName=master&jobName=Job&configuration=stable)](https://dev.azure.com/proctorlabs/katalyst/_build/latest?definitionId=1&branchName=master)
[![Documentation](https://img.shields.io/badge/docs-current-important.svg)](https://docs.rs/katalyst/)
[![MIT License](https://img.shields.io/github/license/proctorlabs/katalyst.svg)](LICENSE)
[![Maintenance](https://img.shields.io/badge/maintenance-experimental-blue.svg)](https://crates.io/crates/katalyst)

Katalyst is a high performance and low memory API Gateway. It can be used as either an
appliance through Docker or it can be used as a library. This project is currently under
heavy development and will likely experience many changes and issues as we work towards the
1.0 release.

## Features

* Configuration via YAML files
* Configuration design done using templating and modular 'expressions' for dynamic route handling
* Request routing with either regex or custom route builders
* Modular design for customization of the gateway, internal modules can be overridden
* Load balance hosts using Round Robin, Least Connection, or Random algorithms
* SSL/TLS Termination
* Highly performant, with at least some use cases and loads outperforming nginx
    * Built on the tokio runtime with Hyper, leveraging async I/O where possible
* Does not require rust nightly, despite the heavy async I/O
* Usable as a rust library, standalone application, or lightweight docker container

## Library usage

For library usage, refer to the official [rust documentation](https://docs.rs/katalyst/).

## Install

Current installation of the binary requires Cargo, though other package formats may be coming soon.

```bash
# Add --force if you need to overwrite an already installed version
cargo install katalyst
```

## Usage

Once installed, starting Katalyst is easy. Use the -c option to specify the config file.
{{version}}

```bash
➤ katalyst -c config.yml
2019-06-25 19:44:03,103 INFO  [katalyst::config::parsers] Loading file from: config.yml
2019-06-25 19:44:03,105 INFO  [katalyst::server] Listening on http://0.0.0.0:8080
2019-06-25 19:44:03,105 INFO  [katalyst::server] Listening on https://0.0.0.0:8443
...
```

Run with the help command or flags to get all CLI options

```bash
➤ katalyst help
katalyst 0.2.0
Phil Proctor <philliptproctor@gmail.com>
High performance, modular API Gateway

USAGE:
    katalyst [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config <config>          Config file [default: katalyst.yaml]
    -l, --log-level <log-level>    Logging level to use [default: info]

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    run     Start the API Gateway (default)
```

## Configuration

Refer to the documentation [here](CONFIG)
