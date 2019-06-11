use super::*;

#[derive(Debug)]
pub struct ModuleRegistry {
    modules: HashMap<String, Arc<dyn ModuleProvider>>,
}

impl ModuleRegistry {
    pub fn register(&mut self, module: Arc<dyn ModuleProvider>) {
        self.modules.insert(module.name().to_string(), module);
    }

    pub fn get(&self, name: &str) -> Result<Arc<dyn ModuleProvider>> {
        Ok(self
            .modules
            .get(name)
            .ok_or_else(|| {
                err!(RequiredComponent,
                format!("Required module {} not found!", name),
                name: name.to_string()
                )
            })?
            .clone())
    }
}

macro_rules! register_modules {
    ($($toreg:expr);*) => {
        impl Default for ModuleRegistry {
            fn default() -> Self {
                let mut result = ModuleRegistry {
                    modules: HashMap::default(),
                };
                $(
                    result.register(Arc::new($toreg));
                )*
                result
            }
        }
    };
}

register_modules! {
    handlers::FileServerModule;
    handlers::HostModule;
    authentication::AlwaysAuthenticator;
    authentication::NeverAuthenticator;
    authentication::HttpAuthenticatorBuilder;
    authentication::WhitelistBuilder;
    plugins::ContentPlugin;
    cache::DefaultCacheHandler;
    cache::MemoryCacheBuilder;
    balancer::LeastConnectionBalancerBuilder;
    balancer::RandomBalancerBuilder;
    balancer::RoundRobinBalancerBuilder
}
