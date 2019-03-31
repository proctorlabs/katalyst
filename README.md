[![MIT License](https://img.shields.io/github/license/proctorlabs/katalyst.svg)](LICENSE)
[![Crate](https://img.shields.io/crates/v/katalyst.svg)](https://crates.io/crates/katalyst)
[![Build](https://img.shields.io/azure-devops/build/proctorlabs/katalyst/1.svg)](https://dev.azure.com/proctorlabs/katalyst/_build?definitionId=1)
[![Maintenance](https://img.shields.io/badge/maintenance-experimental-blue.svg)](https://crates.io/crates/katalyst)

# Katalyst API Gateway

**THIS IS NOT READY FOR USE! This is currently experimental**

## Documentation

[Complete Documentation can be found at docs.rs](https://docs.rs/katalyst/)

Katalyst is a high performance and low memory API Gateway. It can be used as either an
appliance through Docker or it can be used as a library.

## Features

Katalyst is still an experimental work in progress. Please see the [Roadmap](ROADMAP.md)
to see expected features.

Current features include:

* Simple YAML/JSON Gateway configuration
* Sophisticated regex routing
* API hooks for authentication modules
* Load balancing with Round Robin/Least Connection/Random algorithms
* Configurable service locator allowing for internal functionality to be overridden
* Flexible templating for value replacement in downstream requests