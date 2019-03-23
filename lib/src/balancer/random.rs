use super::*;
use rand::Rng;

#[derive(Default, Debug)]
pub struct RandomBalancerBuilder {}

impl KatalystBalancerBuilder for RandomBalancerBuilder {
    fn name(&self) -> &'static str {
        "random"
    }

    fn build(&self, hosts: Vec<String>) -> BalancerBuilderResult {
        let mut arc_hosts = vec![];
        for new_host in hosts.iter() {
            arc_hosts.push(Arc::new(new_host.to_string()));
        }
        Ok(Arc::new(RandomBalancer { hosts: arc_hosts }))
    }
}

#[derive(Debug)]
pub struct RandomBalancer {
    hosts: Vec<Arc<String>>,
}

impl KatalystBalancer for RandomBalancer {
    fn lease(&self) -> BalancerLease {
        Ok(self.hosts[rand::thread_rng().gen_range(0, self.hosts.len())].clone())
    }
}
