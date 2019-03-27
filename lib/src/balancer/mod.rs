mod least_connection;
mod random;
mod round_robin;

use crate::common::KatalystCommonUtilities;
use crate::error::KatalystError;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

pub type BalancerLease = Result<Arc<String>, KatalystError>;
pub type BalancerDirectory = HashMap<&'static str, Arc<KatalystBalancerBuilder>>;
pub type BalancerBuilderResult = Result<Arc<KatalystBalancer>, KatalystError>;

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
