use super::*;
use rand::Rng;

#[derive(Default, Debug)]
pub struct RandomBalancerBuilder;

impl ModuleProvider for RandomBalancerBuilder {
    fn name(&self) -> &'static str {
        "random"
    }

    fn build(
        &self,
        _: ModuleType,
        _: Arc<KatalystCore>,
        doc: &unstructured::Document,
    ) -> Result<Module> {
        let hosts: Vec<String> = doc["servers"].clone().try_into().unwrap_or_default();
        let mut arc_hosts = vec![];
        for new_host in hosts.iter() {
            arc_hosts.push(Arc::new(new_host.to_string()));
        }
        Ok(RandomBalancer { hosts: arc_hosts }.into_module())
    }
}

#[derive(Debug)]
pub struct RandomBalancer {
    hosts: Vec<Arc<String>>,
}

impl LoadBalancerModule for RandomBalancer {
    fn lease(&self) -> BalancerLease {
        Ok(self.hosts[rand::thread_rng().gen_range(0, self.hosts.len())].clone())
    }
}
