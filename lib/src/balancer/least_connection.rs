use super::*;

#[derive(Default, Debug)]
pub struct LeastConnectionBalancerBuilder {}

impl KatalystBalancerBuilder for LeastConnectionBalancerBuilder {
    fn name(&self) -> &'static str {
        "least_connection"
    }

    fn build(&self, hosts: Vec<String>) -> BalancerBuilderResult {
        let mut arc_hosts = vec![];
        for new_host in hosts.iter() {
            arc_hosts.push(Arc::new(new_host.to_string()));
        }
        Ok(Arc::new(LeastConnectionBalancer { hosts: arc_hosts }))
    }
}

#[derive(Debug)]
pub struct LeastConnectionBalancer {
    hosts: Vec<Arc<String>>,
}

impl KatalystBalancer for LeastConnectionBalancer {
    fn lease(&self) -> BalancerLease {
        let element = self.hosts.iter().fold(&self.hosts[0], |last, current| {
            if Arc::strong_count(current) > Arc::strong_count(last) {
                return current;
            }
            last
        });
        Ok(element.clone())
    }
}
