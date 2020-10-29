pub mod config;

use std::error::Error;
use std::convert::TryFrom;
use config::Config;
use log::{info, debug};

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let config_path = Config::guess_path()?;

    info!("Loading settings from configuration file");
    let file_content = std::fs::read_to_string(config_path)?;

    let config = Config::try_from(&file_content[..])?;
    debug!("Configuration {:#?}", config);


    Ok(())
}
