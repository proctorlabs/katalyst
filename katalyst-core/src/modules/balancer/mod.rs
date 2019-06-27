/*!
This module provides all of the built in load balancers for Katalyst.

There are three major load balancer types for Katalyst:

- **least_connection**: Route requests to the service with the least amount of connections currently leased
- **random**: Simply route requests to hosts at random
- **round_robin**: Route one request to one host at a time

For most cases, least_connection is the preferred balancer type.
*/

mod least_connection;
mod random;
mod round_robin;

use crate::prelude::*;
use std::sync::Arc;

pub use least_connection::LeastConnectionBalancerBuilder;
pub use random::RandomBalancerBuilder;
pub use round_robin::RoundRobinBalancerBuilder;

pub(crate) fn default_balancer() -> Arc<LoadBalancerModule> {
    Arc::new(round_robin::RoundRobinBalancer::default())
}
