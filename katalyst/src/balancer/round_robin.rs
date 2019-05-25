use super::*;
use std::sync::RwLock;

#[derive(Default, Debug)]
pub struct RoundRobinBalancerBuilder;

impl KatalystBalancerBuilder for RoundRobinBalancerBuilder {
    fn name(&self) -> &'static str {
        "round_robin"
    }

    fn build(&self, hosts: Vec<String>) -> BalancerBuilderResult {
        let mut arc_hosts = vec![];
        for new_host in hosts.iter() {
            arc_hosts.push(Arc::new(new_host.to_string()));
        }
        Ok(Arc::new(RoundRobinBalancer {
            hosts: arc_hosts,
            host_index: RwLock::new(0),
        }))
    }
}

#[derive(Default, Debug)]
pub struct RoundRobinBalancer {
    hosts: Vec<Arc<String>>,
    host_index: RwLock<usize>,
}

impl RoundRobinBalancer {
    fn get_next_index(&self) -> usize {
        let mut index = self.host_index.write().unwrap();
        *index += 1;
        *index %= self.hosts.len();
        *index
    }
}

impl KatalystBalancer for RoundRobinBalancer {
    fn lease(&self) -> BalancerLease {
        Ok(self.hosts[self.get_next_index()].clone())
    }
}
