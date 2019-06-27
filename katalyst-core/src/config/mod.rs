/*!
Configuration of Katalyst is available in YAML and JSON formats. When running katalyst as a binary,
the configuration is generally loaded from a file specified with the -c option.

# Configuration Format

Although Katalyst supports both JSON and YAML formats, YAML is generally preferred and works better
with the expression syntax in the configuration files. As such, all examples here will be in the YAML
format.

A basic config could look something like this:

```yaml
service:
  interfaces:
    - address: "0.0.0.0:8080"
  cache:
    type: memory_cache

hosts:
  httpbin:
    type: round_robin
    servers:
      - "http://httpbin.org"

routes:
  - path:
      type: regex
      pattern: "^/$"
    handler:
      type: host
      host: httpbin
      path: "/ip"
    cache:
      type: cache_response

```

This example configuration is for a server that listens on port 8080, has one downstream host group
configured with one server (httpbin.org), and has one route configured that will match requests to / and
route them to http://httpbin.org/ip

# Expressions

Many configuration sections support configuration through "expressions". These expressions allow adding
custom logic and dynamic behavior within the configuration itself. In addition, the expression syntax allows
nesting expressions such that the result of one expression (which itself may have nested expressions) can be used
by another expression.

As an example, if you needed to add the base64 encoded remote IP address to a query parameter of a downstream host,
this could be accomplished like so:

```yaml
    handler:
      type: host
      host: downstream_host_group
      path: "/get"
      query:
        encoded_ip: "{{ encode.base64(http.ip()) }}"
```

While this specific example is a bit contived, this flexibility allows you to do a number of things based off of
the state of the incoming request.

TODO: Document the built in expressions

# Modules

Throughout the configuration, custom modules can be used with specialized configuration. The only universal
configuration option for a module is the 'type' field, all other fields are determined by the module itself.
When a module section of the configuration is parsed, that section of the configuration is kept as an
`unstructured::Document` so that the module can define as simple or complex of a configuration as required.
Documentation for the individual modules should contain information about configuration specific to those modules.

# Configuration Sections

As demonstrated in the basic configuration above, these configuration sections are available:

* **service**: Global service options such as listening addresses, cache store, etc.
* **hosts**: This defines "host groups" which are simply groups of servers that can be referred to by name and are load balanced
* **routes**: The list of routes that exist on this server. Incoming requests are matched to a route by a pattern and sent to a handler.

# Service Configuration Options

TODO: **WIP**

## interfaces

```yaml
  interfaces:
    # HTTP Configuration
    - address: "0.0.0.0:8080"
    # HTTPS Configuration
    - address: "0.0.0.0:8443"
      ssl: true
      ssl_cert: "cert.crt"
      ssl_key: "key.rsa"
```

## cache

```yaml
  cache: # This is a 'cache' module
    type: memory_cache
```

# Host Configuration Options

TODO: **WIP**

```yaml
hosts:
  httpbin: # This is a 'load balancer' module
    type: round_robin
    servers:
      - "http://httpbin.org"
```

# Route Configuration Options

TODO: **WIP**

```yaml
routes:
  - path:
      type: regex
      pattern: "^/my/profile$"
    handler: # this is a 'handler' module
      type: host
      host: host_group_name
      path: "/user/{{ auth.claim('userid') }}/profile"
    cache: # this is a 'cache' module
      type: cache_response
    # Other module types currently supported here: authenticator, authorizer, plugin
```
*/

mod builder;
pub(crate) mod parsers;

pub use builder::*;
