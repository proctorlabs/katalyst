use super::*;

#[derive(Default, Debug)]
pub struct LeastConnectionBalancerBuilder;

impl ModuleProvider for LeastConnectionBalancerBuilder {
    fn name(&self) -> &'static str {
        "least_connection"
    }

    fn build(&self, _: ModuleType, doc: &unstructured::Document) -> Result<Module> {
        let hosts: Vec<String> = doc["servers"].clone().try_into().unwrap_or_default();
        let mut arc_hosts = vec![];
        for new_host in hosts.iter() {
            arc_hosts.push(Arc::new(new_host.to_string()));
        }
        Ok(LeastConnectionBalancer { hosts: arc_hosts }.into_module())
    }
}

#[derive(Debug)]
pub struct LeastConnectionBalancer {
    hosts: Vec<Arc<String>>,
}

impl LoadBalancerModule for LeastConnectionBalancer {
    fn lease(&self) -> BalancerLease {
        let element = self.hosts.iter().fold(&self.hosts[0], |last, current| {
            if Arc::strong_count(current) < Arc::strong_count(last) {
                current
            } else {
                last
            }
        });
        Ok(Arc::clone(element))
    }
}
