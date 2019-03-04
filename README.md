# Katalyst API Gateway

## THIS IS NOT READY FOR USE! This is currently experimental

Katalyst is a high performance and low memory API Gateway. It can be used as either an
appliance through Docker or it can be used as a rust library.

### The TODO List

As this is currently experimental and far from complete, here are some of the tasks that
require completion before being ready for production.

- Additional downstream transforms
    - To downstream URL: Headers, Remote IP, JSON Path, pseudo-methods
    - Ability to modify downstream headers
- ~~Proxy forwarding headers~~
    - ~~Strip hop-to-hop headers~~
- Modular authentication
- Modular authorization
- ~~Benchmarking suite through docker compose~~
- Appliance mode
    - Specify configuration file on argv
    - Specify configuration file with env
- ~~Post-error and post-success pipeline hooks working~~
- Cleaner upstream path definitions
- SSL/TLS Client

### The WISH List

These are things not required to come out of experimental, but would be nice to have.

- FFI for other languages
- API for modifying routes after start
- INotify for config file changes
- Caching implementation, local or redis
- Customizable pipeline for request processing
- External API hooks built into standard pipeline
- SSL/TLS termination