use std::convert::TryFrom;
use std::env::var;

use super::config_error::ConfigError;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    cycle_duration: i32,
    short_break: i32,
    long_break: i32,
}

impl Config {
    pub fn guess_path() -> Result<String, ConfigError> {
        match var("POMORS_CONFIG") {
            Ok(val) => Ok(val),
            Err(e) => {
                if let Some(mut path) = dirs::config_dir() {
                    path.push("pomors.toml");
                    Ok(String::from(path.to_str().unwrap()))
                } else {
                    Err(e.into())
                }
            },
        }
    }
}

impl TryFrom<&str> for Config {
    type Error = ConfigError;

    fn try_from(buffer: &str) -> Result<Self, Self::Error> {
        Ok(toml::from_str::<Config>(buffer)?)
    }
}

