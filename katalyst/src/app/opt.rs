use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Katalyst", rename_all = "kebab_case")]
pub struct Opt {
    /// Config file
    #[structopt(short, long, help = "Config file", default_value = "katalyst.yaml")]
    pub config: String,

    /// Filter to apply to input files
    #[structopt(short, long, help = "Logging level to use", default_value = "info")]
    pub log_level: log::Level,

    /// The command to run
    #[structopt(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "command", rename_all = "kebab_case")]
pub enum Command {
    /// Start the API Gateway (default)
    Run,
}

impl Opt {
    pub fn new() -> Self {
        Opt::from_args()
    }
}
