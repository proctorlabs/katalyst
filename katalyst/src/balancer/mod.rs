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
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

/// Result for acquiring a new load balancer lease.
pub type BalancerLease = Result<Arc<String>>;

/// Directory of all currently registered load balancer builders.
pub type BalancerDirectory = HashMap<&'static str, Arc<KatalystBalancerBuilder>>;

/// Result of building a new load balancer
pub type BalancerBuilderResult = Result<Arc<KatalystBalancer>>;

pub trait KatalystBalancerBuilder: Send + Sync + Debug {
    fn name(&self) -> &'static str;

    fn build(&self, hosts: Vec<String>) -> BalancerBuilderResult;
}

pub trait KatalystBalancer: Send + Sync + Debug {
    fn lease(&self) -> BalancerLease;
}

pub(crate) fn all() -> BalancerDirectory {
    let mut result: BalancerDirectory = HashMap::new();
    let mut balancers: Vec<Arc<KatalystBalancerBuilder>> = vec![
        round_robin::RoundRobinBalancerBuilder::arc(),
        random::RandomBalancerBuilder::arc(),
        least_connection::LeastConnectionBalancerBuilder::arc(),
    ];
    while let Some(balancer) = balancers.pop() {
        result.insert(balancer.name(), balancer);
    }
    result
}

pub(crate) fn default_balancer() -> Arc<KatalystBalancer> {
    Arc::new(round_robin::RoundRobinBalancer::default())
}
