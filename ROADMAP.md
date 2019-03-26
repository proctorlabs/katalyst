# Katalyst Roadmap

The Katalyst roadmap: milestones and goals

## Completed Milestones

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

## Alpha Milestones

- Modular authentication: API Completed, needs implementations for HTTP and secure sessions
- Modular authorization: Needs to be able to reject requests based off authenticator claims
- Register signals from OS
- Ability to modify downstream headers
- Cleaner upstream path definitions
- Server SSL/TLS termination
- Health checks
- Caching implementation (local, redis, etc.)
- Continuous integration completed
- Usable documentation

## Later Milestones

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
- API for modifying routes after start
