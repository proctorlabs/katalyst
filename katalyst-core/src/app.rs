use crate::{
    config::{parsers, Builder},
    instance::Instance,
    modules::ModuleRegistry,
    prelude::*,
    server::*,
};
use parking_lot::RwLock;
use signal_hook::{iterator::Signals, SIGINT, SIGQUIT, SIGTERM};
use std::{fmt, sync::Arc};
use tokio::runtime::Runtime;

struct KatalystCore {
    instance: RwLock<Arc<Instance>>,
    servers: RwLock<Vec<Server>>,
    client: ProxyClient,
    compiler: Arc<Compiler>,
    modules: ModuleRegistry,
    rt: RwLock<Runtime>,
}

impl std::fmt::Debug for KatalystCore {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "KatalystCore {{ instance: {:?} }}", self.instance)
    }
}

/// This is the core structure for the API Gateway.
#[derive(Debug, Clone)]
pub struct Katalyst {
    core: Arc<KatalystCore>,
}

impl Katalyst {
    /// Create a new Katalyst instance
    pub fn new() -> Result<Katalyst> {
        Ok(Katalyst {
            core: Arc::new(KatalystCore {
                instance: Default::default(),
                servers: Default::default(),
                client: Default::default(),
                compiler: Arc::new(Default::default()),
                modules: ModuleRegistry::default(),
                rt: RwLock::new(Runtime::new().unwrap()),
            }),
        })
    }

    /// Update the KatalystCore instance with the configuration from the specified file.
    pub fn load(&self, config_file: &str) -> Result<()> {
        let config = parsers::parse_file(config_file)?;
        self.update_instance(config.build(self.clone())?)?;
        Ok(())
    }

    /// Update the KatalystCore instance with the configuration from the provided YAML
    pub fn load_yaml(&self, raw: &str) -> Result<()> {
        let config = parsers::parse_yaml(raw)?;
        self.update_instance(config.build(self.clone())?)?;
        Ok(())
    }

    /// Update the KatalystCore instance with the configuration from the provided JSON
    pub fn load_json(&self, raw: &str) -> Result<()> {
        let config = parsers::parse_json(raw)?;
        self.update_instance(config.build(self.clone())?)?;
        Ok(())
    }

    /// Run the KatalystCore instance. This thread will block and run the async runtime.
    #[inline]
    pub fn run(&self) -> Result<()> {
        self.run_service()?;
        self.wait()?;
        Ok(())
    }

    /// Start the KatalystCore services
    pub fn run_service(&self) -> Result<()> {
        let instance = self.get_instance()?.clone();
        for interface in instance.service.interfaces.iter() {
            let server = Server::new(interface)?;
            server.spawn(self.clone())?;
            let mut servers = self.core.servers.write();
            servers.push(server);
        }
        Ok(())
    }

    /// Register OS signals and respond to them. This method will not return unless
    /// a SIGINT, SIGTERM, or SIGQUIT is received.
    pub fn wait(&self) -> Result<()> {
        let signals = Signals::new(&[SIGINT, SIGTERM, SIGQUIT])?;
        for sig in signals.forever() {
            match sig {
                SIGINT | SIGTERM | SIGQUIT => break,
                _ => (),
            };
        }
        info!("Signal received, shutting down...");
        Ok(())
    }

    /// This is a convenience method to start an instance of KatalystCore from a configuration file.
    /// This will load the configuration from the specified file and run the gateway until an OS
    /// signal is received.
    pub fn start(config_file: &str) -> Result<Self> {
        let app = Katalyst::new()?;
        app.load(config_file)?;
        app.run()?;
        Ok(app)
    }

    /// Update the running configuration of the API Gateway.
    pub fn update_instance(&self, new_instance: Instance) -> Result<()> {
        let mut instance = self.core.instance.write();
        *instance = Arc::new(new_instance);
        Ok(())
    }

    /// Get a copy of the currently running API Gateway configuration.
    pub fn get_instance(&self) -> Result<Arc<Instance>> {
        let instance = self.core.instance.read();
        Ok(instance.clone())
    }

    /// Spawn a future on the runtime backing KatalystCore
    pub fn spawn<F: Future<Item = (), Error = ()> + Send + 'static>(&self, fut: F) -> Result<()> {
        let mut rt = self.core.rt.write();
        rt.spawn(fut);
        Ok(())
    }

    #[inline]
    pub(crate) fn get_client(&self) -> ProxyClient {
        self.core.client.clone()
    }

    #[inline]
    pub(crate) fn get_compiler(&self) -> Arc<Compiler> {
        self.core.compiler.clone()
    }

    #[inline]
    pub(crate) fn get_module(&self, name: &str) -> Result<Arc<ModuleProvider>> {
        self.core.modules.get(name)
    }
}
