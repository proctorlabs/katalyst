<p align="center">

[![Crate](https://img.shields.io/crates/v/katalyst.svg)](https://crates.io/crates/katalyst)
[![Build Status](https://dev.azure.com/proctorlabs/katalyst/_apis/build/status/proctorlabs.katalyst?branchName=master&jobName=Job&configuration=stable)](https://dev.azure.com/proctorlabs/katalyst/_build/latest?definitionId=1&branchName=master)
[![Documentation](https://img.shields.io/badge/docs-current-important.svg)](https://docs.rs/katalyst/)
[![MIT License](https://img.shields.io/github/license/proctorlabs/katalyst.svg)](LICENSE)
[![Maintenance](https://img.shields.io/badge/maintenance-experimental-blue.svg)](https://crates.io/crates/katalyst)

</p>

# Katalyst API Gateway

**THIS IS NOT READY FOR USE! This is currently experimental**

## Documentation

[Complete Documentation can be found at docs.rs](https://docs.rs/katalyst/)

Katalyst is a high performance and low memory API Gateway. It can be used as either an
appliance through Docker or it can be used as a library.

## Features

Katalyst is still an experimental work in progress. Please see the [Features](FEATURES.md)
list to see expected features.

Current features include:

* Simple YAML/JSON Gateway configuration
* Sophisticated regex routing
* API hooks for authentication modules
* Load balancing with Round Robin/Least Connection/Random algorithms
* Configurable service locator allowing for internal functionality to be overridden
* Flexible templating for value replacement in downstream requests
