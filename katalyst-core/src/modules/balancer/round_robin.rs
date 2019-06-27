use super::*;
use parking_lot::Mutex;

#[derive(Default, Debug)]
pub struct RoundRobinBalancerBuilder;

impl ModuleProvider for RoundRobinBalancerBuilder {
    fn name(&self) -> &'static str {
        "round_robin"
    }

    fn build(
        &self,
        _: ModuleType,
        _: Arc<Katalyst>,
        doc: &unstructured::Document,
    ) -> Result<Module> {
        let hosts: Vec<String> = doc["servers"].clone().try_into().unwrap_or_default();
        let mut arc_hosts = vec![];
        for new_host in hosts.iter() {
            arc_hosts.push(Arc::new(new_host.to_string()));
        }
        Ok(RoundRobinBalancer { hosts: arc_hosts, host_index: Mutex::new(0) }.into_module())
    }
}

#[derive(Default, Debug)]
pub struct RoundRobinBalancer {
    hosts: Vec<Arc<String>>,
    host_index: Mutex<usize>,
}

impl RoundRobinBalancer {
    fn get_next_index(&self) -> usize {
        let len = self.hosts.len();
        let mut index = self.host_index.lock();
        *index = (*index + 1) % len;
        *index
    }
}

impl LoadBalancerModule for RoundRobinBalancer {
    fn lease(&self) -> BalancerLease {
        Ok(self.hosts[self.get_next_index()].clone())
    }
}
