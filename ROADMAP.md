# Katalyst Roadmap

The Katalyst roadmap: milestones and goals

## Completed Features

- To downstream URL: Headers, Remote IP
- Proxy forwarding headers
    - Strip hop-to-hop headers
- Benchmarking suite through docker compose
- Appliance mode
    - Specify configuration file on argv
    - Specify configuration file with env
- Post-error and post-success pipeline hooks working
- SSL/TLS Client
- Proper status code/error handling in pipeline
- Host grouping in configuration
- Load balancing
- Ability to modify downstream headers
- Cleaner upstream path definitions
- Continuous integration completed
- Authentication modules
- Module system

## In progress

- Authentication modules: basic, login
- Authorization modules
- Register signals from OS
- Server SSL/TLS termination
- Health checks
- Caching implementation (local, redis, etc.)
- Usable documentation
- INotify for config file changes
- Customizable pipeline for request processing
- External API hooks built into standard pipeline
- Complex transforms: JSON Path, pseudo-methods
- FFI for other languages
- Request aggregation and chaining
- Request tracking
- QoS Options
- Circuit breaking
- Rate limiting
- Service discovery
- Websockets
