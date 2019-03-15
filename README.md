# Katalyst API Gateway

## THIS IS NOT READY FOR USE! This is currently experimental

Katalyst is a high performance and low memory API Gateway. It can be used as either an
appliance through Docker or it can be used as a rust library.

### Beta Requirements

As this is currently experimental and far from complete, here are some of the tasks that
require completion before being ready for production.

- ~~To downstream URL: Headers, Remote IP~~
- ~~Proxy forwarding headers~~
    - ~~Strip hop-to-hop headers~~
- Modular authentication
- Modular authorization
- ~~Benchmarking suite through docker compose~~
- ~~Appliance mode~~
    - ~~Specify configuration file on argv~~
    - ~~Specify configuration file with env~~
- ~~Post-error and post-success pipeline hooks working~~
- ~~SSL/TLS Client~~
- ~~Proper status code/error handling in pipeline~~

### Future Enhancements

- ~~Host grouping in configuration~~
- Load balancing
- Ability to modify downstream headers
- Cleaner upstream path definitions
- API for modifying routes after start
- INotify for config file changes
- Caching implementation (local, redis, etc.)
- Customizable pipeline for request processing
- External API hooks built into standard pipeline
- Server SSL/TLS termination
- Complex transforms: JSON Path, pseudo-methods
- FFI for other languages
- Request aggregation and chaining
- Request tracking
- QoS Options
- Circuit breaking
- Rate limiting
- Service discovery
- Health checks
- Websockets
