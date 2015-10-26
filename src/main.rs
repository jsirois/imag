#[macro_use] extern crate clap;
#[macro_use] extern crate log;
extern crate config;

use cli::CliConfig;
use configuration::Configuration;
use runtime::{ImagLogger, Runtime};
use clap::App;

mod cli;
mod configuration;
mod runtime;
mod module;
mod storage;

fn main() {
    let early_logger = ImagLogger::early().unwrap();
    let yaml = load_yaml!("../etc/cli.yml");
    let app = App::from_yaml(yaml);
    let mut config = CliConfig::new(app);
    let configuration = Configuration::new(&config);

    let logger = ImagLogger::init(&configuration, &config);
    let rt = Runtime::new(configuration, config);

    info!("Hello, world!");
}
